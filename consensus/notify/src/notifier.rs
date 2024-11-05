use crate::notification::Notification;
use calico_notify::{connection::ChannelConnection, notifier::Notifier};

pub type ConsensusNotifier = Notifier<Notification, ChannelConnection<Notification>>;
