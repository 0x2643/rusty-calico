use calico_notify::{scope::Scope, subscription::Command};

use crate::protowire::{
    calicod_request, calicod_response, CalicodRequest, CalicodResponse, NotifyBlockAddedRequestMessage,
    NotifyFinalityConflictRequestMessage, NotifyNewBlockTemplateRequestMessage, NotifyPruningPointUtxoSetOverrideRequestMessage,
    NotifySinkBlueScoreChangedRequestMessage, NotifyUtxosChangedRequestMessage, NotifyVirtualChainChangedRequestMessage,
    NotifyVirtualDaaScoreChangedRequestMessage,
};

impl CalicodRequest {
    pub fn from_notification_type(scope: &Scope, command: Command) -> Self {
        CalicodRequest { id: 0, payload: Some(calicod_request::Payload::from_notification_type(scope, command)) }
    }

    pub fn is_subscription(&self) -> bool {
        self.payload.as_ref().is_some_and(|x| x.is_subscription())
    }
}

impl calicod_request::Payload {
    pub fn from_notification_type(scope: &Scope, command: Command) -> Self {
        match scope {
            Scope::BlockAdded(_) => {
                calicod_request::Payload::NotifyBlockAddedRequest(NotifyBlockAddedRequestMessage { command: command.into() })
            }
            Scope::NewBlockTemplate(_) => {
                calicod_request::Payload::NotifyNewBlockTemplateRequest(NotifyNewBlockTemplateRequestMessage {
                    command: command.into(),
                })
            }

            Scope::VirtualChainChanged(ref scope) => {
                calicod_request::Payload::NotifyVirtualChainChangedRequest(NotifyVirtualChainChangedRequestMessage {
                    command: command.into(),
                    include_accepted_transaction_ids: scope.include_accepted_transaction_ids,
                })
            }
            Scope::FinalityConflict(_) => {
                calicod_request::Payload::NotifyFinalityConflictRequest(NotifyFinalityConflictRequestMessage {
                    command: command.into(),
                })
            }
            Scope::FinalityConflictResolved(_) => {
                calicod_request::Payload::NotifyFinalityConflictRequest(NotifyFinalityConflictRequestMessage {
                    command: command.into(),
                })
            }
            Scope::UtxosChanged(ref scope) => calicod_request::Payload::NotifyUtxosChangedRequest(NotifyUtxosChangedRequestMessage {
                addresses: scope.addresses.iter().map(|x| x.into()).collect::<Vec<String>>(),
                command: command.into(),
            }),
            Scope::SinkBlueScoreChanged(_) => {
                calicod_request::Payload::NotifySinkBlueScoreChangedRequest(NotifySinkBlueScoreChangedRequestMessage {
                    command: command.into(),
                })
            }
            Scope::VirtualDaaScoreChanged(_) => {
                calicod_request::Payload::NotifyVirtualDaaScoreChangedRequest(NotifyVirtualDaaScoreChangedRequestMessage {
                    command: command.into(),
                })
            }
            Scope::PruningPointUtxoSetOverride(_) => {
                calicod_request::Payload::NotifyPruningPointUtxoSetOverrideRequest(NotifyPruningPointUtxoSetOverrideRequestMessage {
                    command: command.into(),
                })
            }
        }
    }

    pub fn is_subscription(&self) -> bool {
        use crate::protowire::calicod_request::Payload;
        matches!(
            self,
            Payload::NotifyBlockAddedRequest(_)
                | Payload::NotifyVirtualChainChangedRequest(_)
                | Payload::NotifyFinalityConflictRequest(_)
                | Payload::NotifyUtxosChangedRequest(_)
                | Payload::NotifySinkBlueScoreChangedRequest(_)
                | Payload::NotifyVirtualDaaScoreChangedRequest(_)
                | Payload::NotifyPruningPointUtxoSetOverrideRequest(_)
                | Payload::NotifyNewBlockTemplateRequest(_)
                | Payload::StopNotifyingUtxosChangedRequest(_)
                | Payload::StopNotifyingPruningPointUtxoSetOverrideRequest(_)
        )
    }
}

impl CalicodResponse {
    pub fn is_notification(&self) -> bool {
        match self.payload {
            Some(ref payload) => payload.is_notification(),
            None => false,
        }
    }
}

#[allow(clippy::match_like_matches_macro)]
impl calicod_response::Payload {
    pub fn is_notification(&self) -> bool {
        use crate::protowire::calicod_response::Payload;
        match self {
            Payload::BlockAddedNotification(_) => true,
            Payload::VirtualChainChangedNotification(_) => true,
            Payload::FinalityConflictNotification(_) => true,
            Payload::FinalityConflictResolvedNotification(_) => true,
            Payload::UtxosChangedNotification(_) => true,
            Payload::SinkBlueScoreChangedNotification(_) => true,
            Payload::VirtualDaaScoreChangedNotification(_) => true,
            Payload::PruningPointUtxoSetOverrideNotification(_) => true,
            Payload::NewBlockTemplateNotification(_) => true,
            _ => false,
        }
    }
}
