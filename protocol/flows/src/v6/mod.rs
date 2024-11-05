use crate::v5::{
    address::{ReceiveAddressesFlow, SendAddressesFlow},
    blockrelay::{flow::HandleRelayInvsFlow, handle_requests::HandleRelayBlockRequests},
    ibd::IbdFlow,
    ping::{ReceivePingsFlow, SendPingsFlow},
    request_antipast::HandleAntipastRequests,
    request_block_locator::RequestBlockLocatorFlow,
    request_headers::RequestHeadersFlow,
    request_ibd_blocks::HandleIbdBlockRequests,
    request_ibd_chain_block_locator::RequestIbdChainBlockLocatorFlow,
    request_pp_proof::RequestPruningPointProofFlow,
    request_pruning_point_utxo_set::RequestPruningPointUtxoSetFlow,
    txrelay::flow::{RelayTransactionsFlow, RequestTransactionsFlow},
};
use crate::{flow_context::FlowContext, flow_trait::Flow};

use calico_p2p_lib::{CalicodMessagePayloadType, Router, SharedIncomingRoute};
use calico_utils::channel;
use std::sync::Arc;

use crate::v6::request_pruning_point_and_anticone::PruningPointAndItsAnticoneRequestsFlow;

pub(crate) mod request_pruning_point_and_anticone;

pub fn register(ctx: FlowContext, router: Arc<Router>) -> Vec<Box<dyn Flow>> {
    // IBD flow <-> invs flow communication uses a job channel in order to always
    // maintain at most a single pending job which can be updated
    let (ibd_sender, relay_receiver) = channel::job();

    let mut flows: Vec<Box<dyn Flow>> = vec![
        Box::new(IbdFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![
                CalicodMessagePayloadType::BlockHeaders,
                CalicodMessagePayloadType::DoneHeaders,
                CalicodMessagePayloadType::IbdBlockLocatorHighestHash,
                CalicodMessagePayloadType::IbdBlockLocatorHighestHashNotFound,
                CalicodMessagePayloadType::BlockWithTrustedDataV4,
                CalicodMessagePayloadType::DoneBlocksWithTrustedData,
                CalicodMessagePayloadType::IbdChainBlockLocator,
                CalicodMessagePayloadType::IbdBlock,
                CalicodMessagePayloadType::TrustedData,
                CalicodMessagePayloadType::PruningPoints,
                CalicodMessagePayloadType::PruningPointProof,
                CalicodMessagePayloadType::UnexpectedPruningPoint,
                CalicodMessagePayloadType::PruningPointUtxoSetChunk,
                CalicodMessagePayloadType::DonePruningPointUtxoSetChunks,
            ]),
            relay_receiver,
        )),
        Box::new(HandleRelayBlockRequests::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![CalicodMessagePayloadType::RequestRelayBlocks]),
        )),
        Box::new(ReceivePingsFlow::new(ctx.clone(), router.clone(), router.subscribe(vec![CalicodMessagePayloadType::Ping]))),
        Box::new(SendPingsFlow::new(ctx.clone(), router.clone(), router.subscribe(vec![CalicodMessagePayloadType::Pong]))),
        Box::new(RequestHeadersFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![CalicodMessagePayloadType::RequestHeaders, CalicodMessagePayloadType::RequestNextHeaders]),
        )),
        Box::new(RequestPruningPointProofFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![CalicodMessagePayloadType::RequestPruningPointProof]),
        )),
        Box::new(RequestIbdChainBlockLocatorFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![CalicodMessagePayloadType::RequestIbdChainBlockLocator]),
        )),
        Box::new(PruningPointAndItsAnticoneRequestsFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![
                CalicodMessagePayloadType::RequestPruningPointAndItsAnticone,
                CalicodMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks,
            ]),
        )),
        Box::new(RequestPruningPointUtxoSetFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![
                CalicodMessagePayloadType::RequestPruningPointUtxoSet,
                CalicodMessagePayloadType::RequestNextPruningPointUtxoSetChunk,
            ]),
        )),
        Box::new(HandleIbdBlockRequests::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![CalicodMessagePayloadType::RequestIbdBlocks]),
        )),
        Box::new(HandleAntipastRequests::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![CalicodMessagePayloadType::RequestAntipast]),
        )),
        Box::new(RelayTransactionsFlow::new(
            ctx.clone(),
            router.clone(),
            router
                .subscribe_with_capacity(vec![CalicodMessagePayloadType::InvTransactions], RelayTransactionsFlow::invs_channel_size()),
            router.subscribe_with_capacity(
                vec![CalicodMessagePayloadType::Transaction, CalicodMessagePayloadType::TransactionNotFound],
                RelayTransactionsFlow::txs_channel_size(),
            ),
        )),
        Box::new(RequestTransactionsFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![CalicodMessagePayloadType::RequestTransactions]),
        )),
        Box::new(ReceiveAddressesFlow::new(ctx.clone(), router.clone(), router.subscribe(vec![CalicodMessagePayloadType::Addresses]))),
        Box::new(SendAddressesFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![CalicodMessagePayloadType::RequestAddresses]),
        )),
        Box::new(RequestBlockLocatorFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![CalicodMessagePayloadType::RequestBlockLocator]),
        )),
    ];

    let invs_route = router.subscribe_with_capacity(vec![CalicodMessagePayloadType::InvRelayBlock], ctx.block_invs_channel_size());
    let shared_invs_route = SharedIncomingRoute::new(invs_route);

    let num_relay_flows = (ctx.config.bps() as usize / 2).max(1);
    flows.extend((0..num_relay_flows).map(|_| {
        Box::new(HandleRelayInvsFlow::new(
            ctx.clone(),
            router.clone(),
            shared_invs_route.clone(),
            router.subscribe(vec![]),
            ibd_sender.clone(),
        )) as Box<dyn Flow>
    }));

    // The reject message is handled as a special case by the router
    // CalicodMessagePayloadType::Reject,

    // We do not register the below two messages since they are deprecated also in go-calico
    // CalicodMessagePayloadType::BlockWithTrustedData,
    // CalicodMessagePayloadType::IbdBlockLocator,

    flows
}
