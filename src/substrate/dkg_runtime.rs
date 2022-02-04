#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        Eq,
        PartialEq,
        Clone,
    )]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 4)]
        Grandpa(grandpa::Event),
        #[codec(index = 5)]
        Balances(balances::Event),
        #[codec(index = 7)]
        Sudo(sudo::Event),
        #[codec(index = 8)]
        ElectionProviderMultiPhase(election_provider_multi_phase::Event),
        #[codec(index = 9)]
        Staking(staking::Event),
        #[codec(index = 10)]
        Session(session::Event),
        #[codec(index = 12)]
        DKG(dkg::Event),
        #[codec(index = 13)]
        DKGProposals(dkg_proposals::Event),
        #[codec(index = 15)]
        DKGProposalHandler(dkg_proposal_handler::Event),
    }
    pub mod system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct fill_block {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for fill_block {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_heap_pages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for set_heap_pages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_code {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_code {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_code_without_checks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_code_without_checks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_storage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for set_storage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct kill_storage {
                pub keys:
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for kill_storage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct kill_prefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for kill_prefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct remark_with_event {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for remark_with_event {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    fill_block,
                    DispatchError,
                > {
                    let call = fill_block { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    remark,
                    DispatchError,
                > {
                    let call = remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_heap_pages,
                    DispatchError,
                > {
                    let call = set_heap_pages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_code,
                    DispatchError,
                > {
                    let call = set_code { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_code_without_checks,
                    DispatchError,
                > {
                    let call = set_code_without_checks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_storage,
                    DispatchError,
                > {
                    let call = set_storage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    kill_storage,
                    DispatchError,
                > {
                    let call = kill_storage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    kill_prefix,
                    DispatchError,
                > {
                    let call = kill_prefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    remark_with_event,
                    DispatchError,
                > {
                    let call = remark_with_event { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ExtrinsicSuccess {
                pub dispatch_info:
                    runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info:
                    runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct CodeUpdated;
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct NewAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct KilledAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Remarked {
                pub sender: ::subxt::sp_core::crypto::AccountId32,
                pub hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value =
                    runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u64,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ExtrinsicData(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value = runtime_types::sp_runtime::generic::digest::Digest;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::dkg_standalone_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value =
                    runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Account>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u64,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::H256,
                    ::subxt::BasicError,
                > {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, BlockHash>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExtrinsicData>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::H256,
                    ::subxt::BasicError,
                > {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::BasicError,
                > {
                    let entry = Digest;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn events(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::dkg_standalone_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EventTopics>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_system::LastRuntimeUpgradeInfo,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::BasicError,
                > {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::BasicError,
                > {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::BasicError,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn block_weights(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockWeights,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 242u8, 5u8, 42u8, 1u8, 0u8, 0u8, 0u8, 0u8,
                            32u8, 74u8, 169u8, 209u8, 1u8, 0u8, 0u8, 64u8,
                            89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8, 192u8,
                            110u8, 150u8, 166u8, 46u8, 1u8, 0u8, 0u8, 1u8, 0u8,
                            152u8, 247u8, 62u8, 93u8, 1u8, 0u8, 0u8, 1u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 89u8,
                            115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8, 192u8, 246u8,
                            232u8, 16u8, 163u8, 1u8, 0u8, 0u8, 1u8, 0u8, 32u8,
                            74u8, 169u8, 209u8, 1u8, 0u8, 0u8, 1u8, 0u8, 136u8,
                            82u8, 106u8, 116u8, 0u8, 0u8, 0u8, 64u8, 89u8,
                            115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn block_length(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockLength,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 60u8, 0u8, 0u8, 0u8, 80u8, 0u8, 0u8, 0u8,
                            80u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn block_hash_count(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[96u8, 9u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn db_weight(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::RuntimeDbWeight,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            64u8, 120u8, 125u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            225u8, 245u8, 5u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn version(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_version::RuntimeVersion,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            76u8, 100u8, 107u8, 103u8, 45u8, 115u8, 116u8,
                            97u8, 110u8, 100u8, 97u8, 108u8, 111u8, 110u8,
                            101u8, 45u8, 110u8, 111u8, 100u8, 101u8, 76u8,
                            100u8, 107u8, 103u8, 45u8, 115u8, 116u8, 97u8,
                            110u8, 100u8, 97u8, 108u8, 111u8, 110u8, 101u8,
                            45u8, 110u8, 111u8, 100u8, 101u8, 1u8, 0u8, 0u8,
                            0u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 44u8,
                            223u8, 106u8, 203u8, 104u8, 153u8, 7u8, 96u8,
                            155u8, 4u8, 0u8, 0u8, 0u8, 55u8, 227u8, 151u8,
                            252u8, 124u8, 145u8, 245u8, 228u8, 1u8, 0u8, 0u8,
                            0u8, 64u8, 254u8, 58u8, 212u8, 1u8, 248u8, 149u8,
                            154u8, 5u8, 0u8, 0u8, 0u8, 210u8, 188u8, 152u8,
                            151u8, 238u8, 208u8, 143u8, 21u8, 3u8, 0u8, 0u8,
                            0u8, 247u8, 139u8, 39u8, 139u8, 229u8, 63u8, 69u8,
                            76u8, 2u8, 0u8, 0u8, 0u8, 221u8, 113u8, 141u8,
                            92u8, 197u8, 50u8, 98u8, 212u8, 1u8, 0u8, 0u8, 0u8,
                            171u8, 60u8, 5u8, 114u8, 41u8, 31u8, 235u8, 139u8,
                            1u8, 0u8, 0u8, 0u8, 237u8, 153u8, 197u8, 172u8,
                            178u8, 94u8, 237u8, 245u8, 3u8, 0u8, 0u8, 0u8,
                            159u8, 95u8, 130u8, 43u8, 111u8, 212u8, 86u8,
                            246u8, 1u8, 0u8, 0u8, 0u8, 188u8, 157u8, 137u8,
                            144u8, 79u8, 91u8, 146u8, 63u8, 1u8, 0u8, 0u8, 0u8,
                            55u8, 200u8, 187u8, 19u8, 80u8, 169u8, 162u8,
                            168u8, 1u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8,
                        ][..],
                    )?)
                }
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u16,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(&mut &[42u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn random_material(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = RandomMaterial;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod timestamp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set,
                    DispatchError,
                > {
                    let call = set { now };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn now(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::BasicError,
                > {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn minimum_period(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[220u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod aura {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Aura";
                const STORAGE: &'static str = "Authorities";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: sp_consensus_aura :: sr25519 :: app_sr25519 :: Public > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Aura";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn authorities (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: sp_consensus_aura :: sr25519 :: app_sr25519 :: Public > , :: subxt :: BasicError >{
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct report_equivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for report_equivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct report_equivocation_unsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for report_equivocation_unsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct note_stalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for note_stalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof : runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    report_equivocation,
                    DispatchError,
                > {
                    let call = report_equivocation {
                        equivocation_proof: ::std::boxed::Box::new(
                            equivocation_proof,
                        ),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof : runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    report_equivocation_unsigned,
                    DispatchError,
                > {
                    let call = report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box::new(
                            equivocation_proof,
                        ),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    note_stalled,
                    DispatchError,
                > {
                    let call = note_stalled {
                        delay,
                        best_finalized_block_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Paused;
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Resumed;
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value = runtime_types::pallet_grandpa::StoredPendingChange<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn state(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<
                        ::core::primitive::u32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = State;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn set_id_session_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SetIdSession>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[232u8, 3u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod balances {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct transfer {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_balance {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for set_balance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct force_transfer {
                pub source: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct transfer_keep_alive {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer_keep_alive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct transfer_all {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for transfer_all {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct force_unreserve {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_unreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    transfer,
                    DispatchError,
                > {
                    let call = transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_balance,
                    DispatchError,
                > {
                    let call = set_balance {
                        who,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_transfer,
                    DispatchError,
                > {
                    let call = force_transfer {
                        source,
                        dest,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    transfer_keep_alive,
                    DispatchError,
                > {
                    let call = transfer_keep_alive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    transfer_all,
                    DispatchError,
                > {
                    let call = transfer_all { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_unreserve,
                    DispatchError,
                > {
                    let call = force_unreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Endowed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct DustLost {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Transfer {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct BalanceSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Reserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Unreserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ReserveRepatriated { pub from : :: subxt :: sp_core :: crypto :: AccountId32 , pub to : :: subxt :: sp_core :: crypto :: AccountId32 , pub amount : :: core :: primitive :: u128 , pub destination_status : runtime_types :: frame_support :: traits :: tokens :: misc :: BalanceStatus , }
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Deposit {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Slashed {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_balances::AccountData<
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value = runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Reserves(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: pallet_balances :: ReserveData < [:: core :: primitive :: u8 ; 8usize] , :: core :: primitive :: u128 > > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_balances::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<
                        ::core::primitive::u128,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Account>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn locks (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > , :: subxt :: BasicError >{
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Locks>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn reserves (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: pallet_balances :: ReserveData < [:: core :: primitive :: u8 ; 8usize] , :: core :: primitive :: u128 > > , :: subxt :: BasicError >{
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reserves>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn existential_deposit(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            213u8, 220u8, 50u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value =
                    runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value =
                    runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn next_fee_multiplier(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::BasicError,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn transaction_byte_fee(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u8,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(&mut &[5u8][..])?)
                }                pub fn weight_to_fee (& self) -> :: core :: result :: Result < :: std :: vec :: Vec < runtime_types :: frame_support :: weights :: WeightToFeeCoefficient < :: core :: primitive :: u128 > > , :: subxt :: BasicError >{
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            4u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 1u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod sudo {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct sudo {
                pub call: ::std::boxed::Box<
                    runtime_types::dkg_standalone_runtime::Call,
                >,
            }
            impl ::subxt::Call for sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct sudo_unchecked_weight {
                pub call: ::std::boxed::Box<
                    runtime_types::dkg_standalone_runtime::Call,
                >,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for sudo_unchecked_weight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_key {
                pub new: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for set_key {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct sudo_as {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub call: ::std::boxed::Box<
                    runtime_types::dkg_standalone_runtime::Call,
                >,
            }
            impl ::subxt::Call for sudo_as {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::dkg_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    sudo,
                    DispatchError,
                > {
                    let call = sudo {
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::dkg_standalone_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    sudo_unchecked_weight,
                    DispatchError,
                > {
                    let call = sudo_unchecked_weight {
                        call: ::std::boxed::Box::new(call),
                        weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_key,
                    DispatchError,
                > {
                    let call = set_key { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    call: runtime_types::dkg_standalone_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    sudo_as,
                    DispatchError,
                > {
                    let call = sudo_as {
                        who,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Sudid {
                pub sudo_result: ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            }
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct SudoAsDone {
                pub sudo_result: ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            }
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Key;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod election_provider_multi_phase {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct submit_unsigned { pub raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , pub witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , }
            impl ::subxt::Call for submit_unsigned {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit_unsigned";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_minimum_untrusted_score {
                pub maybe_next_score:
                    ::core::option::Option<[::core::primitive::u128; 3usize]>,
            }
            impl ::subxt::Call for set_minimum_untrusted_score {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_minimum_untrusted_score";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_emergency_election_result {
                pub supports: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::sp_npos_elections::Support<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                )>,
            }
            impl ::subxt::Call for set_emergency_election_result {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_emergency_election_result";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct submit { pub raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , pub num_signed_submissions : :: core :: primitive :: u32 , }
            impl ::subxt::Call for submit {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn submit_unsigned(
                    &self,
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 >,
                    witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    submit_unsigned,
                    DispatchError,
                > {
                    let call = submit_unsigned {
                        raw_solution: ::std::boxed::Box::new(raw_solution),
                        witness,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_minimum_untrusted_score(
                    &self,
                    maybe_next_score: ::core::option::Option<
                        [::core::primitive::u128; 3usize],
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_minimum_untrusted_score,
                    DispatchError,
                > {
                    let call = set_minimum_untrusted_score { maybe_next_score };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_emergency_election_result(
                    &self,
                    supports: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::sp_npos_elections::Support<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_emergency_election_result,
                    DispatchError,
                > {
                    let call = set_emergency_election_result { supports };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit(
                    &self,
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 >,
                    num_signed_submissions: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    submit,
                    DispatchError,
                > {
                    let call = submit {
                        raw_solution: ::std::boxed::Box::new(raw_solution),
                        num_signed_submissions,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event =
            runtime_types::pallet_election_provider_multi_phase::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct SolutionStored { pub election_compute : runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , pub prev_ejected : :: core :: primitive :: bool , }
            impl ::subxt::Event for SolutionStored {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SolutionStored";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ElectionFinalized { pub election_compute : :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute > , }
            impl ::subxt::Event for ElectionFinalized {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "ElectionFinalized";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Rewarded {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Slashed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct SignedPhaseStarted {
                pub round: ::core::primitive::u32,
            }
            impl ::subxt::Event for SignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SignedPhaseStarted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct UnsignedPhaseStarted {
                pub round: ::core::primitive::u32,
            }
            impl ::subxt::Event for UnsignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "UnsignedPhaseStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Round;
            impl ::subxt::StorageEntry for Round {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "Round";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPhase;
            impl ::subxt::StorageEntry for CurrentPhase {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "CurrentPhase";
                type Value =
                    runtime_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedSolution;
            impl ::subxt::StorageEntry for QueuedSolution {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "QueuedSolution";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: ReadySolution < :: subxt :: sp_core :: crypto :: AccountId32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Snapshot;
            impl ::subxt::StorageEntry for Snapshot {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "Snapshot";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: RoundSnapshot < :: subxt :: sp_core :: crypto :: AccountId32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DesiredTargets;
            impl ::subxt::StorageEntry for DesiredTargets {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "DesiredTargets";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SnapshotMetadata;
            impl ::subxt::StorageEntry for SnapshotMetadata {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SnapshotMetadata";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionNextIndex;
            impl ::subxt::StorageEntry for SignedSubmissionNextIndex {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionNextIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionIndices;
            impl ::subxt::StorageEntry for SignedSubmissionIndices {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionIndices";
                type Value = runtime_types :: frame_support :: storage :: bounded_btree_map :: BoundedBTreeMap < [:: core :: primitive :: u128 ; 3usize] , :: core :: primitive :: u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionsMap(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for SignedSubmissionsMap {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionsMap";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: dkg_standalone_runtime :: NposSolution16 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct MinimumUntrustedScore;
            impl ::subxt::StorageEntry for MinimumUntrustedScore {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "MinimumUntrustedScore";
                type Value = [::core::primitive::u128; 3usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn round(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = Round;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentPhase;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn queued_solution (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: ReadySolution < :: subxt :: sp_core :: crypto :: AccountId32 > > , :: subxt :: BasicError >{
                    let entry = QueuedSolution;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn snapshot (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: RoundSnapshot < :: subxt :: sp_core :: crypto :: AccountId32 > > , :: subxt :: BasicError >{
                    let entry = Snapshot;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn desired_targets(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = DesiredTargets;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn snapshot_metadata (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize > , :: subxt :: BasicError >{
                    let entry = SnapshotMetadata;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signed_submission_next_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = SignedSubmissionNextIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn signed_submission_indices (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_btree_map :: BoundedBTreeMap < [:: core :: primitive :: u128 ; 3usize] , :: core :: primitive :: u32 > , :: subxt :: BasicError >{
                    let entry = SignedSubmissionIndices;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn signed_submissions_map (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , :: subxt :: BasicError >{
                    let entry = SignedSubmissionsMap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signed_submissions_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SignedSubmissionsMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn minimum_untrusted_score(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u128; 3usize]>,
                    ::subxt::BasicError,
                > {
                    let entry = MinimumUntrustedScore;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn unsigned_phase(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn signed_phase(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn solution_improvement_threshold(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[160u8, 134u8, 1u8, 0u8][..],
                    )?)
                }
                pub fn offchain_repeat(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[5u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn miner_tx_priority(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            254u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                            127u8,
                        ][..],
                    )?)
                }
                pub fn miner_max_weight(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[192u8, 124u8, 144u8, 124u8, 45u8, 1u8, 0u8, 0u8]
                            [..],
                    )?)
                }
                pub fn signed_max_submissions(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[10u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn signed_max_weight(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[192u8, 124u8, 144u8, 124u8, 45u8, 1u8, 0u8, 0u8]
                            [..],
                    )?)
                }
                pub fn signed_reward_base(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 16u8, 165u8, 212u8, 232u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn signed_deposit_base(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 16u8, 165u8, 212u8, 232u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn signed_deposit_byte(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            85u8, 160u8, 252u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn signed_deposit_weight(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn voter_snapshot_per_block(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[16u8, 39u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn miner_max_length(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 0u8, 54u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod staking {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct bond {
                pub controller: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for bond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct bond_extra {
                #[codec(compact)]
                pub max_additional: ::core::primitive::u128,
            }
            impl ::subxt::Call for bond_extra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond_extra";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct unbond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for unbond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "unbond";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct withdraw_unbonded {
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for withdraw_unbonded {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "withdraw_unbonded";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct validate {
                pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
            }
            impl ::subxt::Call for validate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "validate";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct nominate {
                pub targets: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            impl ::subxt::Call for nominate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "nominate";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct chill;
            impl ::subxt::Call for chill {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_payee {
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for set_payee {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_payee";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_controller {
                pub controller: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for set_controller {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_controller";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_validator_count {
                #[codec(compact)]
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_validator_count {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_validator_count";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct increase_validator_count {
                #[codec(compact)]
                pub additional: ::core::primitive::u32,
            }
            impl ::subxt::Call for increase_validator_count {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "increase_validator_count";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct scale_validator_count {
                pub factor: runtime_types::sp_arithmetic::per_things::Percent,
            }
            impl ::subxt::Call for scale_validator_count {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "scale_validator_count";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct force_no_eras;
            impl ::subxt::Call for force_no_eras {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_no_eras";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct force_new_era;
            impl ::subxt::Call for force_new_era {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_invulnerables {
                pub invulnerables:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for set_invulnerables {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_invulnerables";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct force_unstake {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for force_unstake {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_unstake";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct force_new_era_always;
            impl ::subxt::Call for force_new_era_always {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era_always";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct cancel_deferred_slash {
                pub era: ::core::primitive::u32,
                pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Call for cancel_deferred_slash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "cancel_deferred_slash";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct payout_stakers {
                pub validator_stash: ::subxt::sp_core::crypto::AccountId32,
                pub era: ::core::primitive::u32,
            }
            impl ::subxt::Call for payout_stakers {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "payout_stakers";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct rebond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for rebond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "rebond";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_history_depth {
                #[codec(compact)]
                pub new_history_depth: ::core::primitive::u32,
                #[codec(compact)]
                pub era_items_deleted: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_history_depth {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_history_depth";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct reap_stash {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for reap_stash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "reap_stash";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct kick {
                pub who: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            impl ::subxt::Call for kick {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "kick";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_staking_configs {
                pub min_nominator_bond: ::core::primitive::u128,
                pub min_validator_bond: ::core::primitive::u128,
                pub max_nominator_count:
                    ::core::option::Option<::core::primitive::u32>,
                pub max_validator_count:
                    ::core::option::Option<::core::primitive::u32>,
                pub chill_threshold: ::core::option::Option<
                    runtime_types::sp_arithmetic::per_things::Percent,
                >,
                pub min_commission:
                    runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for set_staking_configs {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_staking_configs";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct chill_other {
                pub controller: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for chill_other {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill_other";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn bond(
                    &self,
                    controller: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    bond,
                    DispatchError,
                > {
                    let call = bond {
                        controller,
                        value,
                        payee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn bond_extra(
                    &self,
                    max_additional: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    bond_extra,
                    DispatchError,
                > {
                    let call = bond_extra { max_additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unbond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    unbond,
                    DispatchError,
                > {
                    let call = unbond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw_unbonded(
                    &self,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    withdraw_unbonded,
                    DispatchError,
                > {
                    let call = withdraw_unbonded { num_slashing_spans };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn validate(
                    &self,
                    prefs: runtime_types::pallet_staking::ValidatorPrefs,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    validate,
                    DispatchError,
                > {
                    let call = validate { prefs };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn nominate(
                    &self,
                    targets: ::std::vec::Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    nominate,
                    DispatchError,
                > {
                    let call = nominate { targets };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    chill,
                    DispatchError,
                > {
                    let call = chill {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_payee(
                    &self,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_payee,
                    DispatchError,
                > {
                    let call = set_payee { payee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_controller(
                    &self,
                    controller: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_controller,
                    DispatchError,
                > {
                    let call = set_controller { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validator_count(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_validator_count,
                    DispatchError,
                > {
                    let call = set_validator_count { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn increase_validator_count(
                    &self,
                    additional: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    increase_validator_count,
                    DispatchError,
                > {
                    let call = increase_validator_count { additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn scale_validator_count(
                    &self,
                    factor: runtime_types::sp_arithmetic::per_things::Percent,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    scale_validator_count,
                    DispatchError,
                > {
                    let call = scale_validator_count { factor };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_no_eras(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_no_eras,
                    DispatchError,
                > {
                    let call = force_no_eras {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_new_era,
                    DispatchError,
                > {
                    let call = force_new_era {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_invulnerables(
                    &self,
                    invulnerables: ::std::vec::Vec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_invulnerables,
                    DispatchError,
                > {
                    let call = set_invulnerables { invulnerables };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unstake(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_unstake,
                    DispatchError,
                > {
                    let call = force_unstake {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era_always(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_new_era_always,
                    DispatchError,
                > {
                    let call = force_new_era_always {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_deferred_slash(
                    &self,
                    era: ::core::primitive::u32,
                    slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    cancel_deferred_slash,
                    DispatchError,
                > {
                    let call = cancel_deferred_slash { era, slash_indices };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn payout_stakers(
                    &self,
                    validator_stash: ::subxt::sp_core::crypto::AccountId32,
                    era: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    payout_stakers,
                    DispatchError,
                > {
                    let call = payout_stakers {
                        validator_stash,
                        era,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn rebond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    rebond,
                    DispatchError,
                > {
                    let call = rebond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_history_depth(
                    &self,
                    new_history_depth: ::core::primitive::u32,
                    era_items_deleted: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_history_depth,
                    DispatchError,
                > {
                    let call = set_history_depth {
                        new_history_depth,
                        era_items_deleted,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reap_stash(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    reap_stash,
                    DispatchError,
                > {
                    let call = reap_stash {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kick(
                    &self,
                    who: ::std::vec::Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    kick,
                    DispatchError,
                > {
                    let call = kick { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_staking_configs(
                    &self,
                    min_nominator_bond: ::core::primitive::u128,
                    min_validator_bond: ::core::primitive::u128,
                    max_nominator_count: ::core::option::Option<
                        ::core::primitive::u32,
                    >,
                    max_validator_count: ::core::option::Option<
                        ::core::primitive::u32,
                    >,
                    chill_threshold: ::core::option::Option<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                    min_commission : runtime_types :: sp_arithmetic :: per_things :: Perbill,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_staking_configs,
                    DispatchError,
                > {
                    let call = set_staking_configs {
                        min_nominator_bond,
                        min_validator_bond,
                        max_nominator_count,
                        max_validator_count,
                        chill_threshold,
                        min_commission,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill_other(
                    &self,
                    controller: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    chill_other,
                    DispatchError,
                > {
                    let call = chill_other { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_staking::pallet::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct EraPaid(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for EraPaid {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "EraPaid";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Rewarded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct OldSlashingReportDiscarded(pub ::core::primitive::u32);
            impl ::subxt::Event for OldSlashingReportDiscarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "OldSlashingReportDiscarded";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct StakersElected;
            impl ::subxt::Event for StakersElected {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakersElected";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Bonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Bonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Bonded";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Unbonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unbonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Unbonded";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Withdrawn(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdrawn {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Withdrawn";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Kicked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Kicked {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Kicked";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct StakingElectionFailed;
            impl ::subxt::Event for StakingElectionFailed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakingElectionFailed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Chilled(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Chilled {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Chilled";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct PayoutStarted(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for PayoutStarted {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "PayoutStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HistoryDepth;
            impl ::subxt::StorageEntry for HistoryDepth {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "HistoryDepth";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorCount;
            impl ::subxt::StorageEntry for ValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinimumValidatorCount;
            impl ::subxt::StorageEntry for MinimumValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinimumValidatorCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Invulnerables;
            impl ::subxt::StorageEntry for Invulnerables {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Invulnerables";
                type Value =
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Bonded(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Bonded {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Bonded";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct MinNominatorBond;
            impl ::subxt::StorageEntry for MinNominatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinNominatorBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinValidatorBond;
            impl ::subxt::StorageEntry for MinValidatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinValidatorBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinCommission;
            impl ::subxt::StorageEntry for MinCommission {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinCommission";
                type Value = runtime_types::sp_arithmetic::per_things::Perbill;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Ledger(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Ledger {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Ledger";
                type Value = runtime_types::pallet_staking::StakingLedger<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Payee(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Payee {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Payee";
                type Value = runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Validators(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Validators";
                type Value = runtime_types::pallet_staking::ValidatorPrefs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct CounterForValidators;
            impl ::subxt::StorageEntry for CounterForValidators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CounterForValidators";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxValidatorsCount;
            impl ::subxt::StorageEntry for MaxValidatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxValidatorsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nominators(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Nominators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Nominators";
                type Value = runtime_types::pallet_staking::Nominations<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct CounterForNominators;
            impl ::subxt::StorageEntry for CounterForNominators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CounterForNominators";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxNominatorsCount;
            impl ::subxt::StorageEntry for MaxNominatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxNominatorsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentEra;
            impl ::subxt::StorageEntry for CurrentEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentEra";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveEra;
            impl ::subxt::StorageEntry for ActiveEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ActiveEra";
                type Value = runtime_types::pallet_staking::ActiveEraInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ErasStartSessionIndex(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasStartSessionIndex {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStartSessionIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasStakers(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakers {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakers";
                type Value = runtime_types::pallet_staking::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasStakersClipped(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakersClipped {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakersClipped";
                type Value = runtime_types::pallet_staking::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasValidatorPrefs(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasValidatorPrefs {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorPrefs";
                type Value = runtime_types::pallet_staking::ValidatorPrefs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasValidatorReward(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasValidatorReward {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorReward";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasRewardPoints(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasRewardPoints {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasRewardPoints";
                type Value = runtime_types::pallet_staking::EraRewardPoints<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ErasTotalStake(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasTotalStake {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasTotalStake";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct ForceEra;
            impl ::subxt::StorageEntry for ForceEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ForceEra";
                type Value = runtime_types::pallet_staking::Forcing;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SlashRewardFraction;
            impl ::subxt::StorageEntry for SlashRewardFraction {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SlashRewardFraction";
                type Value = runtime_types::sp_arithmetic::per_things::Perbill;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CanceledSlashPayout;
            impl ::subxt::StorageEntry for CanceledSlashPayout {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CanceledSlashPayout";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnappliedSlashes(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnappliedSlashes {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "UnappliedSlashes";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_staking::UnappliedSlash<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct BondedEras;
            impl ::subxt::StorageEntry for BondedEras {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "BondedEras";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorSlashInEra(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ValidatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorSlashInEra";
                type Value = (
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct NominatorSlashInEra(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for NominatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "NominatorSlashInEra";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct SlashingSpans(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SlashingSpans {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SlashingSpans";
                type Value =
                    runtime_types::pallet_staking::slashing::SlashingSpans;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct SpanSlash(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for SpanSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SpanSlash";
                type Value =
                    runtime_types::pallet_staking::slashing::SpanRecord<
                        ::core::primitive::u128,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct EarliestUnappliedSlash;
            impl ::subxt::StorageEntry for EarliestUnappliedSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "EarliestUnappliedSlash";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPlannedSession;
            impl ::subxt::StorageEntry for CurrentPlannedSession {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentPlannedSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OffendingValidators;
            impl ::subxt::StorageEntry for OffendingValidators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "OffendingValidators";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::core::primitive::bool,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_staking::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ChillThreshold;
            impl ::subxt::StorageEntry for ChillThreshold {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ChillThreshold";
                type Value = runtime_types::sp_arithmetic::per_things::Percent;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn history_depth(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = HistoryDepth;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = ValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn minimum_validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = MinimumValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn invulnerables(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Invulnerables;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bonded(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Bonded(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bonded_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Bonded>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn min_nominator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let entry = MinNominatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn min_validator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let entry = MinValidatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn min_commission(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::BasicError,
                > {
                    let entry = MinCommission;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn ledger(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::StakingLedger<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Ledger(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn ledger_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Ledger>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn payee(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Payee(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn payee_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Payee>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn validators(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::BasicError,
                > {
                    let entry = Validators(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validators_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Validators>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = CounterForValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_validators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = MaxValidatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominators(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::Nominations<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Nominators(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominators_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Nominators>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_nominators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = CounterForNominators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_nominators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = MaxNominatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::ActiveEraInfo,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ActiveEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ErasStartSessionIndex(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStartSessionIndex>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ErasStakers(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStakers>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers_clipped(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ErasStakersClipped(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_clipped_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStakersClipped>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_prefs(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::BasicError,
                > {
                    let entry = ErasValidatorPrefs(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_validator_prefs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorPrefs>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_reward(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = ErasValidatorReward(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_validator_reward_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorReward>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_reward_points(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::EraRewardPoints<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ErasRewardPoints(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_reward_points_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasRewardPoints>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_total_stake(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let entry = ErasTotalStake(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_total_stake_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasTotalStake>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn force_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Forcing,
                    ::subxt::BasicError,
                > {
                    let entry = ForceEra;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn slash_reward_fraction(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::BasicError,
                > {
                    let entry = SlashRewardFraction;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn canceled_slash_payout(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let entry = CanceledSlashPayout;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_staking::UnappliedSlash<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = UnappliedSlashes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnappliedSlashes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bonded_eras(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = BondedEras;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_slash_in_era(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = ValidatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn validator_slash_in_era_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ValidatorSlashInEra>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nominator_slash_in_era(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = NominatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominator_slash_in_era_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NominatorSlashInEra>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn slashing_spans(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::slashing::SlashingSpans,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = SlashingSpans(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn slashing_spans_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SlashingSpans>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn span_slash(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::slashing::SpanRecord<
                        ::core::primitive::u128,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = SpanSlash(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn span_slash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SpanSlash>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn earliest_unapplied_slash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = EarliestUnappliedSlash;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_planned_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentPlannedSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn offending_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::bool,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = OffendingValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn chill_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ChillThreshold;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn sessions_per_era(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[6u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn bonding_duration(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[160u8, 2u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn slash_defer_duration(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[168u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_nominator_rewarded_per_validator(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 1u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_nominations(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[16u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod session {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_keys {
                pub keys:
                    runtime_types::dkg_standalone_runtime::opaque::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_keys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct purge_keys;
            impl ::subxt::Call for purge_keys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "purge_keys";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_keys(
                    &self,
                    keys : runtime_types :: dkg_standalone_runtime :: opaque :: SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_keys,
                    DispatchError,
                > {
                    let call = set_keys { keys, proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn purge_keys(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    purge_keys,
                    DispatchError,
                > {
                    let call = purge_keys {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::Event for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Validators;
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "Validators";
                type Value =
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentIndex;
            impl ::subxt::StorageEntry for CurrentIndex {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "CurrentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedChanged;
            impl ::subxt::StorageEntry for QueuedChanged {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedChanged";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedKeys;
            impl ::subxt::StorageEntry for QueuedKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedKeys";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::dkg_standalone_runtime::opaque::SessionKeys,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DisabledValidators;
            impl ::subxt::StorageEntry for DisabledValidators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "DisabledValidators";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeys(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NextKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "NextKeys";
                type Value =
                    runtime_types::dkg_standalone_runtime::opaque::SessionKeys;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct KeyOwner(
                pub runtime_types::sp_core::crypto::KeyTypeId,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for KeyOwner {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "KeyOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Validators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_changed(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::BasicError,
                > {
                    let entry = QueuedChanged;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn queued_keys (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: std :: vec :: Vec < (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: dkg_standalone_runtime :: opaque :: SessionKeys ,) > , :: subxt :: BasicError >{
                    let entry = QueuedKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn disabled_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = DisabledValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn next_keys (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: dkg_standalone_runtime :: opaque :: SessionKeys > , :: subxt :: BasicError >{
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_keys_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextKeys>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: runtime_types::sp_core::crypto::KeyTypeId,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = KeyOwner(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn key_owner_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, KeyOwner>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod historical {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct HistoricalSessions(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for HistoricalSessions {
                const PALLET: &'static str = "Historical";
                const STORAGE: &'static str = "HistoricalSessions";
                type Value = (::subxt::sp_core::H256, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct StoredRange;
            impl ::subxt::StorageEntry for StoredRange {
                const PALLET: &'static str = "Historical";
                const STORAGE: &'static str = "StoredRange";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn historical_sessions(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = HistoricalSessions(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn historical_sessions_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HistoricalSessions>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn stored_range(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = StoredRange;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod dkg {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_threshold {
                pub new_threshold: ::core::primitive::u16,
            }
            impl ::subxt::Call for set_threshold {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "set_threshold";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_refresh_delay {
                pub new_delay: ::core::primitive::u8,
            }
            impl ::subxt::Call for set_refresh_delay {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "set_refresh_delay";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct submit_public_key {
                pub keys_and_signatures:
                    runtime_types::dkg_runtime_primitives::AggregatedPublicKeys,
            }
            impl ::subxt::Call for submit_public_key {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "submit_public_key";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct submit_next_public_key {
                pub keys_and_signatures:
                    runtime_types::dkg_runtime_primitives::AggregatedPublicKeys,
            }
            impl ::subxt::Call for submit_next_public_key {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "submit_next_public_key";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct submit_public_key_signature { pub signature_proposal : runtime_types :: dkg_runtime_primitives :: proposal :: RefreshProposalSigned , }
            impl ::subxt::Call for submit_public_key_signature {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "submit_public_key_signature";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_time_to_restart {
                pub interval: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_time_to_restart {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "set_time_to_restart";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_threshold(
                    &self,
                    new_threshold: ::core::primitive::u16,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_threshold,
                    DispatchError,
                > {
                    let call = set_threshold { new_threshold };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_refresh_delay(
                    &self,
                    new_delay: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_refresh_delay,
                    DispatchError,
                > {
                    let call = set_refresh_delay { new_delay };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_public_key(
                    &self,
                    keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    submit_public_key,
                    DispatchError,
                > {
                    let call = submit_public_key {
                        keys_and_signatures,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_next_public_key(
                    &self,
                    keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    submit_next_public_key,
                    DispatchError,
                > {
                    let call = submit_next_public_key {
                        keys_and_signatures,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_public_key_signature(
                    &self,
                    signature_proposal : runtime_types :: dkg_runtime_primitives :: proposal :: RefreshProposalSigned,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    submit_public_key_signature,
                    DispatchError,
                > {
                    let call =
                        submit_public_key_signature { signature_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_time_to_restart(
                    &self,
                    interval: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_time_to_restart,
                    DispatchError,
                > {
                    let call = set_time_to_restart { interval };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_dkg_metadata::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct PublicKeySubmitted {
                pub compressed_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                pub uncompressed_pub_key:
                    ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for PublicKeySubmitted {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "PublicKeySubmitted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct NextPublicKeySubmitted {
                pub compressed_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                pub uncompressed_pub_key:
                    ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for NextPublicKeySubmitted {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "NextPublicKeySubmitted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct NextPublicKeySignatureSubmitted {
                pub pub_key_sig: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for NextPublicKeySignatureSubmitted {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "NextPublicKeySignatureSubmitted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct UsedSignatures;
            impl ::subxt::StorageEntry for UsedSignatures {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "UsedSignatures";
                type Value =
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RefreshNonce;
            impl ::subxt::StorageEntry for RefreshNonce {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "RefreshNonce";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RefreshDelay;
            impl ::subxt::StorageEntry for RefreshDelay {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "RefreshDelay";
                type Value = runtime_types::sp_arithmetic::per_things::Permill;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RefreshInProgress;
            impl ::subxt::StorageEntry for RefreshInProgress {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "RefreshInProgress";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct TimeToRestart;
            impl ::subxt::StorageEntry for TimeToRestart {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "TimeToRestart";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextDKGPublicKey;
            impl ::subxt::StorageEntry for NextDKGPublicKey {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "NextDKGPublicKey";
                type Value = (
                    ::core::primitive::u64,
                    ::std::vec::Vec<::core::primitive::u8>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextPublicKeySignature;
            impl ::subxt::StorageEntry for NextPublicKeySignature {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "NextPublicKeySignature";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DKGPublicKey;
            impl ::subxt::StorageEntry for DKGPublicKey {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "DKGPublicKey";
                type Value = (
                    ::core::primitive::u64,
                    ::std::vec::Vec<::core::primitive::u8>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DKGPublicKeySignature;
            impl ::subxt::StorageEntry for DKGPublicKeySignature {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "DKGPublicKeySignature";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PreviousPublicKey;
            impl ::subxt::StorageEntry for PreviousPublicKey {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "PreviousPublicKey";
                type Value = (
                    ::core::primitive::u64,
                    ::std::vec::Vec<::core::primitive::u8>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HistoricalRounds(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for HistoricalRounds {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "HistoricalRounds";
                type Value =
                    runtime_types::pallet_dkg_metadata::types::RoundMetadata;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct SignatureThreshold;
            impl ::subxt::StorageEntry for SignatureThreshold {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "SignatureThreshold";
                type Value = ::core::primitive::u16;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "Authorities";
                type Value = ::std::vec::Vec<
                    runtime_types::dkg_runtime_primitives::crypto::Public,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthoritySetId;
            impl ::subxt::StorageEntry for AuthoritySetId {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "AuthoritySetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "NextAuthorities";
                type Value = ::std::vec::Vec<
                    runtime_types::dkg_runtime_primitives::crypto::Public,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentAuthoritiesAccounts;
            impl ::subxt::StorageEntry for CurrentAuthoritiesAccounts {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "CurrentAuthoritiesAccounts";
                type Value =
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthoritiesAccounts;
            impl ::subxt::StorageEntry for NextAuthoritiesAccounts {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "NextAuthoritiesAccounts";
                type Value =
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn used_signatures(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    let entry = UsedSignatures;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn refresh_nonce(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = RefreshNonce;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn refresh_delay(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Permill,
                    ::subxt::BasicError,
                > {
                    let entry = RefreshDelay;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn refresh_in_progress(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::BasicError,
                > {
                    let entry = RefreshInProgress;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn time_to_restart(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = TimeToRestart;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_dkg_public_key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u64,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = NextDKGPublicKey;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_public_key_signature(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextPublicKeySignature;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn dkg_public_key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        ::core::primitive::u64,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    ::subxt::BasicError,
                > {
                    let entry = DKGPublicKey;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn dkg_public_key_signature(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = DKGPublicKeySignature;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn previous_public_key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        ::core::primitive::u64,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    ::subxt::BasicError,
                > {
                    let entry = PreviousPublicKey;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn historical_rounds(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_dkg_metadata::types::RoundMetadata,
                    ::subxt::BasicError,
                > {
                    let entry = HistoricalRounds(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn historical_rounds_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HistoricalRounds>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn signature_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u16,
                    ::subxt::BasicError,
                > {
                    let entry = SignatureThreshold;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::dkg_runtime_primitives::crypto::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authority_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    let entry = AuthoritySetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::dkg_runtime_primitives::crypto::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_authorities_accounts(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentAuthoritiesAccounts;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities_accounts(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthoritiesAccounts;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn refresh_delay(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Permill,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[32u8, 161u8, 7u8, 0u8][..],
                    )?)
                }
                pub fn time_to_restart(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[3u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod dkg_proposals {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_maintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for set_maintainer {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct force_set_maintainer {
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for force_set_maintainer {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "force_set_maintainer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_threshold {
                pub threshold: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_threshold {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "set_threshold";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct set_resource {
                pub id: [::core::primitive::u8; 32usize],
                pub method: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_resource {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "set_resource";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct remove_resource {
                pub id: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Call for remove_resource {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "remove_resource";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct whitelist_chain {
                pub id: runtime_types::dkg_runtime_primitives::ChainIdType<
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for whitelist_chain {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "whitelist_chain";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct add_proposer {
                pub v: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for add_proposer {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "add_proposer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct remove_proposer {
                pub v: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for remove_proposer {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "remove_proposer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct acknowledge_proposal {
                pub nonce: ::core::primitive::u32,
                pub src_id: runtime_types::dkg_runtime_primitives::ChainIdType<
                    ::core::primitive::u32,
                >,
                pub r_id: [::core::primitive::u8; 32usize],
                pub prop: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for acknowledge_proposal {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "acknowledge_proposal";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct reject_proposal {
                pub nonce: ::core::primitive::u32,
                pub src_id: runtime_types::dkg_runtime_primitives::ChainIdType<
                    ::core::primitive::u32,
                >,
                pub r_id: [::core::primitive::u8; 32usize],
                pub prop: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for reject_proposal {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "reject_proposal";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct eval_vote_state {
                pub nonce: ::core::primitive::u32,
                pub src_id: runtime_types::dkg_runtime_primitives::ChainIdType<
                    ::core::primitive::u32,
                >,
                pub prop: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for eval_vote_state {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "eval_vote_state";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_maintainer,
                    DispatchError,
                > {
                    let call = set_maintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_maintainer(
                    &self,
                    new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_set_maintainer,
                    DispatchError,
                > {
                    let call = force_set_maintainer { new_maintainer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_threshold(
                    &self,
                    threshold: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_threshold,
                    DispatchError,
                > {
                    let call = set_threshold { threshold };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_resource(
                    &self,
                    id: [::core::primitive::u8; 32usize],
                    method: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_resource,
                    DispatchError,
                > {
                    let call = set_resource { id, method };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_resource(
                    &self,
                    id: [::core::primitive::u8; 32usize],
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    remove_resource,
                    DispatchError,
                > {
                    let call = remove_resource { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn whitelist_chain(
                    &self,
                    id: runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    whitelist_chain,
                    DispatchError,
                > {
                    let call = whitelist_chain { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_proposer(
                    &self,
                    v: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    add_proposer,
                    DispatchError,
                > {
                    let call = add_proposer { v };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proposer(
                    &self,
                    v: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    remove_proposer,
                    DispatchError,
                > {
                    let call = remove_proposer { v };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn acknowledge_proposal(
                    &self,
                    nonce: ::core::primitive::u32,
                    src_id: runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                    r_id: [::core::primitive::u8; 32usize],
                    prop: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    acknowledge_proposal,
                    DispatchError,
                > {
                    let call = acknowledge_proposal {
                        nonce,
                        src_id,
                        r_id,
                        prop,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_proposal(
                    &self,
                    nonce: ::core::primitive::u32,
                    src_id: runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                    r_id: [::core::primitive::u8; 32usize],
                    prop: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    reject_proposal,
                    DispatchError,
                > {
                    let call = reject_proposal {
                        nonce,
                        src_id,
                        r_id,
                        prop,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn eval_vote_state(
                    &self,
                    nonce: ::core::primitive::u32,
                    src_id: runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                    prop: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    eval_vote_state,
                    DispatchError,
                > {
                    let call = eval_vote_state {
                        nonce,
                        src_id,
                        prop,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_dkg_proposals::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct MaintainerSet {
                pub old_maintainer: ::core::option::Option<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub new_maintainer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for MaintainerSet {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "MaintainerSet";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ProposerThresholdChanged {
                pub new_threshold: ::core::primitive::u32,
            }
            impl ::subxt::Event for ProposerThresholdChanged {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposerThresholdChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ChainWhitelisted {
                pub chain_id:
                    runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
            }
            impl ::subxt::Event for ChainWhitelisted {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ChainWhitelisted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ProposerAdded {
                pub proposer_id: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ProposerAdded {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposerAdded";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ProposerRemoved {
                pub proposer_id: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ProposerRemoved {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposerRemoved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct VoteFor {
                pub chain_id:
                    runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                pub proposal_nonce: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for VoteFor {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "VoteFor";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct VoteAgainst {
                pub chain_id:
                    runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                pub proposal_nonce: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for VoteAgainst {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "VoteAgainst";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ProposalApproved {
                pub chain_id:
                    runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                pub proposal_nonce: ::core::primitive::u32,
            }
            impl ::subxt::Event for ProposalApproved {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposalApproved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ProposalRejected {
                pub chain_id:
                    runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                pub proposal_nonce: ::core::primitive::u32,
            }
            impl ::subxt::Event for ProposalRejected {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposalRejected";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ProposalSucceeded {
                pub chain_id:
                    runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                pub proposal_nonce: ::core::primitive::u32,
            }
            impl ::subxt::Event for ProposalSucceeded {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposalSucceeded";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ProposalFailed {
                pub chain_id:
                    runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                pub proposal_nonce: ::core::primitive::u32,
            }
            impl ::subxt::Event for ProposalFailed {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposalFailed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ProposersReset {
                pub proposers:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Event for ProposersReset {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposersReset";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Maintainer;
            impl ::subxt::StorageEntry for Maintainer {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "Maintainer";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ChainNonces(
                pub  runtime_types::dkg_runtime_primitives::ChainIdType<
                    ::core::primitive::u32,
                >,
            );
            impl ::subxt::StorageEntry for ChainNonces {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "ChainNonces";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct ProposerThreshold;
            impl ::subxt::StorageEntry for ProposerThreshold {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "ProposerThreshold";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Proposers(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Proposers {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "Proposers";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct ProposerCount;
            impl ::subxt::StorageEntry for ProposerCount {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "ProposerCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Votes(
                pub  runtime_types::dkg_runtime_primitives::ChainIdType<
                    ::core::primitive::u32,
                >,
                pub  (
                    ::core::primitive::u32,
                    ::std::vec::Vec<::core::primitive::u8>,
                ),
            );
            impl ::subxt::StorageEntry for Votes {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "Votes";
                type Value =
                    runtime_types::pallet_dkg_proposals::types::ProposalVotes<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct Resources(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for Resources {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "Resources";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct ApprovedPendingProposals(pub ::core::primitive::u8);
            impl ::subxt::StorageEntry for ApprovedPendingProposals {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "ApprovedPendingProposals";
                type Value =
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn maintainer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Maintainer;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn chain_nonces(
                    &self,
                    _0: runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ChainNonces(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn chain_nonces_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ChainNonces>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn proposer_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = ProposerThreshold;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposers(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::BasicError,
                > {
                    let entry = Proposers(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Proposers>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn proposer_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = ProposerCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn votes (& self , _0 : runtime_types :: dkg_runtime_primitives :: ChainIdType < :: core :: primitive :: u32 > , _1 : (:: core :: primitive :: u32 , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_dkg_proposals :: types :: ProposalVotes < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , :: subxt :: BasicError >{
                    let entry = Votes(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn votes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Votes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn resources(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Resources(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn resources_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Resources>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn approved_pending_proposals(
                    &self,
                    _0: ::core::primitive::u8,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ApprovedPendingProposals(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn approved_pending_proposals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ApprovedPendingProposals>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn chain_identifier(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::dkg_runtime_primitives::ChainIdType<
                        ::core::primitive::u32,
                    >,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[1u8, 5u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn proposal_lifetime(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[240u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn dkg_account_id(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::PalletId,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            100u8, 119u8, 47u8, 100u8, 107u8, 103u8, 97u8, 99u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod mmr {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RootHash;
            impl ::subxt::StorageEntry for RootHash {
                const PALLET: &'static str = "MMR";
                const STORAGE: &'static str = "RootHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NumberOfLeaves;
            impl ::subxt::StorageEntry for NumberOfLeaves {
                const PALLET: &'static str = "MMR";
                const STORAGE: &'static str = "NumberOfLeaves";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nodes(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Nodes {
                const PALLET: &'static str = "MMR";
                const STORAGE: &'static str = "Nodes";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Identity,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn root_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::H256,
                    ::subxt::BasicError,
                > {
                    let entry = RootHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn number_of_leaves(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    let entry = NumberOfLeaves;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nodes(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = Nodes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nodes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Nodes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod dkg_proposal_handler {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct submit_signed_proposals { pub props : :: std :: vec :: Vec < runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType > , }
            impl ::subxt::Call for submit_signed_proposals {
                const PALLET: &'static str = "DKGProposalHandler";
                const FUNCTION: &'static str = "submit_signed_proposals";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct force_submit_unsigned_proposal { pub prop : runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType , }
            impl ::subxt::Call for force_submit_unsigned_proposal {
                const PALLET: &'static str = "DKGProposalHandler";
                const FUNCTION: &'static str = "force_submit_unsigned_proposal";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn submit_signed_proposals(
                    &self,
                    props : :: std :: vec :: Vec < runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    submit_signed_proposals,
                    DispatchError,
                > {
                    let call = submit_signed_proposals { props };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_submit_unsigned_proposal(
                    &self,
                    prop : runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_submit_unsigned_proposal,
                    DispatchError,
                > {
                    let call = force_submit_unsigned_proposal { prop };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event =
            runtime_types::pallet_dkg_proposal_handler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ProposalAdded (pub :: subxt :: sp_core :: crypto :: AccountId32 , pub runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType ,) ;
            impl ::subxt::Event for ProposalAdded {
                const PALLET: &'static str = "DKGProposalHandler";
                const EVENT: &'static str = "ProposalAdded";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ProposalSigned { pub chain_id : runtime_types :: dkg_runtime_primitives :: ChainIdType < :: core :: primitive :: u32 > , pub key : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , pub data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , pub signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , }
            impl ::subxt::Event for ProposalSigned {
                const PALLET: &'static str = "DKGProposalHandler";
                const EVENT: &'static str = "ProposalSigned";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct UnsignedProposalQueue (pub runtime_types :: dkg_runtime_primitives :: ChainIdType < :: core :: primitive :: u32 > , pub runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey) ;
            impl ::subxt::StorageEntry for UnsignedProposalQueue {
                const PALLET: &'static str = "DKGProposalHandler";
                const STORAGE: &'static str = "UnsignedProposalQueue";
                type Value = runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct SignedProposals (pub runtime_types :: dkg_runtime_primitives :: ChainIdType < :: core :: primitive :: u32 > , pub runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey) ;
            impl ::subxt::StorageEntry for SignedProposals {
                const PALLET: &'static str = "DKGProposalHandler";
                const STORAGE: &'static str = "SignedProposals";
                type Value = runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn unsigned_proposal_queue (& self , _0 : runtime_types :: dkg_runtime_primitives :: ChainIdType < :: core :: primitive :: u32 > , _1 : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType > , :: subxt :: BasicError >{
                    let entry = UnsignedProposalQueue(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn unsigned_proposal_queue_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnsignedProposalQueue>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn signed_proposals (& self , _0 : runtime_types :: dkg_runtime_primitives :: ChainIdType < :: core :: primitive :: u32 > , _1 : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType > , :: subxt :: BasicError >{
                    let entry = SignedProposals(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signed_proposals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SignedProposals>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_submissions_per_batch(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u16,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(&mut &[100u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod dkgmmr {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct DKGNextAuthorities;
            impl ::subxt::StorageEntry for DKGNextAuthorities {
                const PALLET: &'static str = "DKGMMR";
                const STORAGE: &'static str = "DKGNextAuthorities";
                type Value = runtime_types :: dkg_runtime_primitives :: mmr :: DKGNextAuthoritySet < :: subxt :: sp_core :: H256 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn dkg_next_authorities (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: dkg_runtime_primitives :: mmr :: DKGNextAuthoritySet < :: subxt :: sp_core :: H256 > , :: subxt :: BasicError >{
                    let entry = DKGNextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod dkg_runtime_primitives {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
            }
            pub mod mmr {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct DKGNextAuthoritySet<_0> {
                    pub id: ::core::primitive::u64,
                    pub len: ::core::primitive::u32,
                    pub root: _0,
                }
            }
            pub mod proposal {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum DKGPayloadKey {
                    #[codec(index = 0)]
                    EVMProposal(::core::primitive::u32),
                    #[codec(index = 1)]
                    RefreshVote(::core::primitive::u32),
                    #[codec(index = 2)]
                    AnchorUpdateProposal(::core::primitive::u32),
                    #[codec(index = 3)]
                    TokenAddProposal(::core::primitive::u32),
                    #[codec(index = 4)]
                    TokenRemoveProposal(::core::primitive::u32),
                    #[codec(index = 5)]
                    WrappingFeeUpdateProposal(::core::primitive::u32),
                    #[codec(index = 6)]
                    ResourceIdUpdateProposal(::core::primitive::u32),
                    #[codec(index = 7)]
                    MaxDepositLimitUpdateProposal(::core::primitive::u32),
                    #[codec(index = 8)]
                    MinWithdrawLimitUpdateProposal(::core::primitive::u32),
                    #[codec(index = 9)]
                    MaxExtLimitUpdateProposal(::core::primitive::u32),
                    #[codec(index = 10)]
                    MaxFeeLimitUpdateProposal(::core::primitive::u32),
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum ProposalType {
                    #[codec(index = 0)]
                    RefreshProposal {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    EVMUnsigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    EVMSigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    AnchorUpdate {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    AnchorUpdateSigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    TokenAdd {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 6)]
                    TokenAddSigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 7)]
                    TokenRemove {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    TokenRemoveSigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    WrappingFeeUpdate {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 10)]
                    WrappingFeeUpdateSigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 11)]
                    ResourceIdUpdate {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 12)]
                    ResourceIdUpdateSigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 13)]
                    MaxDepositLimitUpdate {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 14)]
                    MaxDepositLimitUpdateSigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 15)]
                    MinWithdrawalLimitUpdate {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 16)]
                    MinWithdrawalLimitUpdateSigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 17)]
                    MaxExtLimitUpdate {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 18)]
                    MaxExtLimitUpdateSigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 19)]
                    MaxFeeLimitUpdate {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 20)]
                    MaxFeeLimitUpdateSigned {
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct RefreshProposalSigned {
                    pub nonce: ::core::primitive::u32,
                    pub signature: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct AggregatedPublicKeys {
                pub keys_and_signatures: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum ChainIdType<_0> {
                #[codec(index = 0)]
                EVM(_0),
                #[codec(index = 1)]
                Substrate(_0),
                #[codec(index = 2)]
                RelayChain(::std::string::String, _0),
                #[codec(index = 3)]
                Parachain(::std::string::String, _0),
                #[codec(index = 4)]
                CosmosSDK(_0),
                #[codec(index = 5)]
                Solana(_0),
            }
        }
        pub mod dkg_standalone_runtime {
            use super::runtime_types;
            pub mod opaque {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct SessionKeys { pub aura : runtime_types :: sp_consensus_aura :: sr25519 :: app_sr25519 :: Public , pub grandpa : runtime_types :: sp_finality_grandpa :: app :: Public , pub dkg : runtime_types :: dkg_runtime_primitives :: crypto :: Public , }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Call {
                # [codec (index = 0)] System (runtime_types :: frame_system :: pallet :: Call ,) , # [codec (index = 2)] Timestamp (runtime_types :: pallet_timestamp :: pallet :: Call ,) , # [codec (index = 4)] Grandpa (runtime_types :: pallet_grandpa :: pallet :: Call ,) , # [codec (index = 5)] Balances (runtime_types :: pallet_balances :: pallet :: Call ,) , # [codec (index = 7)] Sudo (runtime_types :: pallet_sudo :: pallet :: Call ,) , # [codec (index = 8)] ElectionProviderMultiPhase (runtime_types :: pallet_election_provider_multi_phase :: pallet :: Call ,) , # [codec (index = 9)] Staking (runtime_types :: pallet_staking :: pallet :: pallet :: Call ,) , # [codec (index = 10)] Session (runtime_types :: pallet_session :: pallet :: Call ,) , # [codec (index = 12)] DKG (runtime_types :: pallet_dkg_metadata :: pallet :: Call ,) , # [codec (index = 13)] DKGProposals (runtime_types :: pallet_dkg_proposals :: pallet :: Call ,) , # [codec (index = 15)] DKGProposalHandler (runtime_types :: pallet_dkg_proposal_handler :: pallet :: Call ,) , }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Event {
                # [codec (index = 0)] System (runtime_types :: frame_system :: pallet :: Event ,) , # [codec (index = 4)] Grandpa (runtime_types :: pallet_grandpa :: pallet :: Event ,) , # [codec (index = 5)] Balances (runtime_types :: pallet_balances :: pallet :: Event ,) , # [codec (index = 7)] Sudo (runtime_types :: pallet_sudo :: pallet :: Event ,) , # [codec (index = 8)] ElectionProviderMultiPhase (runtime_types :: pallet_election_provider_multi_phase :: pallet :: Event ,) , # [codec (index = 9)] Staking (runtime_types :: pallet_staking :: pallet :: pallet :: Event ,) , # [codec (index = 10)] Session (runtime_types :: pallet_session :: pallet :: Event ,) , # [codec (index = 12)] DKG (runtime_types :: pallet_dkg_metadata :: pallet :: Event ,) , # [codec (index = 13)] DKGProposals (runtime_types :: pallet_dkg_proposals :: pallet :: Event ,) , # [codec (index = 15)] DKGProposalHandler (runtime_types :: pallet_dkg_proposal_handler :: pallet :: Event ,) , }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct NposSolution16 {
                pub votes1: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::core::primitive::u16,
                )>,
                pub votes2: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    (
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ),
                    ::core::primitive::u16,
                )>,
                pub votes3: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 2usize],
                    ::core::primitive::u16,
                )>,
                pub votes4: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 3usize],
                    ::core::primitive::u16,
                )>,
                pub votes5: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 4usize],
                    ::core::primitive::u16,
                )>,
                pub votes6: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 5usize],
                    ::core::primitive::u16,
                )>,
                pub votes7: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 6usize],
                    ::core::primitive::u16,
                )>,
                pub votes8: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 7usize],
                    ::core::primitive::u16,
                )>,
                pub votes9: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 8usize],
                    ::core::primitive::u16,
                )>,
                pub votes10: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 9usize],
                    ::core::primitive::u16,
                )>,
                pub votes11: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 10usize],
                    ::core::primitive::u16,
                )>,
                pub votes12: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 11usize],
                    ::core::primitive::u16,
                )>,
                pub votes13: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 12usize],
                    ::core::primitive::u16,
                )>,
                pub votes14: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 13usize],
                    ::core::primitive::u16,
                )>,
                pub votes15: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 14usize],
                    ::core::primitive::u16,
                )>,
                pub votes16: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 15usize],
                    ::core::primitive::u16,
                )>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Runtime;
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_btree_map {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct BoundedBTreeMap<_0, _1>(
                        pub ::std::collections::BTreeMap<_0, _1>,
                    );
                }
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Encode,
                            :: subxt :: codec :: Decode,
                            Debug,
                            Eq,
                            PartialEq,
                            Clone,
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class:
                        runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac:
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct CheckMortality(
                        pub runtime_types::sp_runtime::generic::era::Era,
                    );
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct CheckNonce(
                        #[codec(compact)] pub ::core::primitive::u32,
                    );
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct BlockLength {
                    pub max:
                        runtime_types::frame_support::weights::PerDispatchClass<
                            ::core::primitive::u32,
                        >,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct BlockWeights { pub base_block : :: core :: primitive :: u64 , pub max_block : :: core :: primitive :: u64 , pub per_class : runtime_types :: frame_support :: weights :: PerDispatchClass < runtime_types :: frame_system :: limits :: WeightsPerClass > , }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic:
                        ::core::option::Option<::core::primitive::u64>,
                    pub max_total:
                        ::core::option::Option<::core::primitive::u64>,
                    pub reserved:
                        ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    fill_block {
                        ratio:
                            runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 6)]
                    kill_storage {
                        keys: ::std::vec::Vec<
                            ::std::vec::Vec<::core::primitive::u8>,
                        >,
                    },
                    #[codec(index = 7)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess {
                        dispatch_info:
                            runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    ExtrinsicFailed {
                        dispatch_error:
                            runtime_types::sp_runtime::DispatchError,
                        dispatch_info:
                            runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    KilledAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    Remarked {
                        sender: ::subxt::sp_core::crypto::AccountId32,
                        hash: ::subxt::sp_core::H256,
                    },
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    KeepAlive,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Event {
                    # [codec (index = 0)] Endowed { account : :: subxt :: sp_core :: crypto :: AccountId32 , free_balance : :: core :: primitive :: u128 , } , # [codec (index = 1)] DustLost { account : :: subxt :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 2)] Transfer { from : :: subxt :: sp_core :: crypto :: AccountId32 , to : :: subxt :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 3)] BalanceSet { who : :: subxt :: sp_core :: crypto :: AccountId32 , free : :: core :: primitive :: u128 , reserved : :: core :: primitive :: u128 , } , # [codec (index = 4)] Reserved { who : :: subxt :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 5)] Unreserved { who : :: subxt :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 6)] ReserveRepatriated { from : :: subxt :: sp_core :: crypto :: AccountId32 , to : :: subxt :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , destination_status : runtime_types :: frame_support :: traits :: tokens :: misc :: BalanceStatus , } , # [codec (index = 7)] Deposit { who : :: subxt :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 8)] Withdraw { who : :: subxt :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 9)] Slashed { who : :: subxt :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_dkg_metadata {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Call {
                    # [codec (index = 0)] set_threshold { new_threshold : :: core :: primitive :: u16 , } , # [codec (index = 1)] set_refresh_delay { new_delay : :: core :: primitive :: u8 , } , # [codec (index = 2)] submit_public_key { keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys , } , # [codec (index = 3)] submit_next_public_key { keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys , } , # [codec (index = 4)] submit_public_key_signature { signature_proposal : runtime_types :: dkg_runtime_primitives :: proposal :: RefreshProposalSigned , } , # [codec (index = 5)] set_time_to_restart { interval : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidThreshold,
                    #[codec(index = 1)]
                    MustBeAQueuedAuthority,
                    #[codec(index = 2)]
                    MustBeAnActiveAuthority,
                    #[codec(index = 3)]
                    InvalidRefreshDelay,
                    #[codec(index = 4)]
                    InvalidPublicKeys,
                    #[codec(index = 5)]
                    AlreadySubmittedPublicKey,
                    #[codec(index = 6)]
                    AlreadySubmittedSignature,
                    #[codec(index = 7)]
                    UsedSignature,
                    #[codec(index = 8)]
                    InvalidSignature,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    PublicKeySubmitted {
                        compressed_pub_key:
                            ::std::vec::Vec<::core::primitive::u8>,
                        uncompressed_pub_key:
                            ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    NextPublicKeySubmitted {
                        compressed_pub_key:
                            ::std::vec::Vec<::core::primitive::u8>,
                        uncompressed_pub_key:
                            ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    NextPublicKeySignatureSubmitted {
                        pub_key_sig: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct RoundMetadata {
                    pub curr_round_pub_key:
                        ::std::vec::Vec<::core::primitive::u8>,
                    pub next_round_pub_key:
                        ::std::vec::Vec<::core::primitive::u8>,
                    pub refresh_signature:
                        ::std::vec::Vec<::core::primitive::u8>,
                }
            }
        }
        pub mod pallet_dkg_proposal_handler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Call {
                    # [codec (index = 0)] submit_signed_proposals { props : :: std :: vec :: Vec < runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType > , } , # [codec (index = 1)] force_submit_unsigned_proposal { prop : runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType , } , }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    NoneValue,
                    #[codec(index = 1)]
                    StorageOverflow,
                    #[codec(index = 2)]
                    ProposalFormatInvalid,
                    #[codec(index = 3)]
                    ProposalSignatureInvalid,
                    #[codec(index = 4)]
                    ProposalDoesNotExists,
                    #[codec(index = 5)]
                    ProposalAlreadyExists,
                    #[codec(index = 6)]
                    ChainIdInvalid,
                    #[codec(index = 7)]
                    ProposalsLengthOverflow,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Event {
                    # [codec (index = 0)] ProposalAdded (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: dkg_runtime_primitives :: proposal :: ProposalType ,) , # [codec (index = 1)] ProposalSigned { chain_id : runtime_types :: dkg_runtime_primitives :: ChainIdType < :: core :: primitive :: u32 > , key : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
            }
        }
        pub mod pallet_dkg_proposals {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    force_set_maintainer {
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    set_threshold { threshold: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    set_resource {
                        id: [::core::primitive::u8; 32usize],
                        method: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    remove_resource {
                        id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 5)]
                    whitelist_chain {
                        id: runtime_types::dkg_runtime_primitives::ChainIdType<
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 6)]
                    add_proposer {
                        v: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 7)]
                    remove_proposer {
                        v: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 8)]
                    acknowledge_proposal {
                        nonce: ::core::primitive::u32,
                        src_id:
                            runtime_types::dkg_runtime_primitives::ChainIdType<
                                ::core::primitive::u32,
                            >,
                        r_id: [::core::primitive::u8; 32usize],
                        prop: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    reject_proposal {
                        nonce: ::core::primitive::u32,
                        src_id:
                            runtime_types::dkg_runtime_primitives::ChainIdType<
                                ::core::primitive::u32,
                            >,
                        r_id: [::core::primitive::u8; 32usize],
                        prop: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 10)]
                    eval_vote_state {
                        nonce: ::core::primitive::u32,
                        src_id:
                            runtime_types::dkg_runtime_primitives::ChainIdType<
                                ::core::primitive::u32,
                            >,
                        prop: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidPermissions,
                    #[codec(index = 1)]
                    ThresholdNotSet,
                    #[codec(index = 2)]
                    InvalidChainId,
                    #[codec(index = 3)]
                    InvalidThreshold,
                    #[codec(index = 4)]
                    ChainNotWhitelisted,
                    #[codec(index = 5)]
                    ChainAlreadyWhitelisted,
                    #[codec(index = 6)]
                    ResourceDoesNotExist,
                    #[codec(index = 7)]
                    ProposerAlreadyExists,
                    #[codec(index = 8)]
                    ProposerInvalid,
                    #[codec(index = 9)]
                    MustBeProposer,
                    #[codec(index = 10)]
                    ProposerAlreadyVoted,
                    #[codec(index = 11)]
                    ProposalAlreadyExists,
                    #[codec(index = 12)]
                    ProposalDoesNotExist,
                    #[codec(index = 13)]
                    ProposalNotComplete,
                    #[codec(index = 14)]
                    ProposalAlreadyComplete,
                    #[codec(index = 15)]
                    ProposalExpired,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    MaintainerSet {
                        old_maintainer: ::core::option::Option<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        new_maintainer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    ProposerThresholdChanged {
                        new_threshold: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    ChainWhitelisted {
                        chain_id:
                            runtime_types::dkg_runtime_primitives::ChainIdType<
                                ::core::primitive::u32,
                            >,
                    },
                    #[codec(index = 3)]
                    ProposerAdded {
                        proposer_id: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    ProposerRemoved {
                        proposer_id: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    VoteFor {
                        chain_id:
                            runtime_types::dkg_runtime_primitives::ChainIdType<
                                ::core::primitive::u32,
                            >,
                        proposal_nonce: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    VoteAgainst {
                        chain_id:
                            runtime_types::dkg_runtime_primitives::ChainIdType<
                                ::core::primitive::u32,
                            >,
                        proposal_nonce: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 7)]
                    ProposalApproved {
                        chain_id:
                            runtime_types::dkg_runtime_primitives::ChainIdType<
                                ::core::primitive::u32,
                            >,
                        proposal_nonce: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    ProposalRejected {
                        chain_id:
                            runtime_types::dkg_runtime_primitives::ChainIdType<
                                ::core::primitive::u32,
                            >,
                        proposal_nonce: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    ProposalSucceeded {
                        chain_id:
                            runtime_types::dkg_runtime_primitives::ChainIdType<
                                ::core::primitive::u32,
                            >,
                        proposal_nonce: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ProposalFailed {
                        chain_id:
                            runtime_types::dkg_runtime_primitives::ChainIdType<
                                ::core::primitive::u32,
                            >,
                        proposal_nonce: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    ProposersReset {
                        proposers: ::std::vec::Vec<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum ProposalStatus {
                    #[codec(index = 0)]
                    Initiated,
                    #[codec(index = 1)]
                    Approved,
                    #[codec(index = 2)]
                    Rejected,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct ProposalVotes < _0 , _1 > { pub votes_for : :: std :: vec :: Vec < _0 > , pub votes_against : :: std :: vec :: Vec < _0 > , pub status : runtime_types :: pallet_dkg_proposals :: types :: ProposalStatus , pub expiry : _1 , }
            }
        }
        pub mod pallet_election_provider_multi_phase {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Call {
                    # [codec (index = 0)] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < [:: core :: primitive :: u128 ; 3usize] > , } , # [codec (index = 2)] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: sp_core :: crypto :: AccountId32 > ,) > , } , # [codec (index = 3)] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , num_signed_submissions : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    PreDispatchEarlySubmission,
                    #[codec(index = 1)]
                    PreDispatchWrongWinnerCount,
                    #[codec(index = 2)]
                    PreDispatchWeakSubmission,
                    #[codec(index = 3)]
                    SignedQueueFull,
                    #[codec(index = 4)]
                    SignedCannotPayDeposit,
                    #[codec(index = 5)]
                    SignedInvalidWitness,
                    #[codec(index = 6)]
                    SignedTooMuchWeight,
                    #[codec(index = 7)]
                    OcwCallWrongEra,
                    #[codec(index = 8)]
                    MissingSnapshotMetadata,
                    #[codec(index = 9)]
                    InvalidSubmissionIndex,
                    #[codec(index = 10)]
                    CallNotAllowed,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Event {
                    # [codec (index = 0)] SolutionStored { election_compute : runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , prev_ejected : :: core :: primitive :: bool , } , # [codec (index = 1)] ElectionFinalized { election_compute : :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute > , } , # [codec (index = 2)] Rewarded { account : :: subxt :: sp_core :: crypto :: AccountId32 , value : :: core :: primitive :: u128 , } , # [codec (index = 3)] Slashed { account : :: subxt :: sp_core :: crypto :: AccountId32 , value : :: core :: primitive :: u128 , } , # [codec (index = 4)] SignedPhaseStarted { round : :: core :: primitive :: u32 , } , # [codec (index = 5)] UnsignedPhaseStarted { round : :: core :: primitive :: u32 , } , }
            }
            pub mod signed {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct SignedSubmission < _0 , _1 , _2 > { pub who : _0 , pub deposit : _1 , pub raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < _2 > , pub reward : _1 , }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum ElectionCompute {
                #[codec(index = 0)]
                OnChain,
                #[codec(index = 1)]
                Signed,
                #[codec(index = 2)]
                Unsigned,
                #[codec(index = 3)]
                Fallback,
                #[codec(index = 4)]
                Emergency,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Phase<_0> {
                #[codec(index = 0)]
                Off,
                #[codec(index = 1)]
                Signed,
                #[codec(index = 2)]
                Unsigned((::core::primitive::bool, _0)),
                #[codec(index = 3)]
                Emergency,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct RawSolution<_0> {
                pub solution: _0,
                pub score: [::core::primitive::u128; 3usize],
                pub round: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ReadySolution < _0 > { pub supports : :: std :: vec :: Vec < (_0 , runtime_types :: sp_npos_elections :: Support < _0 > ,) > , pub score : [:: core :: primitive :: u128 ; 3usize] , pub compute : runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct RoundSnapshot<_0> {
                pub voters: ::std::vec::Vec<(
                    _0,
                    ::core::primitive::u64,
                    ::std::vec::Vec<_0>,
                )>,
                pub targets: ::std::vec::Vec<_0>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct SolutionOrSnapshotSize {
                #[codec(compact)]
                pub voters: ::core::primitive::u32,
                #[codec(compact)]
                pub targets: ::core::primitive::u32,
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Call {
                    # [codec (index = 0)] report_equivocation { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 > > , key_owner_proof : runtime_types :: sp_core :: Void , } , # [codec (index = 1)] report_equivocation_unsigned { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 > > , key_owner_proof : runtime_types :: sp_core :: Void , } , # [codec (index = 2)] note_stalled { delay : :: core :: primitive :: u32 , best_finalized_block_number : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    PauseFailed,
                    #[codec(index = 1)]
                    ResumeFailed,
                    #[codec(index = 2)]
                    ChangePending,
                    #[codec(index = 3)]
                    TooSoon,
                    #[codec(index = 4)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    Paused,
                    #[codec(index = 2)]
                    Resumed,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct StoredPendingChange < _0 > { pub scheduled_at : _0 , pub delay : _0 , pub next_authorities : runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_finality_grandpa :: app :: Public , :: core :: primitive :: u64 ,) > , pub forced : :: core :: option :: Option < _0 > , }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Call {
                    # [codec (index = 0)] set_keys { keys : runtime_types :: dkg_standalone_runtime :: opaque :: SessionKeys , proof : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 1)] purge_keys , }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidProof,
                    #[codec(index = 1)]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    NoKeys,
                    #[codec(index = 4)]
                    NoAccount,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_staking {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub enum Call {
                        # [codec (index = 0)] bond { controller : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , # [codec (compact)] value : :: core :: primitive :: u128 , payee : runtime_types :: pallet_staking :: RewardDestination < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 1)] bond_extra { # [codec (compact)] max_additional : :: core :: primitive :: u128 , } , # [codec (index = 2)] unbond { # [codec (compact)] value : :: core :: primitive :: u128 , } , # [codec (index = 3)] withdraw_unbonded { num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 4)] validate { prefs : runtime_types :: pallet_staking :: ValidatorPrefs , } , # [codec (index = 5)] nominate { targets : :: std :: vec :: Vec < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , } , # [codec (index = 6)] chill , # [codec (index = 7)] set_payee { payee : runtime_types :: pallet_staking :: RewardDestination < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 8)] set_controller { controller : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 9)] set_validator_count { # [codec (compact)] new : :: core :: primitive :: u32 , } , # [codec (index = 10)] increase_validator_count { # [codec (compact)] additional : :: core :: primitive :: u32 , } , # [codec (index = 11)] scale_validator_count { factor : runtime_types :: sp_arithmetic :: per_things :: Percent , } , # [codec (index = 12)] force_no_eras , # [codec (index = 13)] force_new_era , # [codec (index = 14)] set_invulnerables { invulnerables : :: std :: vec :: Vec < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 15)] force_unstake { stash : :: subxt :: sp_core :: crypto :: AccountId32 , num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 16)] force_new_era_always , # [codec (index = 17)] cancel_deferred_slash { era : :: core :: primitive :: u32 , slash_indices : :: std :: vec :: Vec < :: core :: primitive :: u32 > , } , # [codec (index = 18)] payout_stakers { validator_stash : :: subxt :: sp_core :: crypto :: AccountId32 , era : :: core :: primitive :: u32 , } , # [codec (index = 19)] rebond { # [codec (compact)] value : :: core :: primitive :: u128 , } , # [codec (index = 20)] set_history_depth { # [codec (compact)] new_history_depth : :: core :: primitive :: u32 , # [codec (compact)] era_items_deleted : :: core :: primitive :: u32 , } , # [codec (index = 21)] reap_stash { stash : :: subxt :: sp_core :: crypto :: AccountId32 , num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 22)] kick { who : :: std :: vec :: Vec < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , } , # [codec (index = 23)] set_staking_configs { min_nominator_bond : :: core :: primitive :: u128 , min_validator_bond : :: core :: primitive :: u128 , max_nominator_count : :: core :: option :: Option < :: core :: primitive :: u32 > , max_validator_count : :: core :: option :: Option < :: core :: primitive :: u32 > , chill_threshold : :: core :: option :: Option < runtime_types :: sp_arithmetic :: per_things :: Percent > , min_commission : runtime_types :: sp_arithmetic :: per_things :: Perbill , } , # [codec (index = 24)] chill_other { controller : :: subxt :: sp_core :: crypto :: AccountId32 , } , }
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub enum Error {
                        #[codec(index = 0)]
                        NotController,
                        #[codec(index = 1)]
                        NotStash,
                        #[codec(index = 2)]
                        AlreadyBonded,
                        #[codec(index = 3)]
                        AlreadyPaired,
                        #[codec(index = 4)]
                        EmptyTargets,
                        #[codec(index = 5)]
                        DuplicateIndex,
                        #[codec(index = 6)]
                        InvalidSlashIndex,
                        #[codec(index = 7)]
                        InsufficientBond,
                        #[codec(index = 8)]
                        NoMoreChunks,
                        #[codec(index = 9)]
                        NoUnlockChunk,
                        #[codec(index = 10)]
                        FundedTarget,
                        #[codec(index = 11)]
                        InvalidEraToReward,
                        #[codec(index = 12)]
                        InvalidNumberOfNominations,
                        #[codec(index = 13)]
                        NotSortedAndUnique,
                        #[codec(index = 14)]
                        AlreadyClaimed,
                        #[codec(index = 15)]
                        IncorrectHistoryDepth,
                        #[codec(index = 16)]
                        IncorrectSlashingSpans,
                        #[codec(index = 17)]
                        BadState,
                        #[codec(index = 18)]
                        TooManyTargets,
                        #[codec(index = 19)]
                        BadTarget,
                        #[codec(index = 20)]
                        CannotChillOther,
                        #[codec(index = 21)]
                        TooManyNominators,
                        #[codec(index = 22)]
                        TooManyValidators,
                        #[codec(index = 23)]
                        CommissionTooLow,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub enum Event {
                        #[codec(index = 0)]
                        EraPaid(
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 1)]
                        Rewarded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 2)]
                        Slashed(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 3)]
                        OldSlashingReportDiscarded(::core::primitive::u32),
                        #[codec(index = 4)]
                        StakersElected,
                        #[codec(index = 5)]
                        Bonded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 6)]
                        Unbonded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 7)]
                        Withdrawn(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 8)]
                        Kicked(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                        #[codec(index = 9)]
                        StakingElectionFailed,
                        #[codec(index = 10)]
                        Chilled(::subxt::sp_core::crypto::AccountId32),
                        #[codec(index = 11)]
                        PayoutStarted(
                            ::core::primitive::u32,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                    }
                }
            }
            pub mod slashing {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct SlashingSpans {
                    pub span_index: ::core::primitive::u32,
                    pub last_start: ::core::primitive::u32,
                    pub last_nonzero_slash: ::core::primitive::u32,
                    pub prior: ::std::vec::Vec<::core::primitive::u32>,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct SpanRecord<_0> {
                    pub slashed: _0,
                    pub paid_out: _0,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual:
                    ::std::collections::BTreeMap<_0, ::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Exposure<_0, _1> {
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub own: _1,
                pub others: ::std::vec::Vec<
                    runtime_types::pallet_staking::IndividualExposure<_0, _1>,
                >,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Forcing {
                #[codec(index = 0)]
                NotForcing,
                #[codec(index = 1)]
                ForceNew,
                #[codec(index = 2)]
                ForceNone,
                #[codec(index = 3)]
                ForceAlways,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct IndividualExposure<_0, _1> {
                pub who: _0,
                #[codec(compact)]
                pub value: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Nominations<_0> {
                pub targets: ::std::vec::Vec<_0>,
                pub submitted_in: ::core::primitive::u32,
                pub suppressed: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0Ancient,
                #[codec(index = 1)]
                V2_0_0,
                #[codec(index = 2)]
                V3_0_0,
                #[codec(index = 3)]
                V4_0_0,
                #[codec(index = 4)]
                V5_0_0,
                #[codec(index = 5)]
                V6_0_0,
                #[codec(index = 6)]
                V7_0_0,
                #[codec(index = 7)]
                V8_0_0,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum RewardDestination<_0> {
                #[codec(index = 0)]
                Staked,
                #[codec(index = 1)]
                Stash,
                #[codec(index = 2)]
                Controller,
                #[codec(index = 3)]
                Account(_0),
                #[codec(index = 4)]
                None,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct StakingLedger<_0, _1> {
                pub stash: _0,
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub active: _1,
                pub unlocking: ::std::vec::Vec<
                    runtime_types::pallet_staking::UnlockChunk<_1>,
                >,
                pub claimed_rewards: ::std::vec::Vec<::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct UnappliedSlash<_0, _1> {
                pub validator: _0,
                pub own: _1,
                pub others: ::std::vec::Vec<(_0, _1)>,
                pub reporters: ::std::vec::Vec<_0>,
                pub payout: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct UnlockChunk<_0> {
                #[codec(compact)]
                pub value: _0,
                #[codec(compact)]
                pub era: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ValidatorPrefs {
                #[codec(compact)]
                pub commission:
                    runtime_types::sp_arithmetic::per_things::Perbill,
                pub blocked: ::core::primitive::bool,
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::std::boxed::Box<
                            runtime_types::dkg_standalone_runtime::Call,
                        >,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<
                            runtime_types::dkg_standalone_runtime::Call,
                        >,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        call: ::std::boxed::Box<
                            runtime_types::dkg_standalone_runtime::Call,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid {
                        sudo_result: ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    },
                    #[codec(index = 1)]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    },
                    #[codec(index = 2)]
                    SudoAsDone {
                        sudo_result: ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ChargeTransactionPayment(
                #[codec(compact)] pub ::core::primitive::u128,
            );
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct PerU16(pub ::core::primitive::u16);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Percent(pub ::core::primitive::u8);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Permill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct Public(
                        pub runtime_types::sp_core::sr25519::Public,
                    );
                }
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct Signature(
                    pub runtime_types::sp_core::ed25519::Signature,
                );
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation:
                    runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_npos_elections {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Support<_0> {
                pub total: ::core::primitive::u128,
                pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct Digest { pub logs : :: std :: vec :: Vec < runtime_types :: sp_runtime :: generic :: digest :: DigestItem > , }
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Eq,
                        PartialEq,
                        Clone,
                    )]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)]
                        pub  ::core::marker::PhantomData<(_0, _1, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module {
                    index: ::core::primitive::u8,
                    error: ::core::primitive::u8,
                },
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis: ::std::vec::Vec<(
                    [::core::primitive::u8; 8usize],
                    ::core::primitive::u32,
                )>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue."]
    pub type DispatchError = self::runtime_types::sp_runtime::DispatchError;
    pub struct ErrorDetails {
        pub pallet: &'static str,
        pub error: &'static str,
        pub docs: &'static str,
    }
    impl DispatchError {
        pub fn details(&self) -> Option<ErrorDetails> {
            if let Self::Module { index, error } = self {
                match (index , error) { (0u8 , 0u8) => Some (ErrorDetails { pallet : "System" , error : "InvalidSpecName" , docs : "The name of specification does not match between the current runtime\nand the new runtime." }) , (0u8 , 1u8) => Some (ErrorDetails { pallet : "System" , error : "SpecVersionNeedsToIncrease" , docs : "The specification version is not allowed to decrease between the current runtime\nand the new runtime." }) , (0u8 , 2u8) => Some (ErrorDetails { pallet : "System" , error : "FailedToExtractRuntimeVersion" , docs : "Failed to extract the runtime version from the new runtime.\n\nEither calling `Core_version` or decoding `RuntimeVersion` failed." }) , (0u8 , 3u8) => Some (ErrorDetails { pallet : "System" , error : "NonDefaultComposite" , docs : "Suicide called when the account has non-default composite data." }) , (0u8 , 4u8) => Some (ErrorDetails { pallet : "System" , error : "NonZeroRefCount" , docs : "There is a non-zero reference count preventing the account from being purged." }) , (0u8 , 5u8) => Some (ErrorDetails { pallet : "System" , error : "CallFiltered" , docs : "The origin filter prevent the call to be dispatched." }) , (4u8 , 0u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "PauseFailed" , docs : "Attempt to signal GRANDPA pause when the authority set isn't live\n(either paused or already pending pause)." }) , (4u8 , 1u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "ResumeFailed" , docs : "Attempt to signal GRANDPA resume when the authority set isn't paused\n(either live or already pending resume)." }) , (4u8 , 2u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "ChangePending" , docs : "Attempt to signal GRANDPA change with one already pending." }) , (4u8 , 3u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "TooSoon" , docs : "Cannot signal forced change so soon after last." }) , (4u8 , 4u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "InvalidKeyOwnershipProof" , docs : "A key ownership proof provided as part of an equivocation report is invalid." }) , (4u8 , 5u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "InvalidEquivocationProof" , docs : "An equivocation proof provided as part of an equivocation report is invalid." }) , (4u8 , 6u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "DuplicateOffenceReport" , docs : "A given equivocation report is valid but already previously reported." }) , (5u8 , 0u8) => Some (ErrorDetails { pallet : "Balances" , error : "VestingBalance" , docs : "Vesting balance too high to send value" }) , (5u8 , 1u8) => Some (ErrorDetails { pallet : "Balances" , error : "LiquidityRestrictions" , docs : "Account liquidity restrictions prevent withdrawal" }) , (5u8 , 2u8) => Some (ErrorDetails { pallet : "Balances" , error : "InsufficientBalance" , docs : "Balance too low to send value" }) , (5u8 , 3u8) => Some (ErrorDetails { pallet : "Balances" , error : "ExistentialDeposit" , docs : "Value too low to create account due to existential deposit" }) , (5u8 , 4u8) => Some (ErrorDetails { pallet : "Balances" , error : "KeepAlive" , docs : "Transfer/payment would kill account" }) , (5u8 , 5u8) => Some (ErrorDetails { pallet : "Balances" , error : "ExistingVestingSchedule" , docs : "A vesting schedule already exists for this account" }) , (5u8 , 6u8) => Some (ErrorDetails { pallet : "Balances" , error : "DeadAccount" , docs : "Beneficiary account must pre-exist" }) , (5u8 , 7u8) => Some (ErrorDetails { pallet : "Balances" , error : "TooManyReserves" , docs : "Number of named reserves exceed MaxReserves" }) , (7u8 , 0u8) => Some (ErrorDetails { pallet : "Sudo" , error : "RequireSudo" , docs : "Sender must be the Sudo account" }) , (8u8 , 0u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "PreDispatchEarlySubmission" , docs : "Submission was too early." }) , (8u8 , 1u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "PreDispatchWrongWinnerCount" , docs : "Wrong number of winners presented." }) , (8u8 , 2u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "PreDispatchWeakSubmission" , docs : "Submission was too weak, score-wise." }) , (8u8 , 3u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "SignedQueueFull" , docs : "The queue was full, and the solution was not better than any of the existing ones." }) , (8u8 , 4u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "SignedCannotPayDeposit" , docs : "The origin failed to pay the deposit." }) , (8u8 , 5u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "SignedInvalidWitness" , docs : "Witness data to dispatchable is invalid." }) , (8u8 , 6u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "SignedTooMuchWeight" , docs : "The signed submission consumes too much weight" }) , (8u8 , 7u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "OcwCallWrongEra" , docs : "OCW submitted solution for wrong round" }) , (8u8 , 8u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "MissingSnapshotMetadata" , docs : "Snapshot metadata should exist but didn't." }) , (8u8 , 9u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "InvalidSubmissionIndex" , docs : "`Self::insert_submission` returned an invalid index." }) , (8u8 , 10u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "CallNotAllowed" , docs : "The call is not allowed at this point." }) , (9u8 , 0u8) => Some (ErrorDetails { pallet : "Staking" , error : "NotController" , docs : "Not a controller account." }) , (9u8 , 1u8) => Some (ErrorDetails { pallet : "Staking" , error : "NotStash" , docs : "Not a stash account." }) , (9u8 , 2u8) => Some (ErrorDetails { pallet : "Staking" , error : "AlreadyBonded" , docs : "Stash is already bonded." }) , (9u8 , 3u8) => Some (ErrorDetails { pallet : "Staking" , error : "AlreadyPaired" , docs : "Controller is already paired." }) , (9u8 , 4u8) => Some (ErrorDetails { pallet : "Staking" , error : "EmptyTargets" , docs : "Targets cannot be empty." }) , (9u8 , 5u8) => Some (ErrorDetails { pallet : "Staking" , error : "DuplicateIndex" , docs : "Duplicate index." }) , (9u8 , 6u8) => Some (ErrorDetails { pallet : "Staking" , error : "InvalidSlashIndex" , docs : "Slash record index out of bounds." }) , (9u8 , 7u8) => Some (ErrorDetails { pallet : "Staking" , error : "InsufficientBond" , docs : "Cannot have a validator or nominator role, with value less than the minimum defined by\ngovernance (see `MinValidatorBond` and `MinNominatorBond`). If unbonding is the\nintention, `chill` first to remove one's role as validator/nominator." }) , (9u8 , 8u8) => Some (ErrorDetails { pallet : "Staking" , error : "NoMoreChunks" , docs : "Can not schedule more unlock chunks." }) , (9u8 , 9u8) => Some (ErrorDetails { pallet : "Staking" , error : "NoUnlockChunk" , docs : "Can not rebond without unlocking chunks." }) , (9u8 , 10u8) => Some (ErrorDetails { pallet : "Staking" , error : "FundedTarget" , docs : "Attempting to target a stash that still has funds." }) , (9u8 , 11u8) => Some (ErrorDetails { pallet : "Staking" , error : "InvalidEraToReward" , docs : "Invalid era to reward." }) , (9u8 , 12u8) => Some (ErrorDetails { pallet : "Staking" , error : "InvalidNumberOfNominations" , docs : "Invalid number of nominations." }) , (9u8 , 13u8) => Some (ErrorDetails { pallet : "Staking" , error : "NotSortedAndUnique" , docs : "Items are not sorted and unique." }) , (9u8 , 14u8) => Some (ErrorDetails { pallet : "Staking" , error : "AlreadyClaimed" , docs : "Rewards for this era have already been claimed for this validator." }) , (9u8 , 15u8) => Some (ErrorDetails { pallet : "Staking" , error : "IncorrectHistoryDepth" , docs : "Incorrect previous history depth input provided." }) , (9u8 , 16u8) => Some (ErrorDetails { pallet : "Staking" , error : "IncorrectSlashingSpans" , docs : "Incorrect number of slashing spans provided." }) , (9u8 , 17u8) => Some (ErrorDetails { pallet : "Staking" , error : "BadState" , docs : "Internal state has become somehow corrupted and the operation cannot continue." }) , (9u8 , 18u8) => Some (ErrorDetails { pallet : "Staking" , error : "TooManyTargets" , docs : "Too many nomination targets supplied." }) , (9u8 , 19u8) => Some (ErrorDetails { pallet : "Staking" , error : "BadTarget" , docs : "A nomination target was supplied that was blocked or otherwise not a validator." }) , (9u8 , 20u8) => Some (ErrorDetails { pallet : "Staking" , error : "CannotChillOther" , docs : "The user has enough bond and thus cannot be chilled forcefully by an external person." }) , (9u8 , 21u8) => Some (ErrorDetails { pallet : "Staking" , error : "TooManyNominators" , docs : "There are too many nominators in the system. Governance needs to adjust the staking\nsettings to keep things safe for the runtime." }) , (9u8 , 22u8) => Some (ErrorDetails { pallet : "Staking" , error : "TooManyValidators" , docs : "There are too many validators in the system. Governance needs to adjust the staking\nsettings to keep things safe for the runtime." }) , (9u8 , 23u8) => Some (ErrorDetails { pallet : "Staking" , error : "CommissionTooLow" , docs : "Commission is too low. Must be at least `MinCommission`." }) , (10u8 , 0u8) => Some (ErrorDetails { pallet : "Session" , error : "InvalidProof" , docs : "Invalid ownership proof." }) , (10u8 , 1u8) => Some (ErrorDetails { pallet : "Session" , error : "NoAssociatedValidatorId" , docs : "No associated validator ID for account." }) , (10u8 , 2u8) => Some (ErrorDetails { pallet : "Session" , error : "DuplicatedKey" , docs : "Registered duplicate key." }) , (10u8 , 3u8) => Some (ErrorDetails { pallet : "Session" , error : "NoKeys" , docs : "No keys are associated with this account." }) , (10u8 , 4u8) => Some (ErrorDetails { pallet : "Session" , error : "NoAccount" , docs : "Key setting account is not live, so it's impossible to associate keys." }) , (12u8 , 0u8) => Some (ErrorDetails { pallet : "DKG" , error : "InvalidThreshold" , docs : "Invalid threshold" }) , (12u8 , 1u8) => Some (ErrorDetails { pallet : "DKG" , error : "MustBeAQueuedAuthority" , docs : "Must be queued  to become an authority" }) , (12u8 , 2u8) => Some (ErrorDetails { pallet : "DKG" , error : "MustBeAnActiveAuthority" , docs : "Must be an an authority" }) , (12u8 , 3u8) => Some (ErrorDetails { pallet : "DKG" , error : "InvalidRefreshDelay" , docs : "Refresh delay should be in the range of 0% - 100%" }) , (12u8 , 4u8) => Some (ErrorDetails { pallet : "DKG" , error : "InvalidPublicKeys" , docs : "Invalid public key submission" }) , (12u8 , 5u8) => Some (ErrorDetails { pallet : "DKG" , error : "AlreadySubmittedPublicKey" , docs : "Already submitted a public key" }) , (12u8 , 6u8) => Some (ErrorDetails { pallet : "DKG" , error : "AlreadySubmittedSignature" , docs : "Already submitted a public key signature" }) , (12u8 , 7u8) => Some (ErrorDetails { pallet : "DKG" , error : "UsedSignature" , docs : "Used signature from past sessions" }) , (12u8 , 8u8) => Some (ErrorDetails { pallet : "DKG" , error : "InvalidSignature" , docs : "Invalid public key signature submission" }) , (13u8 , 0u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "InvalidPermissions" , docs : "Account does not have correct permissions" }) , (13u8 , 1u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ThresholdNotSet" , docs : "Proposer threshold not set" }) , (13u8 , 2u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "InvalidChainId" , docs : "Provided chain Id is not valid" }) , (13u8 , 3u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "InvalidThreshold" , docs : "Proposer threshold cannot be 0" }) , (13u8 , 4u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ChainNotWhitelisted" , docs : "Interactions with this chain is not permitted" }) , (13u8 , 5u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ChainAlreadyWhitelisted" , docs : "Chain has already been enabled" }) , (13u8 , 6u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ResourceDoesNotExist" , docs : "Resource ID provided isn't mapped to anything" }) , (13u8 , 7u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ProposerAlreadyExists" , docs : "Proposer already in set" }) , (13u8 , 8u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ProposerInvalid" , docs : "Provided accountId is not a proposer" }) , (13u8 , 9u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "MustBeProposer" , docs : "Protected operation, must be performed by proposer" }) , (13u8 , 10u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ProposerAlreadyVoted" , docs : "Proposer has already submitted some vote for this proposal" }) , (13u8 , 11u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ProposalAlreadyExists" , docs : "A proposal with these parameters has already been submitted" }) , (13u8 , 12u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ProposalDoesNotExist" , docs : "No proposal with the ID was found" }) , (13u8 , 13u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ProposalNotComplete" , docs : "Cannot complete proposal, needs more votes" }) , (13u8 , 14u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ProposalAlreadyComplete" , docs : "Proposal has either failed or succeeded" }) , (13u8 , 15u8) => Some (ErrorDetails { pallet : "DKGProposals" , error : "ProposalExpired" , docs : "Lifetime of proposal has been exceeded" }) , (15u8 , 0u8) => Some (ErrorDetails { pallet : "DKGProposalHandler" , error : "NoneValue" , docs : "Error names should be descriptive." }) , (15u8 , 1u8) => Some (ErrorDetails { pallet : "DKGProposalHandler" , error : "StorageOverflow" , docs : "Errors should have helpful documentation associated with them." }) , (15u8 , 2u8) => Some (ErrorDetails { pallet : "DKGProposalHandler" , error : "ProposalFormatInvalid" , docs : "Proposal format is invalid" }) , (15u8 , 3u8) => Some (ErrorDetails { pallet : "DKGProposalHandler" , error : "ProposalSignatureInvalid" , docs : "Proposal signature is invalid" }) , (15u8 , 4u8) => Some (ErrorDetails { pallet : "DKGProposalHandler" , error : "ProposalDoesNotExists" , docs : "No proposal with the ID was found" }) , (15u8 , 5u8) => Some (ErrorDetails { pallet : "DKGProposalHandler" , error : "ProposalAlreadyExists" , docs : "Proposal with the ID has already been submitted" }) , (15u8 , 6u8) => Some (ErrorDetails { pallet : "DKGProposalHandler" , error : "ChainIdInvalid" , docs : "Chain id is invalid" }) , (15u8 , 7u8) => Some (ErrorDetails { pallet : "DKGProposalHandler" , error : "ProposalsLengthOverflow" , docs : "Proposal length exceeds max allowed per batch" }) , _ => None }
            } else {
                None
            }
        }
    }
    #[doc = r" The default storage entry from which to fetch an account nonce, required for"]
    #[doc = r" constructing a transaction."]
    pub enum DefaultAccountData {}
    impl ::subxt::AccountData for DefaultAccountData {
        type StorageEntry = self::system::storage::Account;
        type AccountId = ::subxt::sp_core::crypto::AccountId32;
        type Index = ::core::primitive::u32;
        fn nonce(
            result: &<Self::StorageEntry as ::subxt::StorageEntry>::Value,
        ) -> Self::Index {
            result.nonce
        }
        fn storage_entry(account_id: Self::AccountId) -> Self::StorageEntry {
            self::system::storage::Account(account_id)
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config, X, A = DefaultAccountData> {
        pub client: ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<(X, A)>,
    }
    impl<T, X, A> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self {
                client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl<'a, T, X, A> RuntimeApi<T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        pub fn constants(&'a self) -> ConstantsApi {
            ConstantsApi
        }
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T, X, A> {
            TransactionApi {
                client: &self.client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(
            &self,
        ) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::constants::ConstantsApi {
            election_provider_multi_phase::constants::ConstantsApi
        }
        pub fn staking(&self) -> staking::constants::ConstantsApi {
            staking::constants::ConstantsApi
        }
        pub fn dkg(&self) -> dkg::constants::ConstantsApi {
            dkg::constants::ConstantsApi
        }
        pub fn dkg_proposals(&self) -> dkg_proposals::constants::ConstantsApi {
            dkg_proposals::constants::ConstantsApi
        }
        pub fn dkg_proposal_handler(
            &self,
        ) -> dkg_proposal_handler::constants::ConstantsApi {
            dkg_proposal_handler::constants::ConstantsApi
        }
    }
    pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi<'a, T> {
            randomness_collective_flip::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn aura(&self) -> aura::storage::StorageApi<'a, T> {
            aura::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(
            &self,
        ) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::storage::StorageApi<'a, T> {
            election_provider_multi_phase::storage::StorageApi::new(self.client)
        }
        pub fn staking(&self) -> staking::storage::StorageApi<'a, T> {
            staking::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn historical(&self) -> historical::storage::StorageApi<'a, T> {
            historical::storage::StorageApi::new(self.client)
        }
        pub fn dkg(&self) -> dkg::storage::StorageApi<'a, T> {
            dkg::storage::StorageApi::new(self.client)
        }
        pub fn dkg_proposals(
            &self,
        ) -> dkg_proposals::storage::StorageApi<'a, T> {
            dkg_proposals::storage::StorageApi::new(self.client)
        }
        pub fn mmr(&self) -> mmr::storage::StorageApi<'a, T> {
            mmr::storage::StorageApi::new(self.client)
        }
        pub fn dkg_proposal_handler(
            &self,
        ) -> dkg_proposal_handler::storage::StorageApi<'a, T> {
            dkg_proposal_handler::storage::StorageApi::new(self.client)
        }
        pub fn dkgmmr(&self) -> dkgmmr::storage::StorageApi<'a, T> {
            dkgmmr::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<(X, A)>,
    }
    impl<'a, T, X, A> TransactionApi<'a, T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T, X, A> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(
            &self,
        ) -> timestamp::calls::TransactionApi<'a, T, X, A> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T, X, A> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X, A> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X, A> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::calls::TransactionApi<'a, T, X, A>
        {
            election_provider_multi_phase::calls::TransactionApi::new(
                self.client,
            )
        }
        pub fn staking(&self) -> staking::calls::TransactionApi<'a, T, X, A> {
            staking::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T, X, A> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn dkg(&self) -> dkg::calls::TransactionApi<'a, T, X, A> {
            dkg::calls::TransactionApi::new(self.client)
        }
        pub fn dkg_proposals(
            &self,
        ) -> dkg_proposals::calls::TransactionApi<'a, T, X, A> {
            dkg_proposals::calls::TransactionApi::new(self.client)
        }
        pub fn dkg_proposal_handler(
            &self,
        ) -> dkg_proposal_handler::calls::TransactionApi<'a, T, X, A> {
            dkg_proposal_handler::calls::TransactionApi::new(self.client)
        }
    }
}
