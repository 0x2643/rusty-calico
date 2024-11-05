use crate::Notification;

pub type ChannelConnection = calico_notify::connection::ChannelConnection<Notification>;
pub use calico_notify::connection::ChannelType;
