use calico_cli_lib::{calico_cli, TerminalOptions};

#[tokio::main]
async fn main() {
    let result = calico_cli(TerminalOptions::new().with_prompt("$ "), None).await;
    if let Err(err) = result {
        println!("{err}");
    }
}
