use crate::protowire::{calicod_request, CalicodRequest, CalicodResponse};

impl From<calicod_request::Payload> for CalicodRequest {
    fn from(item: calicod_request::Payload) -> Self {
        CalicodRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<CalicodRequest> for CalicodRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<CalicodResponse> for CalicodResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod calicod_request_convert {
    use crate::protowire::*;
    use calico_rpc_core::{RpcError, RpcResult};

    impl_into_calicod_request!(Shutdown);
    impl_into_calicod_request!(SubmitBlock);
    impl_into_calicod_request!(GetBlockTemplate);
    impl_into_calicod_request!(GetBlock);
    impl_into_calicod_request!(GetInfo);

    impl_into_calicod_request!(GetCurrentNetwork);
    impl_into_calicod_request!(GetPeerAddresses);
    impl_into_calicod_request!(GetSink);
    impl_into_calicod_request!(GetMempoolEntry);
    impl_into_calicod_request!(GetMempoolEntries);
    impl_into_calicod_request!(GetConnectedPeerInfo);
    impl_into_calicod_request!(AddPeer);
    impl_into_calicod_request!(SubmitTransaction);
    impl_into_calicod_request!(SubmitTransactionReplacement);
    impl_into_calicod_request!(GetSubnetwork);
    impl_into_calicod_request!(GetVirtualChainFromBlock);
    impl_into_calicod_request!(GetBlocks);
    impl_into_calicod_request!(GetBlockCount);
    impl_into_calicod_request!(GetBlockDagInfo);
    impl_into_calicod_request!(ResolveFinalityConflict);
    impl_into_calicod_request!(GetHeaders);
    impl_into_calicod_request!(GetUtxosByAddresses);
    impl_into_calicod_request!(GetBalanceByAddress);
    impl_into_calicod_request!(GetBalancesByAddresses);
    impl_into_calicod_request!(GetSinkBlueScore);
    impl_into_calicod_request!(Ban);
    impl_into_calicod_request!(Unban);
    impl_into_calicod_request!(EstimateNetworkHashesPerSecond);
    impl_into_calicod_request!(GetMempoolEntriesByAddresses);
    impl_into_calicod_request!(GetCoinSupply);
    impl_into_calicod_request!(Ping);
    impl_into_calicod_request!(GetMetrics);
    impl_into_calicod_request!(GetConnections);
    impl_into_calicod_request!(GetSystemInfo);
    impl_into_calicod_request!(GetServerInfo);
    impl_into_calicod_request!(GetSyncStatus);
    impl_into_calicod_request!(GetDaaScoreTimestampEstimate);
    impl_into_calicod_request!(GetFeeEstimate);
    impl_into_calicod_request!(GetFeeEstimateExperimental);
    impl_into_calicod_request!(GetCurrentBlockColor);

    impl_into_calicod_request!(NotifyBlockAdded);
    impl_into_calicod_request!(NotifyNewBlockTemplate);
    impl_into_calicod_request!(NotifyUtxosChanged);
    impl_into_calicod_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_calicod_request!(NotifyFinalityConflict);
    impl_into_calicod_request!(NotifyVirtualDaaScoreChanged);
    impl_into_calicod_request!(NotifyVirtualChainChanged);
    impl_into_calicod_request!(NotifySinkBlueScoreChanged);

    macro_rules! impl_into_calicod_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_calicod_request_ex!(calico_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_calicod_request;

    macro_rules! impl_into_calicod_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for calicod_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for CalicodRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for calicod_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for CalicodRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&calicod_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &calicod_request::Payload) -> RpcResult<Self> {
                    if let calicod_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&CalicodRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &CalicodRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("CalicoRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for CalicodRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(calicod_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for calicod_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    calicod_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_calicod_request_ex;
}

pub mod calicod_response_convert {
    use crate::protowire::*;
    use calico_rpc_core::{RpcError, RpcResult};

    impl_into_calicod_response!(Shutdown);
    impl_into_calicod_response!(SubmitBlock);
    impl_into_calicod_response!(GetBlockTemplate);
    impl_into_calicod_response!(GetBlock);
    impl_into_calicod_response!(GetInfo);
    impl_into_calicod_response!(GetCurrentNetwork);

    impl_into_calicod_response!(GetPeerAddresses);
    impl_into_calicod_response!(GetSink);
    impl_into_calicod_response!(GetMempoolEntry);
    impl_into_calicod_response!(GetMempoolEntries);
    impl_into_calicod_response!(GetConnectedPeerInfo);
    impl_into_calicod_response!(AddPeer);
    impl_into_calicod_response!(SubmitTransaction);
    impl_into_calicod_response!(SubmitTransactionReplacement);
    impl_into_calicod_response!(GetSubnetwork);
    impl_into_calicod_response!(GetVirtualChainFromBlock);
    impl_into_calicod_response!(GetBlocks);
    impl_into_calicod_response!(GetBlockCount);
    impl_into_calicod_response!(GetBlockDagInfo);
    impl_into_calicod_response!(ResolveFinalityConflict);
    impl_into_calicod_response!(GetHeaders);
    impl_into_calicod_response!(GetUtxosByAddresses);
    impl_into_calicod_response!(GetBalanceByAddress);
    impl_into_calicod_response!(GetBalancesByAddresses);
    impl_into_calicod_response!(GetSinkBlueScore);
    impl_into_calicod_response!(Ban);
    impl_into_calicod_response!(Unban);
    impl_into_calicod_response!(EstimateNetworkHashesPerSecond);
    impl_into_calicod_response!(GetMempoolEntriesByAddresses);
    impl_into_calicod_response!(GetCoinSupply);
    impl_into_calicod_response!(Ping);
    impl_into_calicod_response!(GetMetrics);
    impl_into_calicod_response!(GetConnections);
    impl_into_calicod_response!(GetSystemInfo);
    impl_into_calicod_response!(GetServerInfo);
    impl_into_calicod_response!(GetSyncStatus);
    impl_into_calicod_response!(GetDaaScoreTimestampEstimate);
    impl_into_calicod_response!(GetFeeEstimate);
    impl_into_calicod_response!(GetFeeEstimateExperimental);
    impl_into_calicod_response!(GetCurrentBlockColor);

    impl_into_calicod_notify_response!(NotifyBlockAdded);
    impl_into_calicod_notify_response!(NotifyNewBlockTemplate);
    impl_into_calicod_notify_response!(NotifyUtxosChanged);
    impl_into_calicod_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_calicod_notify_response!(NotifyFinalityConflict);
    impl_into_calicod_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_calicod_notify_response!(NotifyVirtualChainChanged);
    impl_into_calicod_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_calicod_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_calicod_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_calicod_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_calicod_response_ex!(calico_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_calicod_response_base!(calico_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_calicod_response;

    macro_rules! impl_into_calicod_response_base {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<$core_struct>> for $protowire_struct {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    item.as_ref().map_err(|x| (*x).clone()).into()
                }
            }

            impl From<RpcError> for $protowire_struct {
                fn from(item: RpcError) -> Self {
                    let x: RpcResult<&$core_struct> = Err(item);
                    x.into()
                }
            }

            impl From<$protowire_struct> for calicod_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    calicod_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for CalicodResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(calicod_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_calicod_response_base;

    macro_rules! impl_into_calicod_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for calicod_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    calicod_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for CalicodResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for calicod_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    calicod_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for CalicodResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_calicod_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&calicod_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &calicod_response::Payload) -> RpcResult<Self> {
                    if let calicod_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&CalicodResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &CalicodResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("CalicoResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_calicod_response_ex;

    macro_rules! impl_into_calicod_notify_response {
        ($name:tt) => {
            impl_into_calicod_response!($name);

            paste::paste! {
                impl_into_calicod_notify_response_ex!(calico_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_calicod_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_calicod_notify_response_ex!(calico_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_calicod_notify_response;

    macro_rules! impl_into_calicod_notify_response_ex {
        ($($core_struct:ident)::+, $protowire_struct:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl<T> From<Result<(), T>> for $protowire_struct
            where
                T: Into<RpcError>,
            {
                fn from(item: Result<(), T>) -> Self {
                    item
                        .map(|_| $($core_struct)::+{})
                        .map_err(|err| err.into()).into()
                }
            }

        };
    }
    use impl_into_calicod_notify_response_ex;
}
