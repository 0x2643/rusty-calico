use crate::imports::*;

#[derive(Default, Handler)]
#[help("Broadcasts a signed transaction to the network.")]
pub struct Broadcast;

impl Broadcast {
    async fn main(self: Arc<Self>, ctx: &Arc<dyn Context>, _argv: Vec<String>, _cmd: &str) -> Result<()> {
        let ctx = ctx.clone().downcast_arc::<CalicoCli>()?;
        ctx.wallet().broadcast().await?;
        Ok(())
    }
}
