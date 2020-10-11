// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::*;
use crate::peer_manager::PeerManager;
use async_std::sync::channel;
use async_std::task;
use beacon::MockBeacon;
use chain::tipset_from_keys;
use db::MemoryDB;
use fil_types::verifier::MockVerifier;
use forest_car::load_car;
use forest_libp2p::{blocksync::make_blocksync_response, NetworkMessage};
use genesis::initialize_genesis;
use libp2p::core::PeerId;
use state_manager::StateManager;
use std::time::Duration;

async fn handle_requests<DB: BlockStore>(mut chan: Receiver<NetworkMessage>, db: DB) {
    loop {
        match chan.next().await {
            Some(NetworkMessage::BlockSyncRequest {
                request,
                response_channel,
                ..
            }) => response_channel
                .send(make_blocksync_response(&db, &request))
                .unwrap(),
            Some(event) => log::warn!("Other request sent to network: {:?}", event),
            None => break,
        }
    }
}

#[async_std::test]
async fn space_race_full_sync() {
    pretty_env_logger::init();

    let db = Arc::new(MemoryDB::default());

    let mut chain_store = ChainStore::new(db.clone());
    let state_manager = Arc::new(StateManager::new(db));

    // let (local_sender, _test_receiver) = channel(20);
    let (network_send, network_recv) = channel(20);

    // Initialize genesis using default (currently space-race) genesis
    let (genesis, _) = initialize_genesis(None, &mut chain_store, &state_manager).unwrap();
    let chain_store = Arc::new(chain_store);
    let genesis = Arc::new(genesis);

    // TODO this is causing the failure, need to test with space race beacon
    let beacon = Arc::new(MockBeacon::new(Duration::from_secs(1)));

    let peer = PeerId::random();
    let peer_manager = PeerManager::default();
    peer_manager.update_peer_head(peer, None).await;
    let network = SyncNetworkContext::new(network_send, Arc::new(peer_manager));

    let provider_db = MemoryDB::default();
    // TODO use shared export
    let bytes = include_bytes!("chain.car");
    let cids: Vec<Cid> = load_car(&provider_db, bytes.as_ref()).unwrap();
    let ts = tipset_from_keys(&provider_db, &TipsetKeys::new(cids)).unwrap();
    let target = Arc::new(ts);

    let worker = SyncWorker {
        state: Default::default(),
        beacon,
        state_manager,
        chain_store,
        network,
        genesis,
        bad_blocks: Default::default(),
        // Mock verifier for now, but can test FullVerifier (depends on params fetched)
        verifier: PhantomData::<MockVerifier>::default(),
    };

    // Setup process to handle requests from syncer
    task::spawn(async { handle_requests(network_recv, provider_db).await });

    worker.sync(target).await.unwrap();
}