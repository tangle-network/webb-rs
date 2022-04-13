use subxt::{DefaultConfig, SubstrateExtrinsicParams};
use webb::substrate::dkg_runtime::api::runtime_types::webb_proposals::header::{TypedChainId, ResourceId};
use webb::substrate::dkg_runtime::api::runtime_types::webb_proposals::nonce::Nonce;
use webb::substrate::dkg_runtime::api::{dkg_proposals, RuntimeApi};

type DKGRuntimeApi =
    RuntimeApi<DefaultConfig, SubstrateExtrinsicParams<DefaultConfig>>;

const URL: &str = "ws://localhost:9944";

async fn get_runtime_api() -> anyhow::Result<DKGRuntimeApi> {
    let api: DKGRuntimeApi = subxt::ClientBuilder::new()
        .set_url(URL)
        .build()
        .await?
        .to_runtime_api();
    Ok(api)
}

#[tokio::test]
#[ignore = "need to be run manually"]
async fn read_chain_nonce() {
    let api = get_runtime_api().await.unwrap();
    let chain_id = TypedChainId::Evm(5001);
    let result = api
        .storage()
        .dkg_proposals()
        .chain_nonces(&chain_id, None)
        .await
        .unwrap();
    assert_eq!(result, Some(Nonce(0)));
    let unkonwn_chain_id = TypedChainId::Evm(5000);
    let result = api
        .storage()
        .dkg_proposals()
        .chain_nonces(&unkonwn_chain_id, None)
        .await
        .unwrap();
    assert_eq!(result, None);
}

#[tokio::test]
#[ignore = "this needs a local node running"]
async fn acknowledge_proposal_works() -> anyhow::Result<()> {
    let api = get_runtime_api().await?;
    let nonce = Nonce(0);
    let src_id = TypedChainId::Evm(5001);
    let r_id: [u8; 32] = hex::decode(
        "0000000000000000e69a847cd5bc0c9480ada0b339d7f0a8cac2b6670000138a",
    )?
    .try_into()
    .unwrap();
    let r_id = ResourceId(r_id);
    let prop = hex::decode("0000000000000000e69a847cd5bc0c9480ada0b339d7f0a8cac2b6670000138a0000000000000000891300000000000003c951dfd2ab1e3e2864239ad09256b25ebadd164d53445c435bb31f036f3d36")?;
    let eve = sp_keyring::AccountKeyring::Eve;
    let signer = subxt::PairSigner::new(eve.pair());
    let result = api
        .tx()
        .dkg_proposals()
        .acknowledge_proposal(nonce.clone(), src_id, r_id, prop)
        .sign_and_submit_then_watch_default(&signer)
        .await?
        .wait_for_finalized_success()
        .await?;
    let event = result
        .find_first::<dkg_proposals::events::ProposalSucceeded>()?
        .expect("event not found");
    assert_eq!(event.proposal_nonce, nonce);
    Ok(())
}
