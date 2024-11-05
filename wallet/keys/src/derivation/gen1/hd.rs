use crate::derivation::traits::*;
use crate::imports::*;
use calico_addresses::{Address, Prefix as AddressPrefix, Version as AddressVersion};
use calico_bip32::types::{ChainCode, HmacSha512, KeyFingerprint, PublicKeyBytes, KEY_SIZE};
use calico_bip32::{
    AddressType, ChildNumber, DerivationPath, ExtendedKey, ExtendedKeyAttrs, ExtendedPrivateKey, ExtendedPublicKey, Prefix,
    PrivateKey, PublicKey, SecretKey, SecretKeyExt,
};
use hmac::Mac;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::fmt::Debug;
// use wasm_bindgen::prelude::*;

fn get_fingerprint<K>(private_key: &K) -> KeyFingerprint
where
    K: PrivateKey,
{
    let public_key_bytes = private_key.public_key().to_bytes();

    let digest = Ripemd160::digest(Sha256::digest(public_key_bytes));
    digest[..4].try_into().expect("digest truncated")
}

#[derive(Clone)]
// #[wasm_bindgen(inspectable)]
pub struct PubkeyDerivationManager {
    /// Derived public key
    public_key: secp256k1::PublicKey,
    /// Extended key attributes.
    attrs: ExtendedKeyAttrs,
    #[allow(dead_code)]
    fingerprint: KeyFingerprint,
    hmac: HmacSha512,
    index: Arc<Mutex<u32>>,
}

impl PubkeyDerivationManager {
    pub fn new(
        public_key: secp256k1::PublicKey,
        attrs: ExtendedKeyAttrs,
        fingerprint: KeyFingerprint,
        hmac: HmacSha512,
        index: u32,
    ) -> Result<Self> {
        let wallet = Self { public_key, attrs, fingerprint, hmac, index: Arc::new(Mutex::new(index)) };

        Ok(wallet)
    }

    pub fn derive_pubkey_range(&self, indexes: std::ops::Range<u32>) -> Result<Vec<secp256k1::PublicKey>> {
        let list = indexes.map(|index| self.derive_pubkey(index)).collect::<Vec<_>>();
        let keys = list.into_iter().collect::<Result<Vec<_>>>()?;
        Ok(keys)
    }

    pub fn derive_pubkey(&self, index: u32) -> Result<secp256k1::PublicKey> {
        let (key, _chain_code) = WalletDerivationManager::derive_public_key_child(&self.public_key, index, self.hmac.clone())?;
        Ok(key)
    }

    pub fn create_address(key: &secp256k1::PublicKey, prefix: AddressPrefix, ecdsa: bool) -> Result<Address> {
        let address = if ecdsa {
            let payload = &key.serialize();
            Address::new(prefix, AddressVersion::PubKeyECDSA, payload)
        } else {
            let payload = &key.x_only_public_key().0.serialize();
            Address::new(prefix, AddressVersion::PubKey, payload)
        };

        Ok(address)
    }

    pub fn public_key(&self) -> ExtendedPublicKey<secp256k1::PublicKey> {
        self.into()
    }

    pub fn attrs(&self) -> &ExtendedKeyAttrs {
        &self.attrs
    }

    /// Serialize the raw public key as a byte array.
    pub fn to_bytes(&self) -> PublicKeyBytes {
        self.public_key().to_bytes()
    }

    /// Serialize this key as an [`ExtendedKey`].
    pub fn to_extended_key(&self, prefix: Prefix) -> ExtendedKey {
        let mut key_bytes = [0u8; KEY_SIZE + 1];
        key_bytes[..].copy_from_slice(&self.to_bytes());
        ExtendedKey { prefix, attrs: self.attrs.clone(), key_bytes }
    }

    pub fn to_string(&self) -> Zeroizing<String> {
        Zeroizing::new(self.to_extended_key(Prefix::KPUB).to_string())
    }
}

// #[wasm_bindgen]
impl PubkeyDerivationManager {
    // #[wasm_bindgen(getter, js_name = publicKey)]
    pub fn get_public_key(&self) -> String {
        self.public_key().to_string(None)
    }
}

impl From<&PubkeyDerivationManager> for ExtendedPublicKey<secp256k1::PublicKey> {
    fn from(inner: &PubkeyDerivationManager) -> ExtendedPublicKey<secp256k1::PublicKey> {
        ExtendedPublicKey { public_key: inner.public_key, attrs: inner.attrs().clone() }
    }
}

#[async_trait]
impl PubkeyDerivationManagerTrait for PubkeyDerivationManager {
    fn new_pubkey(&self) -> Result<secp256k1::PublicKey> {
        self.set_index(self.index()? + 1)?;
        self.current_pubkey()
    }

    fn index(&self) -> Result<u32> {
        Ok(*self.index.lock()?)
    }

    fn set_index(&self, index: u32) -> Result<()> {
        *self.index.lock()? = index;
        Ok(())
    }

    fn current_pubkey(&self) -> Result<secp256k1::PublicKey> {
        let index = self.index()?;
        let key = self.derive_pubkey(index)?;

        Ok(key)
    }

    fn get_range(&self, range: std::ops::Range<u32>) -> Result<Vec<secp256k1::PublicKey>> {
        self.derive_pubkey_range(range)
    }
}

#[derive(Clone)]
pub struct WalletDerivationManager {
    /// extended public key derived upto `m/<Purpose>'/123579'/<Account Index>'`
    extended_public_key: ExtendedPublicKey<secp256k1::PublicKey>,

    /// receive address wallet
    receive_pubkey_manager: Arc<PubkeyDerivationManager>,

    /// change address wallet
    change_pubkey_manager: Arc<PubkeyDerivationManager>,
}

impl WalletDerivationManager {
    pub fn create_extended_key_from_xprv(xprv: &str, is_multisig: bool, account_index: u64) -> Result<(SecretKey, ExtendedKeyAttrs)> {
        let xprv_key = ExtendedPrivateKey::<SecretKey>::from_str(xprv)?;
        Self::derive_extended_key_from_master_key(xprv_key, is_multisig, account_index)
    }

    pub fn derive_extended_key_from_master_key(
        xprv_key: ExtendedPrivateKey<SecretKey>,
        is_multisig: bool,
        account_index: u64,
    ) -> Result<(SecretKey, ExtendedKeyAttrs)> {
        let attrs = xprv_key.attrs();

        let (extended_private_key, attrs) =
            Self::create_extended_key(*xprv_key.private_key(), attrs.clone(), is_multisig, account_index)?;

        Ok((extended_private_key, attrs))
    }

    fn create_extended_key(
        mut private_key: SecretKey,
        mut attrs: ExtendedKeyAttrs,
        is_multisig: bool,
        account_index: u64,
    ) -> Result<(SecretKey, ExtendedKeyAttrs)> {
        let purpose = if is_multisig { 45 } else { 44 };
        let address_path = format!("{purpose}'/123579'/{account_index}'");
        let children = address_path.split('/');
        for child in children {
            (private_key, attrs) = Self::derive_private_key(&private_key, &attrs, child.parse::<ChildNumber>()?)?;
        }

        Ok((private_key, attrs))
    }

    pub fn build_derivate_path(
        is_multisig: bool,
        account_index: u64,
        cosigner_index: Option<u32>,
        address_type: Option<AddressType>,
    ) -> Result<DerivationPath> {
        if is_multisig && cosigner_index.is_none() {
            return Err("cosigner_index is required for multisig path derivation".to_string().into());
        }
        let purpose = if is_multisig { 45 } else { 44 };
        let mut path = format!("m/{purpose}'/123579'/{account_index}'");
        if let Some(cosigner_index) = cosigner_index {
            path = format!("{path}/{}", cosigner_index)
        }
        if let Some(address_type) = address_type {
            path = format!("{path}/{}", address_type.index());
        }
        let path = path.parse::<DerivationPath>()?;
        Ok(path)
    }

    pub fn receive_pubkey_manager(&self) -> &PubkeyDerivationManager {
        &self.receive_pubkey_manager
    }
    pub fn change_pubkey_manager(&self) -> &PubkeyDerivationManager {
        &self.change_pubkey_manager
    }

    pub fn derive_child_pubkey_manager(
        mut public_key: ExtendedPublicKey<secp256k1::PublicKey>,
        address_type: AddressType,
        cosigner_index: Option<u32>,
    ) -> Result<PubkeyDerivationManager> {
        if let Some(cosigner_index) = cosigner_index {
            public_key = public_key.derive_child(ChildNumber::new(cosigner_index, false)?)?;
        }

        public_key = public_key.derive_child(ChildNumber::new(address_type.index(), false)?)?;

        let mut hmac = HmacSha512::new_from_slice(&public_key.attrs().chain_code).map_err(calico_bip32::Error::Hmac)?;
        hmac.update(&public_key.to_bytes());

        PubkeyDerivationManager::new(*public_key.public_key(), public_key.attrs().clone(), public_key.fingerprint(), hmac, 0)
    }

    pub fn derive_public_key(
        public_key: &secp256k1::PublicKey,
        attrs: &ExtendedKeyAttrs,
        index: u32,
    ) -> Result<(secp256k1::PublicKey, ExtendedKeyAttrs)> {
        let fingerprint = public_key.fingerprint();

        let mut hmac = HmacSha512::new_from_slice(&attrs.chain_code).map_err(calico_bip32::Error::Hmac)?;
        hmac.update(&public_key.to_bytes());

        let (key, chain_code) = Self::derive_public_key_child(public_key, index, hmac)?;

        let depth = attrs.depth.checked_add(1).ok_or(calico_bip32::Error::Depth)?;

        let attrs =
            ExtendedKeyAttrs { parent_fingerprint: fingerprint, child_number: ChildNumber::new(index, false)?, chain_code, depth };

        Ok((key, attrs))
    }

    fn derive_public_key_child(
        key: &secp256k1::PublicKey,
        index: u32,
        mut hmac: HmacSha512,
    ) -> Result<(secp256k1::PublicKey, ChainCode)> {
        let child_number = ChildNumber::new(index, false)?;
        hmac.update(&child_number.to_bytes());

        let result = hmac.finalize().into_bytes();
        let (child_key, chain_code) = result.split_at(KEY_SIZE);

        // We should technically loop here if a `secret_key` is zero or overflows
        // the order of the underlying elliptic curve group, incrementing the
        // index, however per "Child key derivation (CKD) functions":
        // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#child-key-derivation-ckd-functions
        //
        // > "Note: this has probability lower than 1 in 2^127."
        //
        // ...so instead, we simply return an error if this were ever to happen,
        // as the chances of it happening are vanishingly small.
        let key = key.derive_child(child_key.try_into()?)?;

        Ok((key, chain_code.try_into()?))
    }

    pub fn derive_private_key(
        private_key: &SecretKey,
        attrs: &ExtendedKeyAttrs,
        child_number: ChildNumber,
    ) -> Result<(SecretKey, ExtendedKeyAttrs)> {
        let fingerprint = get_fingerprint(private_key);

        let hmac = Self::create_hmac(private_key, attrs, child_number.is_hardened())?;

        let (private_key, chain_code) = Self::derive_key(private_key, child_number, hmac)?;

        let depth = attrs.depth.checked_add(1).ok_or(calico_bip32::Error::Depth)?;

        let attrs = ExtendedKeyAttrs { parent_fingerprint: fingerprint, child_number, chain_code, depth };

        Ok((private_key, attrs))
    }

    fn derive_key(private_key: &SecretKey, child_number: ChildNumber, mut hmac: HmacSha512) -> Result<(SecretKey, ChainCode)> {
        hmac.update(&child_number.to_bytes());

        let result = hmac.finalize().into_bytes();
        let (child_key, chain_code) = result.split_at(KEY_SIZE);

        // We should technically loop here if a `secret_key` is zero or overflows
        // the order of the underlying elliptic curve group, incrementing the
        // index, however per "Child key derivation (CKD) functions":
        // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#child-key-derivation-ckd-functions
        //
        // > "Note: this has probability lower than 1 in 2^127."
        //
        // ...so instead, we simply return an error if this were ever to happen,
        // as the chances of it happening are vanishingly small.
        let private_key = private_key.derive_child(child_key.try_into()?)?;

        Ok((private_key, chain_code.try_into()?))
    }

    pub fn create_hmac<K>(private_key: &K, attrs: &ExtendedKeyAttrs, hardened: bool) -> Result<HmacSha512>
    where
        K: PrivateKey<PublicKey = secp256k1::PublicKey>,
    {
        let mut hmac = HmacSha512::new_from_slice(&attrs.chain_code).map_err(calico_bip32::Error::Hmac)?;
        if hardened {
            hmac.update(&[0]);
            hmac.update(&private_key.to_bytes());
        } else {
            hmac.update(&private_key.public_key().to_bytes());
        }

        Ok(hmac)
    }

    /// Serialize the raw public key as a byte array.
    pub fn to_bytes(&self) -> PublicKeyBytes {
        self.extended_public_key.to_bytes()
    }

    pub fn attrs(&self) -> &ExtendedKeyAttrs {
        self.extended_public_key.attrs()
    }

    /// Serialize this key as a self-[`Zeroizing`] `String`.
    pub fn to_string(&self, prefix: Option<Prefix>) -> Zeroizing<String> {
        let key = self.extended_public_key.to_string(Some(prefix.unwrap_or(Prefix::KPUB)));
        Zeroizing::new(key)
    }
}

impl Debug for WalletDerivationManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalletAccount")
            .field("depth", &self.attrs().depth)
            .field("child_number", &self.attrs().child_number)
            .field("chain_code", &faster_hex::hex_string(&self.attrs().chain_code))
            .field("public_key", &faster_hex::hex_string(&self.to_bytes()))
            .field("parent_fingerprint", &self.attrs().parent_fingerprint)
            .finish()
    }
}

#[async_trait]
impl WalletDerivationManagerTrait for WalletDerivationManager {
    /// build wallet from root/master private key
    fn from_master_xprv(xprv: &str, is_multisig: bool, account_index: u64, cosigner_index: Option<u32>) -> Result<Self> {
        let xprv_key = ExtendedPrivateKey::<SecretKey>::from_str(xprv)?;
        let attrs = xprv_key.attrs();

        let (extended_private_key, attrs) =
            Self::create_extended_key(*xprv_key.private_key(), attrs.clone(), is_multisig, account_index)?;

        let extended_public_key = ExtendedPublicKey { public_key: extended_private_key.get_public_key(), attrs };

        let wallet = Self::from_extended_public_key(extended_public_key, cosigner_index)?;

        Ok(wallet)
    }

    fn from_extended_public_key_str(xpub: &str, cosigner_index: Option<u32>) -> Result<Self> {
        let extended_public_key = ExtendedPublicKey::<secp256k1::PublicKey>::from_str(xpub)?;
        let wallet = Self::from_extended_public_key(extended_public_key, cosigner_index)?;
        Ok(wallet)
    }

    fn from_extended_public_key(
        extended_public_key: ExtendedPublicKey<secp256k1::PublicKey>,
        cosigner_index: Option<u32>,
    ) -> Result<Self> {
        let receive_wallet = Self::derive_child_pubkey_manager(extended_public_key.clone(), AddressType::Receive, cosigner_index)?;

        let change_wallet = Self::derive_child_pubkey_manager(extended_public_key.clone(), AddressType::Change, cosigner_index)?;

        let wallet = Self {
            extended_public_key,
            receive_pubkey_manager: Arc::new(receive_wallet),
            change_pubkey_manager: Arc::new(change_wallet),
        };

        Ok(wallet)
    }

    fn receive_pubkey_manager(&self) -> Arc<dyn PubkeyDerivationManagerTrait> {
        self.receive_pubkey_manager.clone()
    }

    fn change_pubkey_manager(&self) -> Arc<dyn PubkeyDerivationManagerTrait> {
        self.change_pubkey_manager.clone()
    }

    #[inline(always)]
    fn new_receive_pubkey(&self) -> Result<secp256k1::PublicKey> {
        let key = self.receive_pubkey_manager.new_pubkey()?;
        Ok(key)
    }

    #[inline(always)]
    fn new_change_pubkey(&self) -> Result<secp256k1::PublicKey> {
        let key = self.change_pubkey_manager.new_pubkey()?;
        Ok(key)
    }

    #[inline(always)]
    fn receive_pubkey(&self) -> Result<secp256k1::PublicKey> {
        let key = self.receive_pubkey_manager.current_pubkey()?;
        Ok(key)
    }

    #[inline(always)]
    fn change_pubkey(&self) -> Result<secp256k1::PublicKey> {
        let key = self.change_pubkey_manager.current_pubkey()?;
        Ok(key)
    }

    #[inline(always)]
    fn derive_receive_pubkey(&self, index: u32) -> Result<secp256k1::PublicKey> {
        let key = self.receive_pubkey_manager.derive_pubkey(index)?;
        Ok(key)
    }

    #[inline(always)]
    fn derive_change_pubkey(&self, index: u32) -> Result<secp256k1::PublicKey> {
        let key = self.change_pubkey_manager.derive_pubkey(index)?;
        Ok(key)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::{PubkeyDerivationManager, WalletDerivationManager, WalletDerivationManagerTrait};
    use calico_addresses::Prefix;

    fn gen1_receive_addresses() -> Vec<&'static str> {
        vec![
            "calico:qz6fmjhdnhc8807szcv5ruth0zhwyqcsskaza5n028zkhkmw7y5evsy3znqyc",
            "calico:qqyt4h39jc5scn577ue5et80dju34mg0fead0xpzwk0ykssup39q6xn7mxexl",
            "calico:qplqf0h6yltdxg9t5pmqrsz9e96e4j3v03lvr9d9unpvhy9evk4ngvjc29k56",
            "calico:qrsgyakl7mms94fu3khhg9rlnd7wf9c7rnlayr0s5rj7vahs20fhun9eaewuj",
            "calico:qpng9s98zf3ytusy4en8ua7h8vgardk7c7xc70g8fhghgem3rmfrv9u2r3ls8",
            "calico:qpcadn4kwcz4xtp395zyaw0h3t9su4gwqgts9mwgg7kxja5p6tdgzddd7zdrg",
            "calico:qrelu04sna67nwqfufm2ndar7suk7elnnvl6hpxknmduuuk3recuqz7hdkmj0",
            "calico:qpcw5env86eskjw3t0cdj8eytsxu8rh4mnqam6lql058anzpwx4gxwypd2c4a",
            "calico:qrx2clln3ysu80qmcaxc4fu0v4n75dl9n57c8qtjr07ct6m9lfu0cp87vrh7q",
            "calico:qpmr7zkhjmlnfnlhd027hgecemydzjwdruauy596eflmkythz43hsv3acejyr",
            "calico:qpg7za4vg0zv45f33uw922ajua5m82hzreh2pac0dq3wa2hnpvxmwf4r3vapw",
            "calico:qzd3qvg036l3aa7p32scm6fnas99rkrgtgwkf4dmp7qh52mew4lj5fltthwnv",
            "calico:qrynzn8y5wr0y8zlpvy27z8k69j3teje8ymfhuqhaznzz5kwxd9gsjj7j66u3",
            "calico:qrwzu7ql5sxqtchf7046fyrrw0dyd77e9u595czctqq2ajydu24hvkxneqqz6",
            "calico:qzsg88lwmwrrajxfwfuafqswam4h028xdm50tu5r06cjwljslyhpw9k6cpx6e",
            "calico:qq9xkt72gshzzmedpslsla5tzl5384ccr70025jajy42frx6zea6vzqpt7v5s",
            "calico:qr5hkfdnx8gpzw4qckkdpxlgn6hfjue952qx74xc8f0h077ah4pfv5k8w5t7x",
            "calico:qq9dq2kwd67j3wuxv5vm2tv6ksuzgjt7j3nsjwdgt8pv4qh93cdf6qncqe5s4",
            "calico:qpk5u86auvseurq967ec4mtvcppf3z50hesq5ay5qhwmnk0c2n4274wyx9f68",
            "calico:qpjwm939e05068s72mejxk23d2vzswm28hpwfenk4enknlgy9fxp7gj7yyf5q",
        ]
    }

    fn gen1_change_addresses() -> Vec<&'static str> {
        vec![
            "calico:qqa2hvsx5tncxkf4kcf8uk05wu5zqjnnwmuwkrdruffxd7n26ukwxnf8fg2dc",
            "calico:qz3gds4wh6s49j63p8lwx3nsdxypnlqa66604p2fcy8fgz2q75fcxw7e5awus",
            "calico:qrypzvzp227d59lpq2rfh9qaych45hmxnn8n8l4cqgqy48kzu3r6ws9zt20vj",
            "calico:qp7g3wxa0k3ngezsv2z64kkx8c9z9wfd3c7x4e4rnaeqz4l5peagk2v9npznq",
            "calico:qqp5mhe5f84u3vut6wsd5ny2k3p4da2ku6dwxye5qh6qyh80v6q07q3jkr93j",
            "calico:qqhvts5zl922cjs6e6eg640mz4vsw0kryu2m2wyqvwmxzvj59583g2vmfheth",
            "calico:qpy23th7l9ftuh77947zudjqrg446lupp7wl8h75dw6qc423wutz2l355u79x",
            "calico:qq9ak5pcxam7szrhu03h6az2ce8zfxaad9ln9tjl4ksdx7qrgq8jzk05rwjtr",
            "calico:qr950649kjfmgm3rxelrn6vs5l9dqy8y47l8a3q8xv9vue6j5c4vupdh768cn",
            "calico:qzlvpmf9thq54ny0uehetgr7yazvm2krqs6hlqjw3ypz6l9vq6kh2fjq56cm3",
            "calico:qppvzf3rxfsxmh95ynwuu7u0h8g3wx79dtfnvfr0ylgz0jeps4m5sw773gd9z",
            "calico:qpje47hrm5lhlrm9tafac3pl4s9xqpw5rl9xqkwn7p5s5f5yndfnx82yghc9z",
            "calico:qqqfucdf7ufch5xjj2035wy7s09wlcwq246c90deg3gzm5u9u3dq2w77cvw43",
            "calico:qqk236zwsz3nveaan67lznr77sjpal5g0n43wgg4g8mdh9ke48x6yw28fdhw7",
            "calico:qrj45aw06akvq07xm66k53qdtqnrwvjve2hhksu98sny7n47n9787m20t55u0",
            "calico:qqhn0c3ewpssv95e0jqaxp03mgglra5th9vlctralh96nmsmlyjvwyt68e5cn",
            "calico:qzeyknwtl7z79cek7pe2jz9jyp9nezd6dznggmlfq8x375680j57uasdgwr9j",
            "calico:qrsfmqjh640arzky9a3c0248kj9w972gfrn4lxmwwkjtucj8x5g2cp8m84z6s",
            "calico:qq8xcfuf32sqhtx5hkk656z8hngdzqz8zdhpk94mk2upsnch9mnw79282d3fs",
            "calico:qqpq5xugsedd35cevt7qkk8amfhm2pe5fm9xskapxhcn2maug0zcvljsxnwug",
        ]
    }

    #[tokio::test]
    async fn hd_wallet_gen1() {
        let master_xprv =
            "kprv5y2qurMHCsXYrNfU3GCihuwG3vMqFji7PZXajMEqyBkNh9UZUJgoHYBLTKu1eM4MvUtomcXPQ3Sw9HZ5ebbM4byoUciHo1zrPJBQfqpLorQ";

        let hd_wallet = WalletDerivationManager::from_master_xprv(master_xprv, false, 0, None);
        assert!(hd_wallet.is_ok(), "Could not parse key");
        let hd_wallet = hd_wallet.unwrap();

        let receive_addresses = gen1_receive_addresses();
        let change_addresses = gen1_change_addresses();

        for index in 0..20 {
            let pubkey = hd_wallet.derive_receive_pubkey(index).unwrap();
            let address: String = PubkeyDerivationManager::create_address(&pubkey, Prefix::Mainnet, false).unwrap().into();
            assert_eq!(receive_addresses[index as usize], address, "receive address at {index} failed");
            let pubkey = hd_wallet.derive_change_pubkey(index).unwrap();
            let address: String = PubkeyDerivationManager::create_address(&pubkey, Prefix::Mainnet, false).unwrap().into();
            assert_eq!(change_addresses[index as usize], address, "change address at {index} failed");
        }
    }

    #[tokio::test]
    async fn wallet_from_mnemonic() {
        let mnemonic = "fringe ceiling crater inject pilot travel gas nurse bulb bullet horn segment snack harbor dice laugh vital cigar push couple plastic into slender worry";
        let mnemonic = calico_bip32::Mnemonic::new(mnemonic, calico_bip32::Language::English).unwrap();
        let xprv = calico_bip32::ExtendedPrivateKey::<calico_bip32::SecretKey>::new(mnemonic.to_seed("")).unwrap();
        let xprv_str = xprv.to_string(calico_bip32::Prefix::KPRV).to_string();
        assert_eq!(
            xprv_str,
            "kprv5y2qurMHCsXYrpeDB395BY2DPKYHUGaCMpFAYRi1cmhwin1bWRyUXVbtTyy54FCGxPnnEvbK9WaiaQgkGS9ngGxmHy1bubZYY6MTokeYP2Q",
            "xprv not matched"
        );

        let wallet = WalletDerivationManager::from_master_xprv(&xprv_str, false, 0, None).unwrap();
        let xpub_str = wallet.to_string(Some(calico_bip32::Prefix::KPUB)).to_string();
        assert_eq!(
            xpub_str,
            "kpub2KKTcD3wF1aBBRE5A1s28ag93yAaZt59vFs8SueHxaYKUsFPabyb6koCFjAarvu6zUthfZ3kAKdELTssgtRsDfAsrZ3MJEwRXmKXf1MxqSr",
            "drived kpub not matched"
        );

        println!("Extended kpub: {}\n", xpub_str);
    }

    #[tokio::test]
    async fn address_test_by_ktrv() {
        let mnemonic = "hunt bitter praise lift buyer topic crane leopard uniform network inquiry over grain pass match crush marine strike doll relax fortune trumpet sunny silk";
        let mnemonic = calico_bip32::Mnemonic::new(mnemonic, calico_bip32::Language::English).unwrap();
        let xprv = calico_bip32::ExtendedPrivateKey::<calico_bip32::SecretKey>::new(mnemonic.to_seed("")).unwrap();
        let ktrv_str = xprv.to_string(calico_bip32::Prefix::KTRV).to_string();
        assert_eq!(
            ktrv_str,
            "ktrv5himbbCxArFU2CHiEQyVHP1ABS1tA1SY88CwePzGeM8gHfWmkNBXehhKsESH7UwcxpjpDdMNbwtBfyPoZ7W59kYfVnUXKRgv8UguDns2FQb",
            "master ktrv not matched"
        );

        let wallet = WalletDerivationManager::from_master_xprv(&ktrv_str, false, 0, None).unwrap();
        let ktub_str = wallet.to_string(Some(calico_bip32::Prefix::KTUB)).to_string();
        assert_eq!(
            ktub_str,
            "ktub22bkfYm57c6wdXa6ADxknMrC8R7UV4Duvmg8iPqPLQnKxdnL7giW7shxEUCKzq1mpYJNoRLCdnLdbXDy4SCKNzLNDb8RjroA8Y3Qt4CUjBQ",
            "drived ktub not matched"
        );

        let key = wallet.derive_receive_pubkey(1).unwrap();
        let address = PubkeyDerivationManager::create_address(&key, Prefix::Testnet, false).unwrap().to_string();
        assert_eq!(address, "calicotest:qrwl2jrhf7qsttz90jecnyje5mhmaf3kylpqqq4gq32xc5rpj2nwj4wucvtly")
    }

    #[tokio::test]
    async fn generate_addresses_by_range() {
        let master_xprv =
            "kprv5y2qurMHCsXYrNfU3GCihuwG3vMqFji7PZXajMEqyBkNh9UZUJgoHYBLTKu1eM4MvUtomcXPQ3Sw9HZ5ebbM4byoUciHo1zrPJBQfqpLorQ";

        let hd_wallet = WalletDerivationManager::from_master_xprv(master_xprv, false, 0, None);
        assert!(hd_wallet.is_ok(), "Could not parse key");
        let hd_wallet = hd_wallet.unwrap();
        let pubkeys = hd_wallet.receive_pubkey_manager().derive_pubkey_range(0..20).unwrap();
        let addresses_receive = pubkeys
            .into_iter()
            .map(|k| PubkeyDerivationManager::create_address(&k, Prefix::Mainnet, false).unwrap().to_string())
            .collect::<Vec<String>>();

        let pubkeys = hd_wallet.change_pubkey_manager().derive_pubkey_range(0..20).unwrap();
        let addresses_change = pubkeys
            .into_iter()
            .map(|k| PubkeyDerivationManager::create_address(&k, Prefix::Mainnet, false).unwrap().to_string())
            .collect::<Vec<String>>();
        println!("receive addresses: {addresses_receive:#?}");
        println!("change addresses: {addresses_change:#?}");
        let receive_addresses = gen1_receive_addresses();
        let change_addresses = gen1_change_addresses();
        for index in 0..20 {
            assert_eq!(receive_addresses[index], addresses_receive[index], "receive address at {index} failed");
            assert_eq!(change_addresses[index], addresses_change[index], "change address at {index} failed");
        }
    }

    #[tokio::test]
    async fn generate_calicotest_addresses() {
        let receive_addresses = [
            "calicotest:qz6fmjhdnhc8807szcv5ruth0zhwyqcsskaza5n028zkhkmw7y5ev7jmefc0s",
            "calicotest:qqyt4h39jc5scn577ue5et80dju34mg0fead0xpzwk0ykssup39q6g95qupdh",
            "calicotest:qplqf0h6yltdxg9t5pmqrsz9e96e4j3v03lvr9d9unpvhy9evk4ngzyj3lwlj",
            "calicotest:qrsgyakl7mms94fu3khhg9rlnd7wf9c7rnlayr0s5rj7vahs20fhuannxrkh6",
            "calicotest:qpng9s98zf3ytusy4en8ua7h8vgardk7c7xc70g8fhghgem3rmfrvt2qct8m0",
            "calicotest:qpcadn4kwcz4xtp395zyaw0h3t9su4gwqgts9mwgg7kxja5p6tdgzrm89c4gq",
            "calicotest:qrelu04sna67nwqfufm2ndar7suk7elnnvl6hpxknmduuuk3recuqvgakvre8",
            "calicotest:qpcw5env86eskjw3t0cdj8eytsxu8rh4mnqam6lql058anzpwx4gxqjtksq74",
            "calicotest:qrx2clln3ysu80qmcaxc4fu0v4n75dl9n57c8qtjr07ct6m9lfu0c035he04g",
            "calicotest:qpmr7zkhjmlnfnlhd027hgecemydzjwdruauy596eflmkythz43hsz8hrr20t",
            "calicotest:qpg7za4vg0zv45f33uw922ajua5m82hzreh2pac0dq3wa2hnpvxmw8rf2k92x",
            "calicotest:qzd3qvg036l3aa7p32scm6fnas99rkrgtgwkf4dmp7qh52mew4lj58fpsdkcy",
            "calicotest:qrynzn8y5wr0y8zlpvy27z8k69j3teje8ymfhuqhaznzz5kwxd9gsuy5fqzhe",
            "calicotest:qrwzu7ql5sxqtchf7046fyrrw0dyd77e9u595czctqq2ajydu24hvcsez6cfj",
            "calicotest:qzsg88lwmwrrajxfwfuafqswam4h028xdm50tu5r06cjwljslyhpwtqsrm733",
            "calicotest:qq9xkt72gshzzmedpslsla5tzl5384ccr70025jajy42frx6zea6vvktsy5lc",
            "calicotest:qr5hkfdnx8gpzw4qckkdpxlgn6hfjue952qx74xc8f0h077ah4pfv6qd4wn4w",
            "calicotest:qq9dq2kwd67j3wuxv5vm2tv6ksuzgjt7j3nsjwdgt8pv4qh93cdf6w9jmrvma",
            "calicotest:qpk5u86auvseurq967ec4mtvcppf3z50hesq5ay5qhwmnk0c2n427mcwal330",
            "calicotest:qpjwm939e05068s72mejxk23d2vzswm28hpwfenk4enknlgy9fxp7xy5l73lg",
        ];

        let master_xprv =
            "kprv5y2qurMHCsXYrNfU3GCihuwG3vMqFji7PZXajMEqyBkNh9UZUJgoHYBLTKu1eM4MvUtomcXPQ3Sw9HZ5ebbM4byoUciHo1zrPJBQfqpLorQ";

        let hd_wallet = WalletDerivationManager::from_master_xprv(master_xprv, false, 0, None);
        assert!(hd_wallet.is_ok(), "Could not parse key");
        let hd_wallet = hd_wallet.unwrap();

        //let mut receive_addresses = vec![]; //gen1_receive_addresses();
        //let change_addresses = gen1_change_addresses();

        for index in 0..20 {
            let key = hd_wallet.derive_receive_pubkey(index).unwrap();
            //let address = Address::new(Prefix::Testnet, calico_addresses::Version::PubKey, key.to_bytes());
            let address = PubkeyDerivationManager::create_address(&key, Prefix::Testnet, false).unwrap();
            //receive_addresses.push(String::from(address));
            assert_eq!(receive_addresses[index as usize], address.to_string(), "receive address at {index} failed");
            //let address: String = hd_wallet.derive_change_address(index).await.unwrap().into();
            //assert_eq!(change_addresses[index as usize], address, "change address at {index} failed");
        }

        println!("receive_addresses: {receive_addresses:#?}");
    }
}
