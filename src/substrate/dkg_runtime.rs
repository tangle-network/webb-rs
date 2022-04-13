#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    use super::api as root_mod;
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
        #[codec(index = 14)]
        DKGProposalHandler(dkg_proposal_handler::Event),
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
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
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
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
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
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
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
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
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
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
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for SetStorage {
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
            pub struct KillStorage {
                pub keys:
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
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
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
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
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RemarkWithEvent {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
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
                    FillBlock,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = FillBlock { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    Remark,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetHeapPages,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetHeapPages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetCode,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetCode { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetCodeWithoutChecks,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetCodeWithoutChecks { code };
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
                    SetStorage,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetStorage { items };
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
                    KillStorage,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = KillStorage { keys };
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
                    KillPrefix,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = KillPrefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    RemarkWithEvent,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = RemarkWithEvent { remark };
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
            pub struct Account<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Account<'_> {
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
            pub struct BlockHash<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash<'_> {
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
            pub struct ExtrinsicData<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData<'_> {
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
            pub struct EventTopics<'a>(pub &'a ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics<'_> {
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
                    _0: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, Account<'a>>,
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
                    _0: &::core::primitive::u32,
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
                    ::subxt::KeyIter<'a, T, BlockHash<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: &::core::primitive::u32,
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
                    ::subxt::KeyIter<'a, T, ExtrinsicData<'a>>,
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
                    _0: &::subxt::sp_core::H256,
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
                    ::subxt::KeyIter<'a, T, EventTopics<'a>>,
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
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn block_weights(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockWeights,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("BlockWeights")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn block_length(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockLength,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("BlockLength")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn block_hash_count(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("BlockHashCount")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn db_weight(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::RuntimeDbWeight,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("DbWeight")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn version(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_version::RuntimeVersion,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("Version")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u16,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("System")?;
                    let constant = pallet.constant("SS58Prefix")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: subxt :: sp_core :: H256 > ;
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
                }                pub async fn random_material (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: subxt :: sp_core :: H256 > , :: subxt :: BasicError >{
                    let entry = RandomMaterial;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for Set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
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
                    Set,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Set { now };
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
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn minimum_period(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("Timestamp")?;
                    let constant = pallet.constant("MinimumPeriod")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
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
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct ReportEquivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for ReportEquivocation {
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
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
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
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for NoteStalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
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
                    ReportEquivocation,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ReportEquivocation {
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
                    ReportEquivocationUnsigned,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ReportEquivocationUnsigned {
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
                    NoteStalled,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = NoteStalled {
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
            pub struct SetIdSession<'a>(pub &'a ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession<'_> {
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
                    _0: &::core::primitive::u64,
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
                    ::subxt::KeyIter<'a, T, SetIdSession<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("Grandpa")?;
                    let constant = pallet.constant("MaxAuthorities")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct Transfer {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
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
            pub struct SetBalance {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetBalance {
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
            pub struct ForceTransfer {
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
            impl ::subxt::Call for ForceTransfer {
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
            pub struct TransferKeepAlive {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
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
            pub struct TransferAll {
                pub dest: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
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
            pub struct ForceUnreserve {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceUnreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
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
                    Transfer,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Transfer { dest, value };
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
                    SetBalance,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetBalance {
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
                    ForceTransfer,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceTransfer {
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
                    TransferKeepAlive,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = TransferKeepAlive { dest, value };
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
                    TransferAll,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = TransferAll { dest, keep_alive };
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
                    ForceUnreserve,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceUnreserve { who, amount };
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
            pub struct Account<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Account<'_> {
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
            pub struct Locks<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks<'_> {
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
            pub struct Reserves<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Reserves<'_> {
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
                    _0: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, Account<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn locks (& self , _0 : & :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > , :: subxt :: BasicError >{
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Locks<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn reserves (& self , _0 : & :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: pallet_balances :: ReserveData < [:: core :: primitive :: u8 ; 8usize] , :: core :: primitive :: u128 > > , :: subxt :: BasicError >{
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reserves<'a>>,
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
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn existential_deposit(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("Balances")?;
                    let constant = pallet.constant("ExistentialDeposit")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("Balances")?;
                    let constant = pallet.constant("MaxLocks")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("Balances")?;
                    let constant = pallet.constant("MaxReserves")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
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
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn transaction_byte_fee(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let pallet =
                        self.client.metadata().pallet("TransactionPayment")?;
                    let constant = pallet.constant("TransactionByteFee")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u8,
                    ::subxt::BasicError,
                > {
                    let pallet =
                        self.client.metadata().pallet("TransactionPayment")?;
                    let constant =
                        pallet.constant("OperationalFeeMultiplier")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }                pub fn weight_to_fee (& self) -> :: core :: result :: Result < :: std :: vec :: Vec < runtime_types :: frame_support :: weights :: WeightToFeeCoefficient < :: core :: primitive :: u128 > > , :: subxt :: BasicError >{
                    let pallet =
                        self.client.metadata().pallet("TransactionPayment")?;
                    let constant = pallet.constant("WeightToFee")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct Sudo {
                pub call: ::std::boxed::Box<
                    runtime_types::dkg_standalone_runtime::Call,
                >,
            }
            impl ::subxt::Call for Sudo {
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
            pub struct SudoUncheckedWeight {
                pub call: ::std::boxed::Box<
                    runtime_types::dkg_standalone_runtime::Call,
                >,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
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
            pub struct SetKey {
                pub new: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for SetKey {
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
            pub struct SudoAs {
                pub who: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub call: ::std::boxed::Box<
                    runtime_types::dkg_standalone_runtime::Call,
                >,
            }
            impl ::subxt::Call for SudoAs {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
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
                    Sudo,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Sudo {
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
                    SudoUncheckedWeight,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SudoUncheckedWeight {
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
                    SetKey,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetKey { new };
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
                    SudoAs,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SudoAs {
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
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct SubmitUnsigned { pub raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , pub witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , }
            impl ::subxt::Call for SubmitUnsigned {
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
            pub struct SetMinimumUntrustedScore {
                pub maybe_next_score:
                    ::core::option::Option<[::core::primitive::u128; 3usize]>,
            }
            impl ::subxt::Call for SetMinimumUntrustedScore {
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
            pub struct SetEmergencyElectionResult {
                pub supports: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::sp_npos_elections::Support<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                )>,
            }
            impl ::subxt::Call for SetEmergencyElectionResult {
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
            pub struct Submit { pub raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , pub num_signed_submissions : :: core :: primitive :: u32 , }
            impl ::subxt::Call for Submit {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct GovernanceFallback {
                pub maybe_max_voters:
                    ::core::option::Option<::core::primitive::u32>,
                pub maybe_max_targets:
                    ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for GovernanceFallback {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "governance_fallback";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
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
                    SubmitUnsigned,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitUnsigned {
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
                    SetMinimumUntrustedScore,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetMinimumUntrustedScore { maybe_next_score };
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
                    SetEmergencyElectionResult,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetEmergencyElectionResult { supports };
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
                    Submit,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Submit {
                        raw_solution: ::std::boxed::Box::new(raw_solution),
                        num_signed_submissions,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn governance_fallback(
                    &self,
                    maybe_max_voters: ::core::option::Option<
                        ::core::primitive::u32,
                    >,
                    maybe_max_targets: ::core::option::Option<
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    GovernanceFallback,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = GovernanceFallback {
                        maybe_max_voters,
                        maybe_max_targets,
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
                type Value = runtime_types :: pallet_election_provider_multi_phase :: RoundSnapshot ;
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
            pub struct SignedSubmissionsMap<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for SignedSubmissionsMap<'_> {
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
                }                pub async fn snapshot (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: RoundSnapshot > , :: subxt :: BasicError >{
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
                }                pub async fn signed_submissions_map (& self , _0 : & :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , :: subxt :: BasicError >{
                    let entry = SignedSubmissionsMap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signed_submissions_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SignedSubmissionsMap<'a>>,
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
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn unsigned_phase(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("UnsignedPhase")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn signed_phase(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("SignedPhase")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn solution_improvement_threshold(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant =
                        pallet.constant("SolutionImprovementThreshold")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn offchain_repeat(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("OffchainRepeat")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn miner_tx_priority(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("MinerTxPriority")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn miner_max_weight(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("MinerMaxWeight")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn signed_max_submissions(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("SignedMaxSubmissions")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn signed_max_weight(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("SignedMaxWeight")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn signed_reward_base(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("SignedRewardBase")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn signed_deposit_base(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("SignedDepositBase")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn signed_deposit_byte(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("SignedDepositByte")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn signed_deposit_weight(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("SignedDepositWeight")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn voter_snapshot_per_block(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("VoterSnapshotPerBlock")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn miner_max_length(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self
                        .client
                        .metadata()
                        .pallet("ElectionProviderMultiPhase")?;
                    let constant = pallet.constant("MinerMaxLength")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
            }
        }
    }
    pub mod staking {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct Bond {
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
            impl ::subxt::Call for Bond {
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
            pub struct BondExtra {
                #[codec(compact)]
                pub max_additional: ::core::primitive::u128,
            }
            impl ::subxt::Call for BondExtra {
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
            pub struct Unbond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Unbond {
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
            pub struct WithdrawUnbonded {
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for WithdrawUnbonded {
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
            pub struct Validate {
                pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
            }
            impl ::subxt::Call for Validate {
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
            pub struct Nominate {
                pub targets: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            impl ::subxt::Call for Nominate {
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
            pub struct Chill;
            impl ::subxt::Call for Chill {
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
            pub struct SetPayee {
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for SetPayee {
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
            pub struct SetController {
                pub controller: ::subxt::sp_runtime::MultiAddress<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for SetController {
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
            pub struct SetValidatorCount {
                #[codec(compact)]
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetValidatorCount {
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
            pub struct IncreaseValidatorCount {
                #[codec(compact)]
                pub additional: ::core::primitive::u32,
            }
            impl ::subxt::Call for IncreaseValidatorCount {
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
            pub struct ScaleValidatorCount {
                pub factor: runtime_types::sp_arithmetic::per_things::Percent,
            }
            impl ::subxt::Call for ScaleValidatorCount {
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
            pub struct ForceNoEras;
            impl ::subxt::Call for ForceNoEras {
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
            pub struct ForceNewEra;
            impl ::subxt::Call for ForceNewEra {
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
            pub struct SetInvulnerables {
                pub invulnerables:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for SetInvulnerables {
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
            pub struct ForceUnstake {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceUnstake {
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
            pub struct ForceNewEraAlways;
            impl ::subxt::Call for ForceNewEraAlways {
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
            pub struct CancelDeferredSlash {
                pub era: ::core::primitive::u32,
                pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Call for CancelDeferredSlash {
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
            pub struct PayoutStakers {
                pub validator_stash: ::subxt::sp_core::crypto::AccountId32,
                pub era: ::core::primitive::u32,
            }
            impl ::subxt::Call for PayoutStakers {
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
            pub struct Rebond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Rebond {
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
            pub struct SetHistoryDepth {
                #[codec(compact)]
                pub new_history_depth: ::core::primitive::u32,
                #[codec(compact)]
                pub era_items_deleted: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHistoryDepth {
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
            pub struct ReapStash {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for ReapStash {
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
            pub struct Kick {
                pub who: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            impl ::subxt::Call for Kick {
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
            pub struct SetStakingConfigs {
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
            impl ::subxt::Call for SetStakingConfigs {
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
            pub struct ChillOther {
                pub controller: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ChillOther {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill_other";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ForceApplyMinCommission {
                pub validator_stash: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ForceApplyMinCommission {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_apply_min_commission";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
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
                    Bond,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Bond {
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
                    BondExtra,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = BondExtra { max_additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unbond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    Unbond,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Unbond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw_unbonded(
                    &self,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    WithdrawUnbonded,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = WithdrawUnbonded { num_slashing_spans };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn validate(
                    &self,
                    prefs: runtime_types::pallet_staking::ValidatorPrefs,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    Validate,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Validate { prefs };
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
                    Nominate,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Nominate { targets };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    Chill,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Chill {};
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
                    SetPayee,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetPayee { payee };
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
                    SetController,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetController { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validator_count(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetValidatorCount,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetValidatorCount { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn increase_validator_count(
                    &self,
                    additional: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    IncreaseValidatorCount,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = IncreaseValidatorCount { additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn scale_validator_count(
                    &self,
                    factor: runtime_types::sp_arithmetic::per_things::Percent,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ScaleValidatorCount,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ScaleValidatorCount { factor };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_no_eras(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceNoEras,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceNoEras {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceNewEra,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceNewEra {};
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
                    SetInvulnerables,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetInvulnerables { invulnerables };
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
                    ForceUnstake,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceUnstake {
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
                    ForceNewEraAlways,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceNewEraAlways {};
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
                    CancelDeferredSlash,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = CancelDeferredSlash { era, slash_indices };
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
                    PayoutStakers,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = PayoutStakers {
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
                    Rebond,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Rebond { value };
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
                    SetHistoryDepth,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetHistoryDepth {
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
                    ReapStash,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ReapStash {
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
                    Kick,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Kick { who };
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
                    SetStakingConfigs,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetStakingConfigs {
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
                    ChillOther,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ChillOther { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_apply_min_commission(
                    &self,
                    validator_stash: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceApplyMinCommission,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceApplyMinCommission { validator_stash };
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
            pub struct Bonded<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Bonded<'_> {
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
            pub struct Ledger<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Ledger<'_> {
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
            pub struct Payee<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Payee<'_> {
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
            pub struct Validators<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Validators<'_> {
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
            pub struct Nominators<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Nominators<'_> {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Nominators";
                type Value = runtime_types::pallet_staking::Nominations;
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
            pub struct ErasStartSessionIndex<'a>(
                pub &'a ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for ErasStartSessionIndex<'_> {
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
            pub struct ErasStakers<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakers<'_> {
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
            pub struct ErasStakersClipped<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakersClipped<'_> {
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
            pub struct ErasValidatorPrefs<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasValidatorPrefs<'_> {
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
            pub struct ErasValidatorReward<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasValidatorReward<'_> {
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
            pub struct ErasRewardPoints<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasRewardPoints<'_> {
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
            pub struct ErasTotalStake<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasTotalStake<'_> {
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
            pub struct UnappliedSlashes<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnappliedSlashes<'_> {
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
            pub struct ValidatorSlashInEra<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ValidatorSlashInEra<'_> {
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
            pub struct NominatorSlashInEra<'a>(
                pub &'a ::core::primitive::u32,
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for NominatorSlashInEra<'_> {
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
            pub struct SlashingSpans<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for SlashingSpans<'_> {
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
            pub struct SpanSlash<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
                pub &'a ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for SpanSlash<'_> {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SpanSlash";
                type Value =
                    runtime_types::pallet_staking::slashing::SpanRecord<
                        ::core::primitive::u128,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &(&self.0, &self.1),
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
                    _0: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, Bonded<'a>>,
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
                    _0: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, Ledger<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn payee(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, Payee<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn validators(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, Validators<'a>>,
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
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::Nominations,
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
                    ::subxt::KeyIter<'a, T, Nominators<'a>>,
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
                    _0: &::core::primitive::u32,
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
                    ::subxt::KeyIter<'a, T, ErasStartSessionIndex<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, ErasStakers<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers_clipped(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, ErasStakersClipped<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_prefs(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, ErasValidatorPrefs<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_reward(
                    &self,
                    _0: &::core::primitive::u32,
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
                    ::subxt::KeyIter<'a, T, ErasValidatorReward<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_reward_points(
                    &self,
                    _0: &::core::primitive::u32,
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
                    ::subxt::KeyIter<'a, T, ErasRewardPoints<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_total_stake(
                    &self,
                    _0: &::core::primitive::u32,
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
                    ::subxt::KeyIter<'a, T, ErasTotalStake<'a>>,
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
                    _0: &::core::primitive::u32,
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
                    ::subxt::KeyIter<'a, T, UnappliedSlashes<'a>>,
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
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, ValidatorSlashInEra<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nominator_slash_in_era(
                    &self,
                    _0: &::core::primitive::u32,
                    _1: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, NominatorSlashInEra<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn slashing_spans(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, SlashingSpans<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn span_slash(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    _1: &::core::primitive::u32,
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
                    ::subxt::KeyIter<'a, T, SpanSlash<'a>>,
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
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn sessions_per_era(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("Staking")?;
                    let constant = pallet.constant("SessionsPerEra")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn bonding_duration(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("Staking")?;
                    let constant = pallet.constant("BondingDuration")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn slash_defer_duration(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("Staking")?;
                    let constant = pallet.constant("SlashDeferDuration")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn max_nominator_rewarded_per_validator(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("Staking")?;
                    let constant =
                        pallet.constant("MaxNominatorRewardedPerValidator")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
            }
        }
    }
    pub mod session {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct SetKeys {
                pub keys:
                    runtime_types::dkg_standalone_runtime::opaque::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetKeys {
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
            pub struct PurgeKeys;
            impl ::subxt::Call for PurgeKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "purge_keys";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
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
                    SetKeys,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetKeys { keys, proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn purge_keys(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    PurgeKeys,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = PurgeKeys {};
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
            pub struct NextKeys<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for NextKeys<'_> {
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
            pub struct KeyOwner<'a>(
                pub &'a runtime_types::sp_core::crypto::KeyTypeId,
                pub &'a [::core::primitive::u8],
            );
            impl ::subxt::StorageEntry for KeyOwner<'_> {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "KeyOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &(&self.0, &self.1),
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
                }                pub async fn next_keys (& self , _0 : & :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: dkg_standalone_runtime :: opaque :: SessionKeys > , :: subxt :: BasicError >{
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_keys_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NextKeys<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: &runtime_types::sp_core::crypto::KeyTypeId,
                    _1: &[::core::primitive::u8],
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
                    ::subxt::KeyIter<'a, T, KeyOwner<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod historical {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct HistoricalSessions<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for HistoricalSessions<'_> {
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
                    _0: &::core::primitive::u32,
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
                    ::subxt::KeyIter<'a, T, HistoricalSessions<'a>>,
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
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct SetSignatureThreshold {
                pub new_threshold: ::core::primitive::u16,
            }
            impl ::subxt::Call for SetSignatureThreshold {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "set_signature_threshold";
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
            pub struct SetKeygenThreshold {
                pub new_threshold: ::core::primitive::u16,
            }
            impl ::subxt::Call for SetKeygenThreshold {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "set_keygen_threshold";
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
            pub struct SetRefreshDelay {
                pub new_delay: ::core::primitive::u8,
            }
            impl ::subxt::Call for SetRefreshDelay {
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
            pub struct SubmitPublicKey {
                pub keys_and_signatures:
                    runtime_types::dkg_runtime_primitives::AggregatedPublicKeys,
            }
            impl ::subxt::Call for SubmitPublicKey {
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
            pub struct SubmitNextPublicKey {
                pub keys_and_signatures:
                    runtime_types::dkg_runtime_primitives::AggregatedPublicKeys,
            }
            impl ::subxt::Call for SubmitNextPublicKey {
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
            pub struct SubmitPublicKeySignature { pub signature_proposal : runtime_types :: dkg_runtime_primitives :: proposal :: RefreshProposalSigned , }
            impl ::subxt::Call for SubmitPublicKeySignature {
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
            )]
            pub struct SubmitMisbehaviourReports { pub reports : runtime_types :: dkg_runtime_primitives :: AggregatedMisbehaviourReports < runtime_types :: dkg_runtime_primitives :: crypto :: Public > , }
            impl ::subxt::Call for SubmitMisbehaviourReports {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "submit_misbehaviour_reports";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct Unjail;
            impl ::subxt::Call for Unjail {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "unjail";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ForceUnjailKeygen {
                pub authority:
                    runtime_types::dkg_runtime_primitives::crypto::Public,
            }
            impl ::subxt::Call for ForceUnjailKeygen {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "force_unjail_keygen";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ForceUnjailSigning {
                pub authority:
                    runtime_types::dkg_runtime_primitives::crypto::Public,
            }
            impl ::subxt::Call for ForceUnjailSigning {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "force_unjail_signing";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ManualIncrementNonce;
            impl ::subxt::Call for ManualIncrementNonce {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "manual_increment_nonce";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ManualRefresh;
            impl ::subxt::Call for ManualRefresh {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "manual_refresh";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ForceChangeAuthorities;
            impl ::subxt::Call for ForceChangeAuthorities {
                const PALLET: &'static str = "DKG";
                const FUNCTION: &'static str = "force_change_authorities";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_signature_threshold(
                    &self,
                    new_threshold: ::core::primitive::u16,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetSignatureThreshold,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetSignatureThreshold { new_threshold };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_keygen_threshold(
                    &self,
                    new_threshold: ::core::primitive::u16,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetKeygenThreshold,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetKeygenThreshold { new_threshold };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_refresh_delay(
                    &self,
                    new_delay: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetRefreshDelay,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetRefreshDelay { new_delay };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_public_key(
                    &self,
                    keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SubmitPublicKey,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitPublicKey {
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
                    SubmitNextPublicKey,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitNextPublicKey {
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
                    SubmitPublicKeySignature,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitPublicKeySignature { signature_proposal };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_misbehaviour_reports(
                    &self,
                    reports : runtime_types :: dkg_runtime_primitives :: AggregatedMisbehaviourReports < runtime_types :: dkg_runtime_primitives :: crypto :: Public >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SubmitMisbehaviourReports,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitMisbehaviourReports { reports };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unjail(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    Unjail,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = Unjail {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unjail_keygen(
                    &self,
                    authority : runtime_types :: dkg_runtime_primitives :: crypto :: Public,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceUnjailKeygen,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceUnjailKeygen { authority };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unjail_signing(
                    &self,
                    authority : runtime_types :: dkg_runtime_primitives :: crypto :: Public,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceUnjailSigning,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceUnjailSigning { authority };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn manual_increment_nonce(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ManualIncrementNonce,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ManualIncrementNonce {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn manual_refresh(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ManualRefresh,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ManualRefresh {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_change_authorities(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceChangeAuthorities,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceChangeAuthorities {};
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
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct PublicKeyChanged {
                pub compressed_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                pub uncompressed_pub_key:
                    ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for PublicKeyChanged {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "PublicKeyChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct PublicKeySignatureChanged {
                pub pub_key_sig: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for PublicKeySignatureChanged {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "PublicKeySignatureChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct MisbehaviourReportsSubmitted {
                pub misbehaviour_type:
                    runtime_types::dkg_runtime_primitives::MisbehaviourType,
                pub reporters: ::std::vec::Vec<
                    runtime_types::dkg_runtime_primitives::crypto::Public,
                >,
            }
            impl ::subxt::Event for MisbehaviourReportsSubmitted {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "MisbehaviourReportsSubmitted";
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
            pub struct RefreshKeysFinished {
                pub next_authority_set_id: ::core::primitive::u64,
            }
            impl ::subxt::Event for RefreshKeysFinished {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "RefreshKeysFinished";
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
            pub struct ShouldManualRefresh;
            impl ::subxt::StorageEntry for ShouldManualRefresh {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "ShouldManualRefresh";
                type Value = ::core::primitive::bool;
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
            pub struct HistoricalRounds<'a>(pub &'a ::core::primitive::u64);
            impl ::subxt::StorageEntry for HistoricalRounds<'_> {
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
            pub struct KeygenThreshold;
            impl ::subxt::StorageEntry for KeygenThreshold {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "KeygenThreshold";
                type Value = ::core::primitive::u16;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextSignatureThreshold;
            impl ::subxt::StorageEntry for NextSignatureThreshold {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "NextSignatureThreshold";
                type Value = ::core::primitive::u16;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeygenThreshold;
            impl ::subxt::StorageEntry for NextKeygenThreshold {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "NextKeygenThreshold";
                type Value = ::core::primitive::u16;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingSignatureThreshold;
            impl ::subxt::StorageEntry for PendingSignatureThreshold {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "PendingSignatureThreshold";
                type Value = ::core::primitive::u16;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingKeygenThreshold;
            impl ::subxt::StorageEntry for PendingKeygenThreshold {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "PendingKeygenThreshold";
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
            pub struct NextAuthoritySetId;
            impl ::subxt::StorageEntry for NextAuthoritySetId {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "NextAuthoritySetId";
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
            pub struct AccountToAuthority<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AccountToAuthority<'_> {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "AccountToAuthority";
                type Value =
                    runtime_types::dkg_runtime_primitives::crypto::Public;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct MisbehaviourReports<'a>(
                pub &'a runtime_types::dkg_runtime_primitives::MisbehaviourType,
                pub &'a ::core::primitive::u64,
                pub &'a runtime_types::dkg_runtime_primitives::crypto::Public,
            );
            impl ::subxt::StorageEntry for MisbehaviourReports<'_> {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "MisbehaviourReports";
                type Value = runtime_types :: dkg_runtime_primitives :: AggregatedMisbehaviourReports < runtime_types :: dkg_runtime_primitives :: crypto :: Public > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &(&self.0, &self.1, &self.2),
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct AuthorityReputations<'a>(
                pub &'a runtime_types::dkg_runtime_primitives::crypto::Public,
            );
            impl ::subxt::StorageEntry for AuthorityReputations<'_> {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "AuthorityReputations";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_256,
                        ),
                    ])
                }
            }
            pub struct JailedKeygenAuthorities<'a>(
                pub &'a runtime_types::dkg_runtime_primitives::crypto::Public,
            );
            impl ::subxt::StorageEntry for JailedKeygenAuthorities<'_> {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "JailedKeygenAuthorities";
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
            pub struct JailedSigningAuthorities<'a>(
                pub &'a runtime_types::dkg_runtime_primitives::crypto::Public,
            );
            impl ::subxt::StorageEntry for JailedSigningAuthorities<'_> {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "JailedSigningAuthorities";
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
            pub struct BestAuthorities;
            impl ::subxt::StorageEntry for BestAuthorities {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "BestAuthorities";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u16,
                    runtime_types::dkg_runtime_primitives::crypto::Public,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextBestAuthorities;
            impl ::subxt::StorageEntry for NextBestAuthorities {
                const PALLET: &'static str = "DKG";
                const STORAGE: &'static str = "NextBestAuthorities";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u16,
                    runtime_types::dkg_runtime_primitives::crypto::Public,
                )>;
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
                pub async fn should_manual_refresh(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::bool,
                    ::subxt::BasicError,
                > {
                    let entry = ShouldManualRefresh;
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
                    _0: &::core::primitive::u64,
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
                    ::subxt::KeyIter<'a, T, HistoricalRounds<'a>>,
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
                pub async fn keygen_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u16,
                    ::subxt::BasicError,
                > {
                    let entry = KeygenThreshold;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_signature_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u16,
                    ::subxt::BasicError,
                > {
                    let entry = NextSignatureThreshold;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keygen_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u16,
                    ::subxt::BasicError,
                > {
                    let entry = NextKeygenThreshold;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_signature_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u16,
                    ::subxt::BasicError,
                > {
                    let entry = PendingSignatureThreshold;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_keygen_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u16,
                    ::subxt::BasicError,
                > {
                    let entry = PendingKeygenThreshold;
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
                pub async fn next_authority_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u64,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthoritySetId;
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
                pub async fn account_to_authority(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::dkg_runtime_primitives::crypto::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = AccountToAuthority(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn account_to_authority_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AccountToAuthority<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn misbehaviour_reports (& self , _0 : & runtime_types :: dkg_runtime_primitives :: MisbehaviourType , _1 : & :: core :: primitive :: u64 , _2 : & runtime_types :: dkg_runtime_primitives :: crypto :: Public , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: dkg_runtime_primitives :: AggregatedMisbehaviourReports < runtime_types :: dkg_runtime_primitives :: crypto :: Public > > , :: subxt :: BasicError >{
                    let entry = MisbehaviourReports(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn misbehaviour_reports_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, MisbehaviourReports<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn authority_reputations(
                    &self,
                    _0: &runtime_types::dkg_runtime_primitives::crypto::Public,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u128,
                    ::subxt::BasicError,
                > {
                    let entry = AuthorityReputations(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authority_reputations_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AuthorityReputations<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn jailed_keygen_authorities(
                    &self,
                    _0: &runtime_types::dkg_runtime_primitives::crypto::Public,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = JailedKeygenAuthorities(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn jailed_keygen_authorities_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, JailedKeygenAuthorities<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn jailed_signing_authorities(
                    &self,
                    _0: &runtime_types::dkg_runtime_primitives::crypto::Public,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = JailedSigningAuthorities(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn jailed_signing_authorities_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, JailedSigningAuthorities<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn best_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u16,
                        runtime_types::dkg_runtime_primitives::crypto::Public,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = BestAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_best_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u16,
                        runtime_types::dkg_runtime_primitives::crypto::Public,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = NextBestAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn refresh_delay(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Permill,
                    ::subxt::BasicError,
                > {
                    let pallet = self.client.metadata().pallet("DKG")?;
                    let constant = pallet.constant("RefreshDelay")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
            }
        }
    }
    pub mod dkg_proposals {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct SetThreshold {
                pub threshold: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetThreshold {
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
            pub struct SetResource {
                pub id: runtime_types::webb_proposals::header::ResourceId,
                pub method: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetResource {
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
            pub struct RemoveResource {
                pub id: runtime_types::webb_proposals::header::ResourceId,
            }
            impl ::subxt::Call for RemoveResource {
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
            pub struct WhitelistChain {
                pub chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
            }
            impl ::subxt::Call for WhitelistChain {
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
            pub struct AddProposer {
                pub native_account: ::subxt::sp_core::crypto::AccountId32,
                pub external_account: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for AddProposer {
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
            pub struct RemoveProposer {
                pub v: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for RemoveProposer {
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
            pub struct AcknowledgeProposal {
                pub nonce: runtime_types::webb_proposals::nonce::Nonce,
                pub src_chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub r_id: runtime_types::webb_proposals::header::ResourceId,
                pub prop: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for AcknowledgeProposal {
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
            pub struct RejectProposal {
                pub nonce: runtime_types::webb_proposals::nonce::Nonce,
                pub src_chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub r_id: runtime_types::webb_proposals::header::ResourceId,
                pub prop: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RejectProposal {
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
            pub struct EvalVoteState {
                pub nonce: runtime_types::webb_proposals::nonce::Nonce,
                pub src_chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub prop: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for EvalVoteState {
                const PALLET: &'static str = "DKGProposals";
                const FUNCTION: &'static str = "eval_vote_state";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_threshold(
                    &self,
                    threshold: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetThreshold,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetThreshold { threshold };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_resource(
                    &self,
                    id: runtime_types::webb_proposals::header::ResourceId,
                    method: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SetResource,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SetResource { id, method };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_resource(
                    &self,
                    id: runtime_types::webb_proposals::header::ResourceId,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    RemoveResource,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = RemoveResource { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn whitelist_chain(
                    &self,
                    chain_id : runtime_types :: webb_proposals :: header :: TypedChainId,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    WhitelistChain,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = WhitelistChain { chain_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_proposer(
                    &self,
                    native_account: ::subxt::sp_core::crypto::AccountId32,
                    external_account: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    AddProposer,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = AddProposer {
                        native_account,
                        external_account,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proposer(
                    &self,
                    v: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    RemoveProposer,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = RemoveProposer { v };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn acknowledge_proposal(
                    &self,
                    nonce: runtime_types::webb_proposals::nonce::Nonce,
                    src_chain_id : runtime_types :: webb_proposals :: header :: TypedChainId,
                    r_id: runtime_types::webb_proposals::header::ResourceId,
                    prop: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    AcknowledgeProposal,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = AcknowledgeProposal {
                        nonce,
                        src_chain_id,
                        r_id,
                        prop,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_proposal(
                    &self,
                    nonce: runtime_types::webb_proposals::nonce::Nonce,
                    src_chain_id : runtime_types :: webb_proposals :: header :: TypedChainId,
                    r_id: runtime_types::webb_proposals::header::ResourceId,
                    prop: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    RejectProposal,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = RejectProposal {
                        nonce,
                        src_chain_id,
                        r_id,
                        prop,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn eval_vote_state(
                    &self,
                    nonce: runtime_types::webb_proposals::nonce::Nonce,
                    src_chain_id : runtime_types :: webb_proposals :: header :: TypedChainId,
                    prop: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    EvalVoteState,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = EvalVoteState {
                        nonce,
                        src_chain_id,
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
                    runtime_types::webb_proposals::header::TypedChainId,
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
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
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
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
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
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
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
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
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
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
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
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
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
            pub struct AuthorityProposersReset {
                pub proposers:
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Event for AuthorityProposersReset {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "AuthorityProposersReset";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ChainNonces<'a>(
                pub &'a runtime_types::webb_proposals::header::TypedChainId,
            );
            impl ::subxt::StorageEntry for ChainNonces<'_> {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "ChainNonces";
                type Value = runtime_types::webb_proposals::nonce::Nonce;
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
            pub struct ProposerSetUpdateProposalNonce;
            impl ::subxt::StorageEntry for ProposerSetUpdateProposalNonce {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "ProposerSetUpdateProposalNonce";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Proposers<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Proposers<'_> {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "Proposers";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct ExternalProposerAccounts<'a>(
                pub &'a ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ExternalProposerAccounts<'_> {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "ExternalProposerAccounts";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct AuthorityProposers;
            impl ::subxt::StorageEntry for AuthorityProposers {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str = "AuthorityProposers";
                type Value =
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExternalAuthorityProposerAccounts;
            impl ::subxt::StorageEntry for ExternalAuthorityProposerAccounts {
                const PALLET: &'static str = "DKGProposals";
                const STORAGE: &'static str =
                    "ExternalAuthorityProposerAccounts";
                type Value =
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
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
            pub struct Votes<'a>(
                pub &'a runtime_types::webb_proposals::header::TypedChainId,
                pub  &'a (
                    runtime_types::webb_proposals::nonce::Nonce,
                    ::std::vec::Vec<::core::primitive::u8>,
                ),
            );
            impl ::subxt::StorageEntry for Votes<'_> {
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
            pub struct Resources<'a>(
                pub &'a runtime_types::webb_proposals::header::ResourceId,
            );
            impl ::subxt::StorageEntry for Resources<'_> {
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
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn chain_nonces(
                    &self,
                    _0: &runtime_types::webb_proposals::header::TypedChainId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::webb_proposals::nonce::Nonce,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ChainNonces(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn chain_nonces_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ChainNonces<'a>>,
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
                pub async fn proposer_set_update_proposal_nonce(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let entry = ProposerSetUpdateProposalNonce;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposers(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
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
                    ::subxt::KeyIter<'a, T, Proposers<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn external_proposer_accounts(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ExternalProposerAccounts(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn external_proposer_accounts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExternalProposerAccounts<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn authority_proposers(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = AuthorityProposers;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn external_authority_proposer_accounts(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    let entry = ExternalAuthorityProposerAccounts;
                    self.client.storage().fetch_or_default(&entry, hash).await
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
                }                pub async fn votes (& self , _0 : & runtime_types :: webb_proposals :: header :: TypedChainId , _1 : & (runtime_types :: webb_proposals :: nonce :: Nonce , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: pallet_dkg_proposals :: types :: ProposalVotes < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , :: subxt :: BasicError >{
                    let entry = Votes(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn votes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Votes<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn resources(
                    &self,
                    _0: &runtime_types::webb_proposals::header::ResourceId,
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
                    ::subxt::KeyIter<'a, T, Resources<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn chain_identifier(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::webb_proposals::header::TypedChainId,
                    ::subxt::BasicError,
                > {
                    let pallet =
                        self.client.metadata().pallet("DKGProposals")?;
                    let constant = pallet.constant("ChainIdentifier")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn proposal_lifetime(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet =
                        self.client.metadata().pallet("DKGProposals")?;
                    let constant = pallet.constant("ProposalLifetime")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
                pub fn period(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u32,
                    ::subxt::BasicError,
                > {
                    let pallet =
                        self.client.metadata().pallet("DKGProposals")?;
                    let constant = pallet.constant("Period")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
                }
            }
        }
    }
    pub mod dkg_proposal_handler {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
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
            pub struct SubmitSignedProposals {
                pub props: ::std::vec::Vec<
                    runtime_types::dkg_runtime_primitives::proposal::Proposal,
                >,
            }
            impl ::subxt::Call for SubmitSignedProposals {
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
            pub struct ForceSubmitUnsignedProposal {
                pub prop:
                    runtime_types::dkg_runtime_primitives::proposal::Proposal,
            }
            impl ::subxt::Call for ForceSubmitUnsignedProposal {
                const PALLET: &'static str = "DKGProposalHandler";
                const FUNCTION: &'static str = "force_submit_unsigned_proposal";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn submit_signed_proposals(
                    &self,
                    props : :: std :: vec :: Vec < runtime_types :: dkg_runtime_primitives :: proposal :: Proposal >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    SubmitSignedProposals,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = SubmitSignedProposals { props };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_submit_unsigned_proposal(
                    &self,
                    prop : runtime_types :: dkg_runtime_primitives :: proposal :: Proposal,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    ForceSubmitUnsignedProposal,
                    DispatchError,
                    root_mod::Event,
                > {
                    let call = ForceSubmitUnsignedProposal { prop };
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
            pub struct InvalidProposalSignature { pub kind : runtime_types :: dkg_runtime_primitives :: proposal :: ProposalKind , pub data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , pub invalid_signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , }
            impl ::subxt::Event for InvalidProposalSignature {
                const PALLET: &'static str = "DKGProposalHandler";
                const EVENT: &'static str = "InvalidProposalSignature";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub struct ProposalSigned { pub key : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , pub target_chain : runtime_types :: webb_proposals :: header :: TypedChainId , pub data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , pub signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , }
            impl ::subxt::Event for ProposalSigned {
                const PALLET: &'static str = "DKGProposalHandler";
                const EVENT: &'static str = "ProposalSigned";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct UnsignedProposalQueue < 'a > (pub & 'a runtime_types :: webb_proposals :: header :: TypedChainId , pub & 'a runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey) ;
            impl ::subxt::StorageEntry for UnsignedProposalQueue<'_> {
                const PALLET: &'static str = "DKGProposalHandler";
                const STORAGE: &'static str = "UnsignedProposalQueue";
                type Value =
                    runtime_types::dkg_runtime_primitives::proposal::Proposal;
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
            pub struct SignedProposals < 'a > (pub & 'a runtime_types :: webb_proposals :: header :: TypedChainId , pub & 'a runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey) ;
            impl ::subxt::StorageEntry for SignedProposals<'_> {
                const PALLET: &'static str = "DKGProposalHandler";
                const STORAGE: &'static str = "SignedProposals";
                type Value =
                    runtime_types::dkg_runtime_primitives::proposal::Proposal;
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
                }                pub async fn unsigned_proposal_queue (& self , _0 : & runtime_types :: webb_proposals :: header :: TypedChainId , _1 : & runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: dkg_runtime_primitives :: proposal :: Proposal > , :: subxt :: BasicError >{
                    let entry = UnsignedProposalQueue(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn unsigned_proposal_queue_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnsignedProposalQueue<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn signed_proposals (& self , _0 : & runtime_types :: webb_proposals :: header :: TypedChainId , _1 : & runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: dkg_runtime_primitives :: proposal :: Proposal > , :: subxt :: BasicError >{
                    let entry = SignedProposals(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signed_proposals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SignedProposals<'a>>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn max_submissions_per_batch(
                    &self,
                ) -> ::core::result::Result<
                    ::core::primitive::u16,
                    ::subxt::BasicError,
                > {
                    let pallet =
                        self.client.metadata().pallet("DKGProposalHandler")?;
                    let constant = pallet.constant("MaxSubmissionsPerBatch")?;
                    let value = ::subxt::codec::Decode::decode(
                        &mut &constant.value[..],
                    )?;
                    Ok(value)
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
                    EVMProposal(runtime_types::webb_proposals::nonce::Nonce),
                    #[codec(index = 1)]
                    RefreshVote(runtime_types::webb_proposals::nonce::Nonce),
                    #[codec(index = 2)]
                    ProposerSetUpdateProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 3)]
                    AnchorCreateProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 4)]
                    AnchorUpdateProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 5)]
                    TokenAddProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 6)]
                    TokenRemoveProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 7)]
                    WrappingFeeUpdateProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 8)]
                    ResourceIdUpdateProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 9)]
                    RescueTokensProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 10)]
                    MaxDepositLimitUpdateProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 11)]
                    MinWithdrawalLimitUpdateProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 12)]
                    SetVerifierProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 13)]
                    SetTreasuryHandlerProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
                    ),
                    #[codec(index = 14)]
                    FeeRecipientUpdateProposal(
                        runtime_types::webb_proposals::nonce::Nonce,
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
                pub enum Proposal {
                    # [codec (index = 0)] Signed { kind : runtime_types :: dkg_runtime_primitives :: proposal :: ProposalKind , data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 1)] Unsigned { kind : runtime_types :: dkg_runtime_primitives :: proposal :: ProposalKind , data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum ProposalKind {
                    #[codec(index = 0)]
                    Refresh,
                    #[codec(index = 1)]
                    ProposerSetUpdate,
                    #[codec(index = 2)]
                    EVM,
                    #[codec(index = 3)]
                    AnchorCreate,
                    #[codec(index = 4)]
                    AnchorUpdate,
                    #[codec(index = 5)]
                    TokenAdd,
                    #[codec(index = 6)]
                    TokenRemove,
                    #[codec(index = 7)]
                    WrappingFeeUpdate,
                    #[codec(index = 8)]
                    ResourceIdUpdate,
                    #[codec(index = 9)]
                    RescueTokens,
                    #[codec(index = 10)]
                    MaxDepositLimitUpdate,
                    #[codec(index = 11)]
                    MinWithdrawalLimitUpdate,
                    #[codec(index = 12)]
                    SetVerifier,
                    #[codec(index = 13)]
                    SetTreasuryHandler,
                    #[codec(index = 14)]
                    FeeRecipientUpdate,
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
                    pub nonce: runtime_types::webb_proposals::nonce::Nonce,
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
            pub struct AggregatedMisbehaviourReports<_0> {
                pub misbehaviour_type:
                    runtime_types::dkg_runtime_primitives::MisbehaviourType,
                pub round_id: ::core::primitive::u64,
                pub offender: _0,
                pub reporters: ::std::vec::Vec<_0>,
                pub signatures:
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
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
            pub enum MisbehaviourType {
                #[codec(index = 0)]
                Keygen,
                #[codec(index = 1)]
                Sign,
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
                # [codec (index = 0)] System (runtime_types :: frame_system :: pallet :: Call ,) , # [codec (index = 2)] Timestamp (runtime_types :: pallet_timestamp :: pallet :: Call ,) , # [codec (index = 4)] Grandpa (runtime_types :: pallet_grandpa :: pallet :: Call ,) , # [codec (index = 5)] Balances (runtime_types :: pallet_balances :: pallet :: Call ,) , # [codec (index = 7)] Sudo (runtime_types :: pallet_sudo :: pallet :: Call ,) , # [codec (index = 8)] ElectionProviderMultiPhase (runtime_types :: pallet_election_provider_multi_phase :: pallet :: Call ,) , # [codec (index = 9)] Staking (runtime_types :: pallet_staking :: pallet :: pallet :: Call ,) , # [codec (index = 10)] Session (runtime_types :: pallet_session :: pallet :: Call ,) , # [codec (index = 12)] DKG (runtime_types :: pallet_dkg_metadata :: pallet :: Call ,) , # [codec (index = 13)] DKGProposals (runtime_types :: pallet_dkg_proposals :: pallet :: Call ,) , # [codec (index = 14)] DKGProposalHandler (runtime_types :: pallet_dkg_proposal_handler :: pallet :: Call ,) , }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Eq,
                PartialEq,
                Clone,
            )]
            pub enum Event {
                # [codec (index = 0)] System (runtime_types :: frame_system :: pallet :: Event ,) , # [codec (index = 4)] Grandpa (runtime_types :: pallet_grandpa :: pallet :: Event ,) , # [codec (index = 5)] Balances (runtime_types :: pallet_balances :: pallet :: Event ,) , # [codec (index = 7)] Sudo (runtime_types :: pallet_sudo :: pallet :: Event ,) , # [codec (index = 8)] ElectionProviderMultiPhase (runtime_types :: pallet_election_provider_multi_phase :: pallet :: Event ,) , # [codec (index = 9)] Staking (runtime_types :: pallet_staking :: pallet :: pallet :: Event ,) , # [codec (index = 10)] Session (runtime_types :: pallet_session :: pallet :: Event ,) , # [codec (index = 12)] DKG (runtime_types :: pallet_dkg_metadata :: pallet :: Event ,) , # [codec (index = 13)] DKGProposals (runtime_types :: pallet_dkg_proposals :: pallet :: Event ,) , # [codec (index = 14)] DKGProposalHandler (runtime_types :: pallet_dkg_proposal_handler :: pallet :: Event ,) , }
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
                        pub ::subxt::KeyedVec<_0, _1>,
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
                    # [codec (index = 0)] set_signature_threshold { new_threshold : :: core :: primitive :: u16 , } , # [codec (index = 1)] set_keygen_threshold { new_threshold : :: core :: primitive :: u16 , } , # [codec (index = 2)] set_refresh_delay { new_delay : :: core :: primitive :: u8 , } , # [codec (index = 3)] submit_public_key { keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys , } , # [codec (index = 4)] submit_next_public_key { keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys , } , # [codec (index = 5)] submit_public_key_signature { signature_proposal : runtime_types :: dkg_runtime_primitives :: proposal :: RefreshProposalSigned , } , # [codec (index = 6)] submit_misbehaviour_reports { reports : runtime_types :: dkg_runtime_primitives :: AggregatedMisbehaviourReports < runtime_types :: dkg_runtime_primitives :: crypto :: Public > , } , # [codec (index = 7)] unjail , # [codec (index = 8)] force_unjail_keygen { authority : runtime_types :: dkg_runtime_primitives :: crypto :: Public , } , # [codec (index = 9)] force_unjail_signing { authority : runtime_types :: dkg_runtime_primitives :: crypto :: Public , } , # [codec (index = 10)] manual_increment_nonce , # [codec (index = 11)] manual_refresh , # [codec (index = 12)] force_change_authorities , }
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
                    NoMappedAccount,
                    #[codec(index = 1)]
                    InvalidThreshold,
                    #[codec(index = 2)]
                    MustBeAQueuedAuthority,
                    #[codec(index = 3)]
                    MustBeAnActiveAuthority,
                    #[codec(index = 4)]
                    InvalidRefreshDelay,
                    #[codec(index = 5)]
                    InvalidPublicKeys,
                    #[codec(index = 6)]
                    AlreadySubmittedPublicKey,
                    #[codec(index = 7)]
                    AlreadySubmittedSignature,
                    #[codec(index = 8)]
                    UsedSignature,
                    #[codec(index = 9)]
                    InvalidSignature,
                    #[codec(index = 10)]
                    InvalidMisbehaviourReports,
                    #[codec(index = 11)]
                    RefreshInProgress,
                    #[codec(index = 12)]
                    ManualRefreshFailed,
                    #[codec(index = 13)]
                    NoNextPublicKey,
                    #[codec(index = 14)]
                    InvalidControllerAccount,
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
                    # [codec (index = 0)] PublicKeySubmitted { compressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , uncompressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 1)] NextPublicKeySubmitted { compressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , uncompressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] NextPublicKeySignatureSubmitted { pub_key_sig : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 3)] PublicKeyChanged { compressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , uncompressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 4)] PublicKeySignatureChanged { pub_key_sig : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 5)] MisbehaviourReportsSubmitted { misbehaviour_type : runtime_types :: dkg_runtime_primitives :: MisbehaviourType , reporters : :: std :: vec :: Vec < runtime_types :: dkg_runtime_primitives :: crypto :: Public > , } , # [codec (index = 6)] RefreshKeysFinished { next_authority_set_id : :: core :: primitive :: u64 , } , }
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
                    # [codec (index = 0)] submit_signed_proposals { props : :: std :: vec :: Vec < runtime_types :: dkg_runtime_primitives :: proposal :: Proposal > , } , # [codec (index = 1)] force_submit_unsigned_proposal { prop : runtime_types :: dkg_runtime_primitives :: proposal :: Proposal , } , }
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
                    # [codec (index = 0)] InvalidProposalSignature { kind : runtime_types :: dkg_runtime_primitives :: proposal :: ProposalKind , data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , invalid_signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 1)] ProposalSigned { key : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , target_chain : runtime_types :: webb_proposals :: header :: TypedChainId , data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
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
                    set_threshold { threshold: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    set_resource {
                        id: runtime_types::webb_proposals::header::ResourceId,
                        method: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    remove_resource {
                        id: runtime_types::webb_proposals::header::ResourceId,
                    },
                    #[codec(index = 3)]
                    whitelist_chain {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                    },
                    #[codec(index = 4)]
                    add_proposer {
                        native_account: ::subxt::sp_core::crypto::AccountId32,
                        external_account:
                            ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    remove_proposer {
                        v: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    acknowledge_proposal {
                        nonce: runtime_types::webb_proposals::nonce::Nonce,
                        src_chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        r_id: runtime_types::webb_proposals::header::ResourceId,
                        prop: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 7)]
                    reject_proposal {
                        nonce: runtime_types::webb_proposals::nonce::Nonce,
                        src_chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        r_id: runtime_types::webb_proposals::header::ResourceId,
                        prop: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    eval_vote_state {
                        nonce: runtime_types::webb_proposals::nonce::Nonce,
                        src_chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
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
                    #[codec(index = 16)]
                    ProposerCountIsZero,
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
                    ProposerThresholdChanged {
                        new_threshold: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    ChainWhitelisted {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                    },
                    #[codec(index = 2)]
                    ProposerAdded {
                        proposer_id: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 3)]
                    ProposerRemoved {
                        proposer_id: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    VoteFor {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    VoteAgainst {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    ProposalApproved {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                    },
                    #[codec(index = 7)]
                    ProposalRejected {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                    },
                    #[codec(index = 8)]
                    ProposalSucceeded {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                    },
                    #[codec(index = 9)]
                    ProposalFailed {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                    },
                    #[codec(index = 10)]
                    AuthorityProposersReset {
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
                    # [codec (index = 0)] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < [:: core :: primitive :: u128 ; 3usize] > , } , # [codec (index = 2)] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: sp_core :: crypto :: AccountId32 > ,) > , } , # [codec (index = 3)] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , num_signed_submissions : :: core :: primitive :: u32 , } , # [codec (index = 4)] governance_fallback { maybe_max_voters : :: core :: option :: Option < :: core :: primitive :: u32 > , maybe_max_targets : :: core :: option :: Option < :: core :: primitive :: u32 > , } , }
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
                    #[codec(index = 11)]
                    FallbackFailed,
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
            pub struct RoundSnapshot { pub voters : :: std :: vec :: Vec < (:: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u64 , runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: subxt :: sp_core :: crypto :: AccountId32 > ,) > , pub targets : :: std :: vec :: Vec < :: subxt :: sp_core :: crypto :: AccountId32 > , }
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
                        # [codec (index = 0)] bond { controller : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , # [codec (compact)] value : :: core :: primitive :: u128 , payee : runtime_types :: pallet_staking :: RewardDestination < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 1)] bond_extra { # [codec (compact)] max_additional : :: core :: primitive :: u128 , } , # [codec (index = 2)] unbond { # [codec (compact)] value : :: core :: primitive :: u128 , } , # [codec (index = 3)] withdraw_unbonded { num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 4)] validate { prefs : runtime_types :: pallet_staking :: ValidatorPrefs , } , # [codec (index = 5)] nominate { targets : :: std :: vec :: Vec < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , } , # [codec (index = 6)] chill , # [codec (index = 7)] set_payee { payee : runtime_types :: pallet_staking :: RewardDestination < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 8)] set_controller { controller : :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 9)] set_validator_count { # [codec (compact)] new : :: core :: primitive :: u32 , } , # [codec (index = 10)] increase_validator_count { # [codec (compact)] additional : :: core :: primitive :: u32 , } , # [codec (index = 11)] scale_validator_count { factor : runtime_types :: sp_arithmetic :: per_things :: Percent , } , # [codec (index = 12)] force_no_eras , # [codec (index = 13)] force_new_era , # [codec (index = 14)] set_invulnerables { invulnerables : :: std :: vec :: Vec < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 15)] force_unstake { stash : :: subxt :: sp_core :: crypto :: AccountId32 , num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 16)] force_new_era_always , # [codec (index = 17)] cancel_deferred_slash { era : :: core :: primitive :: u32 , slash_indices : :: std :: vec :: Vec < :: core :: primitive :: u32 > , } , # [codec (index = 18)] payout_stakers { validator_stash : :: subxt :: sp_core :: crypto :: AccountId32 , era : :: core :: primitive :: u32 , } , # [codec (index = 19)] rebond { # [codec (compact)] value : :: core :: primitive :: u128 , } , # [codec (index = 20)] set_history_depth { # [codec (compact)] new_history_depth : :: core :: primitive :: u32 , # [codec (compact)] era_items_deleted : :: core :: primitive :: u32 , } , # [codec (index = 21)] reap_stash { stash : :: subxt :: sp_core :: crypto :: AccountId32 , num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 22)] kick { who : :: std :: vec :: Vec < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , } , # [codec (index = 23)] set_staking_configs { min_nominator_bond : :: core :: primitive :: u128 , min_validator_bond : :: core :: primitive :: u128 , max_nominator_count : :: core :: option :: Option < :: core :: primitive :: u32 > , max_validator_count : :: core :: option :: Option < :: core :: primitive :: u32 > , chill_threshold : :: core :: option :: Option < runtime_types :: sp_arithmetic :: per_things :: Percent > , min_commission : runtime_types :: sp_arithmetic :: per_things :: Perbill , } , # [codec (index = 24)] chill_other { controller : :: subxt :: sp_core :: crypto :: AccountId32 , } , # [codec (index = 25)] force_apply_min_commission { validator_stash : :: subxt :: sp_core :: crypto :: AccountId32 , } , }
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
                pub individual: ::subxt::KeyedVec<_0, ::core::primitive::u32>,
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
            pub struct Nominations { pub targets : runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < :: subxt :: sp_core :: crypto :: AccountId32 > , pub submitted_in : :: core :: primitive :: u32 , pub suppressed : :: core :: primitive :: bool , }
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
                Module(runtime_types::sp_runtime::ModuleError),
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
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: ::core::primitive::u8,
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
        pub mod webb_proposals {
            use super::runtime_types;
            pub mod header {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub struct ResourceId(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Eq,
                    PartialEq,
                    Clone,
                )]
                pub enum TypedChainId {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Evm(::core::primitive::u32),
                    #[codec(index = 2)]
                    Substrate(::core::primitive::u32),
                    #[codec(index = 3)]
                    PolkadotParachain(::core::primitive::u32),
                    #[codec(index = 4)]
                    KusamaParachain(::core::primitive::u32),
                    #[codec(index = 5)]
                    RococoParachain(::core::primitive::u32),
                    #[codec(index = 6)]
                    Cosmos(::core::primitive::u32),
                    #[codec(index = 7)]
                    Solana(::core::primitive::u32),
                }
            }
            pub mod nonce {
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
                pub struct Nonce(pub ::core::primitive::u32);
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    impl ::subxt::HasModuleError for runtime_types::sp_runtime::DispatchError {
        fn module_error_data(&self) -> Option<::subxt::ModuleErrorData> {
            if let Self::Module(module_error) = self {
                Some(::subxt::ModuleErrorData {
                    pallet_index: module_error.index,
                    error: [module_error.error, 0, 0, 0],
                })
            } else {
                None
            }
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config, X> {
        pub client: ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
    }
    impl<T, X> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self {
                client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl<'a, T, X> RuntimeApi<T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        pub fn constants(&'a self) -> ConstantsApi<'a, T> {
            ConstantsApi {
                client: &self.client,
            }
        }
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T, X> {
            TransactionApi {
                client: &self.client,
                marker: ::core::marker::PhantomData,
            }
        }
        pub fn events(&'a self) -> EventsApi<'a, T> {
            EventsApi {
                client: &self.client,
            }
        }
    }
    pub struct EventsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T: ::subxt::Config> EventsApi<'a, T> {
        pub async fn at(
            &self,
            block_hash: T::Hash,
        ) -> Result<::subxt::events::Events<'a, T, Event>, ::subxt::BasicError>
        {
            ::subxt::events::at::<T, Event>(self.client, block_hash).await
        }
        pub async fn subscribe(
            &self,
        ) -> Result<
            ::subxt::events::EventSubscription<
                'a,
                ::subxt::events::EventSub<T::Header>,
                T,
                Event,
            >,
            ::subxt::BasicError,
        > {
            ::subxt::events::subscribe::<T, Event>(self.client).await
        }
        pub async fn subscribe_finalized(
            &self,
        ) -> Result<
            ::subxt::events::EventSubscription<
                'a,
                ::subxt::events::FinalizedEventSub<'a, T::Header>,
                T,
                Event,
            >,
            ::subxt::BasicError,
        > {
            ::subxt::events::subscribe_finalized::<T, Event>(self.client).await
        }
    }
    pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn system(&self) -> system::constants::ConstantsApi<'a, T> {
            system::constants::ConstantsApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi<'a, T> {
            timestamp::constants::ConstantsApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi<'a, T> {
            grandpa::constants::ConstantsApi::new(self.client)
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi<'a, T> {
            balances::constants::ConstantsApi::new(self.client)
        }
        pub fn transaction_payment(
            &self,
        ) -> transaction_payment::constants::ConstantsApi<'a, T> {
            transaction_payment::constants::ConstantsApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::constants::ConstantsApi<'a, T>
        {
            election_provider_multi_phase::constants::ConstantsApi::new(
                self.client,
            )
        }
        pub fn staking(&self) -> staking::constants::ConstantsApi<'a, T> {
            staking::constants::ConstantsApi::new(self.client)
        }
        pub fn dkg(&self) -> dkg::constants::ConstantsApi<'a, T> {
            dkg::constants::ConstantsApi::new(self.client)
        }
        pub fn dkg_proposals(
            &self,
        ) -> dkg_proposals::constants::ConstantsApi<'a, T> {
            dkg_proposals::constants::ConstantsApi::new(self.client)
        }
        pub fn dkg_proposal_handler(
            &self,
        ) -> dkg_proposal_handler::constants::ConstantsApi<'a, T> {
            dkg_proposal_handler::constants::ConstantsApi::new(self.client)
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
        pub fn dkg_proposal_handler(
            &self,
        ) -> dkg_proposal_handler::storage::StorageApi<'a, T> {
            dkg_proposal_handler::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
    }
    impl<'a, T, X> TransactionApi<'a, T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T, X> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T, X> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::calls::TransactionApi<'a, T, X>
        {
            election_provider_multi_phase::calls::TransactionApi::new(
                self.client,
            )
        }
        pub fn staking(&self) -> staking::calls::TransactionApi<'a, T, X> {
            staking::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T, X> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn dkg(&self) -> dkg::calls::TransactionApi<'a, T, X> {
            dkg::calls::TransactionApi::new(self.client)
        }
        pub fn dkg_proposals(
            &self,
        ) -> dkg_proposals::calls::TransactionApi<'a, T, X> {
            dkg_proposals::calls::TransactionApi::new(self.client)
        }
        pub fn dkg_proposal_handler(
            &self,
        ) -> dkg_proposal_handler::calls::TransactionApi<'a, T, X> {
            dkg_proposal_handler::calls::TransactionApi::new(self.client)
        }
    }
}
