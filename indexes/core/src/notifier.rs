use crate::notification::Notification;
use calico_notify::{connection::ChannelConnection, notifier::Notifier};

pub type IndexNotifier = Notifier<Notification, ChannelConnection<Notification>>;
