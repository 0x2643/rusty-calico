use crate::converter::{consensus::ConsensusConverter, index::IndexConverter};
use calico_notify::collector::CollectorFrom;

pub(crate) type CollectorFromConsensus = CollectorFrom<ConsensusConverter>;

pub(crate) type CollectorFromIndex = CollectorFrom<IndexConverter>;
