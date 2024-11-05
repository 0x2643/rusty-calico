use calico_cli_lib::calico_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_calico_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    calico_cli(options, None).await?;
    Ok(())
}
