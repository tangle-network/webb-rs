use subxt::balances::*;
use subxt::extrinsic::*;
use subxt::sp_runtime::generic::Header;
use subxt::sp_runtime::traits::{BlakeTwo256, IdentifyAccount, Verify};
use subxt::sp_runtime::{MultiSignature, OpaqueExtrinsic};
use subxt::system::*;
use subxt::{sp_core, EventTypeRegistry};

use crate::pallet;

pub use subxt;

/// an easy way to extract the balance type from `T`
pub type BalanceOf<T> = <T as Balances>::Balance;

/// Alias to 512-bit hash when used in the context of a transaction signature on
/// the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it
/// equivalent to the public key of our transaction signing scheme.
pub type AccountId =
    <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of
/// them, but you never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Webb Runtime with `mixer` pallet.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WebbRuntime;

impl subxt::Runtime for WebbRuntime {
    type Extra = DefaultExtra<Self>;
    type Signature = Signature;

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system();
        event_type_registry.with_balances();
        subxt::register_default_type_sizes(event_type_registry);
        event_type_registry.register_type_size::<Balance>("BalanceOf<T>");
        event_type_registry.register_type_size::<pallet::TreeId>("TreeId");
        event_type_registry.register_type_size::<pallet::KeyId>("KeyId");
        event_type_registry
            .register_type_size::<pallet::ScalarData>("ScalarData");
        event_type_registry
            .register_type_size::<pallet::Nullifier>("Nullifier");
        event_type_registry
            .register_type_size::<pallet::Commitment>("Commitment");
        // ORML stuff
        event_type_registry.register_type_size::<Balance>("T::Balance");
        event_type_registry.register_type_size::<AccountId>("T::AccountId");
        event_type_registry
            .register_type_size::<pallet::CurrencyId>("T::CurrencyId");
        event_type_registry
            .register_type_size::<pallet::CurrencyId>("CurrencyIdOf<T>");
        event_type_registry.register_type_size::<pallet::Amount>("Amount");
        event_type_registry.register_type_size::<pallet::Amount>("AmountOf<T>");
        event_type_registry
            .register_type_size::<pallet::BlockLength>("BlockLength");
        // EVM Stuff
        event_type_registry.register_type_size::<ethereum_types::H160>("H160");
        event_type_registry.register_type_size::<ethereum_types::H256>("H256");
        event_type_registry.register_type_size::<ethereum_types::U256>("U256");
        event_type_registry.register_type_size::<Vec<u8>>("Log");
        event_type_registry.register_type_size::<i64>("ExitReason");
    }
}

impl System for WebbRuntime {
    type AccountData = AccountData<BalanceOf<Self>>;
    type AccountId = AccountId;
    type Address = AccountId;
    type BlockNumber = u32;
    type Extrinsic = OpaqueExtrinsic;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Index = Index;
}

impl Balances for WebbRuntime {
    type Balance = Balance;
}

impl pallet::mixer::Mixer for WebbRuntime {
    type Commitment = pallet::Commitment;
    type CurrencyId = pallet::CurrencyId;
    type Nullifier = pallet::Nullifier;
    type ScalarData = pallet::ScalarData;
}

impl pallet::merkle::Merkle for WebbRuntime {
    type TreeId = pallet::TreeId;
    type KeyId = pallet::KeyId;
}

#[cfg(all(test, feature = "integration-tests"))]
mod tests {
    use super::*;
    use crate::pallet::merkle::*;
    use crate::pallet::mixer::*;
    use crate::pallet::ScalarData;
    use sp_keyring::AccountKeyring;
    use subxt::PairSigner;

    type MixerTrees = MixerTreesStore<WebbRuntime>;
    type MixerTreeIds = MixerTreeIdsStore<WebbRuntime>;
    type CachedRoots = CachedRootsStore<WebbRuntime>;

    async fn get_client() -> subxt::Client<WebbRuntime> {
        subxt::ClientBuilder::new()
            .set_url("ws://127.0.0.1:9944")
            .build()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn get_all_mixers() {
        let client = get_client().await;
        let mut iter = client.iter::<MixerTrees>(None).await.unwrap();
        let mut tress = Vec::new();
        while let Some((_, info)) = iter.next().await.unwrap() {
            tress.push(info);
        }

        assert!(!tress.is_empty());

        let ids = client
            .fetch_or_default::<MixerTreeIds>(&MixerTreeIds::default(), None)
            .await
            .unwrap();
        assert!(!ids.is_empty());
    }

    #[tokio::test]
    async fn deposit() {
        let client = get_client().await;
        let signer = PairSigner::new(AccountKeyring::Alice.pair());
        let leaf = ScalarData([1u8; 32]);
        let result = client.deposit_and_watch(&signer, 0, vec![leaf]).await;
        dbg!(&result);
        assert!(result.is_ok());
        let xt = result.unwrap();
        println!("Hash: {:?}", xt.block);
        let maybe_block = client.block(Some(xt.block)).await.unwrap();
        let signed_block = maybe_block.unwrap();
        println!("Number: #{}", signed_block.block.header.number);

        let cached_roots = client
            .fetch(&CachedRoots::new(signed_block.block.header.number, 3), None)
            .await
            .unwrap();
        println!("Cached Roots: {:?}", cached_roots);
    }
}
