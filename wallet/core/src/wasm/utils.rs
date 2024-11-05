use crate::result::Result;
use calico_consensus_core::network::{NetworkType, NetworkTypeT};
use js_sys::BigInt;
use wasm_bindgen::prelude::*;
use workflow_wasm::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "bigint | number | HexString")]
    #[derive(Clone, Debug)]
    pub type ISompiToCalico;
}

/// Convert a Calico string to Sompi represented by bigint.
/// This function provides correct precision handling and
/// can be used to parse user input.
/// @category Wallet SDK
#[wasm_bindgen(js_name = "calicoToSompi")]
pub fn calico_to_sompi(calico: String) -> Option<BigInt> {
    crate::utils::try_calico_str_to_sompi(calico).ok().flatten().map(Into::into)
}

///
/// Convert Sompi to a string representation of the amount in Calico.
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToCalicoString")]
pub fn sompi_to_calico_string(sompi: ISompiToCalico) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    Ok(crate::utils::sompi_to_calico_string(sompi))
}

///
/// Format a Sompi amount to a string representation of the amount in Calico with a suffix
/// based on the network type (e.g. `SPR` for mainnet, `TSPR` for testnet,
/// `SSPR` for simnet, `DSPR` for devnet).
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToCalicoStringWithSuffix")]
pub fn sompi_to_calico_string_with_suffix(sompi: ISompiToCalico, network: &NetworkTypeT) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    let network_type = NetworkType::try_from(network)?;
    Ok(crate::utils::sompi_to_calico_string_with_suffix(sompi, &network_type))
}
