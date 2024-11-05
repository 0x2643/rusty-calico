use super::error::Result;
use calico_grpc_core::{
    ops::CalicodPayloadOps,
    protowire::{CalicodRequest, CalicodResponse},
};
use core::fmt::Debug;
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: CalicodPayloadOps, request: &CalicodRequest) -> CalicodResponseReceiver;
    fn handle_response(&self, response: CalicodResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type CalicodResponseSender = oneshot::Sender<Result<CalicodResponse>>;
pub(crate) type CalicodResponseReceiver = oneshot::Receiver<Result<CalicodResponse>>;
