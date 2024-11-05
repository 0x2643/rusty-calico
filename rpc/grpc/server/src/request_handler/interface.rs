use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use calico_grpc_core::{
    ops::CalicodPayloadOps,
    protowire::{CalicodRequest, CalicodResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type CalicodMethod = Method<ServerContext, Connection, CalicodRequest, CalicodResponse>;
pub type DynCalicodMethod = Arc<dyn MethodTrait<ServerContext, Connection, CalicodRequest, CalicodResponse>>;
pub type CalicodDropFn = DropFn<CalicodRequest, CalicodResponse>;
pub type CalicodRoutingPolicy = RoutingPolicy<CalicodRequest, CalicodResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`CalicodPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<CalicodPayloadOps, DynCalicodMethod>,
    method_not_implemented: DynCalicodMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, calicod_request: CalicodRequest| {
            Box::pin(async move {
                match calicod_request.payload {
                    Some(ref request) => Ok(CalicodResponse {
                        id: calicod_request.id,
                        payload: Some(
                            CalicodPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into()),
                        ),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: CalicodPayloadOps, method: CalicodMethod) {
        let method: DynCalicodMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: CalicodPayloadOps, method: CalicodMethod) {
        let method: DynCalicodMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(
        &mut self,
        op: CalicodPayloadOps,
        tasks: usize,
        queue_size: usize,
        routing_policy: CalicodRoutingPolicy,
    ) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, CalicodRequest, CalicodResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, CalicodRequest, CalicodResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(
        &self,
        op: &CalicodPayloadOps,
        connection: Connection,
        request: CalicodRequest,
    ) -> GrpcServerResult<CalicodResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &CalicodPayloadOps) -> DynCalicodMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
