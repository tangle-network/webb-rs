use subxt::config::PolkadotConfig;
use webb::substrate::tangle_runtime::api::dkg_proposals;
use webb::substrate::tangle_runtime::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use webb::substrate::tangle_runtime::api::runtime_types::webb_proposals::header::TypedChainId;
use webb::substrate::tangle_runtime::api::runtime_types::webb_proposals::nonce::Nonce;
use webb::substrate::tangle_runtime;
use webb::substrate::tangle_runtime::api::runtime_types::webb_proposals::proposal::{Proposal, ProposalKind};

const URL: &str = "ws://localhost:9944";

async fn get_runtime_api() -> anyhow::Result<subxt::OnlineClient<PolkadotConfig>>
{
    let api = subxt::OnlineClient::<PolkadotConfig>::from_url(URL).await?;
    Ok(api)
}

#[tokio::test]
#[ignore = "need to be run manually"]
async fn read_chain_nonce() -> anyhow::Result<()> {
    let client = get_runtime_api().await.unwrap();
    let chain_id = TypedChainId::Evm(5001);
    let nonce_addr = tangle_runtime::api::storage()
        .dkg_proposals()
        .chain_nonces(chain_id);

    let result = client
        .storage()
        .at_latest()
        .await?
        .fetch(&nonce_addr)
        .await?;
    assert_eq!(result, Some(Nonce(0)));
    let unkonwn_chain_id = TypedChainId::Evm(5000);
    let nonce_addr = tangle_runtime::api::storage()
        .dkg_proposals()
        .chain_nonces(unkonwn_chain_id);
    let result = client
        .storage()
        .at_latest()
        .await?
        .fetch(&nonce_addr)
        .await?;
    assert_eq!(result, None);
    Ok(())
}

#[tokio::test]
#[ignore = "this needs a local node running"]
async fn acknowledge_proposal_works() -> anyhow::Result<()> {
    let client = get_runtime_api().await?;
    let nonce = Nonce(0);
    let prop = hex::decode("0000000000000000e69a847cd5bc0c9480ada0b339d7f0a8cac2b6670000138a0000000000000000891300000000000003c951dfd2ab1e3e2864239ad09256b25ebadd164d53445c435bb31f036f3d36")?;
    let eve = subxt_signer::sr25519::dev::eve();

    let acknowlege_proposal_tx = tangle_runtime::api::tx()
        .dkg_proposals()
        .acknowledge_proposal(Proposal::Unsigned {
            kind: ProposalKind::AnchorUpdate,
            data: BoundedVec(prop),
        });
    let result = client
        .tx()
        .sign_and_submit_then_watch_default(&acknowlege_proposal_tx, &eve)
        .await?
        .wait_for_finalized_success()
        .await?;

    let event = result
        .find_first::<dkg_proposals::events::ProposalSucceeded>()?
        .expect("event not found");
    assert_eq!(event.proposal_nonce, nonce);
    Ok(())
}
