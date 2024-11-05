use crate::{block::Block, header::Header, subnets::SUBNETWORK_ID_COINBASE, tx::Transaction};
use calico_hashes::{Hash, ZERO_HASH};
use calico_muhash::EMPTY_MUHASH;

/// The constants uniquely representing the genesis block
#[derive(Clone, Debug)]
pub struct GenesisBlock {
    pub hash: Hash,
    pub version: u16,
    pub hash_merkle_root: Hash,
    pub utxo_commitment: Hash,
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
    pub daa_score: u64,
    pub coinbase_payload: &'static [u8],
}

impl GenesisBlock {
    pub fn build_genesis_transactions(&self) -> Vec<Transaction> {
        vec![Transaction::new(0, Vec::new(), Vec::new(), 0, SUBNETWORK_ID_COINBASE, 0, self.coinbase_payload.to_vec())]
    }
}

impl From<&GenesisBlock> for Header {
    fn from(genesis: &GenesisBlock) -> Self {
        Header::new_finalized(
            genesis.version,
            Vec::new(),
            genesis.hash_merkle_root,
            ZERO_HASH,
            genesis.utxo_commitment,
            genesis.timestamp,
            genesis.bits,
            genesis.nonce,
            genesis.daa_score,
            0.into(),
            0,
            ZERO_HASH,
        )
    }
}

impl From<&GenesisBlock> for Block {
    fn from(genesis: &GenesisBlock) -> Self {
        Block::new(genesis.into(), genesis.build_genesis_transactions())
    }
}

impl From<(&Header, &'static [u8])> for GenesisBlock {
    fn from((header, payload): (&Header, &'static [u8])) -> Self {
        Self {
            hash: header.hash,
            version: header.version,
            hash_merkle_root: header.hash_merkle_root,
            utxo_commitment: header.utxo_commitment,
            timestamp: header.timestamp,
            bits: header.bits,
            nonce: header.nonce,
            daa_score: header.daa_score,
            coinbase_payload: payload,
        }
    }
}

/// The genesis block of the block-DAG which serves as the public transaction ledger for the main network.
pub const GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0x50, 0x6D, 0x03, 0x0E, 0x37, 0xC5, 0x4F, 0x10, 0x68, 0xF8, 0x89, 0x03, 0x6F, 0x52, 0x7E, 0x2F, 0xB6, 0xA5, 0x33, 0x55, 0x2F,
        0x51, 0xEB, 0x00, 0x64, 0xD4, 0x58, 0x73, 0x9B, 0xAE, 0x18, 0x04,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0x97, 0x91, 0x22, 0x53, 0xC6, 0x06, 0x71, 0x60, 0x41, 0xBF, 0xD9, 0x81, 0xB5, 0x7B, 0x75, 0x6E, 0x02, 0xC7, 0x5A, 0xEB, 0x32,
        0x99, 0xA1, 0xFB, 0xE4, 0x14, 0x7F, 0x36, 0xBA, 0xF6, 0xD7, 0xD5,
    ]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 1729382833727,
    bits: 536999497, // Prime number
    nonce: 867,      // CalicoCat
    daa_score: 0,    // Checkpoint DAA score
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,       // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00,       // Subsidy
        0x00, 0x00,                                           // Script version
        0x01,                                                 // Varint
        0x00,                                                 // OP-FALSE
        0x43, 0x61, 0x6C, 0x69, 0x63, 0x6F, 0x43, 0x61, 0x74, // CalicoCat
    ],
};

pub const TESTNET_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0x75, 0xC4, 0x27, 0x86, 0x64, 0x91, 0x51, 0x4B, 0x16, 0xDA, 0xC4, 0xE1, 0x49, 0x3E, 0x62, 0xE1, 0x86, 0xCA, 0x4D, 0x65, 0xC9,
        0x94, 0x61, 0xE7, 0x7B, 0xAB, 0x1D, 0xF7, 0x18, 0x4A, 0x12, 0xEE,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0x02, 0xC0, 0x0E, 0xD5, 0xE6, 0x72, 0x31, 0x8D, 0xD4, 0x78, 0x7A, 0x8F, 0x52, 0xC1, 0xED, 0xE7, 0x26, 0xC2, 0x16, 0xCF, 0x9A,
        0xFD, 0xBF, 0xFB, 0x61, 0xA5, 0x13, 0xFD, 0x58, 0x70, 0x8B, 0x04,
    ]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 1713884672545,
    bits: 511699987, // Prime number
    nonce: 1003,     // CalicoTest
    daa_score: 0,
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00,                                     // Script version
        0x01,                                           // Varint
        0x00,                                           // OP-FALSE
        0x43, 0x61, 0x6C, 0x69, 0x63, 0x6F, 0x54, 0x65, // CalicoTest
        0x73, 0x74,
    ],
};

pub const TESTNET11_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0xC6, 0x95, 0x1F, 0xFA, 0x1C, 0x6A, 0xA5, 0x09, 0x41, 0x19, 0xA2, 0x93, 0x48, 0x67, 0x87, 0xB2, 0x4A, 0x86, 0xC1, 0xC5, 0xA5,
        0xCF, 0xAD, 0x15, 0x7B, 0x74, 0x1B, 0x66, 0x00, 0x05, 0x9A, 0x65,
    ]),
    hash_merkle_root: Hash::from_bytes([
        0xB8, 0x4C, 0x2B, 0x36, 0x6E, 0xC2, 0xF8, 0x38, 0x43, 0x8B, 0xBF, 0x1F, 0xCB, 0xC5, 0xE0, 0xDF, 0x6E, 0xF5, 0xCC, 0x81, 0xDD,
        0x65, 0x74, 0x9C, 0x5A, 0x4B, 0xD7, 0x26, 0x81, 0x43, 0xB8, 0x08,
    ]),
    bits: 504154830, // see `gen_testnet11_genesis`
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00,                                     // Script version
        0x01,                                           // Varint
        0x00,                                           // OP-FALSE
        0x43, 0x61, 0x6C, 0x69, 0x63, 0x6F, 0x54, 0x65, // CalicoTest11
        0x73, 0x74, 31, 31,
    ],
    ..TESTNET_GENESIS
};

pub const SIMNET_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0x80, 0xD6, 0xA5, 0xC8, 0x49, 0x2A, 0x90, 0xF4, 0x7C, 0x82, 0x50, 0x44, 0xB1, 0xE2, 0xA4, 0xCB, 0x51, 0xB4, 0x13, 0x19, 0x4D,
        0x28, 0x24, 0x50, 0xC5, 0xA7, 0x13, 0x39, 0x21, 0xAB, 0x0F, 0x1D,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0xAB, 0x3B, 0xF8, 0x09, 0x19, 0x33, 0x33, 0xAE, 0xEF, 0x2D, 0xC9, 0x27, 0x15, 0xC8, 0x4F, 0x80, 0xF2, 0x62, 0x68, 0xE4, 0xA5,
        0xCE, 0xB4, 0xC1, 0xCE, 0x46, 0x76, 0x8A, 0x42, 0xEA, 0x11, 0xF8,
    ]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 1713885012324,
    bits: 543656363, // Prime number
    nonce: 884,      // CalicoSim
    daa_score: 0,
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,       // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00,       // Subsidy
        0x00, 0x00,                                           // Script version
        0x01,                                                 // Varint
        0x00,                                                 // OP-FALSE
        0x43, 0x61, 0x6C, 0x69, 0x63, 0x6F, 0x53, 0x69, 0x6D, // CalicoSim
    ],
};

pub const DEVNET_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0x35, 0x8D, 0x08, 0xFA, 0x3D, 0x27, 0xF6, 0x8D, 0x67, 0xFD, 0xD4, 0x3D, 0xF6, 0x2C, 0xBD, 0xF9, 0xDA, 0x2B, 0x6C, 0x6F, 0x09,
        0xE3, 0xDC, 0xA3, 0xF7, 0x39, 0x95, 0x27, 0xDD, 0x4D, 0xA5, 0xE7,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0xF4, 0x80, 0xA6, 0x94, 0xE7, 0x7E, 0x51, 0x71, 0x46, 0x02, 0x22, 0x55, 0x9F, 0x94, 0x33, 0x31, 0x9C, 0xC3, 0x2D, 0x4D, 0x53,
        0xA6, 0x7F, 0x05, 0x30, 0x44, 0x47, 0xC6, 0xEE, 0x29, 0xCF, 0x66,
    ]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 1713884849877,
    bits: 541034453, // Prime number
    nonce: 874,      // CalicoDev
    daa_score: 0,
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,       // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00,       // Subsidy
        0x00, 0x00,                                           // Script version
        0x01,                                                 // Varint
        0x00,                                                 // OP-FALSE
        0x43, 0x61, 0x6C, 0x69, 0x63, 0x6F, 0x44, 0x65, 0x76, // CalicoDev
    ],
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::bps::Testnet11Bps, merkle::calc_hash_merkle_root};

    #[test]
    fn test_genesis_hashes() {
        [GENESIS, TESTNET_GENESIS, TESTNET11_GENESIS, SIMNET_GENESIS, DEVNET_GENESIS].into_iter().for_each(|genesis| {
            let block: Block = (&genesis).into();
            assert_hashes_eq(calc_hash_merkle_root(block.transactions.iter(), false), block.header.hash_merkle_root);
            assert_hashes_eq(block.hash(), genesis.hash);
        });
    }

    #[test]
    fn gen_testnet11_genesis() {
        let bps = Testnet11Bps::bps();
        let mut genesis = TESTNET_GENESIS;
        let target = calico_math::Uint256::from_compact_target_bits(genesis.bits);
        let scaled_target = target * bps / 100;
        let scaled_bits = scaled_target.compact_target_bits();
        genesis.bits = scaled_bits;
        if genesis.bits != TESTNET11_GENESIS.bits {
            panic!("Testnet 11: new bits: {}\nnew hash: {:#04x?}", scaled_bits, Block::from(&genesis).hash().as_bytes());
        }
    }

    fn assert_hashes_eq(got: Hash, expected: Hash) {
        if got != expected {
            // Special hex print to ease changing the genesis hash according to the print if needed
            panic!("Got hash {:#04x?} while expecting {:#04x?}", got.as_bytes(), expected.as_bytes());
        }
    }
}
