use crate::pb::calicod_message::Payload as CalicodMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum CalicodMessagePayloadType {
    Addresses = 0,
    Block,
    Transaction,
    BlockLocator,
    RequestAddresses,
    RequestRelayBlocks,
    RequestTransactions,
    IbdBlock,
    InvRelayBlock,
    InvTransactions,
    Ping,
    Pong,
    Verack,
    Version,
    TransactionNotFound,
    Reject,
    PruningPointUtxoSetChunk,
    RequestIbdBlocks,
    UnexpectedPruningPoint,
    IbdBlockLocator,
    IbdBlockLocatorHighestHash,
    RequestNextPruningPointUtxoSetChunk,
    DonePruningPointUtxoSetChunks,
    IbdBlockLocatorHighestHashNotFound,
    BlockWithTrustedData,
    DoneBlocksWithTrustedData,
    RequestPruningPointAndItsAnticone,
    BlockHeaders,
    RequestNextHeaders,
    DoneHeaders,
    RequestPruningPointUtxoSet,
    RequestHeaders,
    RequestBlockLocator,
    PruningPoints,
    RequestPruningPointProof,
    PruningPointProof,
    Ready,
    BlockWithTrustedDataV4,
    TrustedData,
    RequestIbdChainBlockLocator,
    IbdChainBlockLocator,
    RequestAntipast,
    RequestNextPruningPointAndItsAnticoneBlocks,
}

impl From<&CalicodMessagePayload> for CalicodMessagePayloadType {
    fn from(payload: &CalicodMessagePayload) -> Self {
        match payload {
            CalicodMessagePayload::Addresses(_) => CalicodMessagePayloadType::Addresses,
            CalicodMessagePayload::Block(_) => CalicodMessagePayloadType::Block,
            CalicodMessagePayload::Transaction(_) => CalicodMessagePayloadType::Transaction,
            CalicodMessagePayload::BlockLocator(_) => CalicodMessagePayloadType::BlockLocator,
            CalicodMessagePayload::RequestAddresses(_) => CalicodMessagePayloadType::RequestAddresses,
            CalicodMessagePayload::RequestRelayBlocks(_) => CalicodMessagePayloadType::RequestRelayBlocks,
            CalicodMessagePayload::RequestTransactions(_) => CalicodMessagePayloadType::RequestTransactions,
            CalicodMessagePayload::IbdBlock(_) => CalicodMessagePayloadType::IbdBlock,
            CalicodMessagePayload::InvRelayBlock(_) => CalicodMessagePayloadType::InvRelayBlock,
            CalicodMessagePayload::InvTransactions(_) => CalicodMessagePayloadType::InvTransactions,
            CalicodMessagePayload::Ping(_) => CalicodMessagePayloadType::Ping,
            CalicodMessagePayload::Pong(_) => CalicodMessagePayloadType::Pong,
            CalicodMessagePayload::Verack(_) => CalicodMessagePayloadType::Verack,
            CalicodMessagePayload::Version(_) => CalicodMessagePayloadType::Version,
            CalicodMessagePayload::TransactionNotFound(_) => CalicodMessagePayloadType::TransactionNotFound,
            CalicodMessagePayload::Reject(_) => CalicodMessagePayloadType::Reject,
            CalicodMessagePayload::PruningPointUtxoSetChunk(_) => CalicodMessagePayloadType::PruningPointUtxoSetChunk,
            CalicodMessagePayload::RequestIbdBlocks(_) => CalicodMessagePayloadType::RequestIbdBlocks,
            CalicodMessagePayload::UnexpectedPruningPoint(_) => CalicodMessagePayloadType::UnexpectedPruningPoint,
            CalicodMessagePayload::IbdBlockLocator(_) => CalicodMessagePayloadType::IbdBlockLocator,
            CalicodMessagePayload::IbdBlockLocatorHighestHash(_) => CalicodMessagePayloadType::IbdBlockLocatorHighestHash,
            CalicodMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                CalicodMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            CalicodMessagePayload::DonePruningPointUtxoSetChunks(_) => CalicodMessagePayloadType::DonePruningPointUtxoSetChunks,
            CalicodMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                CalicodMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            CalicodMessagePayload::BlockWithTrustedData(_) => CalicodMessagePayloadType::BlockWithTrustedData,
            CalicodMessagePayload::DoneBlocksWithTrustedData(_) => CalicodMessagePayloadType::DoneBlocksWithTrustedData,
            CalicodMessagePayload::RequestPruningPointAndItsAnticone(_) => {
                CalicodMessagePayloadType::RequestPruningPointAndItsAnticone
            }
            CalicodMessagePayload::BlockHeaders(_) => CalicodMessagePayloadType::BlockHeaders,
            CalicodMessagePayload::RequestNextHeaders(_) => CalicodMessagePayloadType::RequestNextHeaders,
            CalicodMessagePayload::DoneHeaders(_) => CalicodMessagePayloadType::DoneHeaders,
            CalicodMessagePayload::RequestPruningPointUtxoSet(_) => CalicodMessagePayloadType::RequestPruningPointUtxoSet,
            CalicodMessagePayload::RequestHeaders(_) => CalicodMessagePayloadType::RequestHeaders,
            CalicodMessagePayload::RequestBlockLocator(_) => CalicodMessagePayloadType::RequestBlockLocator,
            CalicodMessagePayload::PruningPoints(_) => CalicodMessagePayloadType::PruningPoints,
            CalicodMessagePayload::RequestPruningPointProof(_) => CalicodMessagePayloadType::RequestPruningPointProof,
            CalicodMessagePayload::PruningPointProof(_) => CalicodMessagePayloadType::PruningPointProof,
            CalicodMessagePayload::Ready(_) => CalicodMessagePayloadType::Ready,
            CalicodMessagePayload::BlockWithTrustedDataV4(_) => CalicodMessagePayloadType::BlockWithTrustedDataV4,
            CalicodMessagePayload::TrustedData(_) => CalicodMessagePayloadType::TrustedData,
            CalicodMessagePayload::RequestIbdChainBlockLocator(_) => CalicodMessagePayloadType::RequestIbdChainBlockLocator,
            CalicodMessagePayload::IbdChainBlockLocator(_) => CalicodMessagePayloadType::IbdChainBlockLocator,
            CalicodMessagePayload::RequestAntipast(_) => CalicodMessagePayloadType::RequestAntipast,
            CalicodMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                CalicodMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
        }
    }
}
