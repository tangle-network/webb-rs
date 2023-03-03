#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    use super::api as root_mod;
    pub static PALLETS: [&str; 21usize] = [
        "System",
        "Indices",
        "RandomnessCollectiveFlip",
        "Timestamp",
        "Aura",
        "Grandpa",
        "Balances",
        "DKG",
        "DKGProposals",
        "DKGProposalHandler",
        "TransactionPayment",
        "Sudo",
        "ElectionProviderMultiPhase",
        "BagsList",
        "NominationPools",
        "Staking",
        "Session",
        "Historical",
        "BridgeRegistry",
        "Identity",
        "ImOnline",
    ];
    #[derive(
        :: subxt :: ext :: codec :: Decode,
        :: subxt :: ext :: codec :: Encode,
        Clone,
        Debug,
        Eq,
        PartialEq,
    )]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 1)]
        Indices(indices::Event),
        #[codec(index = 5)]
        Grandpa(grandpa::Event),
        #[codec(index = 6)]
        Balances(balances::Event),
        #[codec(index = 7)]
        DKG(dkg::Event),
        #[codec(index = 8)]
        DKGProposals(dkg_proposals::Event),
        #[codec(index = 9)]
        DKGProposalHandler(dkg_proposal_handler::Event),
        #[codec(index = 10)]
        TransactionPayment(transaction_payment::Event),
        #[codec(index = 11)]
        Sudo(sudo::Event),
        #[codec(index = 12)]
        ElectionProviderMultiPhase(election_provider_multi_phase::Event),
        #[codec(index = 13)]
        BagsList(bags_list::Event),
        #[codec(index = 14)]
        NominationPools(nomination_pools::Event),
        #[codec(index = 15)]
        Staking(staking::Event),
        #[codec(index = 16)]
        Session(session::Event),
        #[codec(index = 18)]
        BridgeRegistry(bridge_registry::Event),
        #[codec(index = 19)]
        Identity(identity::Event),
        #[codec(index = 20)]
        ImOnline(im_online::Event),
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct KillStorage {
                pub keys:
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`"]
                #[doc = "# </weight>"]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<Remark> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "remark",
                        Remark { remark },
                        [
                            101u8, 80u8, 195u8, 226u8, 224u8, 247u8, 60u8,
                            128u8, 3u8, 101u8, 51u8, 147u8, 96u8, 126u8, 76u8,
                            230u8, 194u8, 227u8, 191u8, 73u8, 160u8, 146u8,
                            87u8, 147u8, 243u8, 28u8, 228u8, 116u8, 224u8,
                            181u8, 129u8, 160u8,
                        ],
                    )
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<SetHeapPages>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_heap_pages",
                        SetHeapPages { pages },
                        [
                            43u8, 103u8, 128u8, 49u8, 156u8, 136u8, 11u8,
                            204u8, 80u8, 6u8, 244u8, 86u8, 171u8, 44u8, 140u8,
                            225u8, 142u8, 198u8, 43u8, 87u8, 26u8, 45u8, 125u8,
                            222u8, 165u8, 254u8, 172u8, 158u8, 39u8, 178u8,
                            86u8, 87u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                #[doc = "  expensive)."]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                #[doc = "expensive. We will treat this as a full block."]
                #[doc = "# </weight>"]
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetCode> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_code",
                        SetCode { code },
                        [
                            27u8, 104u8, 244u8, 205u8, 188u8, 254u8, 121u8,
                            13u8, 106u8, 120u8, 244u8, 108u8, 97u8, 84u8,
                            100u8, 68u8, 26u8, 69u8, 93u8, 128u8, 107u8, 4u8,
                            3u8, 142u8, 13u8, 134u8, 196u8, 62u8, 113u8, 181u8,
                            14u8, 40u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C)` where `C` length of `code`"]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                #[doc = "block. # </weight>"]
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetCodeWithoutChecks>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_code_without_checks",
                        SetCodeWithoutChecks { code },
                        [
                            102u8, 160u8, 125u8, 235u8, 30u8, 23u8, 45u8,
                            239u8, 112u8, 148u8, 159u8, 158u8, 42u8, 93u8,
                            206u8, 94u8, 80u8, 250u8, 66u8, 195u8, 60u8, 40u8,
                            142u8, 169u8, 183u8, 80u8, 80u8, 96u8, 3u8, 231u8,
                            99u8, 216u8,
                        ],
                    )
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::tx::StaticTxPayload<SetStorage> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_storage",
                        SetStorage { items },
                        [
                            74u8, 43u8, 106u8, 255u8, 50u8, 151u8, 192u8,
                            155u8, 14u8, 90u8, 19u8, 45u8, 165u8, 16u8, 235u8,
                            242u8, 21u8, 131u8, 33u8, 172u8, 119u8, 78u8,
                            140u8, 10u8, 107u8, 202u8, 122u8, 235u8, 181u8,
                            191u8, 22u8, 116u8,
                        ],
                    )
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<KillStorage> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "kill_storage",
                        KillStorage { keys },
                        [
                            174u8, 174u8, 13u8, 174u8, 75u8, 138u8, 128u8,
                            235u8, 222u8, 216u8, 85u8, 18u8, 198u8, 1u8, 138u8,
                            70u8, 19u8, 108u8, 209u8, 41u8, 228u8, 67u8, 130u8,
                            230u8, 160u8, 207u8, 11u8, 180u8, 139u8, 242u8,
                            41u8, 15u8,
                        ],
                    )
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<KillPrefix> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "kill_prefix",
                        KillPrefix { prefix, subkeys },
                        [
                            203u8, 116u8, 217u8, 42u8, 154u8, 215u8, 77u8,
                            217u8, 13u8, 22u8, 193u8, 2u8, 128u8, 115u8, 179u8,
                            115u8, 187u8, 218u8, 129u8, 34u8, 80u8, 4u8, 173u8,
                            120u8, 92u8, 35u8, 237u8, 112u8, 201u8, 207u8,
                            200u8, 48u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<RemarkWithEvent>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "remark_with_event",
                        RemarkWithEvent { remark },
                        [
                            123u8, 225u8, 180u8, 179u8, 144u8, 74u8, 27u8,
                            85u8, 101u8, 75u8, 134u8, 44u8, 181u8, 25u8, 183u8,
                            158u8, 14u8, 213u8, 56u8, 225u8, 136u8, 88u8, 26u8,
                            114u8, 178u8, 43u8, 176u8, 43u8, 240u8, 84u8,
                            116u8, 46u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info:
                    runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info:
                    runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::ext::sp_core::crypto::AccountId32,
                pub hash: ::subxt::ext::sp_core::H256,
            }
            impl ::subxt::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]                pub fn account (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: frame_system :: AccountInfo < :: core :: primitive :: u32 , runtime_types :: pallet_balances :: AccountData < :: core :: primitive :: u128 > > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("System" , "Account" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [176u8 , 187u8 , 21u8 , 220u8 , 159u8 , 204u8 , 127u8 , 14u8 , 21u8 , 69u8 , 77u8 , 114u8 , 230u8 , 141u8 , 107u8 , 79u8 , 23u8 , 16u8 , 174u8 , 243u8 , 252u8 , 42u8 , 65u8 , 120u8 , 229u8 , 38u8 , 210u8 , 255u8 , 22u8 , 40u8 , 109u8 , 223u8 ,])
                }
                #[doc = " The full account information for a particular account ID."]                pub fn account_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: frame_system :: AccountInfo < :: core :: primitive :: u32 , runtime_types :: pallet_balances :: AccountData < :: core :: primitive :: u128 > > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Account",
                        Vec::new(),
                        [
                            176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8,
                            14u8, 21u8, 69u8, 77u8, 114u8, 230u8, 141u8, 107u8,
                            79u8, 23u8, 16u8, 174u8, 243u8, 252u8, 42u8, 65u8,
                            120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8,
                            109u8, 223u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]                pub fn extrinsic_count (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicCount",
                        vec![],
                        [
                            223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8,
                            210u8, 242u8, 53u8, 222u8, 154u8, 123u8, 176u8,
                            249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8, 41u8,
                            151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8,
                            163u8, 231u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]                pub fn block_weight (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: frame_support :: dispatch :: PerDispatchClass < runtime_types :: sp_weights :: weight_v2 :: Weight > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockWeight",
                        vec![],
                        [
                            120u8, 67u8, 71u8, 163u8, 36u8, 202u8, 52u8, 106u8,
                            143u8, 155u8, 144u8, 87u8, 142u8, 241u8, 232u8,
                            183u8, 56u8, 235u8, 27u8, 237u8, 20u8, 202u8, 33u8,
                            85u8, 189u8, 0u8, 28u8, 52u8, 198u8, 40u8, 219u8,
                            54u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]                pub fn all_extrinsics_len (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "AllExtrinsicsLen",
                        vec![],
                        [
                            202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8,
                            74u8, 93u8, 164u8, 254u8, 248u8, 254u8, 192u8,
                            32u8, 117u8, 96u8, 149u8, 53u8, 145u8, 219u8, 64u8,
                            234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8,
                            28u8, 134u8, 60u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]                pub fn block_hash (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: subxt :: ext :: sp_core :: H256 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("System" , "BlockHash" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [50u8 , 112u8 , 176u8 , 239u8 , 175u8 , 18u8 , 205u8 , 20u8 , 241u8 , 195u8 , 21u8 , 228u8 , 186u8 , 57u8 , 200u8 , 25u8 , 38u8 , 44u8 , 106u8 , 20u8 , 168u8 , 80u8 , 76u8 , 235u8 , 12u8 , 51u8 , 137u8 , 149u8 , 200u8 , 4u8 , 220u8 , 237u8 ,])
                }
                #[doc = " Map of block numbers to block hashes."]                pub fn block_hash_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: subxt :: ext :: sp_core :: H256 > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockHash",
                        Vec::new(),
                        [
                            50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8,
                            20u8, 241u8, 195u8, 21u8, 228u8, 186u8, 57u8,
                            200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8, 80u8,
                            76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8,
                            220u8, 237u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]                pub fn extrinsic_data (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("System" , "ExtrinsicData" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [210u8 , 224u8 , 211u8 , 186u8 , 118u8 , 210u8 , 185u8 , 194u8 , 238u8 , 211u8 , 254u8 , 73u8 , 67u8 , 184u8 , 31u8 , 229u8 , 168u8 , 125u8 , 98u8 , 23u8 , 241u8 , 59u8 , 49u8 , 86u8 , 126u8 , 9u8 , 114u8 , 163u8 , 160u8 , 62u8 , 50u8 , 67u8 ,])
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]                pub fn extrinsic_data_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicData",
                        Vec::new(),
                        [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8,
                            194u8, 238u8, 211u8, 254u8, 73u8, 67u8, 184u8,
                            31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8, 59u8,
                            49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8,
                            50u8, 67u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]                pub fn number (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Number",
                        vec![],
                        [
                            228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8,
                            172u8, 126u8, 235u8, 246u8, 139u8, 208u8, 15u8,
                            88u8, 245u8, 141u8, 232u8, 43u8, 204u8, 36u8, 87u8,
                            211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8,
                            235u8, 164u8, 191u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]                pub fn parent_hash (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: subxt :: ext :: sp_core :: H256 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ParentHash",
                        vec![],
                        [
                            232u8, 206u8, 177u8, 119u8, 38u8, 57u8, 233u8,
                            50u8, 225u8, 49u8, 169u8, 176u8, 210u8, 51u8,
                            231u8, 176u8, 234u8, 186u8, 188u8, 112u8, 15u8,
                            152u8, 195u8, 232u8, 201u8, 97u8, 208u8, 249u8,
                            9u8, 163u8, 69u8, 36u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]                pub fn digest (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_runtime :: generic :: digest :: Digest > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Digest",
                        vec![],
                        [
                            83u8, 141u8, 200u8, 132u8, 182u8, 55u8, 197u8,
                            122u8, 13u8, 159u8, 31u8, 42u8, 60u8, 191u8, 89u8,
                            221u8, 242u8, 47u8, 199u8, 213u8, 48u8, 216u8,
                            131u8, 168u8, 245u8, 82u8, 56u8, 190u8, 62u8, 69u8,
                            96u8, 37u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]                pub fn events (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < runtime_types :: frame_system :: EventRecord < runtime_types :: dkg_standalone_runtime :: RuntimeEvent , :: subxt :: ext :: sp_core :: H256 > > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Events",
                        vec![],
                        [
                            172u8, 160u8, 195u8, 98u8, 109u8, 82u8, 120u8,
                            216u8, 235u8, 66u8, 197u8, 94u8, 186u8, 238u8,
                            183u8, 23u8, 92u8, 185u8, 190u8, 5u8, 234u8, 43u8,
                            106u8, 28u8, 239u8, 199u8, 217u8, 64u8, 86u8, 30u8,
                            85u8, 57u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]                pub fn event_count (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventCount",
                        vec![],
                        [
                            236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8,
                            187u8, 26u8, 208u8, 203u8, 113u8, 221u8, 233u8,
                            227u8, 9u8, 249u8, 25u8, 202u8, 185u8, 161u8,
                            144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8,
                            52u8, 61u8, 66u8, 112u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]                pub fn event_topics (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: H256 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < (:: core :: primitive :: u32 , :: core :: primitive :: u32 ,) > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("System" , "EventTopics" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [205u8 , 90u8 , 142u8 , 190u8 , 176u8 , 37u8 , 94u8 , 82u8 , 98u8 , 1u8 , 129u8 , 63u8 , 246u8 , 101u8 , 130u8 , 58u8 , 216u8 , 16u8 , 139u8 , 196u8 , 154u8 , 111u8 , 110u8 , 178u8 , 24u8 , 44u8 , 183u8 , 176u8 , 232u8 , 82u8 , 223u8 , 38u8 ,])
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]                pub fn event_topics_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < (:: core :: primitive :: u32 , :: core :: primitive :: u32 ,) > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventTopics",
                        Vec::new(),
                        [
                            205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8,
                            98u8, 1u8, 129u8, 63u8, 246u8, 101u8, 130u8, 58u8,
                            216u8, 16u8, 139u8, 196u8, 154u8, 111u8, 110u8,
                            178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8,
                            223u8, 38u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]                pub fn last_runtime_upgrade (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: frame_system :: LastRuntimeUpgradeInfo > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "LastRuntimeUpgrade",
                        vec![],
                        [
                            52u8, 37u8, 117u8, 111u8, 57u8, 130u8, 196u8, 14u8,
                            99u8, 77u8, 91u8, 126u8, 178u8, 249u8, 78u8, 34u8,
                            9u8, 194u8, 92u8, 105u8, 113u8, 81u8, 185u8, 127u8,
                            245u8, 184u8, 60u8, 29u8, 234u8, 182u8, 96u8,
                            196u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]                pub fn upgraded_to_u32_ref_count (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: bool > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "UpgradedToU32RefCount",
                        vec![],
                        [
                            171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8,
                            59u8, 175u8, 175u8, 178u8, 20u8, 150u8, 213u8,
                            59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8, 83u8,
                            250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8,
                            131u8, 204u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]                pub fn upgraded_to_triple_ref_count (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: bool > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "UpgradedToTripleRefCount",
                        vec![],
                        [
                            90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8,
                            203u8, 56u8, 201u8, 210u8, 244u8, 232u8, 150u8,
                            18u8, 51u8, 105u8, 14u8, 230u8, 103u8, 155u8,
                            246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8,
                            76u8, 40u8, 185u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]                pub fn execution_phase (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: frame_system :: Phase > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExecutionPhase",
                        vec![],
                        [
                            230u8, 183u8, 221u8, 135u8, 226u8, 223u8, 55u8,
                            104u8, 138u8, 224u8, 103u8, 156u8, 222u8, 99u8,
                            203u8, 199u8, 164u8, 168u8, 193u8, 133u8, 201u8,
                            155u8, 63u8, 95u8, 17u8, 206u8, 165u8, 123u8,
                            161u8, 33u8, 172u8, 93u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::limits::BlockWeights,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockWeights",
                        [
                            118u8, 253u8, 239u8, 217u8, 145u8, 115u8, 85u8,
                            86u8, 172u8, 248u8, 139u8, 32u8, 158u8, 126u8,
                            172u8, 188u8, 197u8, 105u8, 145u8, 235u8, 171u8,
                            50u8, 31u8, 225u8, 167u8, 187u8, 241u8, 87u8, 6u8,
                            17u8, 234u8, 185u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::limits::BlockLength,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockLength",
                        [
                            116u8, 184u8, 225u8, 228u8, 207u8, 203u8, 4u8,
                            220u8, 234u8, 198u8, 150u8, 108u8, 205u8, 87u8,
                            194u8, 131u8, 229u8, 51u8, 140u8, 4u8, 47u8, 12u8,
                            200u8, 144u8, 153u8, 62u8, 51u8, 39u8, 138u8,
                            205u8, 203u8, 236u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_weights::RuntimeDbWeight,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "DbWeight",
                        [
                            124u8, 162u8, 190u8, 149u8, 49u8, 177u8, 162u8,
                            231u8, 62u8, 167u8, 199u8, 181u8, 43u8, 232u8,
                            185u8, 116u8, 195u8, 51u8, 233u8, 223u8, 20u8,
                            129u8, 246u8, 13u8, 65u8, 180u8, 64u8, 9u8, 157u8,
                            59u8, 245u8, 118u8,
                        ],
                    )
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_version::RuntimeVersion,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "Version",
                        [
                            93u8, 98u8, 57u8, 243u8, 229u8, 8u8, 234u8, 231u8,
                            72u8, 230u8, 139u8, 47u8, 63u8, 181u8, 17u8, 2u8,
                            220u8, 231u8, 104u8, 237u8, 185u8, 143u8, 165u8,
                            253u8, 188u8, 76u8, 147u8, 12u8, 170u8, 26u8, 74u8,
                            200u8,
                        ],
                    )
                }
                #[doc = " The designated SS58 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8,
                            169u8, 167u8, 227u8, 41u8, 144u8, 11u8, 236u8,
                            82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8, 90u8,
                            208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8,
                            193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod indices {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Claim {
                pub index: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Transfer {
                pub new: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub index: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Free {
                pub index: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceTransfer {
                pub new: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub index: ::core::primitive::u32,
                pub freeze: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Freeze {
                pub index: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Assign an previously unassigned index."]
                #[doc = ""]
                #[doc = "Payment: `Deposit` is reserved from the sender account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `index`: the index to be claimed. This must not be in use."]
                #[doc = ""]
                #[doc = "Emits `IndexAssigned` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`."]
                #[doc = "- One storage mutation (codec `O(1)`)."]
                #[doc = "- One reserve operation."]
                #[doc = "- One event."]
                #[doc = "-------------------"]
                #[doc = "- DB Weight: 1 Read/Write (Accounts)"]
                #[doc = "# </weight>"]
                pub fn claim(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Claim> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Indices",
                        "claim",
                        Claim { index },
                        [
                            5u8, 24u8, 11u8, 173u8, 226u8, 170u8, 0u8, 30u8,
                            193u8, 102u8, 214u8, 59u8, 252u8, 32u8, 221u8,
                            88u8, 196u8, 189u8, 244u8, 18u8, 233u8, 37u8,
                            228u8, 248u8, 76u8, 175u8, 212u8, 233u8, 238u8,
                            203u8, 162u8, 68u8,
                        ],
                    )
                }
                #[doc = "Assign an index already owned by the sender to another account. The balance reservation"]
                #[doc = "is effectively transferred to the new account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `index`: the index to be re-assigned. This must be owned by the sender."]
                #[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
                #[doc = ""]
                #[doc = "Emits `IndexAssigned` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`."]
                #[doc = "- One storage mutation (codec `O(1)`)."]
                #[doc = "- One transfer operation."]
                #[doc = "- One event."]
                #[doc = "-------------------"]
                #[doc = "- DB Weight:"]
                #[doc = "   - Reads: Indices Accounts, System Account (recipient)"]
                #[doc = "   - Writes: Indices Accounts, System Account (recipient)"]
                #[doc = "# </weight>"]
                pub fn transfer(
                    &self,
                    new: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    index: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Transfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Indices",
                        "transfer",
                        Transfer { new, index },
                        [
                            1u8, 83u8, 197u8, 184u8, 8u8, 96u8, 48u8, 146u8,
                            116u8, 76u8, 229u8, 115u8, 226u8, 215u8, 41u8,
                            154u8, 27u8, 34u8, 205u8, 188u8, 10u8, 169u8,
                            203u8, 39u8, 2u8, 236u8, 181u8, 162u8, 115u8,
                            254u8, 42u8, 28u8,
                        ],
                    )
                }
                #[doc = "Free up an index owned by the sender."]
                #[doc = ""]
                #[doc = "Payment: Any previous deposit placed for the index is unreserved in the sender account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must own the index."]
                #[doc = ""]
                #[doc = "- `index`: the index to be freed. This must be owned by the sender."]
                #[doc = ""]
                #[doc = "Emits `IndexFreed` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`."]
                #[doc = "- One storage mutation (codec `O(1)`)."]
                #[doc = "- One reserve operation."]
                #[doc = "- One event."]
                #[doc = "-------------------"]
                #[doc = "- DB Weight: 1 Read/Write (Accounts)"]
                #[doc = "# </weight>"]
                pub fn free(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Free> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Indices",
                        "free",
                        Free { index },
                        [
                            133u8, 202u8, 225u8, 127u8, 69u8, 145u8, 43u8,
                            13u8, 160u8, 248u8, 215u8, 243u8, 232u8, 166u8,
                            74u8, 203u8, 235u8, 138u8, 255u8, 27u8, 163u8,
                            71u8, 254u8, 217u8, 6u8, 208u8, 202u8, 204u8,
                            238u8, 70u8, 126u8, 252u8,
                        ],
                    )
                }
                #[doc = "Force an index to an account. This doesn't require a deposit. If the index is already"]
                #[doc = "held, then any deposit is reimbursed to its current owner."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Root_."]
                #[doc = ""]
                #[doc = "- `index`: the index to be (re-)assigned."]
                #[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
                #[doc = "- `freeze`: if set to `true`, will freeze the index so it cannot be transferred."]
                #[doc = ""]
                #[doc = "Emits `IndexAssigned` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`."]
                #[doc = "- One storage mutation (codec `O(1)`)."]
                #[doc = "- Up to one reserve operation."]
                #[doc = "- One event."]
                #[doc = "-------------------"]
                #[doc = "- DB Weight:"]
                #[doc = "   - Reads: Indices Accounts, System Account (original owner)"]
                #[doc = "   - Writes: Indices Accounts, System Account (original owner)"]
                #[doc = "# </weight>"]
                pub fn force_transfer(
                    &self,
                    new: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    index: ::core::primitive::u32,
                    freeze: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<ForceTransfer>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Indices",
                        "force_transfer",
                        ForceTransfer { new, index, freeze },
                        [
                            126u8, 8u8, 145u8, 175u8, 177u8, 153u8, 131u8,
                            123u8, 184u8, 53u8, 72u8, 207u8, 21u8, 140u8, 87u8,
                            181u8, 172u8, 64u8, 37u8, 165u8, 121u8, 111u8,
                            173u8, 224u8, 181u8, 79u8, 76u8, 134u8, 93u8,
                            169u8, 65u8, 131u8,
                        ],
                    )
                }
                #[doc = "Freeze an index so it will always point to the sender account. This consumes the"]
                #[doc = "deposit."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the signing account must have a"]
                #[doc = "non-frozen account `index`."]
                #[doc = ""]
                #[doc = "- `index`: the index to be frozen in place."]
                #[doc = ""]
                #[doc = "Emits `IndexFrozen` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`."]
                #[doc = "- One storage mutation (codec `O(1)`)."]
                #[doc = "- Up to one slash operation."]
                #[doc = "- One event."]
                #[doc = "-------------------"]
                #[doc = "- DB Weight: 1 Read/Write (Accounts)"]
                #[doc = "# </weight>"]
                pub fn freeze(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Freeze> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Indices",
                        "freeze",
                        Freeze { index },
                        [
                            121u8, 45u8, 118u8, 2u8, 72u8, 48u8, 38u8, 7u8,
                            234u8, 204u8, 68u8, 20u8, 76u8, 251u8, 205u8,
                            246u8, 149u8, 31u8, 168u8, 186u8, 208u8, 90u8,
                            40u8, 47u8, 100u8, 228u8, 188u8, 33u8, 79u8, 220u8,
                            105u8, 209u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_indices::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A account index was assigned."]
            pub struct IndexAssigned {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for IndexAssigned {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexAssigned";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A account index has been freed up (unassigned)."]
            pub struct IndexFreed {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for IndexFreed {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFreed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A account index has been frozen to its current account ID."]
            pub struct IndexFrozen {
                pub index: ::core::primitive::u32,
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for IndexFrozen {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFrozen";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The lookup from index to account."]                pub fn accounts (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , :: core :: primitive :: bool ,) > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Indices" , "Accounts" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [211u8 , 169u8 , 54u8 , 254u8 , 88u8 , 57u8 , 22u8 , 223u8 , 108u8 , 27u8 , 38u8 , 9u8 , 202u8 , 209u8 , 111u8 , 209u8 , 144u8 , 13u8 , 211u8 , 114u8 , 239u8 , 127u8 , 75u8 , 166u8 , 234u8 , 222u8 , 225u8 , 35u8 , 160u8 , 163u8 , 112u8 , 242u8 ,])
                }
                #[doc = " The lookup from index to account."]                pub fn accounts_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , :: core :: primitive :: bool ,) > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Indices",
                        "Accounts",
                        Vec::new(),
                        [
                            211u8, 169u8, 54u8, 254u8, 88u8, 57u8, 22u8, 223u8,
                            108u8, 27u8, 38u8, 9u8, 202u8, 209u8, 111u8, 209u8,
                            144u8, 13u8, 211u8, 114u8, 239u8, 127u8, 75u8,
                            166u8, 234u8, 222u8, 225u8, 35u8, 160u8, 163u8,
                            112u8, 242u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The deposit needed for reserving an index."]
                pub fn deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Indices",
                        "Deposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8,
                            105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8,
                            160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
                            71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8,
                            148u8, 156u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
                #[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
                #[doc = " the oldest hash."]                pub fn random_material (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < :: subxt :: ext :: sp_core :: H256 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "RandomnessCollectiveFlip",
                        "RandomMaterial",
                        vec![],
                        [
                            152u8, 126u8, 73u8, 88u8, 54u8, 147u8, 6u8, 19u8,
                            214u8, 40u8, 159u8, 30u8, 236u8, 61u8, 240u8, 65u8,
                            178u8, 94u8, 146u8, 152u8, 135u8, 252u8, 160u8,
                            86u8, 123u8, 114u8, 251u8, 140u8, 98u8, 143u8,
                            217u8, 242u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "`MinimumPeriod`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                #[doc = "# </weight>"]
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<Set> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Timestamp",
                        "set",
                        Set { now },
                        [
                            6u8, 97u8, 172u8, 236u8, 118u8, 238u8, 228u8,
                            114u8, 15u8, 115u8, 102u8, 85u8, 66u8, 151u8, 16u8,
                            33u8, 187u8, 17u8, 166u8, 88u8, 127u8, 214u8,
                            182u8, 51u8, 168u8, 88u8, 43u8, 101u8, 185u8, 8u8,
                            1u8, 28u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Current time for the current block."]                pub fn now (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u64 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Timestamp",
                        "Now",
                        vec![],
                        [
                            148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8,
                            16u8, 83u8, 144u8, 221u8, 59u8, 75u8, 158u8, 130u8,
                            39u8, 123u8, 106u8, 134u8, 202u8, 185u8, 83u8,
                            85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8,
                            250u8,
                        ],
                    )
                }
                #[doc = " Did the timestamp get updated in this block?"]                pub fn did_update (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: bool > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Timestamp",
                        "DidUpdate",
                        vec![],
                        [
                            70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8,
                            158u8, 232u8, 175u8, 13u8, 103u8, 135u8, 2u8, 78u8,
                            16u8, 6u8, 39u8, 158u8, 167u8, 85u8, 27u8, 47u8,
                            122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8,
                            231u8, 190u8, 146u8, 59u8, 226u8, 157u8, 101u8,
                            103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8,
                            119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8,
                            239u8, 42u8, 246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current authority set."]                pub fn authorities (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < runtime_types :: sp_consensus_aura :: sr25519 :: app_sr25519 :: Public > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Aura",
                        "Authorities",
                        vec![],
                        [
                            199u8, 89u8, 94u8, 48u8, 249u8, 35u8, 105u8, 90u8,
                            15u8, 86u8, 218u8, 85u8, 22u8, 236u8, 228u8, 36u8,
                            137u8, 64u8, 236u8, 171u8, 242u8, 217u8, 91u8,
                            240u8, 205u8, 205u8, 226u8, 16u8, 147u8, 235u8,
                            181u8, 41u8,
                        ],
                    )
                }
                #[doc = " The current slot of this block."]
                #[doc = ""]
                #[doc = " This will be set in `on_initialize`."]                pub fn current_slot (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_consensus_slots :: Slot > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Aura",
                        "CurrentSlot",
                        vec![],
                        [
                            139u8, 237u8, 185u8, 137u8, 251u8, 179u8, 69u8,
                            167u8, 133u8, 168u8, 204u8, 64u8, 178u8, 123u8,
                            92u8, 250u8, 119u8, 190u8, 208u8, 178u8, 208u8,
                            176u8, 124u8, 187u8, 74u8, 165u8, 33u8, 78u8,
                            161u8, 206u8, 8u8, 108u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ReportEquivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::ext::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::ext::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub fn report_equivocation(
                    &self,
                    equivocation_proof : runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: ext :: sp_core :: H256 , :: core :: primitive :: u32 >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::tx::StaticTxPayload<ReportEquivocation>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Grandpa",
                        "report_equivocation",
                        ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(
                                equivocation_proof,
                            ),
                            key_owner_proof,
                        },
                        [
                            99u8, 59u8, 234u8, 30u8, 150u8, 187u8, 133u8,
                            167u8, 92u8, 34u8, 231u8, 208u8, 141u8, 40u8,
                            182u8, 200u8, 82u8, 198u8, 254u8, 56u8, 72u8, 77u8,
                            41u8, 186u8, 80u8, 213u8, 78u8, 214u8, 215u8,
                            225u8, 187u8, 28u8,
                        ],
                    )
                }
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                #[doc = ""]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof : runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: ext :: sp_core :: H256 , :: core :: primitive :: u32 >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::tx::StaticTxPayload<ReportEquivocationUnsigned>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Grandpa",
                        "report_equivocation_unsigned",
                        ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(
                                equivocation_proof,
                            ),
                            key_owner_proof,
                        },
                        [
                            145u8, 84u8, 164u8, 4u8, 14u8, 22u8, 157u8, 100u8,
                            5u8, 21u8, 60u8, 65u8, 183u8, 32u8, 212u8, 33u8,
                            183u8, 167u8, 54u8, 57u8, 204u8, 4u8, 28u8, 71u8,
                            250u8, 151u8, 1u8, 206u8, 222u8, 102u8, 89u8, 55u8,
                        ],
                    )
                }
                #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                #[doc = ""]
                #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                #[doc = "block of all validators of the new authority set."]
                #[doc = ""]
                #[doc = "Only callable by root."]
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<NoteStalled> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Grandpa",
                        "note_stalled",
                        NoteStalled {
                            delay,
                            best_finalized_block_number,
                        },
                        [
                            197u8, 236u8, 137u8, 32u8, 46u8, 200u8, 144u8,
                            13u8, 89u8, 181u8, 235u8, 73u8, 167u8, 131u8,
                            174u8, 93u8, 42u8, 136u8, 238u8, 59u8, 129u8, 60u8,
                            83u8, 100u8, 5u8, 182u8, 99u8, 250u8, 145u8, 180u8,
                            1u8, 199u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "New authority set has been applied."]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::events::StaticEvent for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl ::subxt::events::StaticEvent for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Current authority set has been resumed."]
            pub struct Resumed;
            impl ::subxt::events::StaticEvent for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " State of the current authority set."]                pub fn state (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_grandpa :: StoredState < :: core :: primitive :: u32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "State",
                        vec![],
                        [
                            211u8, 149u8, 114u8, 217u8, 206u8, 194u8, 115u8,
                            67u8, 12u8, 218u8, 246u8, 213u8, 208u8, 29u8,
                            216u8, 104u8, 2u8, 39u8, 123u8, 172u8, 252u8,
                            210u8, 52u8, 129u8, 147u8, 237u8, 244u8, 68u8,
                            252u8, 169u8, 97u8, 148u8,
                        ],
                    )
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]                pub fn pending_change (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_grandpa :: StoredPendingChange < :: core :: primitive :: u32 > > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "PendingChange",
                        vec![],
                        [
                            178u8, 24u8, 140u8, 7u8, 8u8, 196u8, 18u8, 58u8,
                            3u8, 226u8, 181u8, 47u8, 155u8, 160u8, 70u8, 12u8,
                            75u8, 189u8, 38u8, 255u8, 104u8, 141u8, 64u8, 34u8,
                            134u8, 201u8, 102u8, 21u8, 75u8, 81u8, 218u8, 60u8,
                        ],
                    )
                }
                #[doc = " next block number where we can force a change."]                pub fn next_forced (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "NextForced",
                        vec![],
                        [
                            99u8, 43u8, 245u8, 201u8, 60u8, 9u8, 122u8, 99u8,
                            188u8, 29u8, 67u8, 6u8, 193u8, 133u8, 179u8, 67u8,
                            202u8, 208u8, 62u8, 179u8, 19u8, 169u8, 196u8,
                            119u8, 107u8, 75u8, 100u8, 3u8, 121u8, 18u8, 80u8,
                            156u8,
                        ],
                    )
                }
                #[doc = " `true` if we are currently stalled."]                pub fn stalled (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: core :: primitive :: u32 , :: core :: primitive :: u32 ,) > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "Stalled",
                        vec![],
                        [
                            219u8, 8u8, 37u8, 78u8, 150u8, 55u8, 0u8, 57u8,
                            201u8, 170u8, 186u8, 189u8, 56u8, 161u8, 44u8,
                            15u8, 53u8, 178u8, 224u8, 208u8, 231u8, 109u8,
                            14u8, 209u8, 57u8, 205u8, 237u8, 153u8, 231u8,
                            156u8, 24u8, 185u8,
                        ],
                    )
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]                pub fn current_set_id (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u64 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "CurrentSetId",
                        vec![],
                        [
                            129u8, 7u8, 62u8, 101u8, 199u8, 60u8, 56u8, 33u8,
                            54u8, 158u8, 20u8, 178u8, 244u8, 145u8, 189u8,
                            197u8, 157u8, 163u8, 116u8, 36u8, 105u8, 52u8,
                            149u8, 244u8, 108u8, 94u8, 109u8, 111u8, 244u8,
                            137u8, 7u8, 108u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]                pub fn set_id_session (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u64 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Grandpa" , "SetIdSession" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [91u8 , 175u8 , 145u8 , 127u8 , 242u8 , 81u8 , 13u8 , 231u8 , 110u8 , 11u8 , 166u8 , 169u8 , 103u8 , 146u8 , 123u8 , 133u8 , 157u8 , 15u8 , 33u8 , 234u8 , 108u8 , 13u8 , 88u8 , 115u8 , 254u8 , 9u8 , 145u8 , 199u8 , 102u8 , 47u8 , 53u8 , 134u8 ,])
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]                pub fn set_id_session_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "SetIdSession",
                        Vec::new(),
                        [
                            91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8,
                            231u8, 110u8, 11u8, 166u8, 169u8, 103u8, 146u8,
                            123u8, 133u8, 157u8, 15u8, 33u8, 234u8, 108u8,
                            13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8,
                            47u8, 53u8, 134u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Grandpa",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Transfer {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetBalance {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceTransfer {
                pub source: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct TransferKeepAlive {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct TransferAll {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub keep_alive: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceUnreserve {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub amount: ::core::primitive::u128,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                #[doc = "  types. See related functions below."]
                #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                #[doc = "  computation."]
                #[doc = ""]
                #[doc = "Related functions:"]
                #[doc = ""]
                #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                #[doc = "    that the transfer will not kill the origin account."]
                #[doc = "---------------------------------"]
                #[doc = "- Origin account is already in memory, so no DB operations for them."]
                #[doc = "# </weight>"]
                pub fn transfer(
                    &self,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Transfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer",
                        Transfer { dest, value },
                        [
                            255u8, 181u8, 144u8, 248u8, 64u8, 167u8, 5u8,
                            134u8, 208u8, 20u8, 223u8, 103u8, 235u8, 35u8,
                            66u8, 184u8, 27u8, 94u8, 176u8, 60u8, 233u8, 236u8,
                            145u8, 218u8, 44u8, 138u8, 240u8, 224u8, 16u8,
                            193u8, 220u8, 95u8,
                        ],
                    )
                }
                #[doc = "Set the balances of a given account."]
                #[doc = ""]
                #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                #[doc = "If the new free or reserved balance is below the existential deposit,"]
                #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn set_balance(
                    &self,
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<SetBalance> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "set_balance",
                        SetBalance {
                            who,
                            new_free,
                            new_reserved,
                        },
                        [
                            174u8, 34u8, 80u8, 252u8, 193u8, 51u8, 228u8,
                            236u8, 234u8, 16u8, 173u8, 214u8, 122u8, 21u8,
                            254u8, 7u8, 49u8, 176u8, 18u8, 128u8, 122u8, 68u8,
                            72u8, 181u8, 119u8, 90u8, 167u8, 46u8, 203u8,
                            220u8, 109u8, 110u8,
                        ],
                    )
                }
                #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                #[doc = "specified."]
                #[doc = "# <weight>"]
                #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                #[doc = "  assumed to be in the overlay."]
                #[doc = "# </weight>"]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceTransfer>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "force_transfer",
                        ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            56u8, 80u8, 186u8, 45u8, 134u8, 147u8, 200u8, 19u8,
                            53u8, 221u8, 213u8, 32u8, 13u8, 51u8, 130u8, 42u8,
                            244u8, 85u8, 50u8, 246u8, 189u8, 51u8, 93u8, 1u8,
                            108u8, 142u8, 112u8, 245u8, 104u8, 255u8, 15u8,
                            62u8,
                        ],
                    )
                }
                #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                #[doc = "origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer`] instead."]
                #[doc = ""]
                #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer_keep_alive",
                        TransferKeepAlive { dest, value },
                        [
                            202u8, 239u8, 204u8, 0u8, 52u8, 57u8, 158u8, 8u8,
                            252u8, 178u8, 91u8, 197u8, 238u8, 186u8, 205u8,
                            56u8, 217u8, 250u8, 21u8, 44u8, 239u8, 66u8, 79u8,
                            99u8, 25u8, 106u8, 70u8, 226u8, 50u8, 255u8, 176u8,
                            71u8,
                        ],
                    )
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true). # <weight>"]
                #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                #[doc = "  #</weight>"]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<TransferAll> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer_all",
                        TransferAll { dest, keep_alive },
                        [
                            118u8, 215u8, 198u8, 243u8, 4u8, 173u8, 108u8,
                            224u8, 113u8, 203u8, 149u8, 23u8, 130u8, 176u8,
                            53u8, 205u8, 112u8, 147u8, 88u8, 167u8, 197u8,
                            32u8, 104u8, 117u8, 201u8, 168u8, 144u8, 230u8,
                            120u8, 29u8, 122u8, 159u8,
                        ],
                    )
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceUnreserve>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "force_unreserve",
                        ForceUnreserve { who, amount },
                        [
                            39u8, 229u8, 111u8, 44u8, 147u8, 80u8, 7u8, 26u8,
                            185u8, 121u8, 149u8, 25u8, 151u8, 37u8, 124u8,
                            46u8, 108u8, 136u8, 167u8, 145u8, 103u8, 65u8,
                            33u8, 168u8, 36u8, 214u8, 126u8, 237u8, 180u8,
                            61u8, 108u8, 110u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::ext::sp_core::crypto::AccountId32,
                pub to: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated { pub from : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , pub to : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , pub amount : :: core :: primitive :: u128 , pub destination_status : runtime_types :: frame_support :: traits :: tokens :: misc :: BalanceStatus , }
            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]                pub fn total_issuance (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8,
                            164u8, 115u8, 51u8, 156u8, 156u8, 206u8, 241u8,
                            187u8, 44u8, 84u8, 25u8, 164u8, 235u8, 20u8, 86u8,
                            242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8,
                            51u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]                pub fn inactive_issuance (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "InactiveIssuance",
                        vec![],
                        [
                            74u8, 203u8, 111u8, 142u8, 225u8, 104u8, 173u8,
                            51u8, 226u8, 12u8, 85u8, 135u8, 41u8, 206u8, 177u8,
                            238u8, 94u8, 246u8, 184u8, 250u8, 140u8, 213u8,
                            91u8, 118u8, 163u8, 111u8, 211u8, 46u8, 204u8,
                            160u8, 154u8, 21u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]                pub fn account (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_balances :: AccountData < :: core :: primitive :: u128 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Balances" , "Account" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [246u8 , 154u8 , 253u8 , 71u8 , 192u8 , 192u8 , 192u8 , 236u8 , 128u8 , 80u8 , 40u8 , 252u8 , 201u8 , 43u8 , 3u8 , 131u8 , 19u8 , 49u8 , 141u8 , 240u8 , 172u8 , 217u8 , 215u8 , 109u8 , 87u8 , 135u8 , 248u8 , 57u8 , 98u8 , 185u8 , 22u8 , 4u8 ,])
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]                pub fn account_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_balances :: AccountData < :: core :: primitive :: u128 > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Account",
                        Vec::new(),
                        [
                            246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8,
                            236u8, 128u8, 80u8, 40u8, 252u8, 201u8, 43u8, 3u8,
                            131u8, 19u8, 49u8, 141u8, 240u8, 172u8, 217u8,
                            215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8,
                            185u8, 22u8, 4u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]                pub fn locks (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Balances" , "Locks" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [216u8 , 253u8 , 87u8 , 73u8 , 24u8 , 218u8 , 35u8 , 0u8 , 244u8 , 134u8 , 195u8 , 58u8 , 255u8 , 64u8 , 153u8 , 212u8 , 210u8 , 232u8 , 4u8 , 122u8 , 90u8 , 212u8 , 136u8 , 14u8 , 127u8 , 232u8 , 8u8 , 192u8 , 40u8 , 233u8 , 18u8 , 250u8 ,])
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]                pub fn locks_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_balances :: BalanceLock < :: core :: primitive :: u128 > > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Locks",
                        Vec::new(),
                        [
                            216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8,
                            244u8, 134u8, 195u8, 58u8, 255u8, 64u8, 153u8,
                            212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
                            136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8,
                            18u8, 250u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]                pub fn reserves (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < runtime_types :: pallet_balances :: ReserveData < [:: core :: primitive :: u8 ; 8usize] , :: core :: primitive :: u128 > > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Balances" , "Reserves" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [17u8 , 32u8 , 191u8 , 46u8 , 76u8 , 220u8 , 101u8 , 100u8 , 42u8 , 250u8 , 128u8 , 167u8 , 117u8 , 44u8 , 85u8 , 96u8 , 105u8 , 216u8 , 16u8 , 147u8 , 74u8 , 55u8 , 183u8 , 94u8 , 160u8 , 177u8 , 26u8 , 187u8 , 71u8 , 197u8 , 187u8 , 163u8 ,])
                }
                #[doc = " Named reserves on some account balances."]                pub fn reserves_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < runtime_types :: pallet_balances :: ReserveData < [:: core :: primitive :: u8 ; 8usize] , :: core :: primitive :: u128 > > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Reserves",
                        Vec::new(),
                        [
                            17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8,
                            42u8, 250u8, 128u8, 167u8, 117u8, 44u8, 85u8, 96u8,
                            105u8, 216u8, 16u8, 147u8, 74u8, 55u8, 183u8, 94u8,
                            160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8,
                            163u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open."]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8,
                            105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8,
                            160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
                            71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8,
                            148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod dkg {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetSignatureThreshold {
                pub new_threshold: ::core::primitive::u16,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetKeygenThreshold {
                pub new_threshold: ::core::primitive::u16,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetRefreshDelay {
                pub new_delay: ::core::primitive::u8,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SubmitPublicKey {
                pub keys_and_signatures:
                    runtime_types::dkg_runtime_primitives::AggregatedPublicKeys,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SubmitNextPublicKey {
                pub keys_and_signatures:
                    runtime_types::dkg_runtime_primitives::AggregatedPublicKeys,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SubmitPublicKeySignature { pub signature_proposal : runtime_types :: dkg_runtime_primitives :: proposal :: RefreshProposalSigned , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SubmitMisbehaviourReports { pub reports : runtime_types :: dkg_runtime_primitives :: AggregatedMisbehaviourReports < runtime_types :: dkg_runtime_primitives :: crypto :: Public > , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Unjail;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceUnjailKeygen {
                pub authority:
                    runtime_types::dkg_runtime_primitives::crypto::Public,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceUnjailSigning {
                pub authority:
                    runtime_types::dkg_runtime_primitives::crypto::Public,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceChangeAuthorities;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct TriggerEmergencyKeygen;
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the pending signature threshold for the session following the next session."]
                #[doc = ""]
                #[doc = "We cannot assume that the next DKG has not already completed keygen."]
                #[doc = "After all, if we are in a new session the next DKG may have already completed."]
                #[doc = "Therefore, when we update the thresholds we are updating a threshold"]
                #[doc = "that will become the next threshold after the next session update."]
                #[doc = ""]
                #[doc = "* `origin` - The account origin."]
                #[doc = "* `new_threshold` - The new signature threshold for the DKG."]
                pub fn set_signature_threshold(
                    &self,
                    new_threshold: ::core::primitive::u16,
                ) -> ::subxt::tx::StaticTxPayload<SetSignatureThreshold>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "set_signature_threshold",
                        SetSignatureThreshold { new_threshold },
                        [
                            160u8, 195u8, 237u8, 69u8, 171u8, 13u8, 28u8,
                            110u8, 245u8, 159u8, 172u8, 115u8, 125u8, 245u8,
                            207u8, 23u8, 227u8, 200u8, 227u8, 120u8, 183u8,
                            185u8, 240u8, 153u8, 127u8, 14u8, 106u8, 85u8, 4u8,
                            55u8, 227u8, 65u8,
                        ],
                    )
                }
                #[doc = "Set the pending keygen threshold for the session following the next session."]
                #[doc = ""]
                #[doc = "We cannot assume that the next DKG has not already completed keygen."]
                #[doc = "After all, if we are in a new session the next DKG may have already completed."]
                #[doc = "Therefore, when we update the thresholds we are updating a threshold"]
                #[doc = "that will become the next threshold after the next session update."]
                #[doc = ""]
                #[doc = "* `origin` - The account origin."]
                #[doc = "* `new_threshold` - The new keygen threshold for the DKG."]
                pub fn set_keygen_threshold(
                    &self,
                    new_threshold: ::core::primitive::u16,
                ) -> ::subxt::tx::StaticTxPayload<SetKeygenThreshold>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "set_keygen_threshold",
                        SetKeygenThreshold { new_threshold },
                        [
                            181u8, 23u8, 76u8, 117u8, 56u8, 235u8, 59u8, 254u8,
                            242u8, 114u8, 147u8, 247u8, 248u8, 168u8, 235u8,
                            50u8, 128u8, 167u8, 99u8, 110u8, 2u8, 15u8, 26u8,
                            110u8, 216u8, 7u8, 189u8, 46u8, 73u8, 229u8, 24u8,
                            9u8,
                        ],
                    )
                }
                #[doc = "Sets the delay when a unsigned `RefreshProposal` will be added to the unsigned"]
                #[doc = "proposal queue."]
                #[doc = ""]
                #[doc = "* `origin` - The account origin."]
                #[doc = "* `new_delay` - The percentage of elapsed session duration to wait before adding an"]
                #[doc = "  unsigned refresh proposal to the unsigned proposal queue."]
                pub fn set_refresh_delay(
                    &self,
                    new_delay: ::core::primitive::u8,
                ) -> ::subxt::tx::StaticTxPayload<SetRefreshDelay>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "set_refresh_delay",
                        SetRefreshDelay { new_delay },
                        [
                            16u8, 74u8, 191u8, 229u8, 37u8, 51u8, 217u8, 83u8,
                            211u8, 119u8, 177u8, 127u8, 207u8, 133u8, 218u8,
                            245u8, 162u8, 21u8, 93u8, 42u8, 250u8, 63u8, 218u8,
                            33u8, 217u8, 215u8, 61u8, 122u8, 28u8, 53u8, 157u8,
                            108u8,
                        ],
                    )
                }
                #[doc = "Submits and stores the active public key for the genesis session into the on-chain"]
                #[doc = "storage. This is primarily used to separate the genesis public key submission from"]
                #[doc = "non-genesis rounds."]
                #[doc = ""]
                #[doc = "Can only be submitted by the current authorities. It is also required that a"]
                #[doc = "`SignatureThreshold` of submissions is reached in order to successfully"]
                #[doc = "store the public key on-chain."]
                #[doc = ""]
                #[doc = "* `origin` - The account origin."]
                #[doc = "* `keys_and_signatures` - The aggregated public keys and signatures for possible current"]
                #[doc = "  DKG public keys."]
                pub fn submit_public_key(
                    &self,
                    keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys,
                ) -> ::subxt::tx::StaticTxPayload<SubmitPublicKey>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "submit_public_key",
                        SubmitPublicKey {
                            keys_and_signatures,
                        },
                        [
                            39u8, 55u8, 155u8, 151u8, 44u8, 91u8, 66u8, 109u8,
                            115u8, 132u8, 68u8, 187u8, 202u8, 41u8, 234u8,
                            114u8, 79u8, 78u8, 65u8, 245u8, 192u8, 38u8, 172u8,
                            71u8, 54u8, 170u8, 53u8, 122u8, 58u8, 135u8, 234u8,
                            49u8,
                        ],
                    )
                }
                #[doc = "Submits and stores the next public key for the next session into the on-chain storage."]
                #[doc = ""]
                #[doc = "Can only be submitted by the next authorities. It is also required that a"]
                #[doc = "`NextSignatureThreshold` of submissions is reached in order to successfully"]
                #[doc = "store the public key on-chain."]
                #[doc = ""]
                #[doc = "* `origin` - The account origin."]
                #[doc = "* `keys_and_signatures` - The aggregated public keys and signatures for possible next"]
                #[doc = "  DKG public keys."]
                pub fn submit_next_public_key(
                    &self,
                    keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys,
                ) -> ::subxt::tx::StaticTxPayload<SubmitNextPublicKey>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "submit_next_public_key",
                        SubmitNextPublicKey {
                            keys_and_signatures,
                        },
                        [
                            248u8, 166u8, 238u8, 156u8, 241u8, 216u8, 68u8,
                            78u8, 95u8, 106u8, 167u8, 238u8, 113u8, 117u8,
                            109u8, 169u8, 253u8, 75u8, 154u8, 30u8, 170u8,
                            94u8, 187u8, 192u8, 193u8, 34u8, 50u8, 66u8, 32u8,
                            68u8, 81u8, 205u8,
                        ],
                    )
                }
                #[doc = "Submits the public key signature for the key refresh/rotation process."]
                #[doc = ""]
                #[doc = "The signature is the signature of the next public key `RefreshProposal`, signed by the"]
                #[doc = "current DKG. It is stored on-chain only if it verifies successfully against the current"]
                #[doc = "DKG's public key. Successful storage of this public key signature also removes"]
                #[doc = "the unsigned `RefreshProposal` from the unsigned queue."]
                #[doc = ""]
                #[doc = "For manual refreshes, after the signature is submitted and stored on-chain,"]
                #[doc = "the keys are immediately refreshed and the authority set is immediately rotated"]
                #[doc = "and incremented."]
                #[doc = ""]
                #[doc = "* `origin` - The account origin."]
                #[doc = "* `signature_proposal` - The signed refresh proposal containing the public key signature"]
                #[doc = "  and nonce."]
                pub fn submit_public_key_signature(
                    &self,
                    signature_proposal : runtime_types :: dkg_runtime_primitives :: proposal :: RefreshProposalSigned,
                ) -> ::subxt::tx::StaticTxPayload<SubmitPublicKeySignature>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "submit_public_key_signature",
                        SubmitPublicKeySignature { signature_proposal },
                        [
                            74u8, 197u8, 64u8, 71u8, 121u8, 35u8, 252u8, 67u8,
                            136u8, 25u8, 110u8, 230u8, 219u8, 44u8, 146u8,
                            203u8, 151u8, 165u8, 62u8, 83u8, 236u8, 75u8,
                            220u8, 112u8, 160u8, 192u8, 220u8, 0u8, 203u8,
                            34u8, 195u8, 76u8,
                        ],
                    )
                }
                #[doc = "Submits misbehaviour reports on chain. Signatures of the offending authority are"]
                #[doc = "verified against the current or next authorities depending on the type of misbehaviour."]
                #[doc = "- Keygen: Verifies against the next authorities, since they are doing keygen."]
                #[doc = "- Signing: Verifies against the current authorities, since they are doing signing."]
                #[doc = ""]
                #[doc = "Verifies the reports against the respective thresholds and if enough reports are met"]
                #[doc = "begins to jail and decrease the reputation of the offending authority."]
                #[doc = ""]
                #[doc = "The misbehaviour reputation update is:"]
                #[doc = "\tAUTHORITY_REPUTATION = DECAY_PERCENTAGE * AUTHORITY_REPUTATION"]
                #[doc = ""]
                #[doc = "If there are not enough unjailed keygen authorities to perform a keygen after the next"]
                #[doc = "session, then we deduct the pending keygen threshold (and pending signing threshold)"]
                #[doc = "accordingly."]
                #[doc = ""]
                #[doc = "* `origin` - The account origin."]
                #[doc = "* `reports` - The aggregated misbehaviour reports containing signatures of an offending"]
                #[doc = "  authority"]
                pub fn submit_misbehaviour_reports(
                    &self,
                    reports : runtime_types :: dkg_runtime_primitives :: AggregatedMisbehaviourReports < runtime_types :: dkg_runtime_primitives :: crypto :: Public >,
                ) -> ::subxt::tx::StaticTxPayload<SubmitMisbehaviourReports>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "submit_misbehaviour_reports",
                        SubmitMisbehaviourReports { reports },
                        [
                            209u8, 185u8, 221u8, 91u8, 208u8, 171u8, 23u8,
                            75u8, 150u8, 244u8, 41u8, 196u8, 104u8, 239u8,
                            164u8, 224u8, 251u8, 233u8, 15u8, 100u8, 26u8,
                            205u8, 213u8, 70u8, 163u8, 27u8, 245u8, 142u8,
                            120u8, 250u8, 84u8, 215u8,
                        ],
                    )
                }
                #[doc = "Attempts to remove an authority from all possible jails (keygen & signing)."]
                #[doc = "This can only be called by the controller of the authority in jail. The"]
                #[doc = "origin must map directly to the authority in jail."]
                #[doc = ""]
                #[doc = "The authority's jail sentence for either keygen or signing must be elapsed"]
                #[doc = "for the authority to be removed from the jail."]
                #[doc = ""]
                #[doc = "* `origin` - The account origin."]
                pub fn unjail(&self) -> ::subxt::tx::StaticTxPayload<Unjail> {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "unjail",
                        Unjail {},
                        [
                            209u8, 5u8, 90u8, 85u8, 167u8, 92u8, 85u8, 132u8,
                            48u8, 27u8, 89u8, 149u8, 242u8, 209u8, 97u8, 87u8,
                            13u8, 58u8, 124u8, 41u8, 155u8, 103u8, 62u8, 115u8,
                            58u8, 143u8, 233u8, 105u8, 18u8, 198u8, 173u8,
                            77u8,
                        ],
                    )
                }
                #[doc = "Force removes an authority from keygen jail."]
                #[doc = ""]
                #[doc = "Can only be called by DKG"]
                #[doc = "* `origin` - The account origin."]
                #[doc = "* `authority` - The authority to be removed from the keygen jail."]
                pub fn force_unjail_keygen(
                    &self,
                    authority : runtime_types :: dkg_runtime_primitives :: crypto :: Public,
                ) -> ::subxt::tx::StaticTxPayload<ForceUnjailKeygen>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "force_unjail_keygen",
                        ForceUnjailKeygen { authority },
                        [
                            7u8, 2u8, 72u8, 189u8, 190u8, 208u8, 55u8, 117u8,
                            205u8, 174u8, 223u8, 90u8, 12u8, 136u8, 64u8,
                            212u8, 57u8, 31u8, 227u8, 103u8, 72u8, 157u8,
                            156u8, 101u8, 66u8, 5u8, 149u8, 175u8, 3u8, 98u8,
                            60u8, 44u8,
                        ],
                    )
                }
                #[doc = "Force removes an authority from signing jail."]
                #[doc = ""]
                #[doc = "Can only be called by the root origin."]
                #[doc = ""]
                #[doc = "* `origin` - The account origin."]
                #[doc = "* `authority` - The authority to be removed from the signing jail."]
                pub fn force_unjail_signing(
                    &self,
                    authority : runtime_types :: dkg_runtime_primitives :: crypto :: Public,
                ) -> ::subxt::tx::StaticTxPayload<ForceUnjailSigning>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "force_unjail_signing",
                        ForceUnjailSigning { authority },
                        [
                            195u8, 160u8, 19u8, 141u8, 200u8, 170u8, 41u8,
                            189u8, 18u8, 98u8, 112u8, 34u8, 55u8, 46u8, 101u8,
                            216u8, 177u8, 135u8, 70u8, 24u8, 219u8, 153u8,
                            167u8, 130u8, 110u8, 187u8, 146u8, 103u8, 3u8,
                            253u8, 5u8, 185u8,
                        ],
                    )
                }
                #[doc = "Forcefully rotate the DKG"]
                #[doc = ""]
                #[doc = "This forces the next authorities into the current authority spot and"]
                #[doc = "automatically increments the authority ID. It uses `change_authorities`"]
                #[doc = "to execute the rotation forcefully."]
                pub fn force_change_authorities(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<ForceChangeAuthorities>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "force_change_authorities",
                        ForceChangeAuthorities {},
                        [
                            98u8, 226u8, 238u8, 39u8, 98u8, 79u8, 85u8, 160u8,
                            82u8, 3u8, 34u8, 195u8, 220u8, 178u8, 25u8, 110u8,
                            12u8, 72u8, 165u8, 126u8, 207u8, 173u8, 210u8,
                            74u8, 54u8, 106u8, 255u8, 243u8, 240u8, 22u8,
                            161u8, 255u8,
                        ],
                    )
                }
                #[doc = "Triggers an Emergency Keygen Porotocol."]
                #[doc = ""]
                #[doc = "The keygen protocol will then be executed and the result will be stored in the off chain"]
                #[doc = "storage, which will be picked up by the on chain worker and stored on chain."]
                #[doc = ""]
                #[doc = "Note that, this will clear the next public key and its signature, if any."]
                pub fn trigger_emergency_keygen(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<TriggerEmergencyKeygen>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKG",
                        "trigger_emergency_keygen",
                        TriggerEmergencyKeygen {},
                        [
                            169u8, 123u8, 131u8, 42u8, 202u8, 219u8, 162u8,
                            1u8, 134u8, 223u8, 197u8, 245u8, 64u8, 51u8, 46u8,
                            231u8, 155u8, 25u8, 6u8, 253u8, 170u8, 251u8, 15u8,
                            156u8, 76u8, 127u8, 142u8, 218u8, 142u8, 65u8,
                            197u8, 160u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_dkg_metadata::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Current public key submitted"]
            pub struct PublicKeySubmitted {
                pub compressed_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                pub uncompressed_pub_key:
                    ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::events::StaticEvent for PublicKeySubmitted {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "PublicKeySubmitted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Next public key submitted"]
            pub struct NextPublicKeySubmitted {
                pub compressed_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                pub uncompressed_pub_key:
                    ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::events::StaticEvent for NextPublicKeySubmitted {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "NextPublicKeySubmitted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Next public key signature submitted"]
            pub struct NextPublicKeySignatureSubmitted {
                pub pub_key_sig: ::std::vec::Vec<::core::primitive::u8>,
                pub compressed_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                pub uncompressed_pub_key:
                    ::std::vec::Vec<::core::primitive::u8>,
                pub nonce: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for NextPublicKeySignatureSubmitted {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "NextPublicKeySignatureSubmitted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Current Public Key Changed."]
            pub struct PublicKeyChanged {
                pub compressed_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                pub uncompressed_pub_key:
                    ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::events::StaticEvent for PublicKeyChanged {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "PublicKeyChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Current Public Key Signature Changed."]
            pub struct PublicKeySignatureChanged {
                pub pub_key_sig: ::std::vec::Vec<::core::primitive::u8>,
                pub compressed_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                pub uncompressed_pub_key:
                    ::std::vec::Vec<::core::primitive::u8>,
                pub nonce: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for PublicKeySignatureChanged {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "PublicKeySignatureChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Misbehaviour reports submitted"]
            pub struct MisbehaviourReportsSubmitted {
                pub misbehaviour_type:
                    runtime_types::dkg_runtime_primitives::MisbehaviourType,
                pub reporters: ::std::vec::Vec<
                    runtime_types::dkg_runtime_primitives::crypto::Public,
                >,
            }
            impl ::subxt::events::StaticEvent for MisbehaviourReportsSubmitted {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "MisbehaviourReportsSubmitted";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Refresh DKG Keys Finished (forcefully)."]
            pub struct RefreshKeysFinished {
                pub next_authority_set_id: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for RefreshKeysFinished {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "RefreshKeysFinished";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "NextKeygenThreshold updated"]
            pub struct NextKeygenThresholdUpdated {
                pub next_keygen_threshold: ::core::primitive::u16,
            }
            impl ::subxt::events::StaticEvent for NextKeygenThresholdUpdated {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "NextKeygenThresholdUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "NextSignatureThreshold updated"]
            pub struct NextSignatureThresholdUpdated {
                pub next_signature_threshold: ::core::primitive::u16,
            }
            impl ::subxt::events::StaticEvent for NextSignatureThresholdUpdated {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "NextSignatureThresholdUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An Emergency Keygen Protocol was triggered."]
            pub struct EmergencyKeygenTriggered;
            impl ::subxt::events::StaticEvent for EmergencyKeygenTriggered {
                const PALLET: &'static str = "DKG";
                const EVENT: &'static str = "EmergencyKeygenTriggered";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Public key Signatures for past sessions"]                pub fn used_signatures (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: std :: vec :: Vec < :: core :: primitive :: u8 > > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "UsedSignatures",
                        vec![],
                        [
                            17u8, 166u8, 71u8, 200u8, 53u8, 132u8, 79u8, 208u8,
                            187u8, 231u8, 68u8, 227u8, 163u8, 125u8, 235u8,
                            145u8, 171u8, 160u8, 82u8, 237u8, 170u8, 48u8,
                            173u8, 104u8, 13u8, 113u8, 12u8, 56u8, 47u8, 42u8,
                            250u8, 70u8,
                        ],
                    )
                }
                #[doc = " Nonce value for next refresh proposal"]                pub fn refresh_nonce (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "RefreshNonce",
                        vec![],
                        [
                            184u8, 107u8, 53u8, 61u8, 92u8, 121u8, 77u8, 93u8,
                            141u8, 192u8, 238u8, 92u8, 15u8, 155u8, 1u8, 153u8,
                            55u8, 64u8, 83u8, 144u8, 127u8, 250u8, 207u8, 14u8,
                            62u8, 137u8, 151u8, 230u8, 86u8, 236u8, 27u8,
                            175u8,
                        ],
                    )
                }
                #[doc = " Session progress required to kickstart refresh process"]                pub fn refresh_delay (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_arithmetic :: per_things :: Permill > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "RefreshDelay",
                        vec![],
                        [
                            79u8, 178u8, 57u8, 227u8, 1u8, 170u8, 0u8, 55u8,
                            231u8, 155u8, 114u8, 9u8, 197u8, 101u8, 112u8,
                            176u8, 169u8, 138u8, 67u8, 127u8, 127u8, 59u8,
                            56u8, 99u8, 113u8, 242u8, 15u8, 43u8, 254u8, 79u8,
                            9u8, 161u8,
                        ],
                    )
                }
                #[doc = " Defines the block when next unsigned transaction will be accepted."]
                #[doc = ""]
                #[doc = " To prevent spam of unsigned (and unpayed!) transactions on the network,"]
                #[doc = " we only allow one transaction every `T::UnsignedInterval` blocks."]
                #[doc = " This storage entry defines when new transaction is going to be accepted."]                pub fn next_unsigned_at (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "NextUnsignedAt",
                        vec![],
                        [
                            175u8, 176u8, 254u8, 20u8, 27u8, 31u8, 70u8, 46u8,
                            201u8, 118u8, 22u8, 200u8, 95u8, 116u8, 223u8,
                            63u8, 191u8, 85u8, 55u8, 21u8, 70u8, 24u8, 225u8,
                            203u8, 66u8, 213u8, 94u8, 229u8, 234u8, 223u8,
                            255u8, 28u8,
                        ],
                    )
                }
                #[doc = " Check if there is a refresh in progress."]                pub fn refresh_in_progress (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: bool > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "RefreshInProgress",
                        vec![],
                        [
                            150u8, 114u8, 14u8, 254u8, 132u8, 254u8, 10u8,
                            89u8, 109u8, 4u8, 182u8, 128u8, 114u8, 15u8, 82u8,
                            35u8, 88u8, 86u8, 32u8, 82u8, 83u8, 175u8, 123u8,
                            98u8, 120u8, 180u8, 167u8, 185u8, 57u8, 221u8,
                            12u8, 62u8,
                        ],
                    )
                }
                #[doc = " Should we execute emergency keygen protocol."]                pub fn should_execute_emergency_keygen (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: bool > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "ShouldExecuteEmergencyKeygen",
                        vec![],
                        [
                            144u8, 160u8, 237u8, 194u8, 180u8, 29u8, 245u8,
                            35u8, 182u8, 75u8, 82u8, 239u8, 109u8, 249u8, 92u8,
                            61u8, 49u8, 230u8, 77u8, 15u8, 13u8, 143u8, 5u8,
                            121u8, 32u8, 138u8, 60u8, 10u8, 51u8, 78u8, 25u8,
                            155u8,
                        ],
                    )
                }
                #[doc = " Holds public key for next session"]                pub fn next_dkg_public_key (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: core :: primitive :: u64 , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "NextDKGPublicKey",
                        vec![],
                        [
                            147u8, 213u8, 171u8, 9u8, 247u8, 218u8, 74u8, 10u8,
                            66u8, 24u8, 52u8, 251u8, 125u8, 28u8, 54u8, 12u8,
                            243u8, 205u8, 242u8, 48u8, 179u8, 211u8, 178u8,
                            219u8, 88u8, 247u8, 51u8, 52u8, 27u8, 170u8, 212u8,
                            181u8,
                        ],
                    )
                }
                #[doc = " Signature of the DKG public key for the next session"]                pub fn next_public_key_signature (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "NextPublicKeySignature",
                        vec![],
                        [
                            128u8, 96u8, 220u8, 158u8, 111u8, 181u8, 68u8,
                            32u8, 33u8, 122u8, 61u8, 99u8, 58u8, 84u8, 110u8,
                            13u8, 8u8, 179u8, 11u8, 80u8, 5u8, 90u8, 194u8,
                            230u8, 3u8, 124u8, 27u8, 157u8, 73u8, 143u8, 159u8,
                            98u8,
                        ],
                    )
                }
                #[doc = " Holds active public key for ongoing session"]                pub fn dkg_public_key (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: core :: primitive :: u64 , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "DKGPublicKey",
                        vec![],
                        [
                            134u8, 73u8, 251u8, 94u8, 50u8, 143u8, 130u8, 71u8,
                            180u8, 91u8, 29u8, 20u8, 105u8, 138u8, 225u8,
                            205u8, 180u8, 94u8, 203u8, 106u8, 109u8, 101u8,
                            114u8, 3u8, 182u8, 236u8, 231u8, 124u8, 198u8,
                            106u8, 102u8, 242u8,
                        ],
                    )
                }
                #[doc = " Signature of the current DKG public key"]                pub fn dkg_public_key_signature (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "DKGPublicKeySignature",
                        vec![],
                        [
                            184u8, 31u8, 185u8, 8u8, 102u8, 120u8, 175u8,
                            105u8, 106u8, 6u8, 14u8, 197u8, 211u8, 49u8, 192u8,
                            201u8, 46u8, 42u8, 208u8, 63u8, 234u8, 131u8,
                            207u8, 131u8, 21u8, 119u8, 39u8, 105u8, 27u8,
                            174u8, 173u8, 29u8,
                        ],
                    )
                }
                #[doc = " Holds public key for immediate past session"]                pub fn previous_public_key (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: core :: primitive :: u64 , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "PreviousPublicKey",
                        vec![],
                        [
                            254u8, 74u8, 168u8, 47u8, 143u8, 21u8, 245u8,
                            148u8, 75u8, 45u8, 54u8, 49u8, 22u8, 239u8, 129u8,
                            250u8, 127u8, 70u8, 231u8, 25u8, 215u8, 229u8,
                            130u8, 32u8, 137u8, 160u8, 108u8, 183u8, 65u8,
                            34u8, 241u8, 245u8,
                        ],
                    )
                }
                #[doc = " Tracks current proposer set"]                pub fn historical_rounds (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u64 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_dkg_metadata :: types :: RoundMetadata > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKG" , "HistoricalRounds" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_256)] , [90u8 , 80u8 , 198u8 , 169u8 , 195u8 , 242u8 , 59u8 , 14u8 , 1u8 , 166u8 , 88u8 , 136u8 , 37u8 , 51u8 , 49u8 , 82u8 , 37u8 , 211u8 , 235u8 , 163u8 , 50u8 , 60u8 , 87u8 , 108u8 , 21u8 , 77u8 , 21u8 , 177u8 , 245u8 , 102u8 , 73u8 , 242u8 ,])
                }
                #[doc = " Tracks current proposer set"]                pub fn historical_rounds_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_dkg_metadata :: types :: RoundMetadata > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "HistoricalRounds",
                        Vec::new(),
                        [
                            90u8, 80u8, 198u8, 169u8, 195u8, 242u8, 59u8, 14u8,
                            1u8, 166u8, 88u8, 136u8, 37u8, 51u8, 49u8, 82u8,
                            37u8, 211u8, 235u8, 163u8, 50u8, 60u8, 87u8, 108u8,
                            21u8, 77u8, 21u8, 177u8, 245u8, 102u8, 73u8, 242u8,
                        ],
                    )
                }
                #[doc = " The current signature threshold (i.e. the `t` in t-of-n)"]                pub fn signature_threshold (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u16 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "SignatureThreshold",
                        vec![],
                        [
                            228u8, 213u8, 121u8, 182u8, 49u8, 44u8, 159u8,
                            113u8, 209u8, 234u8, 107u8, 232u8, 192u8, 211u8,
                            144u8, 183u8, 170u8, 37u8, 236u8, 48u8, 177u8, 7u8,
                            62u8, 63u8, 39u8, 134u8, 158u8, 72u8, 52u8, 179u8,
                            184u8, 217u8,
                        ],
                    )
                }
                #[doc = " The current signature threshold (i.e. the `n` in t-of-n)"]                pub fn keygen_threshold (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u16 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "KeygenThreshold",
                        vec![],
                        [
                            52u8, 253u8, 133u8, 19u8, 89u8, 89u8, 8u8, 246u8,
                            87u8, 16u8, 72u8, 213u8, 230u8, 168u8, 223u8, 38u8,
                            33u8, 83u8, 79u8, 28u8, 2u8, 92u8, 141u8, 197u8,
                            73u8, 190u8, 6u8, 177u8, 240u8, 245u8, 119u8, 70u8,
                        ],
                    )
                }
                #[doc = " The current signature threshold (i.e. the `t` in t-of-n)"]                pub fn next_signature_threshold (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u16 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "NextSignatureThreshold",
                        vec![],
                        [
                            83u8, 91u8, 234u8, 198u8, 71u8, 140u8, 138u8,
                            136u8, 26u8, 244u8, 93u8, 37u8, 141u8, 37u8, 91u8,
                            236u8, 135u8, 137u8, 86u8, 35u8, 240u8, 136u8,
                            144u8, 203u8, 230u8, 163u8, 66u8, 121u8, 18u8,
                            128u8, 102u8, 124u8,
                        ],
                    )
                }
                #[doc = " The current signature threshold (i.e. the `n` in t-of-n)"]                pub fn next_keygen_threshold (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u16 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "NextKeygenThreshold",
                        vec![],
                        [
                            244u8, 125u8, 22u8, 245u8, 44u8, 192u8, 133u8,
                            170u8, 115u8, 173u8, 56u8, 200u8, 83u8, 192u8,
                            65u8, 213u8, 71u8, 28u8, 15u8, 200u8, 47u8, 103u8,
                            215u8, 179u8, 6u8, 95u8, 214u8, 89u8, 223u8, 133u8,
                            161u8, 191u8,
                        ],
                    )
                }
                #[doc = " The pending signature threshold (i.e. the `t` in t-of-n)"]                pub fn pending_signature_threshold (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u16 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "PendingSignatureThreshold",
                        vec![],
                        [
                            69u8, 20u8, 129u8, 76u8, 67u8, 68u8, 122u8, 151u8,
                            39u8, 116u8, 35u8, 34u8, 96u8, 168u8, 39u8, 43u8,
                            64u8, 185u8, 126u8, 145u8, 247u8, 150u8, 96u8,
                            125u8, 109u8, 208u8, 254u8, 121u8, 227u8, 235u8,
                            108u8, 169u8,
                        ],
                    )
                }
                #[doc = " The pending signature threshold (i.e. the `n` in t-of-n)"]                pub fn pending_keygen_threshold (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u16 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "PendingKeygenThreshold",
                        vec![],
                        [
                            80u8, 94u8, 41u8, 244u8, 115u8, 174u8, 75u8, 71u8,
                            225u8, 122u8, 125u8, 141u8, 81u8, 69u8, 51u8,
                            200u8, 129u8, 143u8, 14u8, 106u8, 228u8, 177u8,
                            196u8, 167u8, 18u8, 70u8, 31u8, 137u8, 8u8, 233u8,
                            249u8, 202u8,
                        ],
                    )
                }
                #[doc = " The current authorities set"]                pub fn authorities (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < runtime_types :: dkg_runtime_primitives :: crypto :: Public > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "Authorities",
                        vec![],
                        [
                            220u8, 61u8, 168u8, 252u8, 28u8, 159u8, 44u8, 11u8,
                            58u8, 48u8, 39u8, 136u8, 156u8, 90u8, 195u8, 166u8,
                            244u8, 243u8, 110u8, 146u8, 168u8, 44u8, 4u8, 19u8,
                            203u8, 14u8, 209u8, 11u8, 162u8, 87u8, 101u8, 43u8,
                        ],
                    )
                }
                #[doc = " The current authority set id"]                pub fn authority_set_id (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u64 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "AuthoritySetId",
                        vec![],
                        [
                            97u8, 57u8, 86u8, 112u8, 28u8, 206u8, 59u8, 216u8,
                            109u8, 216u8, 119u8, 48u8, 31u8, 112u8, 189u8,
                            19u8, 234u8, 38u8, 14u8, 212u8, 191u8, 203u8, 72u8,
                            164u8, 131u8, 57u8, 77u8, 192u8, 182u8, 168u8,
                            185u8, 114u8,
                        ],
                    )
                }
                #[doc = " The next authority set id"]                pub fn next_authority_set_id (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u64 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "NextAuthoritySetId",
                        vec![],
                        [
                            27u8, 226u8, 90u8, 171u8, 61u8, 158u8, 36u8, 48u8,
                            88u8, 240u8, 189u8, 234u8, 176u8, 40u8, 78u8,
                            239u8, 201u8, 189u8, 111u8, 160u8, 5u8, 232u8,
                            196u8, 228u8, 19u8, 238u8, 185u8, 98u8, 73u8,
                            207u8, 135u8, 20u8,
                        ],
                    )
                }
                #[doc = " Authorities set scheduled to be used with the next session"]                pub fn next_authorities (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < runtime_types :: dkg_runtime_primitives :: crypto :: Public > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "NextAuthorities",
                        vec![],
                        [
                            40u8, 244u8, 137u8, 129u8, 46u8, 26u8, 80u8, 125u8,
                            170u8, 230u8, 63u8, 72u8, 127u8, 251u8, 240u8,
                            247u8, 211u8, 221u8, 6u8, 147u8, 72u8, 6u8, 1u8,
                            190u8, 6u8, 226u8, 96u8, 81u8, 88u8, 68u8, 218u8,
                            130u8,
                        ],
                    )
                }
                #[doc = " Accounts for the current authorities"]                pub fn current_authorities_accounts (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "CurrentAuthoritiesAccounts",
                        vec![],
                        [
                            165u8, 136u8, 152u8, 229u8, 185u8, 164u8, 120u8,
                            192u8, 95u8, 134u8, 168u8, 183u8, 153u8, 117u8,
                            103u8, 234u8, 47u8, 128u8, 234u8, 17u8, 255u8,
                            191u8, 65u8, 109u8, 216u8, 208u8, 40u8, 41u8,
                            161u8, 144u8, 79u8, 182u8,
                        ],
                    )
                }
                #[doc = " Authority account ids scheduled for the next session"]                pub fn next_authorities_accounts (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "NextAuthoritiesAccounts",
                        vec![],
                        [
                            39u8, 14u8, 0u8, 48u8, 106u8, 89u8, 188u8, 174u8,
                            231u8, 104u8, 185u8, 177u8, 141u8, 78u8, 143u8,
                            37u8, 87u8, 227u8, 14u8, 46u8, 113u8, 149u8, 187u8,
                            147u8, 121u8, 47u8, 79u8, 214u8, 14u8, 130u8,
                            170u8, 221u8,
                        ],
                    )
                }
                #[doc = " Authority account ids scheduled for the next session"]                pub fn account_to_authority (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: dkg_runtime_primitives :: crypto :: Public > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKG" , "AccountToAuthority" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_256)] , [153u8 , 156u8 , 238u8 , 107u8 , 10u8 , 129u8 , 109u8 , 32u8 , 111u8 , 45u8 , 223u8 , 84u8 , 30u8 , 115u8 , 139u8 , 254u8 , 58u8 , 194u8 , 205u8 , 73u8 , 249u8 , 102u8 , 217u8 , 125u8 , 137u8 , 206u8 , 87u8 , 34u8 , 100u8 , 45u8 , 223u8 , 252u8 ,])
                }
                #[doc = " Authority account ids scheduled for the next session"]                pub fn account_to_authority_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: dkg_runtime_primitives :: crypto :: Public > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "AccountToAuthority",
                        Vec::new(),
                        [
                            153u8, 156u8, 238u8, 107u8, 10u8, 129u8, 109u8,
                            32u8, 111u8, 45u8, 223u8, 84u8, 30u8, 115u8, 139u8,
                            254u8, 58u8, 194u8, 205u8, 73u8, 249u8, 102u8,
                            217u8, 125u8, 137u8, 206u8, 87u8, 34u8, 100u8,
                            45u8, 223u8, 252u8,
                        ],
                    )
                }
                #[doc = " Tracks misbehaviour reports"]                pub fn misbehaviour_reports (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: dkg_runtime_primitives :: MisbehaviourType > , _1 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u64 > , _2 : impl :: std :: borrow :: Borrow < runtime_types :: dkg_runtime_primitives :: crypto :: Public > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: dkg_runtime_primitives :: AggregatedMisbehaviourReports < runtime_types :: dkg_runtime_primitives :: crypto :: Public > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKG" , "MisbehaviourReports" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new ((_0 . borrow () , _1 . borrow () , _2 . borrow ()) , :: subxt :: storage :: address :: StorageHasher :: Blake2_256)] , [26u8 , 223u8 , 186u8 , 140u8 , 112u8 , 238u8 , 39u8 , 204u8 , 21u8 , 223u8 , 229u8 , 92u8 , 173u8 , 76u8 , 228u8 , 188u8 , 225u8 , 56u8 , 140u8 , 24u8 , 100u8 , 171u8 , 177u8 , 26u8 , 108u8 , 130u8 , 238u8 , 142u8 , 198u8 , 89u8 , 17u8 , 255u8 ,])
                }
                #[doc = " Tracks misbehaviour reports"]                pub fn misbehaviour_reports_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: dkg_runtime_primitives :: AggregatedMisbehaviourReports < runtime_types :: dkg_runtime_primitives :: crypto :: Public > > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "MisbehaviourReports",
                        Vec::new(),
                        [
                            26u8, 223u8, 186u8, 140u8, 112u8, 238u8, 39u8,
                            204u8, 21u8, 223u8, 229u8, 92u8, 173u8, 76u8,
                            228u8, 188u8, 225u8, 56u8, 140u8, 24u8, 100u8,
                            171u8, 177u8, 26u8, 108u8, 130u8, 238u8, 142u8,
                            198u8, 89u8, 17u8, 255u8,
                        ],
                    )
                }
                #[doc = " Tracks authority reputations"]                pub fn authority_reputations (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: dkg_runtime_primitives :: crypto :: Public > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKG" , "AuthorityReputations" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [232u8 , 88u8 , 120u8 , 58u8 , 141u8 , 39u8 , 76u8 , 249u8 , 250u8 , 154u8 , 173u8 , 127u8 , 163u8 , 70u8 , 186u8 , 193u8 , 210u8 , 43u8 , 225u8 , 11u8 , 254u8 , 208u8 , 46u8 , 87u8 , 66u8 , 21u8 , 111u8 , 228u8 , 54u8 , 200u8 , 46u8 , 2u8 ,])
                }
                #[doc = " Tracks authority reputations"]                pub fn authority_reputations_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "AuthorityReputations",
                        Vec::new(),
                        [
                            232u8, 88u8, 120u8, 58u8, 141u8, 39u8, 76u8, 249u8,
                            250u8, 154u8, 173u8, 127u8, 163u8, 70u8, 186u8,
                            193u8, 210u8, 43u8, 225u8, 11u8, 254u8, 208u8,
                            46u8, 87u8, 66u8, 21u8, 111u8, 228u8, 54u8, 200u8,
                            46u8, 2u8,
                        ],
                    )
                }
                #[doc = " Tracks jailed authorities for keygen by mapping"]
                #[doc = " to the block number when the authority was last jailed"]                pub fn jailed_keygen_authorities (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: dkg_runtime_primitives :: crypto :: Public > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKG" , "JailedKeygenAuthorities" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_256)] , [59u8 , 86u8 , 235u8 , 7u8 , 94u8 , 26u8 , 253u8 , 179u8 , 186u8 , 33u8 , 6u8 , 242u8 , 46u8 , 174u8 , 254u8 , 118u8 , 92u8 , 208u8 , 166u8 , 181u8 , 45u8 , 137u8 , 41u8 , 87u8 , 109u8 , 37u8 , 171u8 , 59u8 , 169u8 , 48u8 , 78u8 , 107u8 ,])
                }
                #[doc = " Tracks jailed authorities for keygen by mapping"]
                #[doc = " to the block number when the authority was last jailed"]                pub fn jailed_keygen_authorities_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "JailedKeygenAuthorities",
                        Vec::new(),
                        [
                            59u8, 86u8, 235u8, 7u8, 94u8, 26u8, 253u8, 179u8,
                            186u8, 33u8, 6u8, 242u8, 46u8, 174u8, 254u8, 118u8,
                            92u8, 208u8, 166u8, 181u8, 45u8, 137u8, 41u8, 87u8,
                            109u8, 37u8, 171u8, 59u8, 169u8, 48u8, 78u8, 107u8,
                        ],
                    )
                }
                #[doc = " Tracks jailed authorities for signing by mapping"]
                #[doc = " to the block number when the authority was last jailed"]                pub fn jailed_signing_authorities (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: dkg_runtime_primitives :: crypto :: Public > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKG" , "JailedSigningAuthorities" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_256)] , [32u8 , 100u8 , 53u8 , 234u8 , 245u8 , 105u8 , 136u8 , 132u8 , 225u8 , 219u8 , 138u8 , 165u8 , 124u8 , 174u8 , 201u8 , 219u8 , 255u8 , 214u8 , 104u8 , 204u8 , 14u8 , 32u8 , 181u8 , 79u8 , 135u8 , 172u8 , 26u8 , 1u8 , 115u8 , 98u8 , 110u8 , 89u8 ,])
                }
                #[doc = " Tracks jailed authorities for signing by mapping"]
                #[doc = " to the block number when the authority was last jailed"]                pub fn jailed_signing_authorities_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "JailedSigningAuthorities",
                        Vec::new(),
                        [
                            32u8, 100u8, 53u8, 234u8, 245u8, 105u8, 136u8,
                            132u8, 225u8, 219u8, 138u8, 165u8, 124u8, 174u8,
                            201u8, 219u8, 255u8, 214u8, 104u8, 204u8, 14u8,
                            32u8, 181u8, 79u8, 135u8, 172u8, 26u8, 1u8, 115u8,
                            98u8, 110u8, 89u8,
                        ],
                    )
                }
                #[doc = " The current best authorities of the active keygen set"]                pub fn best_authorities (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < (:: core :: primitive :: u16 , runtime_types :: dkg_runtime_primitives :: crypto :: Public ,) > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "BestAuthorities",
                        vec![],
                        [
                            89u8, 42u8, 37u8, 123u8, 4u8, 150u8, 194u8, 145u8,
                            202u8, 252u8, 78u8, 98u8, 192u8, 4u8, 0u8, 7u8,
                            26u8, 50u8, 9u8, 24u8, 16u8, 175u8, 138u8, 177u8,
                            171u8, 41u8, 221u8, 240u8, 108u8, 80u8, 12u8,
                            102u8,
                        ],
                    )
                }
                #[doc = " The next best authorities of the active keygen set"]                pub fn next_best_authorities (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < (:: core :: primitive :: u16 , runtime_types :: dkg_runtime_primitives :: crypto :: Public ,) > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "NextBestAuthorities",
                        vec![],
                        [
                            79u8, 46u8, 76u8, 164u8, 39u8, 181u8, 122u8, 253u8,
                            149u8, 134u8, 168u8, 88u8, 41u8, 51u8, 183u8, 99u8,
                            161u8, 28u8, 206u8, 185u8, 18u8, 51u8, 8u8, 50u8,
                            195u8, 58u8, 82u8, 51u8, 193u8, 218u8, 5u8, 116u8,
                        ],
                    )
                }
                #[doc = " The last BlockNumber at which the session rotation happened"]                pub fn last_session_rotation_block (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKG",
                        "LastSessionRotationBlock",
                        vec![],
                        [
                            221u8, 138u8, 110u8, 204u8, 255u8, 254u8, 229u8,
                            185u8, 225u8, 170u8, 60u8, 234u8, 12u8, 6u8, 235u8,
                            76u8, 245u8, 206u8, 254u8, 229u8, 239u8, 140u8,
                            111u8, 58u8, 254u8, 131u8, 255u8, 112u8, 88u8,
                            134u8, 241u8, 86u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Percentage session should have progressed for refresh to begin"]
                pub fn refresh_delay(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_arithmetic::per_things::Permill,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "DKG",
                        "RefreshDelay",
                        [
                            225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8,
                            192u8, 254u8, 19u8, 87u8, 80u8, 16u8, 62u8, 42u8,
                            204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
                            177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8,
                            86u8, 227u8,
                        ],
                    )
                }
                #[doc = " Number of blocks of cooldown after unsigned transaction is included."]
                #[doc = ""]
                #[doc = " This ensures that we only accept unsigned transactions once, every `UnsignedInterval`"]
                #[doc = " blocks."]
                pub fn unsigned_interval(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "DKG",
                        "UnsignedInterval",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " A configuration for base priority of unsigned transactions."]
                #[doc = ""]
                #[doc = " This is exposed so that it can be tuned for particular runtime, when"]
                #[doc = " multiple pallets send unsigned transactions."]
                pub fn unsigned_priority(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "DKG",
                        "UnsignedPriority",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8,
                            231u8, 190u8, 146u8, 59u8, 226u8, 157u8, 101u8,
                            103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8,
                            119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8,
                            239u8, 42u8, 246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod dkg_proposals {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetThreshold {
                pub threshold: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetResource {
                pub id: runtime_types::webb_proposals::header::ResourceId,
                pub method: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RemoveResource {
                pub id: runtime_types::webb_proposals::header::ResourceId,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct WhitelistChain {
                pub chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct AddProposer {
                pub native_account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub external_account: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RemoveProposer {
                pub v: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct AcknowledgeProposal {
                pub nonce: runtime_types::webb_proposals::nonce::Nonce,
                pub src_chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub r_id: runtime_types::webb_proposals::header::ResourceId,
                pub prop: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RejectProposal {
                pub nonce: runtime_types::webb_proposals::nonce::Nonce,
                pub src_chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub r_id: runtime_types::webb_proposals::header::ResourceId,
                pub prop: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct EvalVoteState {
                pub nonce: runtime_types::webb_proposals::nonce::Nonce,
                pub src_chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub prop: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Sets the vote threshold for proposals."]
                #[doc = ""]
                #[doc = "This threshold is used to determine how many votes are required"]
                #[doc = "before a proposal is executed."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1) lookup and insert"]
                #[doc = "# </weight>"]
                pub fn set_threshold(
                    &self,
                    threshold: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetThreshold>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposals",
                        "set_threshold",
                        SetThreshold { threshold },
                        [
                            97u8, 156u8, 187u8, 176u8, 219u8, 218u8, 220u8,
                            12u8, 153u8, 121u8, 167u8, 142u8, 88u8, 249u8,
                            99u8, 44u8, 183u8, 26u8, 130u8, 71u8, 228u8, 33u8,
                            233u8, 203u8, 93u8, 138u8, 130u8, 146u8, 242u8,
                            169u8, 196u8, 225u8,
                        ],
                    )
                }
                #[doc = "Stores a method name on chain under an associated resource ID."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1) write"]
                #[doc = "# </weight>"]
                pub fn set_resource(
                    &self,
                    id: runtime_types::webb_proposals::header::ResourceId,
                    method: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetResource> {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposals",
                        "set_resource",
                        SetResource { id, method },
                        [
                            72u8, 53u8, 0u8, 252u8, 156u8, 111u8, 215u8, 92u8,
                            184u8, 69u8, 64u8, 26u8, 251u8, 229u8, 240u8,
                            155u8, 243u8, 21u8, 7u8, 57u8, 14u8, 251u8, 44u8,
                            55u8, 238u8, 91u8, 134u8, 175u8, 239u8, 76u8,
                            181u8, 17u8,
                        ],
                    )
                }
                #[doc = "Removes a resource ID from the resource mapping."]
                #[doc = ""]
                #[doc = "After this call, bridge transfers with the associated resource ID"]
                #[doc = "will be rejected."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1) removal"]
                #[doc = "# </weight>"]
                pub fn remove_resource(
                    &self,
                    id: runtime_types::webb_proposals::header::ResourceId,
                ) -> ::subxt::tx::StaticTxPayload<RemoveResource>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposals",
                        "remove_resource",
                        RemoveResource { id },
                        [
                            98u8, 85u8, 131u8, 94u8, 224u8, 152u8, 145u8,
                            154u8, 26u8, 36u8, 199u8, 221u8, 240u8, 200u8,
                            190u8, 184u8, 0u8, 140u8, 90u8, 36u8, 11u8, 250u8,
                            103u8, 150u8, 246u8, 70u8, 170u8, 94u8, 210u8,
                            172u8, 92u8, 67u8,
                        ],
                    )
                }
                #[doc = "Enables a chain ID as a source or destination for a bridge transfer."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1) lookup and insert"]
                #[doc = "# </weight>"]
                pub fn whitelist_chain(
                    &self,
                    chain_id : runtime_types :: webb_proposals :: header :: TypedChainId,
                ) -> ::subxt::tx::StaticTxPayload<WhitelistChain>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposals",
                        "whitelist_chain",
                        WhitelistChain { chain_id },
                        [
                            143u8, 172u8, 215u8, 87u8, 23u8, 86u8, 95u8, 116u8,
                            205u8, 140u8, 58u8, 104u8, 6u8, 162u8, 28u8, 115u8,
                            64u8, 94u8, 184u8, 169u8, 225u8, 28u8, 180u8, 13u8,
                            242u8, 35u8, 58u8, 114u8, 120u8, 220u8, 48u8,
                            162u8,
                        ],
                    )
                }
                #[doc = "Adds a new proposer to the proposer set."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1) lookup and insert"]
                #[doc = "# </weight>"]
                pub fn add_proposer(
                    &self,
                    native_account: ::subxt::ext::sp_core::crypto::AccountId32,
                    external_account: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<AddProposer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposals",
                        "add_proposer",
                        AddProposer {
                            native_account,
                            external_account,
                        },
                        [
                            160u8, 92u8, 87u8, 9u8, 152u8, 22u8, 53u8, 235u8,
                            151u8, 20u8, 251u8, 54u8, 150u8, 195u8, 177u8,
                            181u8, 190u8, 9u8, 209u8, 246u8, 99u8, 150u8, 40u8,
                            77u8, 191u8, 175u8, 100u8, 75u8, 5u8, 244u8, 184u8,
                            122u8,
                        ],
                    )
                }
                #[doc = "Removes an existing proposer from the set."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1) lookup and removal"]
                #[doc = "# </weight>"]
                pub fn remove_proposer(
                    &self,
                    v: ::subxt::ext::sp_core::crypto::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<RemoveProposer>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposals",
                        "remove_proposer",
                        RemoveProposer { v },
                        [
                            21u8, 136u8, 197u8, 105u8, 101u8, 203u8, 64u8,
                            170u8, 57u8, 37u8, 253u8, 246u8, 124u8, 127u8,
                            131u8, 73u8, 55u8, 218u8, 160u8, 22u8, 232u8,
                            198u8, 208u8, 166u8, 73u8, 24u8, 234u8, 117u8,
                            17u8, 232u8, 72u8, 234u8,
                        ],
                    )
                }
                #[doc = "Commits a vote in favour of the provided proposal."]
                #[doc = ""]
                #[doc = "If a proposal with the given nonce and source chain ID does not"]
                #[doc = "already exist, it will be created with an initial vote in favour"]
                #[doc = "from the caller."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- weight of proposed call, regardless of whether execution is performed"]
                #[doc = "# </weight>"]
                pub fn acknowledge_proposal(
                    &self,
                    nonce: runtime_types::webb_proposals::nonce::Nonce,
                    src_chain_id : runtime_types :: webb_proposals :: header :: TypedChainId,
                    r_id: runtime_types::webb_proposals::header::ResourceId,
                    prop: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<AcknowledgeProposal>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposals",
                        "acknowledge_proposal",
                        AcknowledgeProposal {
                            nonce,
                            src_chain_id,
                            r_id,
                            prop,
                        },
                        [
                            78u8, 188u8, 218u8, 122u8, 96u8, 232u8, 139u8,
                            87u8, 74u8, 5u8, 204u8, 158u8, 14u8, 2u8, 144u8,
                            215u8, 99u8, 138u8, 106u8, 189u8, 215u8, 176u8,
                            172u8, 79u8, 154u8, 114u8, 221u8, 70u8, 113u8,
                            71u8, 24u8, 15u8,
                        ],
                    )
                }
                #[doc = "Commits a vote against a provided proposal."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Fixed, since execution of proposal should not be included"]
                #[doc = "# </weight>"]
                pub fn reject_proposal(
                    &self,
                    nonce: runtime_types::webb_proposals::nonce::Nonce,
                    src_chain_id : runtime_types :: webb_proposals :: header :: TypedChainId,
                    r_id: runtime_types::webb_proposals::header::ResourceId,
                    prop: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<RejectProposal>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposals",
                        "reject_proposal",
                        RejectProposal {
                            nonce,
                            src_chain_id,
                            r_id,
                            prop,
                        },
                        [
                            129u8, 230u8, 170u8, 177u8, 148u8, 77u8, 130u8,
                            25u8, 109u8, 253u8, 56u8, 113u8, 186u8, 139u8,
                            233u8, 31u8, 40u8, 100u8, 178u8, 109u8, 51u8,
                            153u8, 41u8, 236u8, 154u8, 175u8, 34u8, 42u8,
                            245u8, 241u8, 204u8, 234u8,
                        ],
                    )
                }
                #[doc = "Evaluate the state of a proposal given the current vote threshold."]
                #[doc = ""]
                #[doc = "A proposal with enough votes will be either executed or cancelled,"]
                #[doc = "and the status will be updated accordingly."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- weight of proposed call, regardless of whether execution is performed"]
                #[doc = "# </weight>"]
                pub fn eval_vote_state(
                    &self,
                    nonce: runtime_types::webb_proposals::nonce::Nonce,
                    src_chain_id : runtime_types :: webb_proposals :: header :: TypedChainId,
                    prop: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<EvalVoteState>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposals",
                        "eval_vote_state",
                        EvalVoteState {
                            nonce,
                            src_chain_id,
                            prop,
                        },
                        [
                            64u8, 67u8, 168u8, 210u8, 157u8, 227u8, 173u8,
                            217u8, 222u8, 207u8, 157u8, 193u8, 127u8, 132u8,
                            4u8, 101u8, 209u8, 249u8, 24u8, 214u8, 148u8, 18u8,
                            155u8, 43u8, 121u8, 166u8, 194u8, 82u8, 30u8,
                            100u8, 66u8, 179u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_dkg_proposals::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Vote threshold has changed (new_threshold)"]
            pub struct ProposerThresholdChanged {
                pub new_threshold: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for ProposerThresholdChanged {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposerThresholdChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Chain now available for transfers (chain_id)"]
            pub struct ChainWhitelisted {
                pub chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
            }
            impl ::subxt::events::StaticEvent for ChainWhitelisted {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ChainWhitelisted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Proposer added to set"]
            pub struct ProposerAdded {
                pub proposer_id: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for ProposerAdded {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposerAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Proposer removed from set"]
            pub struct ProposerRemoved {
                pub proposer_id: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for ProposerRemoved {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposerRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Vote submitted in favour of proposal"]
            pub struct VoteFor {
                pub chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for VoteFor {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "VoteFor";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Vot submitted against proposal"]
            pub struct VoteAgainst {
                pub chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for VoteAgainst {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "VoteAgainst";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Voting successful for a proposal"]
            pub struct ProposalApproved {
                pub chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
            }
            impl ::subxt::events::StaticEvent for ProposalApproved {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposalApproved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Voting rejected a proposal"]
            pub struct ProposalRejected {
                pub chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
            }
            impl ::subxt::events::StaticEvent for ProposalRejected {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposalRejected";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Execution of call succeeded"]
            pub struct ProposalSucceeded {
                pub chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
            }
            impl ::subxt::events::StaticEvent for ProposalSucceeded {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposalSucceeded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Execution of call failed"]
            pub struct ProposalFailed {
                pub chain_id:
                    runtime_types::webb_proposals::header::TypedChainId,
                pub proposal_nonce: runtime_types::webb_proposals::nonce::Nonce,
            }
            impl ::subxt::events::StaticEvent for ProposalFailed {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "ProposalFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Proposers have been reset"]
            pub struct AuthorityProposersReset {
                pub proposers:
                    ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::events::StaticEvent for AuthorityProposersReset {
                const PALLET: &'static str = "DKGProposals";
                const EVENT: &'static str = "AuthorityProposersReset";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " All whitelisted chains and their respective transaction counts"]                pub fn chain_nonces (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: webb_proposals :: header :: TypedChainId > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: webb_proposals :: nonce :: Nonce > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKGProposals" , "ChainNonces" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_256)] , [153u8 , 41u8 , 103u8 , 222u8 , 96u8 , 64u8 , 161u8 , 162u8 , 39u8 , 95u8 , 154u8 , 67u8 , 167u8 , 127u8 , 163u8 , 149u8 , 38u8 , 175u8 , 245u8 , 223u8 , 182u8 , 86u8 , 163u8 , 10u8 , 143u8 , 190u8 , 137u8 , 192u8 , 227u8 , 12u8 , 135u8 , 227u8 ,])
                }
                #[doc = " All whitelisted chains and their respective transaction counts"]                pub fn chain_nonces_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: webb_proposals :: nonce :: Nonce > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposals",
                        "ChainNonces",
                        Vec::new(),
                        [
                            153u8, 41u8, 103u8, 222u8, 96u8, 64u8, 161u8,
                            162u8, 39u8, 95u8, 154u8, 67u8, 167u8, 127u8,
                            163u8, 149u8, 38u8, 175u8, 245u8, 223u8, 182u8,
                            86u8, 163u8, 10u8, 143u8, 190u8, 137u8, 192u8,
                            227u8, 12u8, 135u8, 227u8,
                        ],
                    )
                }
                #[doc = " Number of votes required for a proposal to execute"]                pub fn proposer_threshold (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposals",
                        "ProposerThreshold",
                        vec![],
                        [
                            129u8, 174u8, 171u8, 36u8, 172u8, 108u8, 139u8,
                            176u8, 152u8, 127u8, 52u8, 68u8, 109u8, 238u8,
                            50u8, 176u8, 49u8, 78u8, 240u8, 36u8, 94u8, 247u8,
                            215u8, 82u8, 109u8, 10u8, 81u8, 156u8, 14u8, 247u8,
                            39u8, 154u8,
                        ],
                    )
                }
                #[doc = " Proposer Set Update Proposal Nonce"]                pub fn proposer_set_update_proposal_nonce (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposals",
                        "ProposerSetUpdateProposalNonce",
                        vec![],
                        [
                            118u8, 52u8, 184u8, 159u8, 206u8, 28u8, 122u8,
                            219u8, 168u8, 206u8, 143u8, 16u8, 128u8, 31u8,
                            254u8, 40u8, 45u8, 92u8, 183u8, 46u8, 80u8, 19u8,
                            131u8, 6u8, 26u8, 105u8, 81u8, 174u8, 10u8, 154u8,
                            186u8, 157u8,
                        ],
                    )
                }
                #[doc = " Tracks current proposer set"]                pub fn proposers (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: bool > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKGProposals" , "Proposers" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [163u8 , 252u8 , 227u8 , 131u8 , 254u8 , 6u8 , 64u8 , 191u8 , 158u8 , 117u8 , 69u8 , 137u8 , 184u8 , 85u8 , 112u8 , 252u8 , 137u8 , 30u8 , 24u8 , 87u8 , 55u8 , 82u8 , 22u8 , 241u8 , 124u8 , 222u8 , 155u8 , 60u8 , 123u8 , 5u8 , 191u8 , 79u8 ,])
                }
                #[doc = " Tracks current proposer set"]                pub fn proposers_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: bool > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposals",
                        "Proposers",
                        Vec::new(),
                        [
                            163u8, 252u8, 227u8, 131u8, 254u8, 6u8, 64u8,
                            191u8, 158u8, 117u8, 69u8, 137u8, 184u8, 85u8,
                            112u8, 252u8, 137u8, 30u8, 24u8, 87u8, 55u8, 82u8,
                            22u8, 241u8, 124u8, 222u8, 155u8, 60u8, 123u8, 5u8,
                            191u8, 79u8,
                        ],
                    )
                }
                #[doc = " Tracks current proposer set external accounts"]
                #[doc = " Currently meant to store Ethereum compatible 64-bytes ECDSA public keys"]                pub fn external_proposer_accounts (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKGProposals" , "ExternalProposerAccounts" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [181u8 , 231u8 , 15u8 , 219u8 , 97u8 , 164u8 , 35u8 , 177u8 , 61u8 , 116u8 , 5u8 , 51u8 , 182u8 , 49u8 , 119u8 , 136u8 , 78u8 , 127u8 , 5u8 , 123u8 , 3u8 , 97u8 , 167u8 , 62u8 , 155u8 , 130u8 , 239u8 , 231u8 , 11u8 , 61u8 , 194u8 , 220u8 ,])
                }
                #[doc = " Tracks current proposer set external accounts"]
                #[doc = " Currently meant to store Ethereum compatible 64-bytes ECDSA public keys"]                pub fn external_proposer_accounts_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposals",
                        "ExternalProposerAccounts",
                        Vec::new(),
                        [
                            181u8, 231u8, 15u8, 219u8, 97u8, 164u8, 35u8,
                            177u8, 61u8, 116u8, 5u8, 51u8, 182u8, 49u8, 119u8,
                            136u8, 78u8, 127u8, 5u8, 123u8, 3u8, 97u8, 167u8,
                            62u8, 155u8, 130u8, 239u8, 231u8, 11u8, 61u8,
                            194u8, 220u8,
                        ],
                    )
                }
                #[doc = " Tracks the authorities that are proposers so we can properly update the proposer set"]
                #[doc = " across sessions and authority changes."]                pub fn authority_proposers (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposals",
                        "AuthorityProposers",
                        vec![],
                        [
                            151u8, 87u8, 196u8, 10u8, 133u8, 183u8, 9u8, 130u8,
                            57u8, 0u8, 86u8, 31u8, 253u8, 33u8, 175u8, 239u8,
                            91u8, 155u8, 179u8, 43u8, 74u8, 159u8, 128u8, 22u8,
                            16u8, 57u8, 164u8, 239u8, 47u8, 151u8, 251u8,
                            187u8,
                        ],
                    )
                }
                #[doc = " Tracks current proposer set external accounts"]                pub fn external_authority_proposer_accounts (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: std :: vec :: Vec < :: core :: primitive :: u8 > > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposals",
                        "ExternalAuthorityProposerAccounts",
                        vec![],
                        [
                            180u8, 223u8, 172u8, 35u8, 236u8, 128u8, 207u8,
                            140u8, 38u8, 75u8, 40u8, 243u8, 244u8, 122u8, 7u8,
                            25u8, 120u8, 152u8, 240u8, 169u8, 165u8, 208u8,
                            190u8, 127u8, 189u8, 117u8, 160u8, 34u8, 243u8,
                            253u8, 223u8, 18u8,
                        ],
                    )
                }
                #[doc = " Number of proposers in set"]                pub fn proposer_count (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposals",
                        "ProposerCount",
                        vec![],
                        [
                            150u8, 116u8, 125u8, 20u8, 135u8, 11u8, 47u8,
                            155u8, 87u8, 113u8, 44u8, 139u8, 67u8, 74u8, 92u8,
                            113u8, 173u8, 62u8, 207u8, 79u8, 125u8, 109u8,
                            170u8, 166u8, 55u8, 85u8, 3u8, 32u8, 155u8, 45u8,
                            236u8, 253u8,
                        ],
                    )
                }
                #[doc = " All known proposals."]
                #[doc = " The key is the hash of the call and the deposit ID, to ensure it's"]
                #[doc = " unique."]                pub fn votes (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: webb_proposals :: header :: TypedChainId > , _1 : impl :: std :: borrow :: Borrow < (runtime_types :: webb_proposals :: nonce :: Nonce , :: std :: vec :: Vec < :: core :: primitive :: u8 > ,) > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_dkg_proposals :: types :: ProposalVotes < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKGProposals" , "Votes" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_256) , :: subxt :: storage :: address :: StorageMapKey :: new (_1 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_256)] , [218u8 , 110u8 , 32u8 , 203u8 , 246u8 , 231u8 , 137u8 , 45u8 , 110u8 , 72u8 , 218u8 , 232u8 , 162u8 , 253u8 , 159u8 , 65u8 , 155u8 , 129u8 , 203u8 , 37u8 , 18u8 , 61u8 , 135u8 , 190u8 , 5u8 , 55u8 , 88u8 , 175u8 , 86u8 , 56u8 , 249u8 , 107u8 ,])
                }
                #[doc = " All known proposals."]
                #[doc = " The key is the hash of the call and the deposit ID, to ensure it's"]
                #[doc = " unique."]                pub fn votes_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_dkg_proposals :: types :: ProposalVotes < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposals",
                        "Votes",
                        Vec::new(),
                        [
                            218u8, 110u8, 32u8, 203u8, 246u8, 231u8, 137u8,
                            45u8, 110u8, 72u8, 218u8, 232u8, 162u8, 253u8,
                            159u8, 65u8, 155u8, 129u8, 203u8, 37u8, 18u8, 61u8,
                            135u8, 190u8, 5u8, 55u8, 88u8, 175u8, 86u8, 56u8,
                            249u8, 107u8,
                        ],
                    )
                }
                #[doc = " Utilized by the bridge software to map resource IDs to actual methods"]                pub fn resources (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: webb_proposals :: header :: ResourceId > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKGProposals" , "Resources" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_256)] , [40u8 , 109u8 , 87u8 , 233u8 , 98u8 , 199u8 , 58u8 , 77u8 , 169u8 , 59u8 , 20u8 , 195u8 , 125u8 , 128u8 , 101u8 , 248u8 , 84u8 , 184u8 , 226u8 , 24u8 , 88u8 , 239u8 , 51u8 , 8u8 , 36u8 , 68u8 , 189u8 , 54u8 , 172u8 , 240u8 , 65u8 , 31u8 ,])
                }
                #[doc = " Utilized by the bridge software to map resource IDs to actual methods"]                pub fn resources_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposals",
                        "Resources",
                        Vec::new(),
                        [
                            40u8, 109u8, 87u8, 233u8, 98u8, 199u8, 58u8, 77u8,
                            169u8, 59u8, 20u8, 195u8, 125u8, 128u8, 101u8,
                            248u8, 84u8, 184u8, 226u8, 24u8, 88u8, 239u8, 51u8,
                            8u8, 36u8, 68u8, 189u8, 54u8, 172u8, 240u8, 65u8,
                            31u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The identifier for this chain."]
                #[doc = " This must be unique and must not collide with existing IDs within a"]
                #[doc = " set of bridged chains."]
                pub fn chain_identifier(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::webb_proposals::header::TypedChainId,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "DKGProposals",
                        "ChainIdentifier",
                        [
                            80u8, 245u8, 165u8, 63u8, 83u8, 75u8, 120u8, 198u8,
                            51u8, 48u8, 231u8, 27u8, 18u8, 145u8, 145u8, 198u8,
                            211u8, 221u8, 19u8, 170u8, 68u8, 87u8, 79u8, 5u8,
                            204u8, 71u8, 188u8, 140u8, 38u8, 17u8, 107u8,
                            146u8,
                        ],
                    )
                }
                pub fn proposal_lifetime(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "DKGProposals",
                        "ProposalLifetime",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The session period"]
                pub fn period(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "DKGProposals",
                        "Period",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod dkg_proposal_handler {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SubmitSignedProposals {
                pub props: ::std::vec::Vec<
                    runtime_types::webb_proposals::proposal::Proposal,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceSubmitUnsignedProposal {
                pub prop: runtime_types::webb_proposals::proposal::Proposal,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn submit_signed_proposals(
                    &self,
                    props: ::std::vec::Vec<
                        runtime_types::webb_proposals::proposal::Proposal,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SubmitSignedProposals>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposalHandler",
                        "submit_signed_proposals",
                        SubmitSignedProposals { props },
                        [
                            132u8, 66u8, 83u8, 114u8, 104u8, 101u8, 251u8,
                            59u8, 8u8, 38u8, 154u8, 186u8, 142u8, 242u8, 122u8,
                            187u8, 147u8, 144u8, 156u8, 71u8, 143u8, 222u8,
                            250u8, 65u8, 142u8, 83u8, 73u8, 33u8, 39u8, 150u8,
                            136u8, 101u8,
                        ],
                    )
                }
                #[doc = "Force submit an unsigned proposal to the DKG"]
                #[doc = ""]
                #[doc = "There are certain proposals we'd like to be proposable only"]
                #[doc = "through root actions. The currently supported proposals are"]
                #[doc = "\t1. Updating"]
                pub fn force_submit_unsigned_proposal(
                    &self,
                    prop: runtime_types::webb_proposals::proposal::Proposal,
                ) -> ::subxt::tx::StaticTxPayload<ForceSubmitUnsignedProposal>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "DKGProposalHandler",
                        "force_submit_unsigned_proposal",
                        ForceSubmitUnsignedProposal { prop },
                        [
                            14u8, 118u8, 150u8, 7u8, 47u8, 232u8, 149u8, 216u8,
                            64u8, 129u8, 39u8, 223u8, 215u8, 109u8, 121u8,
                            53u8, 208u8, 0u8, 48u8, 153u8, 181u8, 208u8, 148u8,
                            176u8, 123u8, 38u8, 241u8, 173u8, 107u8, 21u8,
                            13u8, 86u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event =
            runtime_types::pallet_dkg_proposal_handler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "RuntimeEvent Emitted when we encounter a Proposal with invalid Signature."]
            pub struct InvalidProposalSignature {
                pub kind: runtime_types::webb_proposals::proposal::ProposalKind,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
                pub invalid_signature: ::std::vec::Vec<::core::primitive::u8>,
                pub expected_public_key: ::core::option::Option<
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
                pub actual_public_key: ::core::option::Option<
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
            }
            impl ::subxt::events::StaticEvent for InvalidProposalSignature {
                const PALLET: &'static str = "DKGProposalHandler";
                const EVENT: &'static str = "InvalidProposalSignature";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "RuntimeEvent When a Proposal is added to UnsignedProposalQueue."]
            pub struct ProposalAdded { pub key : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , pub target_chain : runtime_types :: webb_proposals :: header :: TypedChainId , pub data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , }
            impl ::subxt::events::StaticEvent for ProposalAdded {
                const PALLET: &'static str = "DKGProposalHandler";
                const EVENT: &'static str = "ProposalAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "RuntimeEvent When a Proposal is removed from UnsignedProposalQueue."]
            pub struct ProposalRemoved { pub key : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , pub target_chain : runtime_types :: webb_proposals :: header :: TypedChainId , }
            impl ::subxt::events::StaticEvent for ProposalRemoved {
                const PALLET: &'static str = "DKGProposalHandler";
                const EVENT: &'static str = "ProposalRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "RuntimeEvent When a Proposal Gets Signed by DKG."]
            pub struct ProposalSigned { pub key : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , pub target_chain : runtime_types :: webb_proposals :: header :: TypedChainId , pub data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , pub signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , }
            impl ::subxt::events::StaticEvent for ProposalSigned {
                const PALLET: &'static str = "DKGProposalHandler";
                const EVENT: &'static str = "ProposalSigned";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " All unsigned proposals."]                pub fn unsigned_proposal_queue (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: webb_proposals :: header :: TypedChainId > , _1 : impl :: std :: borrow :: Borrow < runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: dkg_runtime_primitives :: proposal :: StoredUnsignedProposal < :: core :: primitive :: u32 > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKGProposalHandler" , "UnsignedProposalQueue" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat) , :: subxt :: storage :: address :: StorageMapKey :: new (_1 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [109u8 , 186u8 , 212u8 , 65u8 , 131u8 , 43u8 , 97u8 , 103u8 , 37u8 , 95u8 , 11u8 , 112u8 , 114u8 , 195u8 , 155u8 , 65u8 , 149u8 , 88u8 , 133u8 , 86u8 , 134u8 , 238u8 , 34u8 , 201u8 , 30u8 , 247u8 , 25u8 , 51u8 , 70u8 , 221u8 , 76u8 , 129u8 ,])
                }
                #[doc = " All unsigned proposals."]                pub fn unsigned_proposal_queue_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: dkg_runtime_primitives :: proposal :: StoredUnsignedProposal < :: core :: primitive :: u32 > > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposalHandler",
                        "UnsignedProposalQueue",
                        Vec::new(),
                        [
                            109u8, 186u8, 212u8, 65u8, 131u8, 43u8, 97u8,
                            103u8, 37u8, 95u8, 11u8, 112u8, 114u8, 195u8,
                            155u8, 65u8, 149u8, 88u8, 133u8, 86u8, 134u8,
                            238u8, 34u8, 201u8, 30u8, 247u8, 25u8, 51u8, 70u8,
                            221u8, 76u8, 129u8,
                        ],
                    )
                }
                #[doc = " Defines the block when next unsigned transaction will be accepted."]
                #[doc = ""]
                #[doc = " To prevent spam of unsigned (and unpayed!) transactions on the network,"]
                #[doc = " we only allow one transaction every `T::UnsignedInterval` blocks."]
                #[doc = " This storage entry defines when new transaction is going to be accepted."]                pub fn next_unsigned_at (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposalHandler",
                        "NextUnsignedAt",
                        vec![],
                        [
                            175u8, 176u8, 254u8, 20u8, 27u8, 31u8, 70u8, 46u8,
                            201u8, 118u8, 22u8, 200u8, 95u8, 116u8, 223u8,
                            63u8, 191u8, 85u8, 55u8, 21u8, 70u8, 24u8, 225u8,
                            203u8, 66u8, 213u8, 94u8, 229u8, 234u8, 223u8,
                            255u8, 28u8,
                        ],
                    )
                }
                #[doc = " All signed proposals."]                pub fn signed_proposals (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: webb_proposals :: header :: TypedChainId > , _1 : impl :: std :: borrow :: Borrow < runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: webb_proposals :: proposal :: Proposal > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("DKGProposalHandler" , "SignedProposals" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat) , :: subxt :: storage :: address :: StorageMapKey :: new (_1 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [210u8 , 218u8 , 231u8 , 118u8 , 12u8 , 140u8 , 31u8 , 43u8 , 148u8 , 147u8 , 187u8 , 20u8 , 208u8 , 23u8 , 85u8 , 156u8 , 200u8 , 185u8 , 11u8 , 245u8 , 140u8 , 62u8 , 142u8 , 9u8 , 241u8 , 111u8 , 7u8 , 153u8 , 208u8 , 76u8 , 69u8 , 73u8 ,])
                }
                #[doc = " All signed proposals."]                pub fn signed_proposals_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: webb_proposals :: proposal :: Proposal > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DKGProposalHandler",
                        "SignedProposals",
                        Vec::new(),
                        [
                            210u8, 218u8, 231u8, 118u8, 12u8, 140u8, 31u8,
                            43u8, 148u8, 147u8, 187u8, 20u8, 208u8, 23u8, 85u8,
                            156u8, 200u8, 185u8, 11u8, 245u8, 140u8, 62u8,
                            142u8, 9u8, 241u8, 111u8, 7u8, 153u8, 208u8, 76u8,
                            69u8, 73u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Max number of signed proposal submissions per batch;"]
                pub fn max_submissions_per_batch(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "DKGProposalHandler",
                        "MaxSubmissionsPerBatch",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8,
                            169u8, 167u8, 227u8, 41u8, 144u8, 11u8, 236u8,
                            82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8, 90u8,
                            208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8,
                            193u8, 29u8, 70u8,
                        ],
                    )
                }
                #[doc = " Max blocks to store an unsigned proposal"]
                pub fn unsigned_proposal_expiry(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "DKGProposalHandler",
                        "UnsignedProposalExpiry",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event =
            runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who`."]
            pub struct TransactionFeePaid {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub actual_fee: ::core::primitive::u128,
                pub tip: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_arithmetic :: fixed_point :: FixedU128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        vec![],
                        [
                            210u8, 0u8, 206u8, 165u8, 183u8, 10u8, 206u8, 52u8,
                            14u8, 90u8, 218u8, 197u8, 189u8, 125u8, 113u8,
                            216u8, 52u8, 161u8, 45u8, 24u8, 245u8, 237u8,
                            121u8, 41u8, 106u8, 29u8, 45u8, 129u8, 250u8,
                            203u8, 206u8, 180u8,
                        ],
                    )
                }                pub fn storage_version (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_transaction_payment :: Releases > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TransactionPayment",
                        "StorageVersion",
                        vec![],
                        [
                            219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8,
                            8u8, 82u8, 176u8, 200u8, 97u8, 150u8, 177u8, 164u8,
                            166u8, 11u8, 34u8, 12u8, 12u8, 198u8, 58u8, 191u8,
                            186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8,
                            228u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8,
                            179u8, 168u8, 110u8, 28u8, 91u8, 221u8, 64u8, 4u8,
                            148u8, 201u8, 193u8, 185u8, 66u8, 226u8, 114u8,
                            97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8,
                            228u8, 183u8, 165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Sudo {
                pub call: ::std::boxed::Box<
                    runtime_types::dkg_standalone_runtime::RuntimeCall,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SudoUncheckedWeight {
                pub call: ::std::boxed::Box<
                    runtime_types::dkg_standalone_runtime::RuntimeCall,
                >,
                pub weight: runtime_types::sp_weights::weight_v2::Weight,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetKey {
                pub new: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SudoAs {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub call: ::std::boxed::Box<
                    runtime_types::dkg_standalone_runtime::RuntimeCall,
                >,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo(
                    &self,
                    call: runtime_types::dkg_standalone_runtime::RuntimeCall,
                ) -> ::subxt::tx::StaticTxPayload<Sudo> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "sudo",
                        Sudo {
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            191u8, 184u8, 7u8, 103u8, 138u8, 68u8, 7u8, 75u8,
                            112u8, 130u8, 244u8, 180u8, 164u8, 252u8, 107u8,
                            175u8, 43u8, 3u8, 137u8, 71u8, 113u8, 114u8, 125u8,
                            66u8, 193u8, 71u8, 143u8, 143u8, 52u8, 182u8,
                            232u8, 249u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- The weight of this call is defined by the caller."]
                #[doc = "# </weight>"]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::dkg_standalone_runtime::RuntimeCall,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::StaticTxPayload<SudoUncheckedWeight>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "sudo_unchecked_weight",
                        SudoUncheckedWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        },
                        [
                            214u8, 135u8, 7u8, 122u8, 139u8, 217u8, 110u8,
                            66u8, 158u8, 61u8, 18u8, 115u8, 167u8, 216u8, 85u8,
                            106u8, 144u8, 190u8, 204u8, 229u8, 71u8, 209u8,
                            187u8, 88u8, 36u8, 241u8, 110u8, 127u8, 240u8,
                            174u8, 220u8, 68u8,
                        ],
                    )
                }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB change."]
                #[doc = "# </weight>"]
                pub fn set_key(
                    &self,
                    new: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetKey> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "set_key",
                        SetKey { new },
                        [
                            34u8, 116u8, 170u8, 18u8, 106u8, 17u8, 231u8,
                            159u8, 110u8, 246u8, 2u8, 27u8, 161u8, 155u8,
                            163u8, 41u8, 138u8, 7u8, 81u8, 98u8, 230u8, 182u8,
                            23u8, 222u8, 240u8, 117u8, 173u8, 232u8, 192u8,
                            55u8, 92u8, 208u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo_as(
                    &self,
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    call: runtime_types::dkg_standalone_runtime::RuntimeCall,
                ) -> ::subxt::tx::StaticTxPayload<SudoAs> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "sudo_as",
                        SudoAs {
                            who,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            219u8, 115u8, 212u8, 30u8, 192u8, 29u8, 155u8,
                            139u8, 64u8, 254u8, 125u8, 38u8, 137u8, 217u8,
                            93u8, 167u8, 214u8, 220u8, 60u8, 131u8, 27u8,
                            162u8, 19u8, 176u8, 53u8, 64u8, 135u8, 229u8,
                            188u8, 250u8, 35u8, 78u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct Sudid {
                pub sudo_result: ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            }
            impl ::subxt::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct SudoAsDone {
                pub sudo_result: ::core::result::Result<
                    (),
                    runtime_types::sp_runtime::DispatchError,
                >,
            }
            impl ::subxt::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]                pub fn key (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Sudo",
                        "Key",
                        vec![],
                        [
                            244u8, 73u8, 188u8, 136u8, 218u8, 163u8, 68u8,
                            179u8, 122u8, 173u8, 34u8, 108u8, 137u8, 28u8,
                            182u8, 16u8, 196u8, 92u8, 138u8, 34u8, 102u8, 80u8,
                            199u8, 88u8, 107u8, 207u8, 36u8, 22u8, 168u8,
                            167u8, 20u8, 142u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod election_provider_multi_phase {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SubmitUnsigned { pub raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , pub witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetMinimumUntrustedScore {
                pub maybe_next_score: ::core::option::Option<
                    runtime_types::sp_npos_elections::ElectionScore,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetEmergencyElectionResult {
                pub supports: ::std::vec::Vec<(
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    runtime_types::sp_npos_elections::Support<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Submit { pub raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct GovernanceFallback {
                pub maybe_max_voters:
                    ::core::option::Option<::core::primitive::u32>,
                pub maybe_max_targets:
                    ::core::option::Option<::core::primitive::u32>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Submit a solution for the unsigned phase."]
                #[doc = ""]
                #[doc = "The dispatch origin fo this call must be __none__."]
                #[doc = ""]
                #[doc = "This submission is checked on the fly. Moreover, this unsigned solution is only"]
                #[doc = "validated when submitted to the pool from the **local** node. Effectively, this means"]
                #[doc = "that only active validators can submit this transaction when authoring a block (similar"]
                #[doc = "to an inherent)."]
                #[doc = ""]
                #[doc = "To prevent any incorrect solution (and thus wasted time/weight), this transaction will"]
                #[doc = "panic if the solution submitted by the validator is invalid in any way, effectively"]
                #[doc = "putting their authoring reward at risk."]
                #[doc = ""]
                #[doc = "No deposit or reward is associated with this submission."]
                pub fn submit_unsigned(
                    &self,
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 >,
                    witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize,
                ) -> ::subxt::tx::StaticTxPayload<SubmitUnsigned>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "ElectionProviderMultiPhase",
                        "submit_unsigned",
                        SubmitUnsigned {
                            raw_solution: ::std::boxed::Box::new(raw_solution),
                            witness,
                        },
                        [
                            100u8, 240u8, 31u8, 34u8, 93u8, 98u8, 93u8, 57u8,
                            41u8, 197u8, 97u8, 58u8, 242u8, 10u8, 69u8, 250u8,
                            185u8, 169u8, 21u8, 8u8, 202u8, 61u8, 36u8, 25u8,
                            4u8, 148u8, 82u8, 56u8, 242u8, 18u8, 27u8, 219u8,
                        ],
                    )
                }
                #[doc = "Set a new value for `MinimumUntrustedScore`."]
                #[doc = ""]
                #[doc = "Dispatch origin must be aligned with `T::ForceOrigin`."]
                #[doc = ""]
                #[doc = "This check can be turned off by setting the value to `None`."]
                pub fn set_minimum_untrusted_score(
                    &self,
                    maybe_next_score: ::core::option::Option<
                        runtime_types::sp_npos_elections::ElectionScore,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetMinimumUntrustedScore>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "ElectionProviderMultiPhase",
                        "set_minimum_untrusted_score",
                        SetMinimumUntrustedScore { maybe_next_score },
                        [
                            63u8, 101u8, 105u8, 146u8, 133u8, 162u8, 149u8,
                            112u8, 150u8, 219u8, 183u8, 213u8, 234u8, 211u8,
                            144u8, 74u8, 106u8, 15u8, 62u8, 196u8, 247u8, 49u8,
                            20u8, 48u8, 3u8, 105u8, 85u8, 46u8, 76u8, 4u8,
                            67u8, 81u8,
                        ],
                    )
                }
                #[doc = "Set a solution in the queue, to be handed out to the client of this pallet in the next"]
                #[doc = "call to `ElectionProvider::elect`."]
                #[doc = ""]
                #[doc = "This can only be set by `T::ForceOrigin`, and only when the phase is `Emergency`."]
                #[doc = ""]
                #[doc = "The solution is not checked for any feasibility and is assumed to be trustworthy, as any"]
                #[doc = "feasibility check itself can in principle cause the election process to fail (due to"]
                #[doc = "memory/weight constrains)."]
                pub fn set_emergency_election_result(
                    &self,
                    supports: ::std::vec::Vec<(
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        runtime_types::sp_npos_elections::Support<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    )>,
                ) -> ::subxt::tx::StaticTxPayload<SetEmergencyElectionResult>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "ElectionProviderMultiPhase",
                        "set_emergency_election_result",
                        SetEmergencyElectionResult { supports },
                        [
                            115u8, 255u8, 205u8, 58u8, 153u8, 1u8, 246u8, 8u8,
                            225u8, 36u8, 66u8, 144u8, 250u8, 145u8, 70u8, 76u8,
                            54u8, 63u8, 251u8, 51u8, 214u8, 204u8, 55u8, 112u8,
                            46u8, 228u8, 255u8, 250u8, 151u8, 5u8, 44u8, 133u8,
                        ],
                    )
                }
                #[doc = "Submit a solution for the signed phase."]
                #[doc = ""]
                #[doc = "The dispatch origin fo this call must be __signed__."]
                #[doc = ""]
                #[doc = "The solution is potentially queued, based on the claimed score and processed at the end"]
                #[doc = "of the signed phase."]
                #[doc = ""]
                #[doc = "A deposit is reserved and recorded for the solution. Based on the outcome, the solution"]
                #[doc = "might be rewarded, slashed, or get all or a part of the deposit back."]
                pub fn submit(
                    &self,
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 >,
                ) -> ::subxt::tx::StaticTxPayload<Submit> {
                    ::subxt::tx::StaticTxPayload::new(
                        "ElectionProviderMultiPhase",
                        "submit",
                        Submit {
                            raw_solution: ::std::boxed::Box::new(raw_solution),
                        },
                        [
                            220u8, 167u8, 40u8, 47u8, 253u8, 244u8, 72u8,
                            124u8, 30u8, 123u8, 127u8, 227u8, 2u8, 66u8, 119u8,
                            64u8, 211u8, 200u8, 210u8, 98u8, 248u8, 132u8,
                            68u8, 25u8, 34u8, 182u8, 230u8, 225u8, 241u8, 58u8,
                            193u8, 134u8,
                        ],
                    )
                }
                #[doc = "Trigger the governance fallback."]
                #[doc = ""]
                #[doc = "This can only be called when [`Phase::Emergency`] is enabled, as an alternative to"]
                #[doc = "calling [`Call::set_emergency_election_result`]."]
                pub fn governance_fallback(
                    &self,
                    maybe_max_voters: ::core::option::Option<
                        ::core::primitive::u32,
                    >,
                    maybe_max_targets: ::core::option::Option<
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<GovernanceFallback>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "ElectionProviderMultiPhase",
                        "governance_fallback",
                        GovernanceFallback {
                            maybe_max_voters,
                            maybe_max_targets,
                        },
                        [
                            206u8, 247u8, 76u8, 85u8, 7u8, 24u8, 231u8, 226u8,
                            192u8, 143u8, 43u8, 67u8, 91u8, 202u8, 88u8, 176u8,
                            130u8, 1u8, 83u8, 229u8, 227u8, 200u8, 179u8, 4u8,
                            113u8, 60u8, 99u8, 190u8, 53u8, 226u8, 142u8,
                            182u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event =
            runtime_types::pallet_election_provider_multi_phase::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A solution was stored with the given compute."]
            #[doc = ""]
            #[doc = "If the solution is signed, this means that it hasn't yet been processed. If the"]
            #[doc = "solution is unsigned, this means that it has also been processed."]
            #[doc = ""]
            #[doc = "The `bool` is `true` when a previous solution was ejected to make room for this one."]
            pub struct SolutionStored { pub compute : runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , pub prev_ejected : :: core :: primitive :: bool , }
            impl ::subxt::events::StaticEvent for SolutionStored {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SolutionStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The election has been finalized, with the given computation and score."]
            pub struct ElectionFinalized { pub compute : runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , pub score : runtime_types :: sp_npos_elections :: ElectionScore , }
            impl ::subxt::events::StaticEvent for ElectionFinalized {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "ElectionFinalized";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An election failed."]
            #[doc = ""]
            #[doc = "Not much can be said about which computes failed in the process."]
            pub struct ElectionFailed;
            impl ::subxt::events::StaticEvent for ElectionFailed {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "ElectionFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An account has been rewarded for their signed submission being finalized."]
            pub struct Rewarded {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Rewarded {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An account has been slashed for submitting an invalid signed submission."]
            pub struct Slashed {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The signed phase of the given round has started."]
            pub struct SignedPhaseStarted {
                pub round: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for SignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SignedPhaseStarted";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The unsigned phase of the given round has started."]
            pub struct UnsignedPhaseStarted {
                pub round: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for UnsignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "UnsignedPhaseStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Internal counter for the number of rounds."]
                #[doc = ""]
                #[doc = " This is useful for de-duplication of transactions submitted to the pool, and general"]
                #[doc = " diagnostics of the pallet."]
                #[doc = ""]
                #[doc = " This is merely incremented once per every time that an upstream `elect` is called."]                pub fn round (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ElectionProviderMultiPhase",
                        "Round",
                        vec![],
                        [
                            16u8, 49u8, 176u8, 52u8, 202u8, 111u8, 120u8, 8u8,
                            217u8, 96u8, 35u8, 14u8, 233u8, 130u8, 47u8, 98u8,
                            34u8, 44u8, 166u8, 188u8, 199u8, 210u8, 21u8, 19u8,
                            70u8, 96u8, 139u8, 8u8, 53u8, 82u8, 165u8, 239u8,
                        ],
                    )
                }
                #[doc = " Current phase."]                pub fn current_phase (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_election_provider_multi_phase :: Phase < :: core :: primitive :: u32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ElectionProviderMultiPhase",
                        "CurrentPhase",
                        vec![],
                        [
                            236u8, 101u8, 8u8, 52u8, 68u8, 240u8, 74u8, 159u8,
                            181u8, 53u8, 153u8, 101u8, 228u8, 81u8, 96u8,
                            161u8, 34u8, 67u8, 35u8, 28u8, 121u8, 44u8, 229u8,
                            45u8, 196u8, 87u8, 73u8, 125u8, 216u8, 245u8,
                            255u8, 15u8,
                        ],
                    )
                }
                #[doc = " Current best solution, signed or unsigned, queued to be returned upon `elect`."]                pub fn queued_solution (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_election_provider_multi_phase :: ReadySolution > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ElectionProviderMultiPhase",
                        "QueuedSolution",
                        vec![],
                        [
                            11u8, 152u8, 13u8, 167u8, 204u8, 209u8, 171u8,
                            249u8, 59u8, 250u8, 58u8, 152u8, 164u8, 121u8,
                            146u8, 112u8, 241u8, 16u8, 159u8, 251u8, 209u8,
                            251u8, 114u8, 29u8, 188u8, 30u8, 84u8, 71u8, 136u8,
                            173u8, 145u8, 236u8,
                        ],
                    )
                }
                #[doc = " Snapshot data of the round."]
                #[doc = ""]
                #[doc = " This is created at the beginning of the signed phase and cleared upon calling `elect`."]                pub fn snapshot (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_election_provider_multi_phase :: RoundSnapshot > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ElectionProviderMultiPhase",
                        "Snapshot",
                        vec![],
                        [
                            239u8, 56u8, 191u8, 77u8, 150u8, 224u8, 248u8,
                            88u8, 132u8, 224u8, 164u8, 83u8, 253u8, 36u8, 46u8,
                            156u8, 72u8, 152u8, 36u8, 206u8, 72u8, 27u8, 226u8,
                            87u8, 146u8, 220u8, 93u8, 178u8, 26u8, 115u8,
                            232u8, 71u8,
                        ],
                    )
                }
                #[doc = " Desired number of targets to elect for this round."]
                #[doc = ""]
                #[doc = " Only exists when [`Snapshot`] is present."]                pub fn desired_targets (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ElectionProviderMultiPhase",
                        "DesiredTargets",
                        vec![],
                        [
                            16u8, 247u8, 4u8, 181u8, 93u8, 79u8, 12u8, 212u8,
                            146u8, 167u8, 80u8, 58u8, 118u8, 52u8, 68u8, 87u8,
                            90u8, 140u8, 31u8, 210u8, 2u8, 116u8, 220u8, 231u8,
                            115u8, 112u8, 118u8, 118u8, 68u8, 34u8, 151u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The metadata of the [`RoundSnapshot`]"]
                #[doc = ""]
                #[doc = " Only exists when [`Snapshot`] is present."]                pub fn snapshot_metadata (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ElectionProviderMultiPhase",
                        "SnapshotMetadata",
                        vec![],
                        [
                            135u8, 122u8, 60u8, 75u8, 194u8, 240u8, 187u8,
                            96u8, 240u8, 203u8, 192u8, 22u8, 117u8, 148u8,
                            118u8, 24u8, 240u8, 213u8, 94u8, 22u8, 194u8, 47u8,
                            181u8, 245u8, 77u8, 149u8, 11u8, 251u8, 117u8,
                            220u8, 205u8, 78u8,
                        ],
                    )
                }
                #[doc = " The next index to be assigned to an incoming signed submission."]
                #[doc = ""]
                #[doc = " Every accepted submission is assigned a unique index; that index is bound to that particular"]
                #[doc = " submission for the duration of the election. On election finalization, the next index is"]
                #[doc = " reset to 0."]
                #[doc = ""]
                #[doc = " We can't just use `SignedSubmissionIndices.len()`, because that's a bounded set; past its"]
                #[doc = " capacity, it will simply saturate. We can't just iterate over `SignedSubmissionsMap`,"]
                #[doc = " because iteration is slow. Instead, we store the value here."]                pub fn signed_submission_next_index (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedSubmissionNextIndex",
                        vec![],
                        [
                            242u8, 11u8, 157u8, 105u8, 96u8, 7u8, 31u8, 20u8,
                            51u8, 141u8, 182u8, 180u8, 13u8, 172u8, 155u8,
                            59u8, 42u8, 238u8, 115u8, 8u8, 6u8, 137u8, 45u8,
                            2u8, 123u8, 187u8, 53u8, 215u8, 19u8, 129u8, 54u8,
                            22u8,
                        ],
                    )
                }
                #[doc = " A sorted, bounded vector of `(score, block_number, index)`, where each `index` points to a"]
                #[doc = " value in `SignedSubmissions`."]
                #[doc = ""]
                #[doc = " We never need to process more than a single signed submission at a time. Signed submissions"]
                #[doc = " can be quite large, so we're willing to pay the cost of multiple database accesses to access"]
                #[doc = " them one at a time instead of reading and decoding all of them at once."]                pub fn signed_submission_indices (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < (runtime_types :: sp_npos_elections :: ElectionScore , :: core :: primitive :: u32 , :: core :: primitive :: u32 ,) > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedSubmissionIndices",
                        vec![],
                        [
                            228u8, 166u8, 94u8, 248u8, 71u8, 26u8, 125u8, 81u8,
                            32u8, 22u8, 46u8, 185u8, 209u8, 123u8, 46u8, 17u8,
                            152u8, 149u8, 222u8, 125u8, 112u8, 230u8, 29u8,
                            177u8, 162u8, 214u8, 66u8, 38u8, 170u8, 121u8,
                            129u8, 100u8,
                        ],
                    )
                }
                #[doc = " Unchecked, signed solutions."]
                #[doc = ""]
                #[doc = " Together with `SubmissionIndices`, this stores a bounded set of `SignedSubmissions` while"]
                #[doc = " allowing us to keep only a single one in memory at a time."]
                #[doc = ""]
                #[doc = " Twox note: the key of the map is an auto-incrementing index which users cannot inspect or"]
                #[doc = " affect; we shouldn't need a cryptographically secure hasher."]                pub fn signed_submissions_map (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("ElectionProviderMultiPhase" , "SignedSubmissionsMap" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [84u8 , 65u8 , 205u8 , 191u8 , 143u8 , 246u8 , 239u8 , 27u8 , 243u8 , 54u8 , 250u8 , 8u8 , 125u8 , 32u8 , 241u8 , 141u8 , 210u8 , 225u8 , 56u8 , 101u8 , 241u8 , 52u8 , 157u8 , 29u8 , 13u8 , 155u8 , 73u8 , 132u8 , 118u8 , 53u8 , 2u8 , 135u8 ,])
                }
                #[doc = " Unchecked, signed solutions."]
                #[doc = ""]
                #[doc = " Together with `SubmissionIndices`, this stores a bounded set of `SignedSubmissions` while"]
                #[doc = " allowing us to keep only a single one in memory at a time."]
                #[doc = ""]
                #[doc = " Twox note: the key of the map is an auto-incrementing index which users cannot inspect or"]
                #[doc = " affect; we shouldn't need a cryptographically secure hasher."]                pub fn signed_submissions_map_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedSubmissionsMap",
                        Vec::new(),
                        [
                            84u8, 65u8, 205u8, 191u8, 143u8, 246u8, 239u8,
                            27u8, 243u8, 54u8, 250u8, 8u8, 125u8, 32u8, 241u8,
                            141u8, 210u8, 225u8, 56u8, 101u8, 241u8, 52u8,
                            157u8, 29u8, 13u8, 155u8, 73u8, 132u8, 118u8, 53u8,
                            2u8, 135u8,
                        ],
                    )
                }
                #[doc = " The minimum score that each 'untrusted' solution must attain in order to be considered"]
                #[doc = " feasible."]
                #[doc = ""]
                #[doc = " Can be set via `set_minimum_untrusted_score`."]                pub fn minimum_untrusted_score (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_npos_elections :: ElectionScore > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ElectionProviderMultiPhase",
                        "MinimumUntrustedScore",
                        vec![],
                        [
                            77u8, 235u8, 181u8, 45u8, 230u8, 12u8, 0u8, 179u8,
                            152u8, 38u8, 74u8, 199u8, 47u8, 84u8, 85u8, 55u8,
                            171u8, 226u8, 217u8, 125u8, 17u8, 194u8, 95u8,
                            157u8, 73u8, 245u8, 75u8, 130u8, 248u8, 7u8, 53u8,
                            226u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Duration of the unsigned phase."]
                pub fn unsigned_phase(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "UnsignedPhase",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Duration of the signed phase."]
                pub fn signed_phase(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedPhase",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The minimum amount of improvement to the solution score that defines a solution as"]
                #[doc = " \"better\" in the Signed phase."]
                pub fn better_signed_threshold(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "BetterSignedThreshold",
                        [
                            225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8,
                            192u8, 254u8, 19u8, 87u8, 80u8, 16u8, 62u8, 42u8,
                            204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
                            177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8,
                            86u8, 227u8,
                        ],
                    )
                }
                #[doc = " The minimum amount of improvement to the solution score that defines a solution as"]
                #[doc = " \"better\" in the Unsigned phase."]
                pub fn better_unsigned_threshold(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "BetterUnsignedThreshold",
                        [
                            225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8,
                            192u8, 254u8, 19u8, 87u8, 80u8, 16u8, 62u8, 42u8,
                            204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
                            177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8,
                            86u8, 227u8,
                        ],
                    )
                }
                #[doc = " The repeat threshold of the offchain worker."]
                #[doc = ""]
                #[doc = " For example, if it is 5, that means that at least 5 blocks will elapse between attempts"]
                #[doc = " to submit the worker's solution."]
                pub fn offchain_repeat(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "OffchainRepeat",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The priority of the unsigned transaction submitted in the unsigned-phase"]
                pub fn miner_tx_priority(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "MinerTxPriority",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8,
                            231u8, 190u8, 146u8, 59u8, 226u8, 157u8, 101u8,
                            103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8,
                            119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8,
                            239u8, 42u8, 246u8,
                        ],
                    )
                }
                #[doc = " Maximum number of signed submissions that can be queued."]
                #[doc = ""]
                #[doc = " It is best to avoid adjusting this during an election, as it impacts downstream data"]
                #[doc = " structures. In particular, `SignedSubmissionIndices<T>` is bounded on this value. If you"]
                #[doc = " update this value during an election, you _must_ ensure that"]
                #[doc = " `SignedSubmissionIndices.len()` is less than or equal to the new value. Otherwise,"]
                #[doc = " attempts to submit new solutions may cause a runtime panic."]
                pub fn signed_max_submissions(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedMaxSubmissions",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maximum weight of a signed solution."]
                #[doc = ""]
                #[doc = " If [`Config::MinerConfig`] is being implemented to submit signed solutions (outside of"]
                #[doc = " this pallet), then [`MinerConfig::solution_weight`] is used to compare against"]
                #[doc = " this value."]
                pub fn signed_max_weight(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedMaxWeight",
                        [
                            206u8, 61u8, 253u8, 247u8, 163u8, 40u8, 161u8,
                            52u8, 134u8, 140u8, 206u8, 83u8, 44u8, 166u8,
                            226u8, 115u8, 181u8, 14u8, 227u8, 130u8, 210u8,
                            32u8, 85u8, 29u8, 230u8, 97u8, 130u8, 165u8, 147u8,
                            134u8, 106u8, 76u8,
                        ],
                    )
                }
                #[doc = " The maximum amount of unchecked solutions to refund the call fee for."]
                pub fn signed_max_refunds(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedMaxRefunds",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Base reward for a signed solution"]
                pub fn signed_reward_base(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedRewardBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8,
                            105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8,
                            160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
                            71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8,
                            148u8, 156u8,
                        ],
                    )
                }
                #[doc = " Base deposit for a signed solution."]
                pub fn signed_deposit_base(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedDepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8,
                            105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8,
                            160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
                            71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8,
                            148u8, 156u8,
                        ],
                    )
                }
                #[doc = " Per-byte deposit for a signed solution."]
                pub fn signed_deposit_byte(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedDepositByte",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8,
                            105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8,
                            160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
                            71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8,
                            148u8, 156u8,
                        ],
                    )
                }
                #[doc = " Per-weight deposit for a signed solution."]
                pub fn signed_deposit_weight(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "SignedDepositWeight",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8,
                            105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8,
                            160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
                            71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8,
                            148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of electing voters to put in the snapshot. At the moment, snapshots"]
                #[doc = " are only over a single block, but once multi-block elections are introduced they will"]
                #[doc = " take place over multiple blocks."]
                pub fn max_electing_voters(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "MaxElectingVoters",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of electable targets to put in the snapshot."]
                pub fn max_electable_targets(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "MaxElectableTargets",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8,
                            169u8, 167u8, 227u8, 41u8, 144u8, 11u8, 236u8,
                            82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8, 90u8,
                            208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8,
                            193u8, 29u8, 70u8,
                        ],
                    )
                }
                #[doc = " The maximum number of winners that can be elected by this `ElectionProvider`"]
                #[doc = " implementation."]
                #[doc = ""]
                #[doc = " Note: This must always be greater or equal to `T::DataProvider::desired_targets()`."]
                pub fn max_winners(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "MaxWinners",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn miner_max_length(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "MinerMaxLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn miner_max_weight(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "MinerMaxWeight",
                        [
                            206u8, 61u8, 253u8, 247u8, 163u8, 40u8, 161u8,
                            52u8, 134u8, 140u8, 206u8, 83u8, 44u8, 166u8,
                            226u8, 115u8, 181u8, 14u8, 227u8, 130u8, 210u8,
                            32u8, 85u8, 29u8, 230u8, 97u8, 130u8, 165u8, 147u8,
                            134u8, 106u8, 76u8,
                        ],
                    )
                }
                pub fn miner_max_votes_per_voter(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ElectionProviderMultiPhase",
                        "MinerMaxVotesPerVoter",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod bags_list {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Rebag {
                pub dislocated: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct PutInFrontOf {
                pub lighter: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Declare that some `dislocated` account has, through rewards or penalties, sufficiently"]
                #[doc = "changed its score that it should properly fall into a different bag than its current"]
                #[doc = "one."]
                #[doc = ""]
                #[doc = "Anyone can call this function about any potentially dislocated account."]
                #[doc = ""]
                #[doc = "Will always update the stored score of `dislocated` to the correct score, based on"]
                #[doc = "`ScoreProvider`."]
                #[doc = ""]
                #[doc = "If `dislocated` does not exists, it returns an error."]
                pub fn rebag(
                    &self,
                    dislocated: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Rebag> {
                    ::subxt::tx::StaticTxPayload::new(
                        "BagsList",
                        "rebag",
                        Rebag { dislocated },
                        [
                            0u8, 168u8, 218u8, 188u8, 236u8, 124u8, 250u8,
                            201u8, 237u8, 20u8, 97u8, 150u8, 117u8, 232u8,
                            116u8, 237u8, 55u8, 151u8, 71u8, 119u8, 42u8, 48u8,
                            10u8, 66u8, 167u8, 208u8, 184u8, 228u8, 146u8,
                            181u8, 84u8, 70u8,
                        ],
                    )
                }
                #[doc = "Move the caller's Id directly in front of `lighter`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and can only be called by the Id of"]
                #[doc = "the account going in front of `lighter`."]
                #[doc = ""]
                #[doc = "Only works if"]
                #[doc = "- both nodes are within the same bag,"]
                #[doc = "- and `origin` has a greater `Score` than `lighter`."]
                pub fn put_in_front_of(
                    &self,
                    lighter: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<PutInFrontOf>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "BagsList",
                        "put_in_front_of",
                        PutInFrontOf { lighter },
                        [
                            104u8, 36u8, 96u8, 80u8, 236u8, 75u8, 203u8, 232u8,
                            136u8, 167u8, 205u8, 143u8, 200u8, 53u8, 124u8,
                            148u8, 76u8, 246u8, 71u8, 246u8, 205u8, 82u8, 32u8,
                            186u8, 33u8, 5u8, 183u8, 127u8, 153u8, 232u8, 80u8,
                            164u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_bags_list::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Moved an account from one bag to another."]
            pub struct Rebagged {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub from: ::core::primitive::u64,
                pub to: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for Rebagged {
                const PALLET: &'static str = "BagsList";
                const EVENT: &'static str = "Rebagged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "Updated the score of some account to the given amount."]
            pub struct ScoreUpdated {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub new_score: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for ScoreUpdated {
                const PALLET: &'static str = "BagsList";
                const EVENT: &'static str = "ScoreUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " A single node, within some bag."]
                #[doc = ""]
                #[doc = " Nodes store links forward and back within their respective bags."]                pub fn list_nodes (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_bags_list :: list :: Node > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("BagsList" , "ListNodes" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [176u8 , 186u8 , 93u8 , 51u8 , 100u8 , 184u8 , 240u8 , 29u8 , 70u8 , 3u8 , 117u8 , 47u8 , 23u8 , 66u8 , 231u8 , 234u8 , 53u8 , 8u8 , 234u8 , 175u8 , 181u8 , 8u8 , 161u8 , 154u8 , 48u8 , 178u8 , 147u8 , 227u8 , 122u8 , 115u8 , 57u8 , 97u8 ,])
                }
                #[doc = " A single node, within some bag."]
                #[doc = ""]
                #[doc = " Nodes store links forward and back within their respective bags."]                pub fn list_nodes_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_bags_list :: list :: Node > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "BagsList",
                        "ListNodes",
                        Vec::new(),
                        [
                            176u8, 186u8, 93u8, 51u8, 100u8, 184u8, 240u8,
                            29u8, 70u8, 3u8, 117u8, 47u8, 23u8, 66u8, 231u8,
                            234u8, 53u8, 8u8, 234u8, 175u8, 181u8, 8u8, 161u8,
                            154u8, 48u8, 178u8, 147u8, 227u8, 122u8, 115u8,
                            57u8, 97u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]                pub fn counter_for_list_nodes (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "BagsList",
                        "CounterForListNodes",
                        vec![],
                        [
                            156u8, 168u8, 97u8, 33u8, 84u8, 117u8, 220u8, 89u8,
                            62u8, 182u8, 24u8, 88u8, 231u8, 244u8, 41u8, 19u8,
                            210u8, 131u8, 87u8, 0u8, 241u8, 230u8, 160u8,
                            142u8, 128u8, 153u8, 83u8, 36u8, 88u8, 247u8, 70u8,
                            130u8,
                        ],
                    )
                }
                #[doc = " A bag stored in storage."]
                #[doc = ""]
                #[doc = " Stores a `Bag` struct, which stores head and tail pointers to itself."]                pub fn list_bags (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u64 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_bags_list :: list :: Bag > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("BagsList" , "ListBags" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [38u8 , 86u8 , 63u8 , 92u8 , 85u8 , 59u8 , 225u8 , 244u8 , 14u8 , 155u8 , 76u8 , 249u8 , 153u8 , 140u8 , 179u8 , 7u8 , 96u8 , 170u8 , 236u8 , 179u8 , 4u8 , 18u8 , 232u8 , 146u8 , 216u8 , 51u8 , 135u8 , 116u8 , 196u8 , 117u8 , 143u8 , 153u8 ,])
                }
                #[doc = " A bag stored in storage."]
                #[doc = ""]
                #[doc = " Stores a `Bag` struct, which stores head and tail pointers to itself."]                pub fn list_bags_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_bags_list :: list :: Bag > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "BagsList",
                        "ListBags",
                        Vec::new(),
                        [
                            38u8, 86u8, 63u8, 92u8, 85u8, 59u8, 225u8, 244u8,
                            14u8, 155u8, 76u8, 249u8, 153u8, 140u8, 179u8, 7u8,
                            96u8, 170u8, 236u8, 179u8, 4u8, 18u8, 232u8, 146u8,
                            216u8, 51u8, 135u8, 116u8, 196u8, 117u8, 143u8,
                            153u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The list of thresholds separating the various bags."]
                #[doc = ""]
                #[doc = " Ids are separated into unsorted bags according to their score. This specifies the"]
                #[doc = " thresholds separating the bags. An id's bag is the largest bag for which the id's score"]
                #[doc = " is less than or equal to its upper threshold."]
                #[doc = ""]
                #[doc = " When ids are iterated, higher bags are iterated completely before lower bags. This means"]
                #[doc = " that iteration is _semi-sorted_: ids of higher score tend to come before ids of lower"]
                #[doc = " score, but peer ids within a particular bag are sorted in insertion order."]
                #[doc = ""]
                #[doc = " # Expressing the constant"]
                #[doc = ""]
                #[doc = " This constant must be sorted in strictly increasing order. Duplicate items are not"]
                #[doc = " permitted."]
                #[doc = ""]
                #[doc = " There is an implied upper limit of `Score::MAX`; that value does not need to be"]
                #[doc = " specified within the bag. For any two threshold lists, if one ends with"]
                #[doc = " `Score::MAX`, the other one does not, and they are otherwise equal, the two"]
                #[doc = " lists will behave identically."]
                #[doc = ""]
                #[doc = " # Calculation"]
                #[doc = ""]
                #[doc = " It is recommended to generate the set of thresholds in a geometric series, such that"]
                #[doc = " there exists some constant ratio such that `threshold[k + 1] == (threshold[k] *"]
                #[doc = " constant_ratio).max(threshold[k] + 1)` for all `k`."]
                #[doc = ""]
                #[doc = " The helpers in the `/utils/frame/generate-bags` module can simplify this calculation."]
                #[doc = ""]
                #[doc = " # Examples"]
                #[doc = ""]
                #[doc = " - If `BagThresholds::get().is_empty()`, then all ids are put into the same bag, and"]
                #[doc = "   iteration is strictly in insertion order."]
                #[doc = " - If `BagThresholds::get().len() == 64`, and the thresholds are determined according to"]
                #[doc = "   the procedure given above, then the constant ratio is equal to 2."]
                #[doc = " - If `BagThresholds::get().len() == 200`, and the thresholds are determined according to"]
                #[doc = "   the procedure given above, then the constant ratio is approximately equal to 1.248."]
                #[doc = " - If the threshold list begins `[1, 2, 3, ...]`, then an id with score 0 or 1 will fall"]
                #[doc = "   into bag 0, an id with score 2 will fall into bag 1, etc."]
                #[doc = ""]
                #[doc = " # Migration"]
                #[doc = ""]
                #[doc = " In the event that this list ever changes, a copy of the old bags list must be retained."]
                #[doc = " With that `List::migrate` can be called, which will perform the appropriate migration."]
                pub fn bag_thresholds(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::core::primitive::u64>,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "BagsList",
                        "BagThresholds",
                        [
                            103u8, 102u8, 255u8, 165u8, 124u8, 54u8, 5u8,
                            172u8, 112u8, 234u8, 25u8, 175u8, 178u8, 19u8,
                            251u8, 73u8, 91u8, 192u8, 227u8, 81u8, 249u8, 45u8,
                            126u8, 116u8, 7u8, 37u8, 9u8, 200u8, 167u8, 182u8,
                            12u8, 131u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod nomination_pools {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Join {
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
                pub pool_id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct BondExtra {
                pub extra: runtime_types::pallet_nomination_pools::BondExtra<
                    ::core::primitive::u128,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ClaimPayout;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Unbond {
                pub member_account: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub unbonding_points: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct PoolWithdrawUnbonded {
                pub pool_id: ::core::primitive::u32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct WithdrawUnbonded {
                pub member_account: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Create {
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
                pub root: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub nominator: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub state_toggler: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct CreateWithPoolId {
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
                pub root: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub nominator: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub state_toggler: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub pool_id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Nominate {
                pub pool_id: ::core::primitive::u32,
                pub validators:
                    ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetState {
                pub pool_id: ::core::primitive::u32,
                pub state: runtime_types::pallet_nomination_pools::PoolState,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetMetadata {
                pub pool_id: ::core::primitive::u32,
                pub metadata: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetConfigs {
                pub min_join_bond:
                    runtime_types::pallet_nomination_pools::ConfigOp<
                        ::core::primitive::u128,
                    >,
                pub min_create_bond:
                    runtime_types::pallet_nomination_pools::ConfigOp<
                        ::core::primitive::u128,
                    >,
                pub max_pools: runtime_types::pallet_nomination_pools::ConfigOp<
                    ::core::primitive::u32,
                >,
                pub max_members:
                    runtime_types::pallet_nomination_pools::ConfigOp<
                        ::core::primitive::u32,
                    >,
                pub max_members_per_pool:
                    runtime_types::pallet_nomination_pools::ConfigOp<
                        ::core::primitive::u32,
                    >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct UpdateRoles {
                pub pool_id: ::core::primitive::u32,
                pub new_root: runtime_types::pallet_nomination_pools::ConfigOp<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                >,
                pub new_nominator:
                    runtime_types::pallet_nomination_pools::ConfigOp<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                pub new_state_toggler:
                    runtime_types::pallet_nomination_pools::ConfigOp<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Chill {
                pub pool_id: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Stake funds with a pool. The amount to bond is transferred from the member to the"]
                #[doc = "pools account and immediately increases the pools bond."]
                #[doc = ""]
                #[doc = "# Note"]
                #[doc = ""]
                #[doc = "* An account can only be a member of a single pool."]
                #[doc = "* An account cannot join the same pool multiple times."]
                #[doc = "* This call will *not* dust the member account, so the member must have at least"]
                #[doc = "  `existential deposit + amount` in their account."]
                #[doc = "* Only a pool with [`PoolState::Open`] can be joined"]
                pub fn join(
                    &self,
                    amount: ::core::primitive::u128,
                    pool_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Join> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "join",
                        Join { amount, pool_id },
                        [
                            205u8, 66u8, 42u8, 72u8, 146u8, 148u8, 119u8,
                            162u8, 101u8, 183u8, 46u8, 176u8, 221u8, 204u8,
                            197u8, 20u8, 75u8, 226u8, 29u8, 118u8, 208u8, 60u8,
                            192u8, 247u8, 222u8, 100u8, 69u8, 80u8, 172u8,
                            13u8, 69u8, 250u8,
                        ],
                    )
                }
                #[doc = "Bond `extra` more funds from `origin` into the pool to which they already belong."]
                #[doc = ""]
                #[doc = "Additional funds can come from either the free balance of the account, of from the"]
                #[doc = "accumulated rewards, see [`BondExtra`]."]
                #[doc = ""]
                #[doc = "Bonding extra funds implies an automatic payout of all pending rewards as well."]
                pub fn bond_extra(
                    &self,
                    extra: runtime_types::pallet_nomination_pools::BondExtra<
                        ::core::primitive::u128,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<BondExtra> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "bond_extra",
                        BondExtra { extra },
                        [
                            50u8, 72u8, 181u8, 216u8, 249u8, 27u8, 250u8,
                            177u8, 253u8, 22u8, 240u8, 100u8, 184u8, 202u8,
                            197u8, 34u8, 21u8, 188u8, 248u8, 191u8, 11u8, 10u8,
                            236u8, 161u8, 168u8, 37u8, 38u8, 238u8, 61u8,
                            183u8, 86u8, 55u8,
                        ],
                    )
                }
                #[doc = "A bonded member can use this to claim their payout based on the rewards that the pool"]
                #[doc = "has accumulated since their last claimed payout (OR since joining if this is there first"]
                #[doc = "time claiming rewards). The payout will be transferred to the member's account."]
                #[doc = ""]
                #[doc = "The member will earn rewards pro rata based on the members stake vs the sum of the"]
                #[doc = "members in the pools stake. Rewards do not \"expire\"."]
                pub fn claim_payout(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<ClaimPayout> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "claim_payout",
                        ClaimPayout {},
                        [
                            128u8, 58u8, 138u8, 55u8, 64u8, 16u8, 129u8, 25u8,
                            211u8, 229u8, 193u8, 115u8, 47u8, 45u8, 155u8,
                            221u8, 218u8, 1u8, 222u8, 5u8, 236u8, 32u8, 88u8,
                            0u8, 198u8, 72u8, 196u8, 181u8, 104u8, 16u8, 212u8,
                            29u8,
                        ],
                    )
                }
                #[doc = "Unbond up to `unbonding_points` of the `member_account`'s funds from the pool. It"]
                #[doc = "implicitly collects the rewards one last time, since not doing so would mean some"]
                #[doc = "rewards would be forfeited."]
                #[doc = ""]
                #[doc = "Under certain conditions, this call can be dispatched permissionlessly (i.e. by any"]
                #[doc = "account)."]
                #[doc = ""]
                #[doc = "# Conditions for a permissionless dispatch."]
                #[doc = ""]
                #[doc = "* The pool is blocked and the caller is either the root or state-toggler. This is"]
                #[doc = "  refereed to as a kick."]
                #[doc = "* The pool is destroying and the member is not the depositor."]
                #[doc = "* The pool is destroying, the member is the depositor and no other members are in the"]
                #[doc = "  pool."]
                #[doc = ""]
                #[doc = "## Conditions for permissioned dispatch (i.e. the caller is also the"]
                #[doc = "`member_account`):"]
                #[doc = ""]
                #[doc = "* The caller is not the depositor."]
                #[doc = "* The caller is the depositor, the pool is destroying and no other members are in the"]
                #[doc = "  pool."]
                #[doc = ""]
                #[doc = "# Note"]
                #[doc = ""]
                #[doc = "If there are too many unlocking chunks to unbond with the pool account,"]
                #[doc = "[`Call::pool_withdraw_unbonded`] can be called to try and minimize unlocking chunks."]
                #[doc = "The [`StakingInterface::unbond`] will implicitly call [`Call::pool_withdraw_unbonded`]"]
                #[doc = "to try to free chunks if necessary (ie. if unbound was called and no unlocking chunks"]
                #[doc = "are available). However, it may not be possible to release the current unlocking chunks,"]
                #[doc = "in which case, the result of this call will likely be the `NoMoreChunks` error from the"]
                #[doc = "staking system."]
                pub fn unbond(
                    &self,
                    member_account: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    unbonding_points: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Unbond> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "unbond",
                        Unbond {
                            member_account,
                            unbonding_points,
                        },
                        [
                            139u8, 71u8, 78u8, 184u8, 141u8, 89u8, 179u8,
                            123u8, 153u8, 30u8, 116u8, 186u8, 148u8, 49u8,
                            48u8, 98u8, 33u8, 21u8, 29u8, 106u8, 180u8, 212u8,
                            37u8, 251u8, 237u8, 21u8, 255u8, 13u8, 236u8, 73u8,
                            250u8, 57u8,
                        ],
                    )
                }
                #[doc = "Call `withdraw_unbonded` for the pools account. This call can be made by any account."]
                #[doc = ""]
                #[doc = "This is useful if their are too many unlocking chunks to call `unbond`, and some"]
                #[doc = "can be cleared by withdrawing. In the case there are too many unlocking chunks, the user"]
                #[doc = "would probably see an error like `NoMoreChunks` emitted from the staking system when"]
                #[doc = "they attempt to unbond."]
                pub fn pool_withdraw_unbonded(
                    &self,
                    pool_id: ::core::primitive::u32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<PoolWithdrawUnbonded>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "pool_withdraw_unbonded",
                        PoolWithdrawUnbonded {
                            pool_id,
                            num_slashing_spans,
                        },
                        [
                            152u8, 245u8, 131u8, 247u8, 106u8, 214u8, 154u8,
                            8u8, 7u8, 210u8, 149u8, 218u8, 118u8, 46u8, 242u8,
                            182u8, 191u8, 119u8, 28u8, 199u8, 36u8, 49u8,
                            219u8, 123u8, 58u8, 203u8, 211u8, 226u8, 217u8,
                            36u8, 56u8, 0u8,
                        ],
                    )
                }
                #[doc = "Withdraw unbonded funds from `member_account`. If no bonded funds can be unbonded, an"]
                #[doc = "error is returned."]
                #[doc = ""]
                #[doc = "Under certain conditions, this call can be dispatched permissionlessly (i.e. by any"]
                #[doc = "account)."]
                #[doc = ""]
                #[doc = "# Conditions for a permissionless dispatch"]
                #[doc = ""]
                #[doc = "* The pool is in destroy mode and the target is not the depositor."]
                #[doc = "* The target is the depositor and they are the only member in the sub pools."]
                #[doc = "* The pool is blocked and the caller is either the root or state-toggler."]
                #[doc = ""]
                #[doc = "# Conditions for permissioned dispatch"]
                #[doc = ""]
                #[doc = "* The caller is the target and they are not the depositor."]
                #[doc = ""]
                #[doc = "# Note"]
                #[doc = ""]
                #[doc = "If the target is the depositor, the pool will be destroyed."]
                pub fn withdraw_unbonded(
                    &self,
                    member_account: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<WithdrawUnbonded>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "withdraw_unbonded",
                        WithdrawUnbonded {
                            member_account,
                            num_slashing_spans,
                        },
                        [
                            192u8, 183u8, 121u8, 87u8, 176u8, 70u8, 91u8,
                            226u8, 156u8, 79u8, 87u8, 34u8, 227u8, 84u8, 22u8,
                            235u8, 3u8, 181u8, 166u8, 194u8, 147u8, 72u8, 27u8,
                            221u8, 57u8, 14u8, 44u8, 70u8, 253u8, 236u8, 44u8,
                            84u8,
                        ],
                    )
                }
                #[doc = "Create a new delegation pool."]
                #[doc = ""]
                #[doc = "# Arguments"]
                #[doc = ""]
                #[doc = "* `amount` - The amount of funds to delegate to the pool. This also acts of a sort of"]
                #[doc = "  deposit since the pools creator cannot fully unbond funds until the pool is being"]
                #[doc = "  destroyed."]
                #[doc = "* `index` - A disambiguation index for creating the account. Likely only useful when"]
                #[doc = "  creating multiple pools in the same extrinsic."]
                #[doc = "* `root` - The account to set as [`PoolRoles::root`]."]
                #[doc = "* `nominator` - The account to set as the [`PoolRoles::nominator`]."]
                #[doc = "* `state_toggler` - The account to set as the [`PoolRoles::state_toggler`]."]
                #[doc = ""]
                #[doc = "# Note"]
                #[doc = ""]
                #[doc = "In addition to `amount`, the caller will transfer the existential deposit; so the caller"]
                #[doc = "needs at have at least `amount + existential_deposit` transferrable."]
                pub fn create(
                    &self,
                    amount: ::core::primitive::u128,
                    root: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    nominator: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    state_toggler: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Create> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "create",
                        Create {
                            amount,
                            root,
                            nominator,
                            state_toggler,
                        },
                        [
                            73u8, 99u8, 205u8, 59u8, 21u8, 182u8, 163u8, 158u8,
                            99u8, 182u8, 182u8, 63u8, 212u8, 84u8, 48u8, 244u8,
                            95u8, 153u8, 86u8, 104u8, 92u8, 93u8, 191u8, 79u8,
                            163u8, 123u8, 20u8, 121u8, 241u8, 194u8, 79u8,
                            112u8,
                        ],
                    )
                }
                #[doc = "Create a new delegation pool with a previously used pool id"]
                #[doc = ""]
                #[doc = "# Arguments"]
                #[doc = ""]
                #[doc = "same as `create` with the inclusion of"]
                #[doc = "* `pool_id` - `A valid PoolId."]
                pub fn create_with_pool_id(
                    &self,
                    amount: ::core::primitive::u128,
                    root: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    nominator: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    state_toggler: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    pool_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<CreateWithPoolId>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "create_with_pool_id",
                        CreateWithPoolId {
                            amount,
                            root,
                            nominator,
                            state_toggler,
                            pool_id,
                        },
                        [
                            113u8, 67u8, 242u8, 175u8, 174u8, 4u8, 116u8, 44u8,
                            157u8, 35u8, 13u8, 23u8, 55u8, 80u8, 255u8, 103u8,
                            196u8, 247u8, 105u8, 152u8, 108u8, 145u8, 180u8,
                            169u8, 252u8, 159u8, 175u8, 241u8, 122u8, 136u8,
                            252u8, 95u8,
                        ],
                    )
                }
                #[doc = "Nominate on behalf of the pool."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be signed by the pool nominator or the pool"]
                #[doc = "root role."]
                #[doc = ""]
                #[doc = "This directly forward the call to the staking pallet, on behalf of the pool bonded"]
                #[doc = "account."]
                pub fn nominate(
                    &self,
                    pool_id: ::core::primitive::u32,
                    validators: ::std::vec::Vec<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Nominate> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "nominate",
                        Nominate {
                            pool_id,
                            validators,
                        },
                        [
                            10u8, 235u8, 64u8, 157u8, 36u8, 249u8, 186u8, 27u8,
                            79u8, 172u8, 25u8, 3u8, 203u8, 19u8, 192u8, 182u8,
                            36u8, 103u8, 13u8, 20u8, 89u8, 140u8, 159u8, 4u8,
                            132u8, 242u8, 192u8, 146u8, 55u8, 251u8, 216u8,
                            255u8,
                        ],
                    )
                }
                #[doc = "Set a new state for the pool."]
                #[doc = ""]
                #[doc = "If a pool is already in the `Destroying` state, then under no condition can its state"]
                #[doc = "change again."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be either:"]
                #[doc = ""]
                #[doc = "1. signed by the state toggler, or the root role of the pool,"]
                #[doc = "2. if the pool conditions to be open are NOT met (as described by `ok_to_be_open`), and"]
                #[doc = "   then the state of the pool can be permissionlessly changed to `Destroying`."]
                pub fn set_state(
                    &self,
                    pool_id: ::core::primitive::u32,
                    state: runtime_types::pallet_nomination_pools::PoolState,
                ) -> ::subxt::tx::StaticTxPayload<SetState> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "set_state",
                        SetState { pool_id, state },
                        [
                            104u8, 40u8, 213u8, 88u8, 159u8, 115u8, 35u8,
                            249u8, 78u8, 180u8, 99u8, 1u8, 225u8, 218u8, 192u8,
                            151u8, 25u8, 194u8, 192u8, 187u8, 39u8, 170u8,
                            212u8, 125u8, 75u8, 250u8, 248u8, 175u8, 159u8,
                            161u8, 151u8, 162u8,
                        ],
                    )
                }
                #[doc = "Set a new metadata for the pool."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be signed by the state toggler, or the root role"]
                #[doc = "of the pool."]
                pub fn set_metadata(
                    &self,
                    pool_id: ::core::primitive::u32,
                    metadata: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "set_metadata",
                        SetMetadata { pool_id, metadata },
                        [
                            156u8, 81u8, 170u8, 161u8, 34u8, 100u8, 183u8,
                            174u8, 5u8, 81u8, 31u8, 76u8, 12u8, 42u8, 77u8,
                            1u8, 6u8, 26u8, 168u8, 7u8, 8u8, 115u8, 158u8,
                            151u8, 30u8, 211u8, 52u8, 177u8, 234u8, 87u8,
                            125u8, 127u8,
                        ],
                    )
                }
                #[doc = "Update configurations for the nomination pools. The origin for this call must be"]
                #[doc = "Root."]
                #[doc = ""]
                #[doc = "# Arguments"]
                #[doc = ""]
                #[doc = "* `min_join_bond` - Set [`MinJoinBond`]."]
                #[doc = "* `min_create_bond` - Set [`MinCreateBond`]."]
                #[doc = "* `max_pools` - Set [`MaxPools`]."]
                #[doc = "* `max_members` - Set [`MaxPoolMembers`]."]
                #[doc = "* `max_members_per_pool` - Set [`MaxPoolMembersPerPool`]."]
                pub fn set_configs(
                    &self,
                    min_join_bond : runtime_types :: pallet_nomination_pools :: ConfigOp < :: core :: primitive :: u128 >,
                    min_create_bond : runtime_types :: pallet_nomination_pools :: ConfigOp < :: core :: primitive :: u128 >,
                    max_pools: runtime_types::pallet_nomination_pools::ConfigOp<
                        ::core::primitive::u32,
                    >,
                    max_members : runtime_types :: pallet_nomination_pools :: ConfigOp < :: core :: primitive :: u32 >,
                    max_members_per_pool : runtime_types :: pallet_nomination_pools :: ConfigOp < :: core :: primitive :: u32 >,
                ) -> ::subxt::tx::StaticTxPayload<SetConfigs> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "set_configs",
                        SetConfigs {
                            min_join_bond,
                            min_create_bond,
                            max_pools,
                            max_members,
                            max_members_per_pool,
                        },
                        [
                            143u8, 196u8, 211u8, 30u8, 71u8, 15u8, 150u8,
                            243u8, 7u8, 178u8, 179u8, 168u8, 40u8, 116u8,
                            220u8, 140u8, 18u8, 206u8, 6u8, 189u8, 190u8, 37u8,
                            68u8, 41u8, 45u8, 233u8, 247u8, 172u8, 185u8, 34u8,
                            243u8, 187u8,
                        ],
                    )
                }
                #[doc = "Update the roles of the pool."]
                #[doc = ""]
                #[doc = "The root is the only entity that can change any of the roles, including itself,"]
                #[doc = "excluding the depositor, who can never change."]
                #[doc = ""]
                #[doc = "It emits an event, notifying UIs of the role change. This event is quite relevant to"]
                #[doc = "most pool members and they should be informed of changes to pool roles."]
                pub fn update_roles(
                    &self,
                    pool_id: ::core::primitive::u32,
                    new_root: runtime_types::pallet_nomination_pools::ConfigOp<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                    new_nominator : runtime_types :: pallet_nomination_pools :: ConfigOp < :: subxt :: ext :: sp_core :: crypto :: AccountId32 >,
                    new_state_toggler : runtime_types :: pallet_nomination_pools :: ConfigOp < :: subxt :: ext :: sp_core :: crypto :: AccountId32 >,
                ) -> ::subxt::tx::StaticTxPayload<UpdateRoles> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "update_roles",
                        UpdateRoles {
                            pool_id,
                            new_root,
                            new_nominator,
                            new_state_toggler,
                        },
                        [
                            247u8, 95u8, 234u8, 56u8, 181u8, 229u8, 158u8,
                            97u8, 69u8, 165u8, 38u8, 17u8, 27u8, 209u8, 204u8,
                            250u8, 91u8, 193u8, 35u8, 93u8, 215u8, 131u8,
                            148u8, 73u8, 67u8, 188u8, 92u8, 32u8, 34u8, 37u8,
                            113u8, 93u8,
                        ],
                    )
                }
                #[doc = "Chill on behalf of the pool."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be signed by the pool nominator or the pool"]
                #[doc = "root role, same as [`Pallet::nominate`]."]
                #[doc = ""]
                #[doc = "This directly forward the call to the staking pallet, on behalf of the pool bonded"]
                #[doc = "account."]
                pub fn chill(
                    &self,
                    pool_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Chill> {
                    ::subxt::tx::StaticTxPayload::new(
                        "NominationPools",
                        "chill",
                        Chill { pool_id },
                        [
                            41u8, 114u8, 128u8, 121u8, 244u8, 15u8, 15u8, 52u8,
                            129u8, 88u8, 239u8, 167u8, 216u8, 38u8, 123u8,
                            240u8, 172u8, 229u8, 132u8, 64u8, 175u8, 87u8,
                            217u8, 27u8, 11u8, 124u8, 1u8, 140u8, 40u8, 191u8,
                            187u8, 36u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Events of this pallet."]
        pub type Event = runtime_types::pallet_nomination_pools::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A pool has been created."]
            pub struct Created {
                pub depositor: ::subxt::ext::sp_core::crypto::AccountId32,
                pub pool_id: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Created {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "Created";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A member has became bonded in a pool."]
            pub struct Bonded {
                pub member: ::subxt::ext::sp_core::crypto::AccountId32,
                pub pool_id: ::core::primitive::u32,
                pub bonded: ::core::primitive::u128,
                pub joined: ::core::primitive::bool,
            }
            impl ::subxt::events::StaticEvent for Bonded {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "Bonded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A payout has been made to a member."]
            pub struct PaidOut {
                pub member: ::subxt::ext::sp_core::crypto::AccountId32,
                pub pool_id: ::core::primitive::u32,
                pub payout: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for PaidOut {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "PaidOut";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A member has unbonded from their pool."]
            #[doc = ""]
            #[doc = "- `balance` is the corresponding balance of the number of points that has been"]
            #[doc = "  requested to be unbonded (the argument of the `unbond` transaction) from the bonded"]
            #[doc = "  pool."]
            #[doc = "- `points` is the number of points that are issued as a result of `balance` being"]
            #[doc = "dissolved into the corresponding unbonding pool."]
            #[doc = "- `era` is the era in which the balance will be unbonded."]
            #[doc = "In the absence of slashing, these values will match. In the presence of slashing, the"]
            #[doc = "number of points that are issued in the unbonding pool will be less than the amount"]
            #[doc = "requested to be unbonded."]
            pub struct Unbonded {
                pub member: ::subxt::ext::sp_core::crypto::AccountId32,
                pub pool_id: ::core::primitive::u32,
                pub balance: ::core::primitive::u128,
                pub points: ::core::primitive::u128,
                pub era: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Unbonded {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "Unbonded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A member has withdrawn from their pool."]
            #[doc = ""]
            #[doc = "The given number of `points` have been dissolved in return of `balance`."]
            #[doc = ""]
            #[doc = "Similar to `Unbonded` event, in the absence of slashing, the ratio of point to balance"]
            #[doc = "will be 1."]
            pub struct Withdrawn {
                pub member: ::subxt::ext::sp_core::crypto::AccountId32,
                pub pool_id: ::core::primitive::u32,
                pub balance: ::core::primitive::u128,
                pub points: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdrawn {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "Withdrawn";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A pool has been destroyed."]
            pub struct Destroyed {
                pub pool_id: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Destroyed {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The state of a pool has changed"]
            pub struct StateChanged {
                pub pool_id: ::core::primitive::u32,
                pub new_state:
                    runtime_types::pallet_nomination_pools::PoolState,
            }
            impl ::subxt::events::StaticEvent for StateChanged {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "StateChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A member has been removed from a pool."]
            #[doc = ""]
            #[doc = "The removal can be voluntary (withdrawn all unbonded funds) or involuntary (kicked)."]
            pub struct MemberRemoved {
                pub pool_id: ::core::primitive::u32,
                pub member: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for MemberRemoved {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "MemberRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The roles of a pool have been updated to the given new roles. Note that the depositor"]
            #[doc = "can never change."]
            pub struct RolesUpdated {
                pub root: ::core::option::Option<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                >,
                pub state_toggler: ::core::option::Option<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                >,
                pub nominator: ::core::option::Option<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::events::StaticEvent for RolesUpdated {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "RolesUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The active balance of pool `pool_id` has been slashed to `balance`."]
            pub struct PoolSlashed {
                pub pool_id: ::core::primitive::u32,
                pub balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for PoolSlashed {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "PoolSlashed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The unbond pool at `era` of pool `pool_id` has been slashed to `balance`."]
            pub struct UnbondingPoolSlashed {
                pub pool_id: ::core::primitive::u32,
                pub era: ::core::primitive::u32,
                pub balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for UnbondingPoolSlashed {
                const PALLET: &'static str = "NominationPools";
                const EVENT: &'static str = "UnbondingPoolSlashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Minimum amount to bond to join a pool."]                pub fn min_join_bond (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "MinJoinBond",
                        vec![],
                        [
                            125u8, 239u8, 45u8, 225u8, 74u8, 129u8, 247u8,
                            184u8, 205u8, 58u8, 45u8, 186u8, 126u8, 170u8,
                            112u8, 120u8, 23u8, 190u8, 247u8, 97u8, 131u8,
                            126u8, 215u8, 44u8, 147u8, 122u8, 132u8, 212u8,
                            217u8, 84u8, 240u8, 91u8,
                        ],
                    )
                }
                #[doc = " Minimum bond required to create a pool."]
                #[doc = ""]
                #[doc = " This is the amount that the depositor must put as their initial stake in the pool, as an"]
                #[doc = " indication of \"skin in the game\"."]
                #[doc = ""]
                #[doc = " This is the value that will always exist in the staking ledger of the pool bonded account"]
                #[doc = " while all other accounts leave."]                pub fn min_create_bond (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "MinCreateBond",
                        vec![],
                        [
                            31u8, 208u8, 240u8, 158u8, 23u8, 218u8, 212u8,
                            138u8, 92u8, 210u8, 207u8, 170u8, 32u8, 60u8, 5u8,
                            21u8, 84u8, 162u8, 1u8, 111u8, 181u8, 243u8, 24u8,
                            148u8, 193u8, 253u8, 248u8, 190u8, 16u8, 222u8,
                            219u8, 67u8,
                        ],
                    )
                }
                #[doc = " Maximum number of nomination pools that can exist. If `None`, then an unbounded number of"]
                #[doc = " pools can exist."]                pub fn max_pools (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "MaxPools",
                        vec![],
                        [
                            216u8, 111u8, 68u8, 103u8, 33u8, 50u8, 109u8, 3u8,
                            176u8, 195u8, 23u8, 73u8, 112u8, 138u8, 9u8, 194u8,
                            233u8, 73u8, 68u8, 215u8, 162u8, 255u8, 217u8,
                            173u8, 141u8, 27u8, 72u8, 199u8, 7u8, 240u8, 25u8,
                            34u8,
                        ],
                    )
                }
                #[doc = " Maximum number of members that can exist in the system. If `None`, then the count"]
                #[doc = " members are not bound on a system wide basis."]                pub fn max_pool_members (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "MaxPoolMembers",
                        vec![],
                        [
                            82u8, 217u8, 26u8, 234u8, 223u8, 241u8, 66u8,
                            182u8, 43u8, 233u8, 59u8, 242u8, 202u8, 254u8,
                            69u8, 50u8, 254u8, 196u8, 166u8, 89u8, 120u8, 87u8,
                            76u8, 148u8, 31u8, 197u8, 49u8, 88u8, 206u8, 41u8,
                            242u8, 62u8,
                        ],
                    )
                }
                #[doc = " Maximum number of members that may belong to pool. If `None`, then the count of"]
                #[doc = " members is not bound on a per pool basis."]                pub fn max_pool_members_per_pool (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "MaxPoolMembersPerPool",
                        vec![],
                        [
                            93u8, 241u8, 16u8, 169u8, 138u8, 199u8, 128u8,
                            149u8, 65u8, 30u8, 55u8, 11u8, 41u8, 252u8, 83u8,
                            250u8, 9u8, 33u8, 152u8, 239u8, 195u8, 147u8, 16u8,
                            248u8, 180u8, 153u8, 88u8, 231u8, 248u8, 169u8,
                            186u8, 48u8,
                        ],
                    )
                }
                #[doc = " Active members."]                pub fn pool_members (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_nomination_pools :: PoolMember > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("NominationPools" , "PoolMembers" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [252u8 , 236u8 , 201u8 , 127u8 , 219u8 , 1u8 , 19u8 , 144u8 , 5u8 , 108u8 , 70u8 , 30u8 , 177u8 , 232u8 , 253u8 , 237u8 , 211u8 , 91u8 , 63u8 , 62u8 , 155u8 , 151u8 , 153u8 , 165u8 , 206u8 , 53u8 , 111u8 , 31u8 , 60u8 , 120u8 , 100u8 , 249u8 ,])
                }
                #[doc = " Active members."]                pub fn pool_members_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_nomination_pools :: PoolMember > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "PoolMembers",
                        Vec::new(),
                        [
                            252u8, 236u8, 201u8, 127u8, 219u8, 1u8, 19u8,
                            144u8, 5u8, 108u8, 70u8, 30u8, 177u8, 232u8, 253u8,
                            237u8, 211u8, 91u8, 63u8, 62u8, 155u8, 151u8,
                            153u8, 165u8, 206u8, 53u8, 111u8, 31u8, 60u8,
                            120u8, 100u8, 249u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]                pub fn counter_for_pool_members (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "CounterForPoolMembers",
                        vec![],
                        [
                            114u8, 126u8, 27u8, 138u8, 119u8, 44u8, 45u8,
                            129u8, 84u8, 107u8, 171u8, 206u8, 117u8, 141u8,
                            20u8, 75u8, 229u8, 237u8, 31u8, 229u8, 124u8,
                            190u8, 27u8, 124u8, 63u8, 59u8, 167u8, 42u8, 62u8,
                            212u8, 160u8, 2u8,
                        ],
                    )
                }
                #[doc = " Storage for bonded pools."]                pub fn bonded_pools (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_nomination_pools :: BondedPoolInner > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("NominationPools" , "BondedPools" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [34u8 , 51u8 , 86u8 , 95u8 , 237u8 , 118u8 , 40u8 , 212u8 , 128u8 , 227u8 , 113u8 , 6u8 , 116u8 , 28u8 , 96u8 , 223u8 , 63u8 , 249u8 , 33u8 , 152u8 , 61u8 , 7u8 , 205u8 , 220u8 , 221u8 , 174u8 , 207u8 , 39u8 , 53u8 , 176u8 , 13u8 , 74u8 ,])
                }
                #[doc = " Storage for bonded pools."]                pub fn bonded_pools_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_nomination_pools :: BondedPoolInner > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "BondedPools",
                        Vec::new(),
                        [
                            34u8, 51u8, 86u8, 95u8, 237u8, 118u8, 40u8, 212u8,
                            128u8, 227u8, 113u8, 6u8, 116u8, 28u8, 96u8, 223u8,
                            63u8, 249u8, 33u8, 152u8, 61u8, 7u8, 205u8, 220u8,
                            221u8, 174u8, 207u8, 39u8, 53u8, 176u8, 13u8, 74u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]                pub fn counter_for_bonded_pools (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "CounterForBondedPools",
                        vec![],
                        [
                            134u8, 94u8, 199u8, 73u8, 174u8, 253u8, 66u8,
                            242u8, 233u8, 244u8, 140u8, 170u8, 242u8, 40u8,
                            41u8, 185u8, 183u8, 151u8, 58u8, 111u8, 221u8,
                            225u8, 81u8, 71u8, 169u8, 219u8, 223u8, 135u8, 8u8,
                            171u8, 180u8, 236u8,
                        ],
                    )
                }
                #[doc = " Reward pools. This is where there rewards for each pool accumulate. When a members payout"]
                #[doc = " is claimed, the balance comes out fo the reward pool. Keyed by the bonded pools account."]                pub fn reward_pools (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_nomination_pools :: RewardPool > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("NominationPools" , "RewardPools" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [139u8 , 123u8 , 46u8 , 107u8 , 9u8 , 83u8 , 141u8 , 12u8 , 188u8 , 225u8 , 170u8 , 215u8 , 154u8 , 21u8 , 100u8 , 95u8 , 237u8 , 245u8 , 46u8 , 216u8 , 199u8 , 184u8 , 187u8 , 155u8 , 8u8 , 16u8 , 34u8 , 177u8 , 153u8 , 65u8 , 109u8 , 198u8 ,])
                }
                #[doc = " Reward pools. This is where there rewards for each pool accumulate. When a members payout"]
                #[doc = " is claimed, the balance comes out fo the reward pool. Keyed by the bonded pools account."]                pub fn reward_pools_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_nomination_pools :: RewardPool > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "RewardPools",
                        Vec::new(),
                        [
                            139u8, 123u8, 46u8, 107u8, 9u8, 83u8, 141u8, 12u8,
                            188u8, 225u8, 170u8, 215u8, 154u8, 21u8, 100u8,
                            95u8, 237u8, 245u8, 46u8, 216u8, 199u8, 184u8,
                            187u8, 155u8, 8u8, 16u8, 34u8, 177u8, 153u8, 65u8,
                            109u8, 198u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]                pub fn counter_for_reward_pools (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "CounterForRewardPools",
                        vec![],
                        [
                            209u8, 139u8, 212u8, 116u8, 210u8, 178u8, 213u8,
                            38u8, 75u8, 23u8, 188u8, 57u8, 253u8, 213u8, 95u8,
                            118u8, 182u8, 250u8, 45u8, 205u8, 17u8, 175u8,
                            17u8, 201u8, 234u8, 14u8, 98u8, 49u8, 143u8, 135u8,
                            201u8, 81u8,
                        ],
                    )
                }
                #[doc = " Groups of unbonding pools. Each group of unbonding pools belongs to a bonded pool,"]
                #[doc = " hence the name sub-pools. Keyed by the bonded pools account."]                pub fn sub_pools_storage (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_nomination_pools :: SubPools > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("NominationPools" , "SubPoolsStorage" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [231u8 , 13u8 , 111u8 , 248u8 , 1u8 , 208u8 , 179u8 , 134u8 , 224u8 , 196u8 , 94u8 , 201u8 , 229u8 , 29u8 , 155u8 , 211u8 , 163u8 , 150u8 , 157u8 , 34u8 , 68u8 , 238u8 , 55u8 , 4u8 , 222u8 , 96u8 , 186u8 , 29u8 , 205u8 , 237u8 , 80u8 , 42u8 ,])
                }
                #[doc = " Groups of unbonding pools. Each group of unbonding pools belongs to a bonded pool,"]
                #[doc = " hence the name sub-pools. Keyed by the bonded pools account."]                pub fn sub_pools_storage_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_nomination_pools :: SubPools > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "SubPoolsStorage",
                        Vec::new(),
                        [
                            231u8, 13u8, 111u8, 248u8, 1u8, 208u8, 179u8,
                            134u8, 224u8, 196u8, 94u8, 201u8, 229u8, 29u8,
                            155u8, 211u8, 163u8, 150u8, 157u8, 34u8, 68u8,
                            238u8, 55u8, 4u8, 222u8, 96u8, 186u8, 29u8, 205u8,
                            237u8, 80u8, 42u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]                pub fn counter_for_sub_pools_storage (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "CounterForSubPoolsStorage",
                        vec![],
                        [
                            212u8, 145u8, 212u8, 226u8, 234u8, 31u8, 26u8,
                            240u8, 107u8, 91u8, 171u8, 120u8, 41u8, 195u8,
                            16u8, 86u8, 55u8, 127u8, 103u8, 93u8, 128u8, 48u8,
                            69u8, 104u8, 168u8, 236u8, 81u8, 54u8, 2u8, 184u8,
                            215u8, 51u8,
                        ],
                    )
                }
                #[doc = " Metadata for the pool."]                pub fn metadata (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("NominationPools" , "Metadata" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [108u8 , 250u8 , 163u8 , 54u8 , 192u8 , 143u8 , 239u8 , 62u8 , 97u8 , 163u8 , 161u8 , 215u8 , 171u8 , 225u8 , 49u8 , 18u8 , 37u8 , 200u8 , 143u8 , 254u8 , 136u8 , 26u8 , 54u8 , 187u8 , 39u8 , 3u8 , 216u8 , 24u8 , 188u8 , 25u8 , 243u8 , 251u8 ,])
                }
                #[doc = " Metadata for the pool."]                pub fn metadata_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < :: core :: primitive :: u8 > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "Metadata",
                        Vec::new(),
                        [
                            108u8, 250u8, 163u8, 54u8, 192u8, 143u8, 239u8,
                            62u8, 97u8, 163u8, 161u8, 215u8, 171u8, 225u8,
                            49u8, 18u8, 37u8, 200u8, 143u8, 254u8, 136u8, 26u8,
                            54u8, 187u8, 39u8, 3u8, 216u8, 24u8, 188u8, 25u8,
                            243u8, 251u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]                pub fn counter_for_metadata (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "CounterForMetadata",
                        vec![],
                        [
                            190u8, 232u8, 77u8, 134u8, 245u8, 89u8, 160u8,
                            187u8, 163u8, 68u8, 188u8, 204u8, 31u8, 145u8,
                            219u8, 165u8, 213u8, 1u8, 167u8, 90u8, 175u8,
                            218u8, 147u8, 144u8, 158u8, 226u8, 23u8, 233u8,
                            55u8, 168u8, 161u8, 237u8,
                        ],
                    )
                }
                #[doc = " Ever increasing number of all pools created so far."]                pub fn last_pool_id (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "LastPoolId",
                        vec![],
                        [
                            50u8, 254u8, 218u8, 41u8, 213u8, 184u8, 170u8,
                            166u8, 31u8, 29u8, 196u8, 57u8, 215u8, 20u8, 40u8,
                            40u8, 19u8, 22u8, 9u8, 184u8, 11u8, 21u8, 21u8,
                            125u8, 97u8, 38u8, 219u8, 209u8, 2u8, 238u8, 247u8,
                            51u8,
                        ],
                    )
                }
                #[doc = " A reverse lookup from the pool's account id to its id."]
                #[doc = ""]
                #[doc = " This is only used for slashing. In all other instances, the pool id is used, and the"]
                #[doc = " accounts are deterministically derived from it."]                pub fn reverse_pool_id_lookup (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("NominationPools" , "ReversePoolIdLookup" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [178u8 , 161u8 , 51u8 , 220u8 , 128u8 , 1u8 , 135u8 , 83u8 , 236u8 , 159u8 , 36u8 , 237u8 , 120u8 , 128u8 , 6u8 , 191u8 , 41u8 , 159u8 , 94u8 , 178u8 , 174u8 , 235u8 , 221u8 , 173u8 , 44u8 , 81u8 , 211u8 , 255u8 , 231u8 , 81u8 , 16u8 , 87u8 ,])
                }
                #[doc = " A reverse lookup from the pool's account id to its id."]
                #[doc = ""]
                #[doc = " This is only used for slashing. In all other instances, the pool id is used, and the"]
                #[doc = " accounts are deterministically derived from it."]                pub fn reverse_pool_id_lookup_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "ReversePoolIdLookup",
                        Vec::new(),
                        [
                            178u8, 161u8, 51u8, 220u8, 128u8, 1u8, 135u8, 83u8,
                            236u8, 159u8, 36u8, 237u8, 120u8, 128u8, 6u8,
                            191u8, 41u8, 159u8, 94u8, 178u8, 174u8, 235u8,
                            221u8, 173u8, 44u8, 81u8, 211u8, 255u8, 231u8,
                            81u8, 16u8, 87u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]                pub fn counter_for_reverse_pool_id_lookup (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "NominationPools",
                        "CounterForReversePoolIdLookup",
                        vec![],
                        [
                            148u8, 83u8, 81u8, 33u8, 188u8, 72u8, 148u8, 208u8,
                            245u8, 178u8, 52u8, 245u8, 229u8, 140u8, 100u8,
                            152u8, 8u8, 217u8, 161u8, 80u8, 226u8, 42u8, 15u8,
                            252u8, 90u8, 197u8, 120u8, 114u8, 144u8, 90u8,
                            199u8, 123u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The nomination pool's pallet id."]
                pub fn pallet_id(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::PalletId,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "NominationPools",
                        "PalletId",
                        [
                            139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8,
                            69u8, 112u8, 154u8, 174u8, 45u8, 83u8, 245u8, 51u8,
                            132u8, 173u8, 5u8, 186u8, 24u8, 243u8, 9u8, 12u8,
                            214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8,
                            39u8,
                        ],
                    )
                }
                #[doc = " The maximum pool points-to-balance ratio that an `open` pool can have."]
                #[doc = ""]
                #[doc = " This is important in the event slashing takes place and the pool's points-to-balance"]
                #[doc = " ratio becomes disproportional."]
                #[doc = ""]
                #[doc = " Moreover, this relates to the `RewardCounter` type as well, as the arithmetic operations"]
                #[doc = " are a function of number of points, and by setting this value to e.g. 10, you ensure"]
                #[doc = " that the total number of points in the system are at most 10 times the total_issuance of"]
                #[doc = " the chain, in the absolute worse case."]
                #[doc = ""]
                #[doc = " For a value of 10, the threshold would be a pool points-to-balance ratio of 10:1."]
                #[doc = " Such a scenario would also be the equivalent of the pool being 90% slashed."]
                pub fn max_points_to_balance(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "NominationPools",
                        "MaxPointsToBalance",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8,
                            179u8, 168u8, 110u8, 28u8, 91u8, 221u8, 64u8, 4u8,
                            148u8, 201u8, 193u8, 185u8, 66u8, 226u8, 114u8,
                            97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8,
                            228u8, 183u8, 165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod staking {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Bond {
                pub controller: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct BondExtra {
                #[codec(compact)]
                pub max_additional: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Unbond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct WithdrawUnbonded {
                pub num_slashing_spans: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Validate {
                pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Nominate {
                pub targets: ::std::vec::Vec<
                    ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Chill;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetPayee {
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetController {
                pub controller: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetValidatorCount {
                #[codec(compact)]
                pub new: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct IncreaseValidatorCount {
                #[codec(compact)]
                pub additional: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ScaleValidatorCount {
                pub factor: runtime_types::sp_arithmetic::per_things::Percent,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceNoEras;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceNewEra;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetInvulnerables {
                pub invulnerables:
                    ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceUnstake {
                pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceNewEraAlways;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct CancelDeferredSlash {
                pub era: ::core::primitive::u32,
                pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct PayoutStakers {
                pub validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
                pub era: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Rebond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ReapStash {
                pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Kick {
                pub who: ::std::vec::Vec<
                    ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetStakingConfigs {
                pub min_nominator_bond:
                    runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        ::core::primitive::u128,
                    >,
                pub min_validator_bond:
                    runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        ::core::primitive::u128,
                    >,
                pub max_nominator_count:
                    runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        ::core::primitive::u32,
                    >,
                pub max_validator_count:
                    runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        ::core::primitive::u32,
                    >,
                pub chill_threshold:
                    runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                pub min_commission:
                    runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ChillOther {
                pub controller: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceApplyMinCommission {
                pub validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Take the origin account as a stash and lock up `value` of its balance. `controller` will"]
                #[doc = "be the account that controls it."]
                #[doc = ""]
                #[doc = "`value` must be more than the `minimum_balance` specified by `T::Currency`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ by the stash account."]
                #[doc = ""]
                #[doc = "Emits `Bonded`."]
                #[doc = "# <weight>"]
                #[doc = "- Independent of the arguments. Moderate complexity."]
                #[doc = "- O(1)."]
                #[doc = "- Three extra DB entries."]
                #[doc = ""]
                #[doc = "NOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned"]
                #[doc = "unless the `origin` falls below _existential deposit_ and gets removed as dust."]
                #[doc = "------------------"]
                #[doc = "# </weight>"]
                pub fn bond(
                    &self,
                    controller: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    value: ::core::primitive::u128,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Bond> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "bond",
                        Bond {
                            controller,
                            value,
                            payee,
                        },
                        [
                            140u8, 13u8, 108u8, 181u8, 212u8, 177u8, 190u8,
                            212u8, 163u8, 40u8, 120u8, 232u8, 126u8, 213u8,
                            6u8, 181u8, 99u8, 252u8, 58u8, 54u8, 139u8, 64u8,
                            67u8, 76u8, 53u8, 226u8, 11u8, 133u8, 235u8, 159u8,
                            103u8, 210u8,
                        ],
                    )
                }
                #[doc = "Add some extra amount that have appeared in the stash `free_balance` into the balance up"]
                #[doc = "for staking."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ by the stash, not the controller."]
                #[doc = ""]
                #[doc = "Use this if there are additional funds in your stash account that you wish to bond."]
                #[doc = "Unlike [`bond`](Self::bond) or [`unbond`](Self::unbond) this function does not impose"]
                #[doc = "any limitation on the amount that can be added."]
                #[doc = ""]
                #[doc = "Emits `Bonded`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Independent of the arguments. Insignificant complexity."]
                #[doc = "- O(1)."]
                #[doc = "# </weight>"]
                pub fn bond_extra(
                    &self,
                    max_additional: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<BondExtra> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "bond_extra",
                        BondExtra { max_additional },
                        [
                            60u8, 45u8, 82u8, 223u8, 113u8, 95u8, 0u8, 71u8,
                            59u8, 108u8, 228u8, 9u8, 95u8, 210u8, 113u8, 106u8,
                            252u8, 15u8, 19u8, 128u8, 11u8, 187u8, 4u8, 151u8,
                            103u8, 143u8, 24u8, 33u8, 149u8, 82u8, 35u8, 192u8,
                        ],
                    )
                }
                #[doc = "Schedule a portion of the stash to be unlocked ready for transfer out after the bond"]
                #[doc = "period ends. If this leaves an amount actively bonded less than"]
                #[doc = "T::Currency::minimum_balance(), then it is increased to the full amount."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                #[doc = ""]
                #[doc = "Once the unlock period is done, you can call `withdraw_unbonded` to actually move"]
                #[doc = "the funds out of management ready for transfer."]
                #[doc = ""]
                #[doc = "No more than a limited number of unlocking chunks (see `MaxUnlockingChunks`)"]
                #[doc = "can co-exists at the same time. If there are no unlocking chunks slots available"]
                #[doc = "[`Call::withdraw_unbonded`] is called to remove some of the chunks (if possible)."]
                #[doc = ""]
                #[doc = "If a user encounters the `InsufficientBond` error when calling this extrinsic,"]
                #[doc = "they should call `chill` first in order to free up their bonded funds."]
                #[doc = ""]
                #[doc = "Emits `Unbonded`."]
                #[doc = ""]
                #[doc = "See also [`Call::withdraw_unbonded`]."]
                pub fn unbond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Unbond> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "unbond",
                        Unbond { value },
                        [
                            85u8, 62u8, 34u8, 127u8, 60u8, 241u8, 134u8, 60u8,
                            125u8, 91u8, 31u8, 193u8, 50u8, 230u8, 237u8, 42u8,
                            114u8, 230u8, 240u8, 146u8, 14u8, 109u8, 185u8,
                            151u8, 148u8, 44u8, 147u8, 182u8, 192u8, 253u8,
                            51u8, 87u8,
                        ],
                    )
                }
                #[doc = "Remove any unlocked chunks from the `unlocking` queue from our management."]
                #[doc = ""]
                #[doc = "This essentially frees up that balance to be used by the stash account to do"]
                #[doc = "whatever it wants."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ by the controller."]
                #[doc = ""]
                #[doc = "Emits `Withdrawn`."]
                #[doc = ""]
                #[doc = "See also [`Call::unbond`]."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Complexity O(S) where S is the number of slashing spans to remove"]
                #[doc = "NOTE: Weight annotation is the kill scenario, we refund otherwise."]
                #[doc = "# </weight>"]
                pub fn withdraw_unbonded(
                    &self,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<WithdrawUnbonded>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "withdraw_unbonded",
                        WithdrawUnbonded { num_slashing_spans },
                        [
                            95u8, 223u8, 122u8, 217u8, 76u8, 208u8, 86u8,
                            129u8, 31u8, 104u8, 70u8, 154u8, 23u8, 250u8,
                            165u8, 192u8, 149u8, 249u8, 158u8, 159u8, 194u8,
                            224u8, 118u8, 134u8, 204u8, 157u8, 72u8, 136u8,
                            19u8, 193u8, 183u8, 84u8,
                        ],
                    )
                }
                #[doc = "Declare the desire to validate for the origin controller."]
                #[doc = ""]
                #[doc = "Effects will be felt at the beginning of the next era."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                pub fn validate(
                    &self,
                    prefs: runtime_types::pallet_staking::ValidatorPrefs,
                ) -> ::subxt::tx::StaticTxPayload<Validate> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "validate",
                        Validate { prefs },
                        [
                            191u8, 116u8, 139u8, 35u8, 250u8, 211u8, 86u8,
                            240u8, 35u8, 9u8, 19u8, 44u8, 148u8, 35u8, 91u8,
                            106u8, 200u8, 172u8, 108u8, 145u8, 194u8, 146u8,
                            61u8, 145u8, 233u8, 168u8, 2u8, 26u8, 145u8, 101u8,
                            114u8, 157u8,
                        ],
                    )
                }
                #[doc = "Declare the desire to nominate `targets` for the origin controller."]
                #[doc = ""]
                #[doc = "Effects will be felt at the beginning of the next era."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- The transaction's complexity is proportional to the size of `targets` (N)"]
                #[doc = "which is capped at CompactAssignments::LIMIT (T::MaxNominations)."]
                #[doc = "- Both the reads and writes follow a similar pattern."]
                #[doc = "# </weight>"]
                pub fn nominate(
                    &self,
                    targets: ::std::vec::Vec<
                        ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Nominate> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "nominate",
                        Nominate { targets },
                        [
                            249u8, 66u8, 140u8, 39u8, 26u8, 221u8, 135u8,
                            225u8, 98u8, 255u8, 13u8, 54u8, 106u8, 215u8,
                            129u8, 156u8, 190u8, 83u8, 178u8, 170u8, 116u8,
                            27u8, 8u8, 244u8, 56u8, 73u8, 164u8, 223u8, 199u8,
                            115u8, 168u8, 83u8,
                        ],
                    )
                }
                #[doc = "Declare no desire to either validate or nominate."]
                #[doc = ""]
                #[doc = "Effects will be felt at the beginning of the next era."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Independent of the arguments. Insignificant complexity."]
                #[doc = "- Contains one read."]
                #[doc = "- Writes are limited to the `origin` account key."]
                #[doc = "# </weight>"]
                pub fn chill(&self) -> ::subxt::tx::StaticTxPayload<Chill> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "chill",
                        Chill {},
                        [
                            94u8, 20u8, 196u8, 31u8, 220u8, 125u8, 115u8,
                            167u8, 140u8, 3u8, 20u8, 132u8, 81u8, 120u8, 215u8,
                            166u8, 230u8, 56u8, 16u8, 222u8, 31u8, 153u8,
                            120u8, 62u8, 153u8, 67u8, 220u8, 239u8, 11u8,
                            234u8, 127u8, 122u8,
                        ],
                    )
                }
                #[doc = "(Re-)set the payment target for a controller."]
                #[doc = ""]
                #[doc = "Effects will be felt instantly (as soon as this function is completed successfully)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Independent of the arguments. Insignificant complexity."]
                #[doc = "- Contains a limited number of reads."]
                #[doc = "- Writes are limited to the `origin` account key."]
                #[doc = "---------"]
                #[doc = "- Weight: O(1)"]
                #[doc = "- DB Weight:"]
                #[doc = "    - Read: Ledger"]
                #[doc = "    - Write: Payee"]
                #[doc = "# </weight>"]
                pub fn set_payee(
                    &self,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetPayee> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "set_payee",
                        SetPayee { payee },
                        [
                            96u8, 8u8, 254u8, 164u8, 87u8, 46u8, 120u8, 11u8,
                            197u8, 63u8, 20u8, 178u8, 167u8, 236u8, 149u8,
                            245u8, 14u8, 171u8, 108u8, 195u8, 250u8, 133u8,
                            0u8, 75u8, 192u8, 159u8, 84u8, 220u8, 242u8, 133u8,
                            60u8, 62u8,
                        ],
                    )
                }
                #[doc = "(Re-)set the controller of a stash."]
                #[doc = ""]
                #[doc = "Effects will be felt instantly (as soon as this function is completed successfully)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ by the stash, not the controller."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Independent of the arguments. Insignificant complexity."]
                #[doc = "- Contains a limited number of reads."]
                #[doc = "- Writes are limited to the `origin` account key."]
                #[doc = "----------"]
                #[doc = "Weight: O(1)"]
                #[doc = "DB Weight:"]
                #[doc = "- Read: Bonded, Ledger New Controller, Ledger Old Controller"]
                #[doc = "- Write: Bonded, Ledger New Controller, Ledger Old Controller"]
                #[doc = "# </weight>"]
                pub fn set_controller(
                    &self,
                    controller: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetController>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "set_controller",
                        SetController { controller },
                        [
                            154u8, 80u8, 184u8, 176u8, 74u8, 106u8, 72u8,
                            242u8, 64u8, 81u8, 169u8, 157u8, 200u8, 97u8,
                            117u8, 192u8, 143u8, 166u8, 38u8, 235u8, 75u8,
                            161u8, 177u8, 229u8, 229u8, 82u8, 95u8, 39u8, 40u8,
                            116u8, 9u8, 204u8,
                        ],
                    )
                }
                #[doc = "Sets the ideal number of validators."]
                #[doc = ""]
                #[doc = "The dispatch origin must be Root."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight: O(1)"]
                #[doc = "Write: Validator Count"]
                #[doc = "# </weight>"]
                pub fn set_validator_count(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetValidatorCount>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "set_validator_count",
                        SetValidatorCount { new },
                        [
                            55u8, 232u8, 95u8, 66u8, 228u8, 217u8, 11u8, 27u8,
                            3u8, 202u8, 199u8, 242u8, 70u8, 160u8, 250u8,
                            187u8, 194u8, 91u8, 15u8, 36u8, 215u8, 36u8, 160u8,
                            108u8, 251u8, 60u8, 240u8, 202u8, 249u8, 235u8,
                            28u8, 94u8,
                        ],
                    )
                }
                #[doc = "Increments the ideal number of validators upto maximum of"]
                #[doc = "`ElectionProviderBase::MaxWinners`."]
                #[doc = ""]
                #[doc = "The dispatch origin must be Root."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Same as [`Self::set_validator_count`]."]
                #[doc = "# </weight>"]
                pub fn increase_validator_count(
                    &self,
                    additional: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<IncreaseValidatorCount>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "increase_validator_count",
                        IncreaseValidatorCount { additional },
                        [
                            239u8, 184u8, 155u8, 213u8, 25u8, 22u8, 193u8,
                            13u8, 102u8, 192u8, 82u8, 153u8, 249u8, 192u8,
                            60u8, 158u8, 8u8, 78u8, 175u8, 219u8, 46u8, 51u8,
                            222u8, 193u8, 193u8, 201u8, 78u8, 90u8, 58u8, 86u8,
                            196u8, 17u8,
                        ],
                    )
                }
                #[doc = "Scale up the ideal number of validators by a factor upto maximum of"]
                #[doc = "`ElectionProviderBase::MaxWinners`."]
                #[doc = ""]
                #[doc = "The dispatch origin must be Root."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Same as [`Self::set_validator_count`]."]
                #[doc = "# </weight>"]
                pub fn scale_validator_count(
                    &self,
                    factor: runtime_types::sp_arithmetic::per_things::Percent,
                ) -> ::subxt::tx::StaticTxPayload<ScaleValidatorCount>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "scale_validator_count",
                        ScaleValidatorCount { factor },
                        [
                            198u8, 68u8, 227u8, 94u8, 110u8, 157u8, 209u8,
                            217u8, 112u8, 37u8, 78u8, 142u8, 12u8, 193u8,
                            219u8, 167u8, 149u8, 112u8, 49u8, 139u8, 74u8,
                            81u8, 172u8, 72u8, 253u8, 224u8, 56u8, 194u8,
                            185u8, 90u8, 87u8, 125u8,
                        ],
                    )
                }
                #[doc = "Force there to be no new eras indefinitely."]
                #[doc = ""]
                #[doc = "The dispatch origin must be Root."]
                #[doc = ""]
                #[doc = "# Warning"]
                #[doc = ""]
                #[doc = "The election process starts multiple blocks before the end of the era."]
                #[doc = "Thus the election process may be ongoing when this is called. In this case the"]
                #[doc = "election will continue until the next era is triggered."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- No arguments."]
                #[doc = "- Weight: O(1)"]
                #[doc = "- Write: ForceEra"]
                #[doc = "# </weight>"]
                pub fn force_no_eras(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<ForceNoEras> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "force_no_eras",
                        ForceNoEras {},
                        [
                            16u8, 81u8, 207u8, 168u8, 23u8, 236u8, 11u8, 75u8,
                            141u8, 107u8, 92u8, 2u8, 53u8, 111u8, 252u8, 116u8,
                            91u8, 120u8, 75u8, 24u8, 125u8, 53u8, 9u8, 28u8,
                            242u8, 87u8, 245u8, 55u8, 40u8, 103u8, 151u8,
                            178u8,
                        ],
                    )
                }
                #[doc = "Force there to be a new era at the end of the next session. After this, it will be"]
                #[doc = "reset to normal (non-forced) behaviour."]
                #[doc = ""]
                #[doc = "The dispatch origin must be Root."]
                #[doc = ""]
                #[doc = "# Warning"]
                #[doc = ""]
                #[doc = "The election process starts multiple blocks before the end of the era."]
                #[doc = "If this is called just before a new era is triggered, the election process may not"]
                #[doc = "have enough blocks to get a result."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- No arguments."]
                #[doc = "- Weight: O(1)"]
                #[doc = "- Write ForceEra"]
                #[doc = "# </weight>"]
                pub fn force_new_era(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<ForceNewEra> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "force_new_era",
                        ForceNewEra {},
                        [
                            230u8, 242u8, 169u8, 196u8, 78u8, 145u8, 24u8,
                            191u8, 113u8, 68u8, 5u8, 138u8, 48u8, 51u8, 109u8,
                            126u8, 73u8, 136u8, 162u8, 158u8, 174u8, 201u8,
                            213u8, 230u8, 215u8, 44u8, 200u8, 32u8, 75u8, 27u8,
                            23u8, 254u8,
                        ],
                    )
                }
                #[doc = "Set the validators who cannot be slashed (if any)."]
                #[doc = ""]
                #[doc = "The dispatch origin must be Root."]
                pub fn set_invulnerables(
                    &self,
                    invulnerables: ::std::vec::Vec<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetInvulnerables>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "set_invulnerables",
                        SetInvulnerables { invulnerables },
                        [
                            2u8, 148u8, 221u8, 111u8, 153u8, 48u8, 222u8, 36u8,
                            228u8, 84u8, 18u8, 35u8, 168u8, 239u8, 53u8, 245u8,
                            27u8, 76u8, 18u8, 203u8, 206u8, 9u8, 8u8, 81u8,
                            35u8, 224u8, 22u8, 133u8, 58u8, 99u8, 103u8, 39u8,
                        ],
                    )
                }
                #[doc = "Force a current staker to become completely unstaked, immediately."]
                #[doc = ""]
                #[doc = "The dispatch origin must be Root."]
                pub fn force_unstake(
                    &self,
                    stash: ::subxt::ext::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<ForceUnstake>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "force_unstake",
                        ForceUnstake {
                            stash,
                            num_slashing_spans,
                        },
                        [
                            94u8, 247u8, 238u8, 47u8, 250u8, 6u8, 96u8, 175u8,
                            173u8, 123u8, 161u8, 187u8, 162u8, 214u8, 176u8,
                            233u8, 33u8, 33u8, 167u8, 239u8, 40u8, 223u8, 19u8,
                            131u8, 230u8, 39u8, 175u8, 200u8, 36u8, 182u8,
                            76u8, 207u8,
                        ],
                    )
                }
                #[doc = "Force there to be a new era at the end of sessions indefinitely."]
                #[doc = ""]
                #[doc = "The dispatch origin must be Root."]
                #[doc = ""]
                #[doc = "# Warning"]
                #[doc = ""]
                #[doc = "The election process starts multiple blocks before the end of the era."]
                #[doc = "If this is called just before a new era is triggered, the election process may not"]
                #[doc = "have enough blocks to get a result."]
                pub fn force_new_era_always(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<ForceNewEraAlways>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "force_new_era_always",
                        ForceNewEraAlways {},
                        [
                            179u8, 118u8, 189u8, 54u8, 248u8, 141u8, 207u8,
                            142u8, 80u8, 37u8, 241u8, 185u8, 138u8, 254u8,
                            117u8, 147u8, 225u8, 118u8, 34u8, 177u8, 197u8,
                            158u8, 8u8, 82u8, 202u8, 108u8, 208u8, 26u8, 64u8,
                            33u8, 74u8, 43u8,
                        ],
                    )
                }
                #[doc = "Cancel enactment of a deferred slash."]
                #[doc = ""]
                #[doc = "Can be called by the `T::SlashCancelOrigin`."]
                #[doc = ""]
                #[doc = "Parameters: era and indices of the slashes for that era to kill."]
                pub fn cancel_deferred_slash(
                    &self,
                    era: ::core::primitive::u32,
                    slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::tx::StaticTxPayload<CancelDeferredSlash>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "cancel_deferred_slash",
                        CancelDeferredSlash { era, slash_indices },
                        [
                            120u8, 57u8, 162u8, 105u8, 91u8, 250u8, 129u8,
                            240u8, 110u8, 234u8, 170u8, 98u8, 164u8, 65u8,
                            106u8, 101u8, 19u8, 88u8, 146u8, 210u8, 171u8,
                            44u8, 37u8, 50u8, 65u8, 178u8, 37u8, 223u8, 239u8,
                            197u8, 116u8, 168u8,
                        ],
                    )
                }
                #[doc = "Pay out all the stakers behind a single validator for a single era."]
                #[doc = ""]
                #[doc = "- `validator_stash` is the stash account of the validator. Their nominators, up to"]
                #[doc = "  `T::MaxNominatorRewardedPerValidator`, will also receive their rewards."]
                #[doc = "- `era` may be any era between `[current_era - history_depth; current_era]`."]
                #[doc = ""]
                #[doc = "The origin of this call must be _Signed_. Any account can call this function, even if"]
                #[doc = "it is not one of the stakers."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Time complexity: at most O(MaxNominatorRewardedPerValidator)."]
                #[doc = "- Contains a limited number of reads and writes."]
                #[doc = "-----------"]
                #[doc = "N is the Number of payouts for the validator (including the validator)"]
                #[doc = "Weight:"]
                #[doc = "- Reward Destination Staked: O(N)"]
                #[doc = "- Reward Destination Controller (Creating): O(N)"]
                #[doc = ""]
                #[doc = "  NOTE: weights are assuming that payouts are made to alive stash account (Staked)."]
                #[doc = "  Paying even a dead controller is cheaper weight-wise. We don't do any refunds here."]
                #[doc = "# </weight>"]
                pub fn payout_stakers(
                    &self,
                    validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
                    era: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<PayoutStakers>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "payout_stakers",
                        PayoutStakers {
                            validator_stash,
                            era,
                        },
                        [
                            184u8, 194u8, 33u8, 118u8, 7u8, 203u8, 89u8, 119u8,
                            214u8, 76u8, 178u8, 20u8, 82u8, 111u8, 57u8, 132u8,
                            212u8, 43u8, 232u8, 91u8, 252u8, 49u8, 42u8, 115u8,
                            1u8, 181u8, 154u8, 207u8, 144u8, 206u8, 205u8,
                            33u8,
                        ],
                    )
                }
                #[doc = "Rebond a portion of the stash scheduled to be unlocked."]
                #[doc = ""]
                #[doc = "The dispatch origin must be signed by the controller."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Time complexity: O(L), where L is unlocking chunks"]
                #[doc = "- Bounded by `MaxUnlockingChunks`."]
                #[doc = "- Storage changes: Can't increase storage, only decrease it."]
                #[doc = "# </weight>"]
                pub fn rebond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Rebond> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "rebond",
                        Rebond { value },
                        [
                            25u8, 22u8, 191u8, 172u8, 133u8, 101u8, 139u8,
                            102u8, 134u8, 16u8, 136u8, 56u8, 137u8, 162u8, 4u8,
                            253u8, 196u8, 30u8, 234u8, 49u8, 102u8, 68u8,
                            145u8, 96u8, 148u8, 219u8, 162u8, 17u8, 177u8,
                            184u8, 34u8, 113u8,
                        ],
                    )
                }
                #[doc = "Remove all data structures concerning a staker/stash once it is at a state where it can"]
                #[doc = "be considered `dust` in the staking system. The requirements are:"]
                #[doc = ""]
                #[doc = "1. the `total_balance` of the stash is below existential deposit."]
                #[doc = "2. or, the `ledger.total` of the stash is below existential deposit."]
                #[doc = ""]
                #[doc = "The former can happen in cases like a slash; the latter when a fully unbonded account"]
                #[doc = "is still receiving staking rewards in `RewardDestination::Staked`."]
                #[doc = ""]
                #[doc = "It can be called by anyone, as long as `stash` meets the above requirements."]
                #[doc = ""]
                #[doc = "Refunds the transaction fees upon successful execution."]
                pub fn reap_stash(
                    &self,
                    stash: ::subxt::ext::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<ReapStash> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "reap_stash",
                        ReapStash {
                            stash,
                            num_slashing_spans,
                        },
                        [
                            34u8, 168u8, 120u8, 161u8, 95u8, 199u8, 106u8,
                            233u8, 61u8, 240u8, 166u8, 31u8, 183u8, 165u8,
                            158u8, 179u8, 32u8, 130u8, 27u8, 164u8, 112u8,
                            44u8, 14u8, 125u8, 227u8, 87u8, 70u8, 203u8, 194u8,
                            24u8, 212u8, 177u8,
                        ],
                    )
                }
                #[doc = "Remove the given nominations from the calling validator."]
                #[doc = ""]
                #[doc = "Effects will be felt at the beginning of the next era."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
                #[doc = ""]
                #[doc = "- `who`: A list of nominator stash accounts who are nominating this validator which"]
                #[doc = "  should no longer be nominating this validator."]
                #[doc = ""]
                #[doc = "Note: Making this call only makes sense if you first set the validator preferences to"]
                #[doc = "block any further nominations."]
                pub fn kick(
                    &self,
                    who: ::std::vec::Vec<
                        ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Kick> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "kick",
                        Kick { who },
                        [
                            94u8, 27u8, 18u8, 16u8, 126u8, 129u8, 47u8, 169u8,
                            114u8, 84u8, 48u8, 95u8, 235u8, 237u8, 33u8, 118u8,
                            115u8, 243u8, 166u8, 120u8, 121u8, 70u8, 227u8,
                            240u8, 205u8, 240u8, 211u8, 202u8, 251u8, 232u8,
                            209u8, 12u8,
                        ],
                    )
                }
                #[doc = "Update the various staking configurations ."]
                #[doc = ""]
                #[doc = "* `min_nominator_bond`: The minimum active bond needed to be a nominator."]
                #[doc = "* `min_validator_bond`: The minimum active bond needed to be a validator."]
                #[doc = "* `max_nominator_count`: The max number of users who can be a nominator at once. When"]
                #[doc = "  set to `None`, no limit is enforced."]
                #[doc = "* `max_validator_count`: The max number of users who can be a validator at once. When"]
                #[doc = "  set to `None`, no limit is enforced."]
                #[doc = "* `chill_threshold`: The ratio of `max_nominator_count` or `max_validator_count` which"]
                #[doc = "  should be filled in order for the `chill_other` transaction to work."]
                #[doc = "* `min_commission`: The minimum amount of commission that each validators must maintain."]
                #[doc = "  This is checked only upon calling `validate`. Existing validators are not affected."]
                #[doc = ""]
                #[doc = "RuntimeOrigin must be Root to call this function."]
                #[doc = ""]
                #[doc = "NOTE: Existing nominators and validators will not be affected by this update."]
                #[doc = "to kick people under the new limits, `chill_other` should be called."]
                pub fn set_staking_configs(
                    &self,
                    min_nominator_bond : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < :: core :: primitive :: u128 >,
                    min_validator_bond : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < :: core :: primitive :: u128 >,
                    max_nominator_count : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < :: core :: primitive :: u32 >,
                    max_validator_count : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < :: core :: primitive :: u32 >,
                    chill_threshold : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < runtime_types :: sp_arithmetic :: per_things :: Percent >,
                    min_commission : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < runtime_types :: sp_arithmetic :: per_things :: Perbill >,
                ) -> ::subxt::tx::StaticTxPayload<SetStakingConfigs>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "set_staking_configs",
                        SetStakingConfigs {
                            min_nominator_bond,
                            min_validator_bond,
                            max_nominator_count,
                            max_validator_count,
                            chill_threshold,
                            min_commission,
                        },
                        [
                            176u8, 168u8, 155u8, 176u8, 27u8, 79u8, 223u8,
                            92u8, 88u8, 93u8, 223u8, 69u8, 179u8, 250u8, 138u8,
                            138u8, 87u8, 220u8, 36u8, 3u8, 126u8, 213u8, 16u8,
                            68u8, 3u8, 16u8, 218u8, 151u8, 98u8, 169u8, 217u8,
                            75u8,
                        ],
                    )
                }
                #[doc = "Declare a `controller` to stop participating as either a validator or nominator."]
                #[doc = ""]
                #[doc = "Effects will be felt at the beginning of the next era."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_, but can be called by anyone."]
                #[doc = ""]
                #[doc = "If the caller is the same as the controller being targeted, then no further checks are"]
                #[doc = "enforced, and this function behaves just like `chill`."]
                #[doc = ""]
                #[doc = "If the caller is different than the controller being targeted, the following conditions"]
                #[doc = "must be met:"]
                #[doc = ""]
                #[doc = "* `controller` must belong to a nominator who has become non-decodable,"]
                #[doc = ""]
                #[doc = "Or:"]
                #[doc = ""]
                #[doc = "* A `ChillThreshold` must be set and checked which defines how close to the max"]
                #[doc = "  nominators or validators we must reach before users can start chilling one-another."]
                #[doc = "* A `MaxNominatorCount` and `MaxValidatorCount` must be set which is used to determine"]
                #[doc = "  how close we are to the threshold."]
                #[doc = "* A `MinNominatorBond` and `MinValidatorBond` must be set and checked, which determines"]
                #[doc = "  if this is a person that should be chilled because they have not met the threshold"]
                #[doc = "  bond required."]
                #[doc = ""]
                #[doc = "This can be helpful if bond requirements are updated, and we need to remove old users"]
                #[doc = "who do not satisfy these requirements."]
                pub fn chill_other(
                    &self,
                    controller: ::subxt::ext::sp_core::crypto::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<ChillOther> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "chill_other",
                        ChillOther { controller },
                        [
                            140u8, 98u8, 4u8, 203u8, 91u8, 131u8, 123u8, 119u8,
                            169u8, 47u8, 188u8, 23u8, 205u8, 170u8, 82u8,
                            220u8, 166u8, 170u8, 135u8, 176u8, 68u8, 228u8,
                            14u8, 67u8, 42u8, 52u8, 140u8, 231u8, 62u8, 167u8,
                            80u8, 173u8,
                        ],
                    )
                }
                #[doc = "Force a validator to have at least the minimum commission. This will not affect a"]
                #[doc = "validator who already has a commission greater than or equal to the minimum. Any account"]
                #[doc = "can call this."]
                pub fn force_apply_min_commission(
                    &self,
                    validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<ForceApplyMinCommission>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Staking",
                        "force_apply_min_commission",
                        ForceApplyMinCommission { validator_stash },
                        [
                            136u8, 163u8, 85u8, 134u8, 240u8, 247u8, 183u8,
                            227u8, 226u8, 202u8, 102u8, 186u8, 138u8, 119u8,
                            78u8, 123u8, 229u8, 135u8, 129u8, 241u8, 119u8,
                            106u8, 41u8, 182u8, 121u8, 181u8, 242u8, 175u8,
                            74u8, 207u8, 64u8, 106u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_staking::pallet::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The era payout has been set; the first balance is the validator-payout; the second is"]
            #[doc = "the remainder from the maximum amount of reward."]
            pub struct EraPaid {
                pub era_index: ::core::primitive::u32,
                pub validator_payout: ::core::primitive::u128,
                pub remainder: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for EraPaid {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "EraPaid";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The nominator has been rewarded by this amount."]
            pub struct Rewarded {
                pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Rewarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A staker (validator or nominator) has been slashed by the given amount."]
            pub struct Slashed {
                pub staker: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A slash for the given validator, for the given percentage of their stake, at the given"]
            #[doc = "era as been reported."]
            pub struct SlashReported {
                pub validator: ::subxt::ext::sp_core::crypto::AccountId32,
                pub fraction: runtime_types::sp_arithmetic::per_things::Perbill,
                pub slash_era: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for SlashReported {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "SlashReported";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An old slashing report from a prior era was discarded because it could"]
            #[doc = "not be processed."]
            pub struct OldSlashingReportDiscarded {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for OldSlashingReportDiscarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "OldSlashingReportDiscarded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A new set of stakers was elected."]
            pub struct StakersElected;
            impl ::subxt::events::StaticEvent for StakersElected {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakersElected";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An account has bonded this amount. \\[stash, amount\\]"]
            #[doc = ""]
            #[doc = "NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,"]
            #[doc = "it will not be emitted for staking rewards when they are added to stake."]
            pub struct Bonded {
                pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Bonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Bonded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An account has unbonded this amount."]
            pub struct Unbonded {
                pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unbonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Unbonded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`"]
            #[doc = "from the unlocking queue."]
            pub struct Withdrawn {
                pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdrawn {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Withdrawn";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A nominator has been kicked from a validator."]
            pub struct Kicked {
                pub nominator: ::subxt::ext::sp_core::crypto::AccountId32,
                pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Kicked {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Kicked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The election failed. No new era is planned."]
            pub struct StakingElectionFailed;
            impl ::subxt::events::StaticEvent for StakingElectionFailed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakingElectionFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "An account has stopped participating as either a validator or nominator."]
            pub struct Chilled {
                pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Chilled {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Chilled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "The stakers' rewards are getting paid."]
            pub struct PayoutStarted {
                pub era_index: ::core::primitive::u32,
                pub validator_stash: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for PayoutStarted {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "PayoutStarted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A validator has set their preferences."]
            pub struct ValidatorPrefsSet {
                pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
                pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
            }
            impl ::subxt::events::StaticEvent for ValidatorPrefsSet {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "ValidatorPrefsSet";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The ideal number of active validators."]                pub fn validator_count (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ValidatorCount",
                        vec![],
                        [
                            245u8, 75u8, 214u8, 110u8, 66u8, 164u8, 86u8,
                            206u8, 69u8, 89u8, 12u8, 111u8, 117u8, 16u8, 228u8,
                            184u8, 207u8, 6u8, 0u8, 126u8, 221u8, 67u8, 125u8,
                            218u8, 188u8, 245u8, 156u8, 188u8, 34u8, 85u8,
                            208u8, 197u8,
                        ],
                    )
                }
                #[doc = " Minimum number of staking participants before emergency conditions are imposed."]                pub fn minimum_validator_count (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "MinimumValidatorCount",
                        vec![],
                        [
                            82u8, 95u8, 128u8, 55u8, 136u8, 134u8, 71u8, 117u8,
                            135u8, 76u8, 44u8, 46u8, 174u8, 34u8, 170u8, 228u8,
                            175u8, 1u8, 234u8, 162u8, 91u8, 252u8, 127u8, 68u8,
                            243u8, 241u8, 13u8, 107u8, 214u8, 70u8, 87u8,
                            249u8,
                        ],
                    )
                }
                #[doc = " Any validators that may never be slashed or forcibly kicked. It's a Vec since they're"]
                #[doc = " easy to initialize and the performance hit is minimal (we expect no more than four"]
                #[doc = " invulnerables) and restricted to testnets."]                pub fn invulnerables (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "Invulnerables",
                        vec![],
                        [
                            77u8, 78u8, 63u8, 199u8, 150u8, 167u8, 135u8,
                            130u8, 192u8, 51u8, 202u8, 119u8, 68u8, 49u8,
                            241u8, 68u8, 82u8, 90u8, 226u8, 201u8, 96u8, 170u8,
                            21u8, 173u8, 236u8, 116u8, 148u8, 8u8, 174u8, 92u8,
                            7u8, 11u8,
                        ],
                    )
                }
                #[doc = " Map from all locked \"stash\" accounts to the controller account."]                pub fn bonded (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "Bonded" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [35u8 , 197u8 , 156u8 , 60u8 , 22u8 , 59u8 , 103u8 , 83u8 , 77u8 , 15u8 , 118u8 , 193u8 , 155u8 , 97u8 , 229u8 , 36u8 , 119u8 , 128u8 , 224u8 , 162u8 , 21u8 , 46u8 , 199u8 , 221u8 , 15u8 , 74u8 , 59u8 , 70u8 , 77u8 , 218u8 , 73u8 , 165u8 ,])
                }
                #[doc = " Map from all locked \"stash\" accounts to the controller account."]                pub fn bonded_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "Bonded",
                        Vec::new(),
                        [
                            35u8, 197u8, 156u8, 60u8, 22u8, 59u8, 103u8, 83u8,
                            77u8, 15u8, 118u8, 193u8, 155u8, 97u8, 229u8, 36u8,
                            119u8, 128u8, 224u8, 162u8, 21u8, 46u8, 199u8,
                            221u8, 15u8, 74u8, 59u8, 70u8, 77u8, 218u8, 73u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The minimum active bond to become and maintain the role of a nominator."]                pub fn min_nominator_bond (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "MinNominatorBond",
                        vec![],
                        [
                            187u8, 66u8, 149u8, 226u8, 72u8, 219u8, 57u8,
                            246u8, 102u8, 47u8, 71u8, 12u8, 219u8, 204u8,
                            127u8, 223u8, 58u8, 134u8, 81u8, 165u8, 200u8,
                            142u8, 196u8, 158u8, 26u8, 38u8, 165u8, 19u8, 91u8,
                            251u8, 119u8, 84u8,
                        ],
                    )
                }
                #[doc = " The minimum active bond to become and maintain the role of a validator."]                pub fn min_validator_bond (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "MinValidatorBond",
                        vec![],
                        [
                            48u8, 105u8, 85u8, 178u8, 142u8, 208u8, 208u8,
                            19u8, 236u8, 130u8, 129u8, 169u8, 35u8, 245u8,
                            66u8, 182u8, 92u8, 20u8, 22u8, 109u8, 155u8, 174u8,
                            87u8, 118u8, 242u8, 216u8, 193u8, 154u8, 4u8, 5u8,
                            66u8, 56u8,
                        ],
                    )
                }
                #[doc = " The minimum active nominator stake of the last successful election."]                pub fn minimum_active_stake (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "MinimumActiveStake",
                        vec![],
                        [
                            172u8, 190u8, 228u8, 47u8, 47u8, 192u8, 182u8,
                            59u8, 9u8, 18u8, 103u8, 46u8, 175u8, 54u8, 17u8,
                            79u8, 89u8, 107u8, 255u8, 200u8, 182u8, 107u8,
                            89u8, 157u8, 55u8, 16u8, 77u8, 46u8, 154u8, 169u8,
                            103u8, 151u8,
                        ],
                    )
                }
                #[doc = " The minimum amount of commission that validators can set."]
                #[doc = ""]
                #[doc = " If set to `0`, no limit exists."]                pub fn min_commission (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_arithmetic :: per_things :: Perbill > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "MinCommission",
                        vec![],
                        [
                            61u8, 101u8, 69u8, 27u8, 220u8, 179u8, 5u8, 71u8,
                            66u8, 227u8, 84u8, 98u8, 18u8, 141u8, 183u8, 49u8,
                            98u8, 46u8, 123u8, 114u8, 198u8, 85u8, 15u8, 175u8,
                            243u8, 239u8, 133u8, 129u8, 146u8, 174u8, 254u8,
                            158u8,
                        ],
                    )
                }
                #[doc = " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]                pub fn ledger (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: StakingLedger > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "Ledger" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [31u8 , 205u8 , 3u8 , 165u8 , 22u8 , 22u8 , 62u8 , 92u8 , 33u8 , 189u8 , 124u8 , 120u8 , 177u8 , 70u8 , 27u8 , 242u8 , 188u8 , 184u8 , 204u8 , 188u8 , 242u8 , 140u8 , 128u8 , 230u8 , 85u8 , 99u8 , 181u8 , 173u8 , 67u8 , 252u8 , 37u8 , 236u8 ,])
                }
                #[doc = " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]                pub fn ledger_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: StakingLedger > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "Ledger",
                        Vec::new(),
                        [
                            31u8, 205u8, 3u8, 165u8, 22u8, 22u8, 62u8, 92u8,
                            33u8, 189u8, 124u8, 120u8, 177u8, 70u8, 27u8,
                            242u8, 188u8, 184u8, 204u8, 188u8, 242u8, 140u8,
                            128u8, 230u8, 85u8, 99u8, 181u8, 173u8, 67u8,
                            252u8, 37u8, 236u8,
                        ],
                    )
                }
                #[doc = " Where the reward payment should be made. Keyed by stash."]                pub fn payee (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: RewardDestination < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "Payee" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [195u8 , 125u8 , 82u8 , 213u8 , 216u8 , 64u8 , 76u8 , 63u8 , 187u8 , 163u8 , 20u8 , 230u8 , 153u8 , 13u8 , 189u8 , 232u8 , 119u8 , 118u8 , 107u8 , 17u8 , 102u8 , 245u8 , 36u8 , 42u8 , 232u8 , 137u8 , 177u8 , 165u8 , 169u8 , 246u8 , 199u8 , 57u8 ,])
                }
                #[doc = " Where the reward payment should be made. Keyed by stash."]                pub fn payee_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: RewardDestination < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "Payee",
                        Vec::new(),
                        [
                            195u8, 125u8, 82u8, 213u8, 216u8, 64u8, 76u8, 63u8,
                            187u8, 163u8, 20u8, 230u8, 153u8, 13u8, 189u8,
                            232u8, 119u8, 118u8, 107u8, 17u8, 102u8, 245u8,
                            36u8, 42u8, 232u8, 137u8, 177u8, 165u8, 169u8,
                            246u8, 199u8, 57u8,
                        ],
                    )
                }
                #[doc = " The map from (wannabe) validator stash key to the preferences of that validator."]                pub fn validators (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: ValidatorPrefs > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "Validators" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [80u8 , 77u8 , 66u8 , 18u8 , 197u8 , 250u8 , 41u8 , 185u8 , 43u8 , 24u8 , 149u8 , 164u8 , 208u8 , 60u8 , 144u8 , 29u8 , 251u8 , 195u8 , 236u8 , 196u8 , 108u8 , 58u8 , 80u8 , 115u8 , 246u8 , 66u8 , 226u8 , 241u8 , 201u8 , 172u8 , 229u8 , 152u8 ,])
                }
                #[doc = " The map from (wannabe) validator stash key to the preferences of that validator."]                pub fn validators_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: ValidatorPrefs > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "Validators",
                        Vec::new(),
                        [
                            80u8, 77u8, 66u8, 18u8, 197u8, 250u8, 41u8, 185u8,
                            43u8, 24u8, 149u8, 164u8, 208u8, 60u8, 144u8, 29u8,
                            251u8, 195u8, 236u8, 196u8, 108u8, 58u8, 80u8,
                            115u8, 246u8, 66u8, 226u8, 241u8, 201u8, 172u8,
                            229u8, 152u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]                pub fn counter_for_validators (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "CounterForValidators",
                        vec![],
                        [
                            139u8, 25u8, 223u8, 6u8, 160u8, 239u8, 212u8, 85u8,
                            36u8, 185u8, 69u8, 63u8, 21u8, 156u8, 144u8, 241u8,
                            112u8, 85u8, 49u8, 78u8, 88u8, 11u8, 8u8, 48u8,
                            118u8, 34u8, 62u8, 159u8, 239u8, 122u8, 90u8, 45u8,
                        ],
                    )
                }
                #[doc = " The maximum validator count before we stop allowing new validators to join."]
                #[doc = ""]
                #[doc = " When this value is not set, no limits are enforced."]                pub fn max_validators_count (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "MaxValidatorsCount",
                        vec![],
                        [
                            250u8, 62u8, 16u8, 68u8, 192u8, 216u8, 236u8,
                            211u8, 217u8, 9u8, 213u8, 49u8, 41u8, 37u8, 58u8,
                            62u8, 131u8, 112u8, 64u8, 26u8, 133u8, 7u8, 130u8,
                            1u8, 71u8, 158u8, 14u8, 55u8, 169u8, 239u8, 223u8,
                            245u8,
                        ],
                    )
                }
                #[doc = " The map from nominator stash key to their nomination preferences, namely the validators that"]
                #[doc = " they wish to support."]
                #[doc = ""]
                #[doc = " Note that the keys of this storage map might become non-decodable in case the"]
                #[doc = " [`Config::MaxNominations`] configuration is decreased. In this rare case, these nominators"]
                #[doc = " are still existent in storage, their key is correct and retrievable (i.e. `contains_key`"]
                #[doc = " indicates that they exist), but their value cannot be decoded. Therefore, the non-decodable"]
                #[doc = " nominators will effectively not-exist, until they re-submit their preferences such that it"]
                #[doc = " is within the bounds of the newly set `Config::MaxNominations`."]
                #[doc = ""]
                #[doc = " This implies that `::iter_keys().count()` and `::iter().count()` might return different"]
                #[doc = " values for this map. Moreover, the main `::count()` is aligned with the former, namely the"]
                #[doc = " number of keys that exist."]
                #[doc = ""]
                #[doc = " Lastly, if any of the nominators become non-decodable, they can be chilled immediately via"]
                #[doc = " [`Call::chill_other`] dispatchable by anyone."]                pub fn nominators (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: Nominations > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "Nominators" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [1u8 , 154u8 , 55u8 , 170u8 , 215u8 , 64u8 , 56u8 , 83u8 , 254u8 , 19u8 , 152u8 , 85u8 , 164u8 , 171u8 , 206u8 , 129u8 , 184u8 , 45u8 , 221u8 , 181u8 , 229u8 , 133u8 , 200u8 , 231u8 , 16u8 , 146u8 , 247u8 , 21u8 , 77u8 , 122u8 , 165u8 , 134u8 ,])
                }
                #[doc = " The map from nominator stash key to their nomination preferences, namely the validators that"]
                #[doc = " they wish to support."]
                #[doc = ""]
                #[doc = " Note that the keys of this storage map might become non-decodable in case the"]
                #[doc = " [`Config::MaxNominations`] configuration is decreased. In this rare case, these nominators"]
                #[doc = " are still existent in storage, their key is correct and retrievable (i.e. `contains_key`"]
                #[doc = " indicates that they exist), but their value cannot be decoded. Therefore, the non-decodable"]
                #[doc = " nominators will effectively not-exist, until they re-submit their preferences such that it"]
                #[doc = " is within the bounds of the newly set `Config::MaxNominations`."]
                #[doc = ""]
                #[doc = " This implies that `::iter_keys().count()` and `::iter().count()` might return different"]
                #[doc = " values for this map. Moreover, the main `::count()` is aligned with the former, namely the"]
                #[doc = " number of keys that exist."]
                #[doc = ""]
                #[doc = " Lastly, if any of the nominators become non-decodable, they can be chilled immediately via"]
                #[doc = " [`Call::chill_other`] dispatchable by anyone."]                pub fn nominators_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: Nominations > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "Nominators",
                        Vec::new(),
                        [
                            1u8, 154u8, 55u8, 170u8, 215u8, 64u8, 56u8, 83u8,
                            254u8, 19u8, 152u8, 85u8, 164u8, 171u8, 206u8,
                            129u8, 184u8, 45u8, 221u8, 181u8, 229u8, 133u8,
                            200u8, 231u8, 16u8, 146u8, 247u8, 21u8, 77u8,
                            122u8, 165u8, 134u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]                pub fn counter_for_nominators (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "CounterForNominators",
                        vec![],
                        [
                            31u8, 94u8, 130u8, 138u8, 75u8, 8u8, 38u8, 162u8,
                            181u8, 5u8, 125u8, 116u8, 9u8, 51u8, 22u8, 234u8,
                            40u8, 117u8, 215u8, 46u8, 82u8, 117u8, 225u8, 1u8,
                            9u8, 208u8, 83u8, 63u8, 39u8, 187u8, 207u8, 191u8,
                        ],
                    )
                }
                #[doc = " The maximum nominator count before we stop allowing new validators to join."]
                #[doc = ""]
                #[doc = " When this value is not set, no limits are enforced."]                pub fn max_nominators_count (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "MaxNominatorsCount",
                        vec![],
                        [
                            180u8, 190u8, 180u8, 66u8, 235u8, 173u8, 76u8,
                            160u8, 197u8, 92u8, 96u8, 165u8, 220u8, 188u8,
                            32u8, 119u8, 3u8, 73u8, 86u8, 49u8, 104u8, 17u8,
                            186u8, 98u8, 221u8, 175u8, 109u8, 254u8, 207u8,
                            245u8, 125u8, 179u8,
                        ],
                    )
                }
                #[doc = " The current era index."]
                #[doc = ""]
                #[doc = " This is the latest planned era, depending on how the Session pallet queues the validator"]
                #[doc = " set, it might be active or not."]                pub fn current_era (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "CurrentEra",
                        vec![],
                        [
                            105u8, 150u8, 49u8, 122u8, 4u8, 78u8, 8u8, 121u8,
                            34u8, 136u8, 157u8, 227u8, 59u8, 139u8, 7u8, 253u8,
                            7u8, 10u8, 117u8, 71u8, 240u8, 74u8, 86u8, 36u8,
                            198u8, 37u8, 153u8, 93u8, 196u8, 22u8, 192u8,
                            243u8,
                        ],
                    )
                }
                #[doc = " The active era information, it holds index and start."]
                #[doc = ""]
                #[doc = " The active era is the era being currently rewarded. Validator set of this era must be"]
                #[doc = " equal to [`SessionInterface::validators`]."]                pub fn active_era (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: ActiveEraInfo > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ActiveEra",
                        vec![],
                        [
                            15u8, 112u8, 251u8, 183u8, 108u8, 61u8, 28u8, 71u8,
                            44u8, 150u8, 162u8, 4u8, 143u8, 121u8, 11u8, 37u8,
                            83u8, 29u8, 193u8, 21u8, 210u8, 116u8, 190u8,
                            236u8, 213u8, 235u8, 49u8, 97u8, 189u8, 142u8,
                            251u8, 124u8,
                        ],
                    )
                }
                #[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
                #[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]                pub fn eras_start_session_index (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "ErasStartSessionIndex" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [92u8 , 157u8 , 168u8 , 144u8 , 132u8 , 3u8 , 212u8 , 80u8 , 230u8 , 229u8 , 251u8 , 218u8 , 97u8 , 55u8 , 79u8 , 100u8 , 163u8 , 91u8 , 32u8 , 246u8 , 122u8 , 78u8 , 149u8 , 214u8 , 103u8 , 249u8 , 119u8 , 20u8 , 101u8 , 116u8 , 110u8 , 185u8 ,])
                }
                #[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
                #[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]                pub fn eras_start_session_index_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ErasStartSessionIndex",
                        Vec::new(),
                        [
                            92u8, 157u8, 168u8, 144u8, 132u8, 3u8, 212u8, 80u8,
                            230u8, 229u8, 251u8, 218u8, 97u8, 55u8, 79u8,
                            100u8, 163u8, 91u8, 32u8, 246u8, 122u8, 78u8,
                            149u8, 214u8, 103u8, 249u8, 119u8, 20u8, 101u8,
                            116u8, 110u8, 185u8,
                        ],
                    )
                }
                #[doc = " Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]                pub fn eras_stakers (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > , _1 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: Exposure < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "ErasStakers" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat) , :: subxt :: storage :: address :: StorageMapKey :: new (_1 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [192u8 , 50u8 , 152u8 , 151u8 , 92u8 , 180u8 , 206u8 , 15u8 , 139u8 , 210u8 , 128u8 , 65u8 , 92u8 , 253u8 , 43u8 , 35u8 , 139u8 , 171u8 , 73u8 , 185u8 , 32u8 , 78u8 , 20u8 , 197u8 , 154u8 , 90u8 , 233u8 , 231u8 , 23u8 , 22u8 , 187u8 , 156u8 ,])
                }
                #[doc = " Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]                pub fn eras_stakers_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: Exposure < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ErasStakers",
                        Vec::new(),
                        [
                            192u8, 50u8, 152u8, 151u8, 92u8, 180u8, 206u8,
                            15u8, 139u8, 210u8, 128u8, 65u8, 92u8, 253u8, 43u8,
                            35u8, 139u8, 171u8, 73u8, 185u8, 32u8, 78u8, 20u8,
                            197u8, 154u8, 90u8, 233u8, 231u8, 23u8, 22u8,
                            187u8, 156u8,
                        ],
                    )
                }
                #[doc = " Clipped Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the"]
                #[doc = " `T::MaxNominatorRewardedPerValidator` biggest stakers."]
                #[doc = " (Note: the field `total` and `own` of the exposure remains unchanged)."]
                #[doc = " This is used to limit the i/o cost for the nominator payout."]
                #[doc = ""]
                #[doc = " This is keyed fist by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]                pub fn eras_stakers_clipped (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > , _1 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: Exposure < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "ErasStakersClipped" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat) , :: subxt :: storage :: address :: StorageMapKey :: new (_1 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [43u8 , 159u8 , 113u8 , 223u8 , 122u8 , 169u8 , 98u8 , 153u8 , 26u8 , 55u8 , 71u8 , 119u8 , 174u8 , 48u8 , 158u8 , 45u8 , 214u8 , 26u8 , 136u8 , 215u8 , 46u8 , 161u8 , 185u8 , 17u8 , 174u8 , 204u8 , 206u8 , 246u8 , 49u8 , 87u8 , 134u8 , 169u8 ,])
                }
                #[doc = " Clipped Exposure of validator at era."]
                #[doc = ""]
                #[doc = " This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the"]
                #[doc = " `T::MaxNominatorRewardedPerValidator` biggest stakers."]
                #[doc = " (Note: the field `total` and `own` of the exposure remains unchanged)."]
                #[doc = " This is used to limit the i/o cost for the nominator payout."]
                #[doc = ""]
                #[doc = " This is keyed fist by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]
                #[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]                pub fn eras_stakers_clipped_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: Exposure < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ErasStakersClipped",
                        Vec::new(),
                        [
                            43u8, 159u8, 113u8, 223u8, 122u8, 169u8, 98u8,
                            153u8, 26u8, 55u8, 71u8, 119u8, 174u8, 48u8, 158u8,
                            45u8, 214u8, 26u8, 136u8, 215u8, 46u8, 161u8,
                            185u8, 17u8, 174u8, 204u8, 206u8, 246u8, 49u8,
                            87u8, 134u8, 169u8,
                        ],
                    )
                }
                #[doc = " Similar to `ErasStakers`, this holds the preferences of validators."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]                pub fn eras_validator_prefs (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > , _1 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: ValidatorPrefs > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "ErasValidatorPrefs" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat) , :: subxt :: storage :: address :: StorageMapKey :: new (_1 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [6u8 , 196u8 , 209u8 , 138u8 , 252u8 , 18u8 , 203u8 , 86u8 , 129u8 , 62u8 , 4u8 , 56u8 , 234u8 , 114u8 , 141u8 , 136u8 , 127u8 , 224u8 , 142u8 , 89u8 , 150u8 , 33u8 , 31u8 , 50u8 , 140u8 , 108u8 , 124u8 , 77u8 , 188u8 , 102u8 , 230u8 , 174u8 ,])
                }
                #[doc = " Similar to `ErasStakers`, this holds the preferences of validators."]
                #[doc = ""]
                #[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
                #[doc = ""]
                #[doc = " Is it removed after `HISTORY_DEPTH` eras."]                pub fn eras_validator_prefs_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: ValidatorPrefs > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ErasValidatorPrefs",
                        Vec::new(),
                        [
                            6u8, 196u8, 209u8, 138u8, 252u8, 18u8, 203u8, 86u8,
                            129u8, 62u8, 4u8, 56u8, 234u8, 114u8, 141u8, 136u8,
                            127u8, 224u8, 142u8, 89u8, 150u8, 33u8, 31u8, 50u8,
                            140u8, 108u8, 124u8, 77u8, 188u8, 102u8, 230u8,
                            174u8,
                        ],
                    )
                }
                #[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]                pub fn eras_validator_reward (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "ErasValidatorReward" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [87u8 , 80u8 , 156u8 , 123u8 , 107u8 , 77u8 , 203u8 , 37u8 , 231u8 , 84u8 , 124u8 , 155u8 , 227u8 , 212u8 , 212u8 , 179u8 , 84u8 , 161u8 , 223u8 , 255u8 , 254u8 , 107u8 , 52u8 , 89u8 , 98u8 , 169u8 , 136u8 , 241u8 , 104u8 , 3u8 , 244u8 , 161u8 ,])
                }
                #[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
                #[doc = ""]
                #[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]                pub fn eras_validator_reward_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ErasValidatorReward",
                        Vec::new(),
                        [
                            87u8, 80u8, 156u8, 123u8, 107u8, 77u8, 203u8, 37u8,
                            231u8, 84u8, 124u8, 155u8, 227u8, 212u8, 212u8,
                            179u8, 84u8, 161u8, 223u8, 255u8, 254u8, 107u8,
                            52u8, 89u8, 98u8, 169u8, 136u8, 241u8, 104u8, 3u8,
                            244u8, 161u8,
                        ],
                    )
                }
                #[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
                #[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]                pub fn eras_reward_points (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: EraRewardPoints < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "ErasRewardPoints" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [194u8 , 29u8 , 20u8 , 83u8 , 200u8 , 47u8 , 158u8 , 102u8 , 88u8 , 65u8 , 24u8 , 255u8 , 120u8 , 178u8 , 23u8 , 232u8 , 15u8 , 64u8 , 206u8 , 0u8 , 170u8 , 40u8 , 18u8 , 149u8 , 45u8 , 90u8 , 179u8 , 127u8 , 52u8 , 59u8 , 37u8 , 192u8 ,])
                }
                #[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
                #[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]                pub fn eras_reward_points_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: EraRewardPoints < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ErasRewardPoints",
                        Vec::new(),
                        [
                            194u8, 29u8, 20u8, 83u8, 200u8, 47u8, 158u8, 102u8,
                            88u8, 65u8, 24u8, 255u8, 120u8, 178u8, 23u8, 232u8,
                            15u8, 64u8, 206u8, 0u8, 170u8, 40u8, 18u8, 149u8,
                            45u8, 90u8, 179u8, 127u8, 52u8, 59u8, 37u8, 192u8,
                        ],
                    )
                }
                #[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
                #[doc = " If total hasn't been set or has been removed then 0 stake is returned."]                pub fn eras_total_stake (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "ErasTotalStake" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [224u8 , 240u8 , 168u8 , 69u8 , 148u8 , 140u8 , 249u8 , 240u8 , 4u8 , 46u8 , 77u8 , 11u8 , 224u8 , 65u8 , 26u8 , 239u8 , 1u8 , 110u8 , 53u8 , 11u8 , 247u8 , 235u8 , 142u8 , 234u8 , 22u8 , 43u8 , 24u8 , 36u8 , 37u8 , 43u8 , 170u8 , 40u8 ,])
                }
                #[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
                #[doc = " If total hasn't been set or has been removed then 0 stake is returned."]                pub fn eras_total_stake_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ErasTotalStake",
                        Vec::new(),
                        [
                            224u8, 240u8, 168u8, 69u8, 148u8, 140u8, 249u8,
                            240u8, 4u8, 46u8, 77u8, 11u8, 224u8, 65u8, 26u8,
                            239u8, 1u8, 110u8, 53u8, 11u8, 247u8, 235u8, 142u8,
                            234u8, 22u8, 43u8, 24u8, 36u8, 37u8, 43u8, 170u8,
                            40u8,
                        ],
                    )
                }
                #[doc = " Mode of era forcing."]                pub fn force_era (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: Forcing > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ForceEra",
                        vec![],
                        [
                            221u8, 41u8, 71u8, 21u8, 28u8, 193u8, 65u8, 97u8,
                            103u8, 37u8, 145u8, 146u8, 183u8, 194u8, 57u8,
                            131u8, 214u8, 136u8, 68u8, 156u8, 140u8, 194u8,
                            69u8, 151u8, 115u8, 177u8, 92u8, 147u8, 29u8, 40u8,
                            41u8, 31u8,
                        ],
                    )
                }
                #[doc = " The percentage of the slash that is distributed to reporters."]
                #[doc = ""]
                #[doc = " The rest of the slashed value is handled by the `Slash`."]                pub fn slash_reward_fraction (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_arithmetic :: per_things :: Perbill > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "SlashRewardFraction",
                        vec![],
                        [
                            167u8, 79u8, 143u8, 202u8, 199u8, 100u8, 129u8,
                            162u8, 23u8, 165u8, 106u8, 170u8, 244u8, 86u8,
                            144u8, 242u8, 65u8, 207u8, 115u8, 224u8, 231u8,
                            155u8, 55u8, 139u8, 101u8, 129u8, 242u8, 196u8,
                            130u8, 50u8, 3u8, 117u8,
                        ],
                    )
                }
                #[doc = " The amount of currency given to reporters of a slash event which was"]
                #[doc = " canceled by extraordinary circumstances (e.g. governance)."]                pub fn canceled_slash_payout (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "CanceledSlashPayout",
                        vec![],
                        [
                            126u8, 218u8, 66u8, 92u8, 82u8, 124u8, 145u8,
                            161u8, 40u8, 176u8, 14u8, 211u8, 178u8, 216u8, 8u8,
                            156u8, 83u8, 14u8, 91u8, 15u8, 200u8, 170u8, 3u8,
                            127u8, 141u8, 139u8, 151u8, 98u8, 74u8, 96u8,
                            238u8, 29u8,
                        ],
                    )
                }
                #[doc = " All unapplied slashes that are queued for later."]                pub fn unapplied_slashes (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < runtime_types :: pallet_staking :: UnappliedSlash < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "UnappliedSlashes" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [130u8 , 4u8 , 163u8 , 163u8 , 28u8 , 85u8 , 34u8 , 156u8 , 47u8 , 125u8 , 57u8 , 0u8 , 133u8 , 176u8 , 130u8 , 2u8 , 175u8 , 180u8 , 167u8 , 203u8 , 230u8 , 82u8 , 198u8 , 183u8 , 55u8 , 82u8 , 221u8 , 248u8 , 100u8 , 173u8 , 206u8 , 151u8 ,])
                }
                #[doc = " All unapplied slashes that are queued for later."]                pub fn unapplied_slashes_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < runtime_types :: pallet_staking :: UnappliedSlash < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "UnappliedSlashes",
                        Vec::new(),
                        [
                            130u8, 4u8, 163u8, 163u8, 28u8, 85u8, 34u8, 156u8,
                            47u8, 125u8, 57u8, 0u8, 133u8, 176u8, 130u8, 2u8,
                            175u8, 180u8, 167u8, 203u8, 230u8, 82u8, 198u8,
                            183u8, 55u8, 82u8, 221u8, 248u8, 100u8, 173u8,
                            206u8, 151u8,
                        ],
                    )
                }
                #[doc = " A mapping from still-bonded eras to the first session index of that era."]
                #[doc = ""]
                #[doc = " Must contains information for eras for the range:"]
                #[doc = " `[active_era - bounding_duration; active_era]`"]                pub fn bonded_eras (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < (:: core :: primitive :: u32 , :: core :: primitive :: u32 ,) > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "BondedEras",
                        vec![],
                        [
                            243u8, 162u8, 236u8, 198u8, 122u8, 182u8, 37u8,
                            55u8, 171u8, 156u8, 235u8, 223u8, 226u8, 129u8,
                            89u8, 206u8, 2u8, 155u8, 222u8, 154u8, 116u8,
                            124u8, 4u8, 119u8, 155u8, 94u8, 248u8, 30u8, 171u8,
                            51u8, 78u8, 106u8,
                        ],
                    )
                }
                #[doc = " All slashing events on validators, mapped by era to the highest slash proportion"]
                #[doc = " and slash value of the era."]                pub fn validator_slash_in_era (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > , _1 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (runtime_types :: sp_arithmetic :: per_things :: Perbill , :: core :: primitive :: u128 ,) > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "ValidatorSlashInEra" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat) , :: subxt :: storage :: address :: StorageMapKey :: new (_1 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [237u8 , 80u8 , 3u8 , 237u8 , 9u8 , 40u8 , 212u8 , 15u8 , 251u8 , 196u8 , 85u8 , 29u8 , 27u8 , 151u8 , 98u8 , 122u8 , 189u8 , 147u8 , 205u8 , 40u8 , 202u8 , 194u8 , 158u8 , 96u8 , 138u8 , 16u8 , 116u8 , 71u8 , 140u8 , 163u8 , 121u8 , 197u8 ,])
                }
                #[doc = " All slashing events on validators, mapped by era to the highest slash proportion"]
                #[doc = " and slash value of the era."]                pub fn validator_slash_in_era_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (runtime_types :: sp_arithmetic :: per_things :: Perbill , :: core :: primitive :: u128 ,) > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ValidatorSlashInEra",
                        Vec::new(),
                        [
                            237u8, 80u8, 3u8, 237u8, 9u8, 40u8, 212u8, 15u8,
                            251u8, 196u8, 85u8, 29u8, 27u8, 151u8, 98u8, 122u8,
                            189u8, 147u8, 205u8, 40u8, 202u8, 194u8, 158u8,
                            96u8, 138u8, 16u8, 116u8, 71u8, 140u8, 163u8,
                            121u8, 197u8,
                        ],
                    )
                }
                #[doc = " All slashing events on nominators, mapped by era to the highest slash value of the era."]                pub fn nominator_slash_in_era (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > , _1 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "NominatorSlashInEra" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat) , :: subxt :: storage :: address :: StorageMapKey :: new (_1 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [249u8 , 85u8 , 170u8 , 41u8 , 179u8 , 194u8 , 180u8 , 12u8 , 53u8 , 101u8 , 80u8 , 96u8 , 166u8 , 71u8 , 239u8 , 23u8 , 153u8 , 19u8 , 152u8 , 38u8 , 138u8 , 136u8 , 221u8 , 200u8 , 18u8 , 165u8 , 26u8 , 228u8 , 195u8 , 199u8 , 62u8 , 4u8 ,])
                }
                #[doc = " All slashing events on nominators, mapped by era to the highest slash value of the era."]                pub fn nominator_slash_in_era_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u128 > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "NominatorSlashInEra",
                        Vec::new(),
                        [
                            249u8, 85u8, 170u8, 41u8, 179u8, 194u8, 180u8,
                            12u8, 53u8, 101u8, 80u8, 96u8, 166u8, 71u8, 239u8,
                            23u8, 153u8, 19u8, 152u8, 38u8, 138u8, 136u8,
                            221u8, 200u8, 18u8, 165u8, 26u8, 228u8, 195u8,
                            199u8, 62u8, 4u8,
                        ],
                    )
                }
                #[doc = " Slashing spans for stash accounts."]                pub fn slashing_spans (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: slashing :: SlashingSpans > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "SlashingSpans" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [106u8 , 115u8 , 118u8 , 52u8 , 89u8 , 77u8 , 246u8 , 5u8 , 255u8 , 204u8 , 44u8 , 5u8 , 66u8 , 36u8 , 227u8 , 252u8 , 86u8 , 159u8 , 186u8 , 152u8 , 196u8 , 21u8 , 74u8 , 201u8 , 133u8 , 93u8 , 142u8 , 191u8 , 20u8 , 27u8 , 218u8 , 157u8 ,])
                }
                #[doc = " Slashing spans for stash accounts."]                pub fn slashing_spans_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: slashing :: SlashingSpans > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "SlashingSpans",
                        Vec::new(),
                        [
                            106u8, 115u8, 118u8, 52u8, 89u8, 77u8, 246u8, 5u8,
                            255u8, 204u8, 44u8, 5u8, 66u8, 36u8, 227u8, 252u8,
                            86u8, 159u8, 186u8, 152u8, 196u8, 21u8, 74u8,
                            201u8, 133u8, 93u8, 142u8, 191u8, 20u8, 27u8,
                            218u8, 157u8,
                        ],
                    )
                }
                #[doc = " Records information about the maximum slash of a stash within a slashing span,"]
                #[doc = " as well as how much reward has been paid out."]                pub fn span_slash (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > , _1 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: slashing :: SpanRecord < :: core :: primitive :: u128 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Staking" , "SpanSlash" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new ((_0 . borrow () , _1 . borrow ()) , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [160u8 , 63u8 , 115u8 , 190u8 , 233u8 , 148u8 , 75u8 , 3u8 , 11u8 , 59u8 , 184u8 , 220u8 , 205u8 , 64u8 , 28u8 , 190u8 , 116u8 , 210u8 , 225u8 , 230u8 , 224u8 , 163u8 , 103u8 , 157u8 , 100u8 , 29u8 , 86u8 , 167u8 , 84u8 , 217u8 , 109u8 , 200u8 ,])
                }
                #[doc = " Records information about the maximum slash of a stash within a slashing span,"]
                #[doc = " as well as how much reward has been paid out."]                pub fn span_slash_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: slashing :: SpanRecord < :: core :: primitive :: u128 > > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "SpanSlash",
                        Vec::new(),
                        [
                            160u8, 63u8, 115u8, 190u8, 233u8, 148u8, 75u8, 3u8,
                            11u8, 59u8, 184u8, 220u8, 205u8, 64u8, 28u8, 190u8,
                            116u8, 210u8, 225u8, 230u8, 224u8, 163u8, 103u8,
                            157u8, 100u8, 29u8, 86u8, 167u8, 84u8, 217u8,
                            109u8, 200u8,
                        ],
                    )
                }
                #[doc = " The last planned session scheduled by the session pallet."]
                #[doc = ""]
                #[doc = " This is basically in sync with the call to [`pallet_session::SessionManager::new_session`]."]                pub fn current_planned_session (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "CurrentPlannedSession",
                        vec![],
                        [
                            38u8, 22u8, 56u8, 250u8, 17u8, 154u8, 99u8, 37u8,
                            155u8, 253u8, 100u8, 117u8, 5u8, 239u8, 31u8,
                            190u8, 53u8, 241u8, 11u8, 185u8, 163u8, 227u8,
                            10u8, 77u8, 210u8, 64u8, 156u8, 218u8, 105u8, 16u8,
                            1u8, 57u8,
                        ],
                    )
                }
                #[doc = " Indices of validators that have offended in the active era and whether they are currently"]
                #[doc = " disabled."]
                #[doc = ""]
                #[doc = " This value should be a superset of disabled validators since not all offences lead to the"]
                #[doc = " validator being disabled (if there was no slash). This is needed to track the percentage of"]
                #[doc = " validators that have offended in the current era, ensuring a new era is forced if"]
                #[doc = " `OffendingValidatorsThreshold` is reached. The vec is always kept sorted so that we can find"]
                #[doc = " whether a given validator has previously offended using binary search. It gets cleared when"]
                #[doc = " the era ends."]                pub fn offending_validators (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < (:: core :: primitive :: u32 , :: core :: primitive :: bool ,) > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "OffendingValidators",
                        vec![],
                        [
                            94u8, 254u8, 0u8, 50u8, 76u8, 232u8, 51u8, 153u8,
                            118u8, 14u8, 70u8, 101u8, 112u8, 215u8, 173u8,
                            82u8, 182u8, 104u8, 167u8, 103u8, 187u8, 168u8,
                            86u8, 16u8, 51u8, 235u8, 51u8, 119u8, 38u8, 154u8,
                            42u8, 113u8,
                        ],
                    )
                }
                #[doc = " True if network has been upgraded to this version."]
                #[doc = " Storage version of the pallet."]
                #[doc = ""]
                #[doc = " This is set to v7.0.0 for new networks."]                pub fn storage_version (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_staking :: Releases > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "StorageVersion",
                        vec![],
                        [
                            70u8, 24u8, 179u8, 189u8, 168u8, 164u8, 175u8,
                            150u8, 215u8, 43u8, 18u8, 110u8, 180u8, 137u8,
                            237u8, 187u8, 185u8, 50u8, 31u8, 57u8, 16u8, 110u8,
                            6u8, 170u8, 19u8, 7u8, 160u8, 134u8, 232u8, 227u8,
                            151u8, 116u8,
                        ],
                    )
                }
                #[doc = " The threshold for when users can start calling `chill_other` for other validators /"]
                #[doc = " nominators. The threshold is compared to the actual number of validators / nominators"]
                #[doc = " (`CountFor*`) in the system compared to the configured max (`Max*Count`)."]                pub fn chill_threshold (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_arithmetic :: per_things :: Percent > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Staking",
                        "ChillThreshold",
                        vec![],
                        [
                            174u8, 165u8, 249u8, 105u8, 24u8, 151u8, 115u8,
                            166u8, 199u8, 251u8, 28u8, 5u8, 50u8, 95u8, 144u8,
                            110u8, 220u8, 76u8, 14u8, 23u8, 179u8, 41u8, 11u8,
                            248u8, 28u8, 154u8, 159u8, 255u8, 156u8, 109u8,
                            98u8, 92u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Maximum number of nominations per nominator."]
                pub fn max_nominations(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Staking",
                        "MaxNominations",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Number of eras to keep in history."]
                #[doc = ""]
                #[doc = " Following information is kept for eras in `[current_era -"]
                #[doc = " HistoryDepth, current_era]`: `ErasStakers`, `ErasStakersClipped`,"]
                #[doc = " `ErasValidatorPrefs`, `ErasValidatorReward`, `ErasRewardPoints`,"]
                #[doc = " `ErasTotalStake`, `ErasStartSessionIndex`,"]
                #[doc = " `StakingLedger.claimed_rewards`."]
                #[doc = ""]
                #[doc = " Must be more than the number of eras delayed by session."]
                #[doc = " I.e. active era must always be in history. I.e. `active_era >"]
                #[doc = " current_era - history_depth` must be guaranteed."]
                #[doc = ""]
                #[doc = " If migrating an existing pallet from storage value to config value,"]
                #[doc = " this should be set to same value or greater as in storage."]
                #[doc = ""]
                #[doc = " Note: `HistoryDepth` is used as the upper bound for the `BoundedVec`"]
                #[doc = " item `StakingLedger.claimed_rewards`. Setting this value lower than"]
                #[doc = " the existing value can lead to inconsistencies in the"]
                #[doc = " `StakingLedger` and will need to be handled properly in a migration."]
                #[doc = " The test `reducing_history_depth_abrupt` shows this effect."]
                pub fn history_depth(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Staking",
                        "HistoryDepth",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Number of sessions per era."]
                pub fn sessions_per_era(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Staking",
                        "SessionsPerEra",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Number of eras that staked funds must remain bonded for."]
                pub fn bonding_duration(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Staking",
                        "BondingDuration",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Number of eras that slashes are deferred by, after computation."]
                #[doc = ""]
                #[doc = " This should be less than the bonding duration. Set to 0 if slashes"]
                #[doc = " should be applied immediately, without opportunity for intervention."]
                pub fn slash_defer_duration(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Staking",
                        "SlashDeferDuration",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of nominators rewarded for each validator."]
                #[doc = ""]
                #[doc = " For each validator only the `$MaxNominatorRewardedPerValidator` biggest stakers can"]
                #[doc = " claim their reward. This used to limit the i/o cost for the nominator payout."]
                pub fn max_nominator_rewarded_per_validator(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Staking",
                        "MaxNominatorRewardedPerValidator",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of `unlocking` chunks a [`StakingLedger`] can"]
                #[doc = " have. Effectively determines how many unique eras a staker may be"]
                #[doc = " unbonding in."]
                #[doc = ""]
                #[doc = " Note: `MaxUnlockingChunks` is used as the upper bound for the"]
                #[doc = " `BoundedVec` item `StakingLedger.unlocking`. Setting this value"]
                #[doc = " lower than the existing value can lead to inconsistencies in the"]
                #[doc = " `StakingLedger` and will need to be handled properly in a runtime"]
                #[doc = " migration. The test `reducing_max_unlocking_chunks_abrupt` shows"]
                #[doc = " this effect."]
                pub fn max_unlocking_chunks(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Staking",
                        "MaxUnlockingChunks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod session {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetKeys {
                pub keys:
                    runtime_types::dkg_standalone_runtime::opaque::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct PurgeKeys;
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Sets the session key(s) of the function caller to `keys`."]
                #[doc = "Allows an account to set its session key prior to becoming a validator."]
                #[doc = "This doesn't take effect until the next session."]
                #[doc = ""]
                #[doc = "The dispatch origin of this function must be signed."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
                #[doc = "  `T::Keys::key_ids()` which is fixed."]
                #[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
                #[doc = "- DbWrites: `origin account`, `NextKeys`"]
                #[doc = "- DbReads per key id: `KeyOwner`"]
                #[doc = "- DbWrites per key id: `KeyOwner`"]
                #[doc = "# </weight>"]
                pub fn set_keys(
                    &self,
                    keys : runtime_types :: dkg_standalone_runtime :: opaque :: SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetKeys> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Session",
                        "set_keys",
                        SetKeys { keys, proof },
                        [
                            96u8, 118u8, 0u8, 203u8, 157u8, 82u8, 2u8, 238u8,
                            94u8, 39u8, 248u8, 255u8, 155u8, 43u8, 191u8, 64u8,
                            200u8, 223u8, 108u8, 198u8, 114u8, 0u8, 53u8, 15u8,
                            203u8, 47u8, 205u8, 234u8, 131u8, 177u8, 139u8,
                            185u8,
                        ],
                    )
                }
                #[doc = "Removes any session key(s) of the function caller."]
                #[doc = ""]
                #[doc = "This doesn't take effect until the next session."]
                #[doc = ""]
                #[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
                #[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
                #[doc = "means being a controller account) or directly convertible into a validator ID (which"]
                #[doc = "usually means being a stash account)."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
                #[doc = "  of `T::Keys::key_ids()` which is fixed."]
                #[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
                #[doc = "- DbWrites: `NextKeys`, `origin account`"]
                #[doc = "- DbWrites per key id: `KeyOwner`"]
                #[doc = "# </weight>"]
                pub fn purge_keys(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<PurgeKeys> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Session",
                        "purge_keys",
                        PurgeKeys {},
                        [
                            200u8, 255u8, 4u8, 213u8, 188u8, 92u8, 99u8, 116u8,
                            163u8, 152u8, 29u8, 35u8, 133u8, 119u8, 246u8,
                            44u8, 91u8, 31u8, 145u8, 23u8, 213u8, 64u8, 71u8,
                            242u8, 207u8, 239u8, 231u8, 37u8, 61u8, 63u8,
                            190u8, 35u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "New session has happened. Note that the argument is the session index, not the"]
            #[doc = "block number as the type might suggest."]
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current set of validators."]                pub fn validators (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "Validators",
                        vec![],
                        [
                            144u8, 235u8, 200u8, 43u8, 151u8, 57u8, 147u8,
                            172u8, 201u8, 202u8, 242u8, 96u8, 57u8, 76u8,
                            124u8, 77u8, 42u8, 113u8, 218u8, 220u8, 230u8,
                            32u8, 151u8, 152u8, 172u8, 106u8, 60u8, 227u8,
                            122u8, 118u8, 137u8, 68u8,
                        ],
                    )
                }
                #[doc = " Current index of the session."]                pub fn current_index (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "CurrentIndex",
                        vec![],
                        [
                            148u8, 179u8, 159u8, 15u8, 197u8, 95u8, 214u8,
                            30u8, 209u8, 251u8, 183u8, 231u8, 91u8, 25u8,
                            181u8, 191u8, 143u8, 252u8, 227u8, 80u8, 159u8,
                            66u8, 194u8, 67u8, 113u8, 74u8, 111u8, 91u8, 218u8,
                            187u8, 130u8, 40u8,
                        ],
                    )
                }
                #[doc = " True if the underlying economic identities or weighting behind the validators"]
                #[doc = " has changed in the queued validator set."]                pub fn queued_changed (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: bool > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "QueuedChanged",
                        vec![],
                        [
                            105u8, 140u8, 235u8, 218u8, 96u8, 100u8, 252u8,
                            10u8, 58u8, 221u8, 244u8, 251u8, 67u8, 91u8, 80u8,
                            202u8, 152u8, 42u8, 50u8, 113u8, 200u8, 247u8,
                            59u8, 213u8, 77u8, 195u8, 1u8, 150u8, 220u8, 18u8,
                            245u8, 46u8,
                        ],
                    )
                }
                #[doc = " The queued keys for the next session. When the next session begins, these keys"]
                #[doc = " will be used to determine the validator's session keys."]                pub fn queued_keys (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , runtime_types :: dkg_standalone_runtime :: opaque :: SessionKeys ,) > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "QueuedKeys",
                        vec![],
                        [
                            6u8, 142u8, 45u8, 83u8, 248u8, 49u8, 224u8, 50u8,
                            60u8, 233u8, 70u8, 144u8, 29u8, 121u8, 17u8, 65u8,
                            76u8, 193u8, 222u8, 91u8, 128u8, 19u8, 85u8, 65u8,
                            58u8, 89u8, 159u8, 156u8, 138u8, 76u8, 181u8,
                            188u8,
                        ],
                    )
                }
                #[doc = " Indices of disabled validators."]
                #[doc = ""]
                #[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
                #[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
                #[doc = " a new set of identities."]                pub fn disabled_validators (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < :: core :: primitive :: u32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "DisabledValidators",
                        vec![],
                        [
                            135u8, 22u8, 22u8, 97u8, 82u8, 217u8, 144u8, 141u8,
                            121u8, 240u8, 189u8, 16u8, 176u8, 88u8, 177u8,
                            31u8, 20u8, 242u8, 73u8, 104u8, 11u8, 110u8, 214u8,
                            34u8, 52u8, 217u8, 106u8, 33u8, 174u8, 174u8,
                            198u8, 84u8,
                        ],
                    )
                }
                #[doc = " The next session keys for a validator."]                pub fn next_keys (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: dkg_standalone_runtime :: opaque :: SessionKeys > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Session" , "NextKeys" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [227u8 , 242u8 , 225u8 , 78u8 , 154u8 , 151u8 , 147u8 , 206u8 , 94u8 , 82u8 , 85u8 , 255u8 , 86u8 , 111u8 , 54u8 , 199u8 , 116u8 , 193u8 , 170u8 , 217u8 , 87u8 , 169u8 , 81u8 , 99u8 , 247u8 , 214u8 , 241u8 , 221u8 , 116u8 , 133u8 , 146u8 , 117u8 ,])
                }
                #[doc = " The next session keys for a validator."]                pub fn next_keys_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: dkg_standalone_runtime :: opaque :: SessionKeys > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "NextKeys",
                        Vec::new(),
                        [
                            227u8, 242u8, 225u8, 78u8, 154u8, 151u8, 147u8,
                            206u8, 94u8, 82u8, 85u8, 255u8, 86u8, 111u8, 54u8,
                            199u8, 116u8, 193u8, 170u8, 217u8, 87u8, 169u8,
                            81u8, 99u8, 247u8, 214u8, 241u8, 221u8, 116u8,
                            133u8, 146u8, 117u8,
                        ],
                    )
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]                pub fn key_owner (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: sp_core :: crypto :: KeyTypeId > , _1 : impl :: std :: borrow :: Borrow < [:: core :: primitive :: u8] > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Session" , "KeyOwner" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new ((_0 . borrow () , _1 . borrow ()) , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [4u8 , 91u8 , 25u8 , 84u8 , 250u8 , 201u8 , 174u8 , 129u8 , 201u8 , 58u8 , 197u8 , 199u8 , 137u8 , 240u8 , 118u8 , 33u8 , 99u8 , 2u8 , 195u8 , 57u8 , 53u8 , 172u8 , 0u8 , 148u8 , 203u8 , 144u8 , 149u8 , 64u8 , 135u8 , 254u8 , 242u8 , 215u8 ,])
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]                pub fn key_owner_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "KeyOwner",
                        Vec::new(),
                        [
                            4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8,
                            201u8, 58u8, 197u8, 199u8, 137u8, 240u8, 118u8,
                            33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8, 0u8,
                            148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8,
                            242u8, 215u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod historical {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Mapping from historical session indices to session-data root hash and validator count."]                pub fn historical_sessions (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: subxt :: ext :: sp_core :: H256 , :: core :: primitive :: u32 ,) > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Historical" , "HistoricalSessions" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [94u8 , 72u8 , 245u8 , 151u8 , 214u8 , 10u8 , 12u8 , 113u8 , 13u8 , 141u8 , 176u8 , 178u8 , 115u8 , 238u8 , 224u8 , 181u8 , 18u8 , 5u8 , 71u8 , 65u8 , 189u8 , 148u8 , 161u8 , 106u8 , 24u8 , 211u8 , 72u8 , 66u8 , 221u8 , 244u8 , 117u8 , 184u8 ,])
                }
                #[doc = " Mapping from historical session indices to session-data root hash and validator count."]                pub fn historical_sessions_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: subxt :: ext :: sp_core :: H256 , :: core :: primitive :: u32 ,) > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Historical",
                        "HistoricalSessions",
                        Vec::new(),
                        [
                            94u8, 72u8, 245u8, 151u8, 214u8, 10u8, 12u8, 113u8,
                            13u8, 141u8, 176u8, 178u8, 115u8, 238u8, 224u8,
                            181u8, 18u8, 5u8, 71u8, 65u8, 189u8, 148u8, 161u8,
                            106u8, 24u8, 211u8, 72u8, 66u8, 221u8, 244u8,
                            117u8, 184u8,
                        ],
                    )
                }
                #[doc = " The range of historical sessions we store. [first, last)"]                pub fn stored_range (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: core :: primitive :: u32 , :: core :: primitive :: u32 ,) > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Historical",
                        "StoredRange",
                        vec![],
                        [
                            89u8, 239u8, 197u8, 93u8, 135u8, 62u8, 142u8,
                            237u8, 64u8, 200u8, 164u8, 4u8, 130u8, 233u8, 16u8,
                            238u8, 166u8, 206u8, 71u8, 42u8, 171u8, 84u8, 8u8,
                            245u8, 183u8, 216u8, 212u8, 16u8, 190u8, 3u8,
                            167u8, 189u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod bridge_registry {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetMetadata {
                pub bridge_index: ::core::primitive::u32,
                pub info:
                    runtime_types::pallet_bridge_registry::types::BridgeInfo,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ForceResetIndices {
                pub resource_ids: ::std::vec::Vec<
                    runtime_types::webb_proposals::header::ResourceId,
                >,
                pub bridge_index: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set an account's identity information and reserve the appropriate deposit."]
                #[doc = ""]
                #[doc = "If the account already has identity information, the deposit is taken as part payment"]
                #[doc = "for the new deposit."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `info`: The identity information."]
                #[doc = ""]
                #[doc = "Emits `ResourceSet` if successful."]
                pub fn set_metadata(
                    &self,
                    bridge_index: ::core::primitive::u32,
                    info : runtime_types :: pallet_bridge_registry :: types :: BridgeInfo,
                ) -> ::subxt::tx::StaticTxPayload<SetMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "BridgeRegistry",
                        "set_metadata",
                        SetMetadata { bridge_index, info },
                        [
                            61u8, 127u8, 197u8, 96u8, 192u8, 145u8, 189u8,
                            42u8, 76u8, 101u8, 10u8, 85u8, 74u8, 15u8, 78u8,
                            133u8, 69u8, 145u8, 232u8, 46u8, 231u8, 201u8,
                            135u8, 143u8, 11u8, 181u8, 176u8, 40u8, 117u8,
                            87u8, 11u8, 87u8,
                        ],
                    )
                }
                pub fn force_reset_indices(
                    &self,
                    resource_ids: ::std::vec::Vec<
                        runtime_types::webb_proposals::header::ResourceId,
                    >,
                    bridge_index: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<ForceResetIndices>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "BridgeRegistry",
                        "force_reset_indices",
                        ForceResetIndices {
                            resource_ids,
                            bridge_index,
                        },
                        [
                            162u8, 5u8, 147u8, 58u8, 45u8, 163u8, 101u8, 251u8,
                            103u8, 30u8, 134u8, 106u8, 142u8, 117u8, 183u8,
                            108u8, 14u8, 208u8, 208u8, 82u8, 155u8, 61u8, 61u8,
                            62u8, 81u8, 181u8, 88u8, 220u8, 122u8, 176u8, 50u8,
                            29u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_bridge_registry::pallet::Event;
        pub mod events {
            use super::runtime_types;
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Details of the module's parameters"]                pub fn next_bridge_index (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "BridgeRegistry",
                        "NextBridgeIndex",
                        vec![],
                        [
                            47u8, 238u8, 32u8, 185u8, 116u8, 5u8, 125u8, 28u8,
                            39u8, 110u8, 178u8, 155u8, 45u8, 72u8, 248u8,
                            161u8, 75u8, 161u8, 207u8, 101u8, 55u8, 226u8,
                            233u8, 233u8, 14u8, 62u8, 168u8, 20u8, 146u8, 8u8,
                            152u8, 222u8,
                        ],
                    )
                }
                #[doc = " Details of the module's parameters"]                pub fn bridges (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_bridge_registry :: types :: BridgeMetadata > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("BridgeRegistry" , "Bridges" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_256)] , [224u8 , 222u8 , 82u8 , 52u8 , 223u8 , 254u8 , 145u8 , 5u8 , 238u8 , 195u8 , 156u8 , 210u8 , 23u8 , 136u8 , 25u8 , 156u8 , 225u8 , 95u8 , 119u8 , 183u8 , 51u8 , 9u8 , 19u8 , 73u8 , 60u8 , 61u8 , 59u8 , 235u8 , 44u8 , 98u8 , 118u8 , 252u8 ,])
                }
                #[doc = " Details of the module's parameters"]                pub fn bridges_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_bridge_registry :: types :: BridgeMetadata > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "BridgeRegistry",
                        "Bridges",
                        Vec::new(),
                        [
                            224u8, 222u8, 82u8, 52u8, 223u8, 254u8, 145u8, 5u8,
                            238u8, 195u8, 156u8, 210u8, 23u8, 136u8, 25u8,
                            156u8, 225u8, 95u8, 119u8, 183u8, 51u8, 9u8, 19u8,
                            73u8, 60u8, 61u8, 59u8, 235u8, 44u8, 98u8, 118u8,
                            252u8,
                        ],
                    )
                }
                #[doc = " Details of the module's parameters"]                pub fn resource_to_bridge_index (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: webb_proposals :: header :: ResourceId > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("BridgeRegistry" , "ResourceToBridgeIndex" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_256)] , [130u8 , 216u8 , 179u8 , 142u8 , 98u8 , 4u8 , 178u8 , 25u8 , 179u8 , 57u8 , 140u8 , 220u8 , 202u8 , 186u8 , 74u8 , 115u8 , 145u8 , 63u8 , 69u8 , 239u8 , 102u8 , 91u8 , 1u8 , 43u8 , 46u8 , 165u8 , 18u8 , 191u8 , 7u8 , 97u8 , 84u8 , 111u8 ,])
                }
                #[doc = " Details of the module's parameters"]                pub fn resource_to_bridge_index_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "BridgeRegistry",
                        "ResourceToBridgeIndex",
                        Vec::new(),
                        [
                            130u8, 216u8, 179u8, 142u8, 98u8, 4u8, 178u8, 25u8,
                            179u8, 57u8, 140u8, 220u8, 202u8, 186u8, 74u8,
                            115u8, 145u8, 63u8, 69u8, 239u8, 102u8, 91u8, 1u8,
                            43u8, 46u8, 165u8, 18u8, 191u8, 7u8, 97u8, 84u8,
                            111u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Maximum number of additional fields that may be stored in a bridge's metadata. Needed to"]
                #[doc = " bound the I/O required to access an identity, but can be pretty high."]
                pub fn max_additional_fields(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "BridgeRegistry",
                        "MaxAdditionalFields",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maximum number of resources that may be stored in a bridge. This is not to be confused"]
                #[doc = " with the actual maximum supported by the bridge. Needed to bound the I/O"]
                #[doc = " required to access a metadata object, but can be pretty high."]
                pub fn max_resources(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "BridgeRegistry",
                        "MaxResources",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod identity {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Identity pallet declaration."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct AddRegistrar {
                pub account: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetIdentity {
                pub info: ::std::boxed::Box<
                    runtime_types::pallet_identity::types::IdentityInfo,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetSubs {
                pub subs: ::std::vec::Vec<(
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    runtime_types::pallet_identity::types::Data,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ClearIdentity;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RequestJudgement {
                #[codec(compact)]
                pub reg_index: ::core::primitive::u32,
                #[codec(compact)]
                pub max_fee: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct CancelRequest {
                pub reg_index: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetFee {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                #[codec(compact)]
                pub fee: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetAccountId {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub new: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SetFields {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub fields: runtime_types::pallet_identity::types::BitFlags<
                    runtime_types::pallet_identity::types::IdentityField,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ProvideJudgement {
                #[codec(compact)]
                pub reg_index: ::core::primitive::u32,
                pub target: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub judgement: runtime_types::pallet_identity::types::Judgement<
                    ::core::primitive::u128,
                >,
                pub identity: ::subxt::ext::sp_core::H256,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct KillIdentity {
                pub target: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct AddSub {
                pub sub: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub data: runtime_types::pallet_identity::types::Data,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RenameSub {
                pub sub: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
                pub data: runtime_types::pallet_identity::types::Data,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RemoveSub {
                pub sub: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct QuitSub;
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Add a registrar to the system."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `T::RegistrarOrigin`."]
                #[doc = ""]
                #[doc = "- `account`: the account of the registrar."]
                #[doc = ""]
                #[doc = "Emits `RegistrarAdded` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(R)` where `R` registrar-count (governance-bounded and code-bounded)."]
                #[doc = "- One storage mutation (codec `O(R)`)."]
                #[doc = "- One event."]
                #[doc = "# </weight>"]
                pub fn add_registrar(
                    &self,
                    account: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<AddRegistrar>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "add_registrar",
                        AddRegistrar { account },
                        [
                            96u8, 200u8, 92u8, 23u8, 3u8, 144u8, 56u8, 53u8,
                            245u8, 210u8, 33u8, 36u8, 183u8, 233u8, 41u8, 1u8,
                            127u8, 2u8, 25u8, 5u8, 15u8, 133u8, 4u8, 107u8,
                            206u8, 155u8, 114u8, 39u8, 14u8, 235u8, 115u8,
                            172u8,
                        ],
                    )
                }
                #[doc = "Set an account's identity information and reserve the appropriate deposit."]
                #[doc = ""]
                #[doc = "If the account already has identity information, the deposit is taken as part payment"]
                #[doc = "for the new deposit."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `info`: The identity information."]
                #[doc = ""]
                #[doc = "Emits `IdentitySet` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(X + X' + R)`"]
                #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)"]
                #[doc = "  - where `R` judgements-count (registrar-count-bounded)"]
                #[doc = "- One balance reserve operation."]
                #[doc = "- One storage mutation (codec-read `O(X' + R)`, codec-write `O(X + R)`)."]
                #[doc = "- One event."]
                #[doc = "# </weight>"]
                pub fn set_identity(
                    &self,
                    info: runtime_types::pallet_identity::types::IdentityInfo,
                ) -> ::subxt::tx::StaticTxPayload<SetIdentity> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "set_identity",
                        SetIdentity {
                            info: ::std::boxed::Box::new(info),
                        },
                        [
                            130u8, 89u8, 118u8, 6u8, 134u8, 166u8, 35u8, 192u8,
                            73u8, 6u8, 171u8, 20u8, 225u8, 255u8, 152u8, 142u8,
                            111u8, 8u8, 206u8, 200u8, 64u8, 52u8, 110u8, 123u8,
                            42u8, 101u8, 191u8, 242u8, 133u8, 139u8, 154u8,
                            205u8,
                        ],
                    )
                }
                #[doc = "Set the sub-accounts of the sender."]
                #[doc = ""]
                #[doc = "Payment: Any aggregate balance reserved by previous `set_subs` calls will be returned"]
                #[doc = "and an amount `SubAccountDeposit` will be reserved for each item in `subs`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "identity."]
                #[doc = ""]
                #[doc = "- `subs`: The identity's (new) sub-accounts."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(P + S)`"]
                #[doc = "  - where `P` old-subs-count (hard- and deposit-bounded)."]
                #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                #[doc = "- At most one balance operations."]
                #[doc = "- DB:"]
                #[doc = "  - `P + S` storage mutations (codec complexity `O(1)`)"]
                #[doc = "  - One storage read (codec complexity `O(P)`)."]
                #[doc = "  - One storage write (codec complexity `O(S)`)."]
                #[doc = "  - One storage-exists (`IdentityOf::contains_key`)."]
                #[doc = "# </weight>"]
                pub fn set_subs(
                    &self,
                    subs: ::std::vec::Vec<(
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                ) -> ::subxt::tx::StaticTxPayload<SetSubs> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "set_subs",
                        SetSubs { subs },
                        [
                            177u8, 219u8, 84u8, 183u8, 5u8, 32u8, 192u8, 82u8,
                            174u8, 68u8, 198u8, 224u8, 56u8, 85u8, 134u8,
                            171u8, 30u8, 132u8, 140u8, 236u8, 117u8, 24u8,
                            150u8, 218u8, 146u8, 194u8, 144u8, 92u8, 103u8,
                            206u8, 46u8, 90u8,
                        ],
                    )
                }
                #[doc = "Clear an account's identity info and all sub-accounts and return all deposits."]
                #[doc = ""]
                #[doc = "Payment: All reserved balances on the account are returned."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "identity."]
                #[doc = ""]
                #[doc = "Emits `IdentityCleared` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(R + S + X)`"]
                #[doc = "  - where `R` registrar-count (governance-bounded)."]
                #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                #[doc = "- One balance-unreserve operation."]
                #[doc = "- `2` storage reads and `S + 2` storage deletions."]
                #[doc = "- One event."]
                #[doc = "# </weight>"]
                pub fn clear_identity(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<ClearIdentity>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "clear_identity",
                        ClearIdentity {},
                        [
                            75u8, 44u8, 74u8, 122u8, 149u8, 202u8, 114u8,
                            230u8, 0u8, 255u8, 140u8, 122u8, 14u8, 196u8,
                            205u8, 249u8, 220u8, 94u8, 216u8, 34u8, 63u8, 14u8,
                            8u8, 205u8, 74u8, 23u8, 181u8, 129u8, 252u8, 110u8,
                            231u8, 114u8,
                        ],
                    )
                }
                #[doc = "Request a judgement from a registrar."]
                #[doc = ""]
                #[doc = "Payment: At most `max_fee` will be reserved for payment to the registrar if judgement"]
                #[doc = "given."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"]
                #[doc = "registered identity."]
                #[doc = ""]
                #[doc = "- `reg_index`: The index of the registrar whose judgement is requested."]
                #[doc = "- `max_fee`: The maximum fee that may be paid. This should just be auto-populated as:"]
                #[doc = ""]
                #[doc = "```nocompile"]
                #[doc = "Self::registrars().get(reg_index).unwrap().fee"]
                #[doc = "```"]
                #[doc = ""]
                #[doc = "Emits `JudgementRequested` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(R + X)`."]
                #[doc = "- One balance-reserve operation."]
                #[doc = "- Storage: 1 read `O(R)`, 1 mutate `O(X + R)`."]
                #[doc = "- One event."]
                #[doc = "# </weight>"]
                pub fn request_judgement(
                    &self,
                    reg_index: ::core::primitive::u32,
                    max_fee: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<RequestJudgement>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "request_judgement",
                        RequestJudgement { reg_index, max_fee },
                        [
                            186u8, 149u8, 61u8, 54u8, 159u8, 194u8, 77u8,
                            161u8, 220u8, 157u8, 3u8, 216u8, 23u8, 105u8,
                            119u8, 76u8, 144u8, 198u8, 157u8, 45u8, 235u8,
                            139u8, 87u8, 82u8, 81u8, 12u8, 25u8, 134u8, 225u8,
                            92u8, 182u8, 101u8,
                        ],
                    )
                }
                #[doc = "Cancel a previous request."]
                #[doc = ""]
                #[doc = "Payment: A previously reserved deposit is returned on success."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"]
                #[doc = "registered identity."]
                #[doc = ""]
                #[doc = "- `reg_index`: The index of the registrar whose judgement is no longer requested."]
                #[doc = ""]
                #[doc = "Emits `JudgementUnrequested` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(R + X)`."]
                #[doc = "- One balance-reserve operation."]
                #[doc = "- One storage mutation `O(R + X)`."]
                #[doc = "- One event"]
                #[doc = "# </weight>"]
                pub fn cancel_request(
                    &self,
                    reg_index: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<CancelRequest>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "cancel_request",
                        CancelRequest { reg_index },
                        [
                            83u8, 180u8, 239u8, 126u8, 32u8, 51u8, 17u8, 20u8,
                            180u8, 3u8, 59u8, 96u8, 24u8, 32u8, 136u8, 92u8,
                            58u8, 254u8, 68u8, 70u8, 50u8, 11u8, 51u8, 91u8,
                            180u8, 79u8, 81u8, 84u8, 216u8, 138u8, 6u8, 215u8,
                        ],
                    )
                }
                #[doc = "Set the fee required for a judgement to be requested from a registrar."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                #[doc = "of the registrar whose index is `index`."]
                #[doc = ""]
                #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                #[doc = "- `fee`: the new fee."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(R)`."]
                #[doc = "- One storage mutation `O(R)`."]
                #[doc = "- Benchmark: 7.315 + R * 0.329 µs (min squares analysis)"]
                #[doc = "# </weight>"]
                pub fn set_fee(
                    &self,
                    index: ::core::primitive::u32,
                    fee: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<SetFee> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "set_fee",
                        SetFee { index, fee },
                        [
                            21u8, 157u8, 123u8, 182u8, 160u8, 190u8, 117u8,
                            37u8, 136u8, 133u8, 104u8, 234u8, 31u8, 145u8,
                            115u8, 154u8, 125u8, 40u8, 2u8, 87u8, 118u8, 56u8,
                            247u8, 73u8, 89u8, 0u8, 251u8, 3u8, 58u8, 105u8,
                            239u8, 211u8,
                        ],
                    )
                }
                #[doc = "Change the account associated with a registrar."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                #[doc = "of the registrar whose index is `index`."]
                #[doc = ""]
                #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                #[doc = "- `new`: the new account ID."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(R)`."]
                #[doc = "- One storage mutation `O(R)`."]
                #[doc = "- Benchmark: 8.823 + R * 0.32 µs (min squares analysis)"]
                #[doc = "# </weight>"]
                pub fn set_account_id(
                    &self,
                    index: ::core::primitive::u32,
                    new: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetAccountId>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "set_account_id",
                        SetAccountId { index, new },
                        [
                            14u8, 154u8, 84u8, 48u8, 59u8, 133u8, 45u8, 204u8,
                            255u8, 85u8, 157u8, 88u8, 56u8, 207u8, 113u8,
                            184u8, 233u8, 139u8, 129u8, 118u8, 59u8, 9u8,
                            211u8, 184u8, 32u8, 141u8, 126u8, 208u8, 179u8,
                            4u8, 2u8, 95u8,
                        ],
                    )
                }
                #[doc = "Set the field information for a registrar."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                #[doc = "of the registrar whose index is `index`."]
                #[doc = ""]
                #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                #[doc = "- `fields`: the fields that the registrar concerns themselves with."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(R)`."]
                #[doc = "- One storage mutation `O(R)`."]
                #[doc = "- Benchmark: 7.464 + R * 0.325 µs (min squares analysis)"]
                #[doc = "# </weight>"]
                pub fn set_fields(
                    &self,
                    index: ::core::primitive::u32,
                    fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetFields> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "set_fields",
                        SetFields { index, fields },
                        [
                            50u8, 196u8, 179u8, 71u8, 66u8, 65u8, 235u8, 7u8,
                            51u8, 14u8, 81u8, 173u8, 201u8, 58u8, 6u8, 151u8,
                            174u8, 245u8, 102u8, 184u8, 28u8, 84u8, 125u8,
                            93u8, 126u8, 134u8, 92u8, 203u8, 200u8, 129u8,
                            240u8, 252u8,
                        ],
                    )
                }
                #[doc = "Provide a judgement for an account's identity."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                #[doc = "of the registrar whose index is `reg_index`."]
                #[doc = ""]
                #[doc = "- `reg_index`: the index of the registrar whose judgement is being made."]
                #[doc = "- `target`: the account whose identity the judgement is upon. This must be an account"]
                #[doc = "  with a registered identity."]
                #[doc = "- `judgement`: the judgement of the registrar of index `reg_index` about `target`."]
                #[doc = "- `identity`: The hash of the [`IdentityInfo`] for that the judgement is provided."]
                #[doc = ""]
                #[doc = "Emits `JudgementGiven` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(R + X)`."]
                #[doc = "- One balance-transfer operation."]
                #[doc = "- Up to one account-lookup operation."]
                #[doc = "- Storage: 1 read `O(R)`, 1 mutate `O(R + X)`."]
                #[doc = "- One event."]
                #[doc = "# </weight>"]
                pub fn provide_judgement(
                    &self,
                    reg_index: ::core::primitive::u32,
                    target: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    judgement: runtime_types::pallet_identity::types::Judgement<
                        ::core::primitive::u128,
                    >,
                    identity: ::subxt::ext::sp_core::H256,
                ) -> ::subxt::tx::StaticTxPayload<ProvideJudgement>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "provide_judgement",
                        ProvideJudgement {
                            reg_index,
                            target,
                            judgement,
                            identity,
                        },
                        [
                            83u8, 253u8, 77u8, 208u8, 198u8, 25u8, 202u8,
                            213u8, 223u8, 184u8, 231u8, 185u8, 186u8, 216u8,
                            54u8, 62u8, 3u8, 7u8, 107u8, 152u8, 126u8, 195u8,
                            175u8, 221u8, 134u8, 169u8, 199u8, 124u8, 232u8,
                            157u8, 67u8, 75u8,
                        ],
                    )
                }
                #[doc = "Remove an account's identity and sub-account information and slash the deposits."]
                #[doc = ""]
                #[doc = "Payment: Reserved balances from `set_subs` and `set_identity` are slashed and handled by"]
                #[doc = "`Slash`. Verification request deposits are not returned; they should be cancelled"]
                #[doc = "manually using `cancel_request`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must match `T::ForceOrigin`."]
                #[doc = ""]
                #[doc = "- `target`: the account whose identity the judgement is upon. This must be an account"]
                #[doc = "  with a registered identity."]
                #[doc = ""]
                #[doc = "Emits `IdentityKilled` if successful."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(R + S + X)`."]
                #[doc = "- One balance-reserve operation."]
                #[doc = "- `S + 2` storage mutations."]
                #[doc = "- One event."]
                #[doc = "# </weight>"]
                pub fn kill_identity(
                    &self,
                    target: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<KillIdentity>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "kill_identity",
                        KillIdentity { target },
                        [
                            65u8, 106u8, 116u8, 209u8, 219u8, 181u8, 185u8,
                            75u8, 146u8, 194u8, 187u8, 170u8, 7u8, 34u8, 140u8,
                            87u8, 107u8, 112u8, 229u8, 34u8, 65u8, 71u8, 58u8,
                            152u8, 74u8, 253u8, 137u8, 69u8, 149u8, 214u8,
                            158u8, 19u8,
                        ],
                    )
                }
                #[doc = "Add the given account to the sender's subs."]
                #[doc = ""]
                #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                #[doc = "to the sender."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "sub identity of `sub`."]
                pub fn add_sub(
                    &self,
                    sub: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    data: runtime_types::pallet_identity::types::Data,
                ) -> ::subxt::tx::StaticTxPayload<AddSub> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "add_sub",
                        AddSub { sub, data },
                        [
                            206u8, 112u8, 143u8, 96u8, 152u8, 12u8, 174u8,
                            226u8, 23u8, 27u8, 154u8, 188u8, 195u8, 233u8,
                            185u8, 180u8, 246u8, 218u8, 154u8, 129u8, 138u8,
                            52u8, 212u8, 109u8, 54u8, 211u8, 219u8, 255u8,
                            39u8, 79u8, 154u8, 123u8,
                        ],
                    )
                }
                #[doc = "Alter the associated name of the given sub-account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "sub identity of `sub`."]
                pub fn rename_sub(
                    &self,
                    sub: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                    data: runtime_types::pallet_identity::types::Data,
                ) -> ::subxt::tx::StaticTxPayload<RenameSub> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "rename_sub",
                        RenameSub { sub, data },
                        [
                            110u8, 28u8, 134u8, 225u8, 44u8, 242u8, 20u8, 22u8,
                            197u8, 49u8, 173u8, 178u8, 106u8, 181u8, 103u8,
                            90u8, 27u8, 73u8, 102u8, 130u8, 2u8, 216u8, 172u8,
                            186u8, 124u8, 244u8, 128u8, 6u8, 112u8, 128u8,
                            25u8, 245u8,
                        ],
                    )
                }
                #[doc = "Remove the given account from the sender's subs."]
                #[doc = ""]
                #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                #[doc = "to the sender."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "sub identity of `sub`."]
                pub fn remove_sub(
                    &self,
                    sub: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<RemoveSub> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "remove_sub",
                        RemoveSub { sub },
                        [
                            92u8, 201u8, 70u8, 170u8, 248u8, 110u8, 179u8,
                            186u8, 213u8, 197u8, 150u8, 156u8, 156u8, 50u8,
                            19u8, 158u8, 186u8, 61u8, 106u8, 64u8, 84u8, 38u8,
                            73u8, 134u8, 132u8, 233u8, 50u8, 152u8, 40u8, 18u8,
                            212u8, 121u8,
                        ],
                    )
                }
                #[doc = "Remove the sender as a sub-account."]
                #[doc = ""]
                #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                #[doc = "to the sender (*not* the original depositor)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "super-identity."]
                #[doc = ""]
                #[doc = "NOTE: This should not normally be used, but is provided in the case that the non-"]
                #[doc = "controller of an account is maliciously registered as a sub-account."]
                pub fn quit_sub(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<QuitSub> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Identity",
                        "quit_sub",
                        QuitSub {},
                        [
                            62u8, 57u8, 73u8, 72u8, 119u8, 216u8, 250u8, 155u8,
                            57u8, 169u8, 157u8, 44u8, 87u8, 51u8, 63u8, 231u8,
                            77u8, 7u8, 0u8, 119u8, 244u8, 42u8, 179u8, 51u8,
                            254u8, 240u8, 55u8, 25u8, 142u8, 38u8, 87u8, 44u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_identity::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A name was set or reset (which will remove all judgements)."]
            pub struct IdentitySet {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for IdentitySet {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentitySet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A name was cleared, and the given balance returned."]
            pub struct IdentityCleared {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for IdentityCleared {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityCleared";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A name was removed and the given balance slashed."]
            pub struct IdentityKilled {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for IdentityKilled {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityKilled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A judgement was asked from a registrar."]
            pub struct JudgementRequested {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for JudgementRequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementRequested";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A judgement request was retracted."]
            pub struct JudgementUnrequested {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for JudgementUnrequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementUnrequested";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A judgement was given by a registrar."]
            pub struct JudgementGiven {
                pub target: ::subxt::ext::sp_core::crypto::AccountId32,
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for JudgementGiven {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementGiven";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A registrar was added."]
            pub struct RegistrarAdded {
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for RegistrarAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "RegistrarAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A sub-identity was added to an identity and the deposit paid."]
            pub struct SubIdentityAdded {
                pub sub: ::subxt::ext::sp_core::crypto::AccountId32,
                pub main: ::subxt::ext::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for SubIdentityAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A sub-identity was removed from an identity and the deposit freed."]
            pub struct SubIdentityRemoved {
                pub sub: ::subxt::ext::sp_core::crypto::AccountId32,
                pub main: ::subxt::ext::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for SubIdentityRemoved {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A sub-identity was cleared, and the given deposit repatriated from the"]
            #[doc = "main identity account to the sub-identity account."]
            pub struct SubIdentityRevoked {
                pub sub: ::subxt::ext::sp_core::crypto::AccountId32,
                pub main: ::subxt::ext::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for SubIdentityRevoked {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRevoked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Information that is pertinent to identify the entity behind an account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]                pub fn identity_of (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_identity :: types :: Registration < :: core :: primitive :: u128 > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Identity" , "IdentityOf" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [193u8 , 195u8 , 180u8 , 188u8 , 129u8 , 250u8 , 180u8 , 219u8 , 22u8 , 95u8 , 175u8 , 170u8 , 143u8 , 188u8 , 80u8 , 124u8 , 234u8 , 228u8 , 245u8 , 39u8 , 72u8 , 153u8 , 107u8 , 199u8 , 23u8 , 75u8 , 47u8 , 247u8 , 104u8 , 208u8 , 171u8 , 82u8 ,])
                }
                #[doc = " Information that is pertinent to identify the entity behind an account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]                pub fn identity_of_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: pallet_identity :: types :: Registration < :: core :: primitive :: u128 > > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Identity",
                        "IdentityOf",
                        Vec::new(),
                        [
                            193u8, 195u8, 180u8, 188u8, 129u8, 250u8, 180u8,
                            219u8, 22u8, 95u8, 175u8, 170u8, 143u8, 188u8,
                            80u8, 124u8, 234u8, 228u8, 245u8, 39u8, 72u8,
                            153u8, 107u8, 199u8, 23u8, 75u8, 47u8, 247u8,
                            104u8, 208u8, 171u8, 82u8,
                        ],
                    )
                }
                #[doc = " The super-identity of an alternative \"sub\" identity together with its name, within that"]
                #[doc = " context. If the account is not some other account's sub-identity, then just `None`."]                pub fn super_of (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , runtime_types :: pallet_identity :: types :: Data ,) > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Identity" , "SuperOf" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Blake2_128Concat)] , [170u8 , 249u8 , 112u8 , 249u8 , 75u8 , 176u8 , 21u8 , 29u8 , 152u8 , 149u8 , 69u8 , 113u8 , 20u8 , 92u8 , 113u8 , 130u8 , 135u8 , 62u8 , 18u8 , 204u8 , 166u8 , 193u8 , 133u8 , 167u8 , 248u8 , 117u8 , 80u8 , 137u8 , 158u8 , 111u8 , 100u8 , 137u8 ,])
                }
                #[doc = " The super-identity of an alternative \"sub\" identity together with its name, within that"]
                #[doc = " context. If the account is not some other account's sub-identity, then just `None`."]                pub fn super_of_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , runtime_types :: pallet_identity :: types :: Data ,) > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Identity",
                        "SuperOf",
                        Vec::new(),
                        [
                            170u8, 249u8, 112u8, 249u8, 75u8, 176u8, 21u8,
                            29u8, 152u8, 149u8, 69u8, 113u8, 20u8, 92u8, 113u8,
                            130u8, 135u8, 62u8, 18u8, 204u8, 166u8, 193u8,
                            133u8, 167u8, 248u8, 117u8, 80u8, 137u8, 158u8,
                            111u8, 100u8, 137u8,
                        ],
                    )
                }
                #[doc = " Alternative \"sub\" identities of this account."]
                #[doc = ""]
                #[doc = " The first item is the deposit, the second is a vector of the accounts."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]                pub fn subs_of (& self , _0 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: core :: primitive :: u128 , runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("Identity" , "SubsOf" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [128u8 , 15u8 , 175u8 , 155u8 , 216u8 , 225u8 , 200u8 , 169u8 , 215u8 , 206u8 , 110u8 , 22u8 , 204u8 , 89u8 , 212u8 , 210u8 , 159u8 , 169u8 , 53u8 , 7u8 , 44u8 , 164u8 , 91u8 , 151u8 , 7u8 , 227u8 , 38u8 , 230u8 , 175u8 , 84u8 , 6u8 , 4u8 ,])
                }
                #[doc = " Alternative \"sub\" identities of this account."]
                #[doc = ""]
                #[doc = " The first item is the deposit, the second is a vector of the accounts."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]                pub fn subs_of_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < (:: core :: primitive :: u128 , runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Identity",
                        "SubsOf",
                        Vec::new(),
                        [
                            128u8, 15u8, 175u8, 155u8, 216u8, 225u8, 200u8,
                            169u8, 215u8, 206u8, 110u8, 22u8, 204u8, 89u8,
                            212u8, 210u8, 159u8, 169u8, 53u8, 7u8, 44u8, 164u8,
                            91u8, 151u8, 7u8, 227u8, 38u8, 230u8, 175u8, 84u8,
                            6u8, 4u8,
                        ],
                    )
                }
                #[doc = " The set of registrars. Not expected to get very big as can only be added through a"]
                #[doc = " special origin (likely a council motion)."]
                #[doc = ""]
                #[doc = " The index into this can be cast to `RegistrarIndex` to get a valid value."]                pub fn registrars (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < :: core :: option :: Option < runtime_types :: pallet_identity :: types :: RegistrarInfo < :: core :: primitive :: u128 , :: subxt :: ext :: sp_core :: crypto :: AccountId32 > > > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Identity",
                        "Registrars",
                        vec![],
                        [
                            157u8, 87u8, 39u8, 240u8, 154u8, 54u8, 241u8,
                            229u8, 76u8, 9u8, 62u8, 252u8, 40u8, 143u8, 186u8,
                            182u8, 233u8, 187u8, 251u8, 61u8, 236u8, 229u8,
                            19u8, 55u8, 42u8, 36u8, 82u8, 173u8, 215u8, 155u8,
                            229u8, 111u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The amount held on deposit for a registered identity"]
                pub fn basic_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Identity",
                        "BasicDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8,
                            105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8,
                            160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
                            71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8,
                            148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount held on deposit per additional field for a registered identity."]
                pub fn field_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Identity",
                        "FieldDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8,
                            105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8,
                            160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
                            71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8,
                            148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount held on deposit for a registered subaccount. This should account for the fact"]
                #[doc = " that one storage item's value will increase by the size of an account ID, and there will"]
                #[doc = " be another trie item whose value is the size of an account ID plus 32 bytes."]
                pub fn sub_account_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::primitive::u128,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Identity",
                        "SubAccountDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8,
                            105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8,
                            160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8,
                            71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8,
                            148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of sub-accounts allowed per identified account."]
                pub fn max_sub_accounts(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Identity",
                        "MaxSubAccounts",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maximum number of additional fields that may be stored in an ID. Needed to bound the I/O"]
                #[doc = " required to access an identity, but can be pretty high."]
                pub fn max_additional_fields(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Identity",
                        "MaxAdditionalFields",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maxmimum number of registrars allowed in the system. Needed to bound the complexity"]
                #[doc = " of, e.g., updating judgements."]
                pub fn max_registrars(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Identity",
                        "MaxRegistrars",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8,
                            200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8,
                            26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8,
                            113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod im_online {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Heartbeat { pub heartbeat : runtime_types :: pallet_im_online :: Heartbeat < :: core :: primitive :: u32 > , pub signature : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Signature , }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "# <weight>"]
                #[doc = "- Complexity: `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is"]
                #[doc = "  length of `heartbeat.network_state.external_address`"]
                #[doc = "  - `O(K)`: decoding of length `K`"]
                #[doc = "  - `O(E)`: decoding/encoding of length `E`"]
                #[doc = "- DbReads: pallet_session `Validators`, pallet_session `CurrentIndex`, `Keys`,"]
                #[doc = "  `ReceivedHeartbeats`"]
                #[doc = "- DbWrites: `ReceivedHeartbeats`"]
                #[doc = "# </weight>"]
                pub fn heartbeat(
                    &self,
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<
                        ::core::primitive::u32,
                    >,
                    signature : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Signature,
                ) -> ::subxt::tx::StaticTxPayload<Heartbeat> {
                    ::subxt::tx::StaticTxPayload::new(
                        "ImOnline",
                        "heartbeat",
                        Heartbeat {
                            heartbeat,
                            signature,
                        },
                        [
                            212u8, 23u8, 174u8, 246u8, 60u8, 220u8, 178u8,
                            137u8, 53u8, 146u8, 165u8, 225u8, 179u8, 209u8,
                            233u8, 152u8, 129u8, 210u8, 126u8, 32u8, 216u8,
                            22u8, 76u8, 196u8, 255u8, 128u8, 246u8, 161u8,
                            30u8, 186u8, 249u8, 34u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "A new heartbeat was received from `AuthorityId`."]
            pub struct HeartbeatReceived { pub authority_id : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public , }
            impl ::subxt::events::StaticEvent for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "At the end of the session, no offence was committed."]
            pub struct AllGood;
            impl ::subxt::events::StaticEvent for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            #[doc = "At the end of the session, at least one validator was found to be offline."]
            pub struct SomeOffline {
                pub offline: ::std::vec::Vec<(
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                )>,
            }
            impl ::subxt::events::StaticEvent for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The block number after which it's ok to send heartbeats in the current"]
                #[doc = " session."]
                #[doc = ""]
                #[doc = " At the beginning of each session we set this to a value that should fall"]
                #[doc = " roughly in the middle of the session duration. The idea is to first wait for"]
                #[doc = " the validators to produce a block in the current session, so that the"]
                #[doc = " heartbeat later on will not be necessary."]
                #[doc = ""]
                #[doc = " This value will only be used as a fallback if we fail to get a proper session"]
                #[doc = " progress estimate from `NextSessionRotation`, as those estimates should be"]
                #[doc = " more accurate then the value we calculate for `HeartbeatAfter`."]                pub fn heartbeat_after (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ImOnline",
                        "HeartbeatAfter",
                        vec![],
                        [
                            108u8, 100u8, 85u8, 198u8, 226u8, 122u8, 94u8,
                            225u8, 97u8, 154u8, 135u8, 95u8, 106u8, 28u8,
                            185u8, 78u8, 192u8, 196u8, 35u8, 191u8, 12u8, 19u8,
                            163u8, 46u8, 232u8, 235u8, 193u8, 81u8, 126u8,
                            204u8, 25u8, 228u8,
                        ],
                    )
                }
                #[doc = " The current set of keys that may issue a heartbeat."]                pub fn keys (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: sp_core :: bounded :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ImOnline",
                        "Keys",
                        vec![],
                        [
                            6u8, 198u8, 221u8, 58u8, 14u8, 166u8, 245u8, 103u8,
                            191u8, 20u8, 69u8, 233u8, 147u8, 245u8, 24u8, 64u8,
                            207u8, 180u8, 39u8, 208u8, 252u8, 236u8, 247u8,
                            112u8, 187u8, 97u8, 70u8, 11u8, 248u8, 148u8,
                            208u8, 106u8,
                        ],
                    )
                }
                #[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to"]
                #[doc = " `WrapperOpaque<BoundedOpaqueNetworkState>`."]                pub fn received_heartbeats (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > , _1 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: frame_support :: traits :: misc :: WrapperOpaque < runtime_types :: pallet_im_online :: BoundedOpaqueNetworkState > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("ImOnline" , "ReceivedHeartbeats" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat) , :: subxt :: storage :: address :: StorageMapKey :: new (_1 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [233u8 , 128u8 , 140u8 , 233u8 , 55u8 , 146u8 , 172u8 , 54u8 , 54u8 , 57u8 , 141u8 , 106u8 , 168u8 , 59u8 , 147u8 , 253u8 , 119u8 , 48u8 , 50u8 , 251u8 , 242u8 , 109u8 , 251u8 , 2u8 , 136u8 , 80u8 , 146u8 , 121u8 , 180u8 , 219u8 , 245u8 , 37u8 ,])
                }
                #[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to"]
                #[doc = " `WrapperOpaque<BoundedOpaqueNetworkState>`."]                pub fn received_heartbeats_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: frame_support :: traits :: misc :: WrapperOpaque < runtime_types :: pallet_im_online :: BoundedOpaqueNetworkState > > , () , () , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ImOnline",
                        "ReceivedHeartbeats",
                        Vec::new(),
                        [
                            233u8, 128u8, 140u8, 233u8, 55u8, 146u8, 172u8,
                            54u8, 54u8, 57u8, 141u8, 106u8, 168u8, 59u8, 147u8,
                            253u8, 119u8, 48u8, 50u8, 251u8, 242u8, 109u8,
                            251u8, 2u8, 136u8, 80u8, 146u8, 121u8, 180u8,
                            219u8, 245u8, 37u8,
                        ],
                    )
                }
                #[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
                #[doc = " number of blocks authored by the given authority."]                pub fn authored_blocks (& self , _0 : impl :: std :: borrow :: Borrow < :: core :: primitive :: u32 > , _1 : impl :: std :: borrow :: Borrow < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    :: subxt :: storage :: address :: StaticStorageAddress :: new ("ImOnline" , "AuthoredBlocks" , vec ! [:: subxt :: storage :: address :: StorageMapKey :: new (_0 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat) , :: subxt :: storage :: address :: StorageMapKey :: new (_1 . borrow () , :: subxt :: storage :: address :: StorageHasher :: Twox64Concat)] , [50u8 , 4u8 , 242u8 , 240u8 , 247u8 , 184u8 , 114u8 , 245u8 , 233u8 , 170u8 , 24u8 , 197u8 , 18u8 , 245u8 , 8u8 , 28u8 , 33u8 , 115u8 , 166u8 , 245u8 , 221u8 , 223u8 , 56u8 , 144u8 , 33u8 , 139u8 , 10u8 , 227u8 , 228u8 , 223u8 , 103u8 , 151u8 ,])
                }
                #[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
                #[doc = " number of blocks authored by the given authority."]                pub fn authored_blocks_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: core :: primitive :: u32 > , () , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ImOnline",
                        "AuthoredBlocks",
                        Vec::new(),
                        [
                            50u8, 4u8, 242u8, 240u8, 247u8, 184u8, 114u8,
                            245u8, 233u8, 170u8, 24u8, 197u8, 18u8, 245u8, 8u8,
                            28u8, 33u8, 115u8, 166u8, 245u8, 221u8, 223u8,
                            56u8, 144u8, 33u8, 139u8, 10u8, 227u8, 228u8,
                            223u8, 103u8, 151u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A configuration for base priority of unsigned transactions."]
                #[doc = ""]
                #[doc = " This is exposed so that it can be tuned for particular runtime, when"]
                #[doc = " multiple pallets send unsigned transactions."]
                pub fn unsigned_priority(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "ImOnline",
                        "UnsignedPriority",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8,
                            231u8, 190u8, 146u8, 59u8, 226u8, 157u8, 101u8,
                            103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8,
                            119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8,
                            239u8, 42u8, 246u8,
                        ],
                    )
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
            }
            pub mod proposal {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct RefreshProposalSigned {
                    pub nonce: runtime_types::webb_proposals::nonce::Nonce,
                    pub signature: ::std::vec::Vec<::core::primitive::u8>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct StoredUnsignedProposal<_0> {
                    pub proposal:
                        runtime_types::webb_proposals::proposal::Proposal,
                    pub timestamp: _0,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct AggregatedMisbehaviourReports<_0> {
                pub misbehaviour_type:
                    runtime_types::dkg_runtime_primitives::MisbehaviourType,
                pub session_id: ::core::primitive::u64,
                pub offender: _0,
                pub reporters: ::std::vec::Vec<_0>,
                pub signatures:
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct AggregatedPublicKeys {
                pub keys_and_signatures: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct SessionKeys { pub aura : runtime_types :: sp_consensus_aura :: sr25519 :: app_sr25519 :: Public , pub grandpa : runtime_types :: sp_finality_grandpa :: app :: Public , pub im_online : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public , pub dkg : runtime_types :: dkg_runtime_primitives :: crypto :: Public , }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct NposSolution16 {
                pub votes1: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes2: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    (
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ),
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes3: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 2usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes4: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 3usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes5: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 4usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes6: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 5usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes7: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 6usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes8: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 7usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes9: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 8usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes10: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 9usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes11: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 10usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes12: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 11usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes13: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 12usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes14: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 13usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes15: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 14usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
                pub votes16: ::std::vec::Vec<(
                    ::subxt::ext::codec::Compact<::core::primitive::u32>,
                    [(
                        ::subxt::ext::codec::Compact<::core::primitive::u16>,
                        ::subxt::ext::codec::Compact<
                            runtime_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 15usize],
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub enum RuntimeCall {
                # [codec (index = 0)] System (runtime_types :: frame_system :: pallet :: Call ,) , # [codec (index = 1)] Indices (runtime_types :: pallet_indices :: pallet :: Call ,) , # [codec (index = 3)] Timestamp (runtime_types :: pallet_timestamp :: pallet :: Call ,) , # [codec (index = 5)] Grandpa (runtime_types :: pallet_grandpa :: pallet :: Call ,) , # [codec (index = 6)] Balances (runtime_types :: pallet_balances :: pallet :: Call ,) , # [codec (index = 7)] DKG (runtime_types :: pallet_dkg_metadata :: pallet :: Call ,) , # [codec (index = 8)] DKGProposals (runtime_types :: pallet_dkg_proposals :: pallet :: Call ,) , # [codec (index = 9)] DKGProposalHandler (runtime_types :: pallet_dkg_proposal_handler :: pallet :: Call ,) , # [codec (index = 11)] Sudo (runtime_types :: pallet_sudo :: pallet :: Call ,) , # [codec (index = 12)] ElectionProviderMultiPhase (runtime_types :: pallet_election_provider_multi_phase :: pallet :: Call ,) , # [codec (index = 13)] BagsList (runtime_types :: pallet_bags_list :: pallet :: Call ,) , # [codec (index = 14)] NominationPools (runtime_types :: pallet_nomination_pools :: pallet :: Call ,) , # [codec (index = 15)] Staking (runtime_types :: pallet_staking :: pallet :: pallet :: Call ,) , # [codec (index = 16)] Session (runtime_types :: pallet_session :: pallet :: Call ,) , # [codec (index = 18)] BridgeRegistry (runtime_types :: pallet_bridge_registry :: pallet :: Call ,) , # [codec (index = 19)] Identity (runtime_types :: pallet_identity :: pallet :: Call ,) , # [codec (index = 20)] ImOnline (runtime_types :: pallet_im_online :: pallet :: Call ,) , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub enum RuntimeEvent {
                # [codec (index = 0)] System (runtime_types :: frame_system :: pallet :: Event ,) , # [codec (index = 1)] Indices (runtime_types :: pallet_indices :: pallet :: Event ,) , # [codec (index = 5)] Grandpa (runtime_types :: pallet_grandpa :: pallet :: Event ,) , # [codec (index = 6)] Balances (runtime_types :: pallet_balances :: pallet :: Event ,) , # [codec (index = 7)] DKG (runtime_types :: pallet_dkg_metadata :: pallet :: Event ,) , # [codec (index = 8)] DKGProposals (runtime_types :: pallet_dkg_proposals :: pallet :: Event ,) , # [codec (index = 9)] DKGProposalHandler (runtime_types :: pallet_dkg_proposal_handler :: pallet :: Event ,) , # [codec (index = 10)] TransactionPayment (runtime_types :: pallet_transaction_payment :: pallet :: Event ,) , # [codec (index = 11)] Sudo (runtime_types :: pallet_sudo :: pallet :: Event ,) , # [codec (index = 12)] ElectionProviderMultiPhase (runtime_types :: pallet_election_provider_multi_phase :: pallet :: Event ,) , # [codec (index = 13)] BagsList (runtime_types :: pallet_bags_list :: pallet :: Event ,) , # [codec (index = 14)] NominationPools (runtime_types :: pallet_nomination_pools :: pallet :: Event ,) , # [codec (index = 15)] Staking (runtime_types :: pallet_staking :: pallet :: pallet :: Event ,) , # [codec (index = 16)] Session (runtime_types :: pallet_session :: pallet :: Event ,) , # [codec (index = 18)] BridgeRegistry (runtime_types :: pallet_bridge_registry :: pallet :: Event ,) , # [codec (index = 19)] Identity (runtime_types :: pallet_identity :: pallet :: Event ,) , # [codec (index = 20)] ImOnline (runtime_types :: pallet_im_online :: pallet :: Event ,) , }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class:
                        runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct WrapperOpaque<_0>(
                        #[codec(compact)] pub ::core::primitive::u32,
                        pub _0,
                    );
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Clone,
                            Debug,
                            Eq,
                            PartialEq,
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
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct CheckMortality(
                        pub runtime_types::sp_runtime::generic::era::Era,
                    );
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct CheckNonce(
                        #[codec(compact)] pub ::core::primitive::u32,
                    );
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct BlockLength { pub max : runtime_types :: frame_support :: dispatch :: PerDispatchClass < :: core :: primitive :: u32 > , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct BlockWeights { pub base_block : runtime_types :: sp_weights :: weight_v2 :: Weight , pub max_block : runtime_types :: sp_weights :: weight_v2 :: Weight , pub per_class : runtime_types :: frame_support :: dispatch :: PerDispatchClass < runtime_types :: frame_system :: limits :: WeightsPerClass > , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic:
                        runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic: ::core::option::Option<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                    pub max_total: ::core::option::Option<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                    pub reserved: ::core::option::Option<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`"]
                    #[doc = "# </weight>"]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Set the new runtime code."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                    #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                    #[doc = "  expensive)."]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                    #[doc = "expensive. We will treat this as a full block."]
                    #[doc = "# </weight>"]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C)` where `C` length of `code`"]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                    #[doc = "block. # </weight>"]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::std::vec::Vec<
                            ::std::vec::Vec<::core::primitive::u8>,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    # [codec (index = 0)] # [doc = "An extrinsic completed successfully."] ExtrinsicSuccess { dispatch_info : runtime_types :: frame_support :: dispatch :: DispatchInfo , } , # [codec (index = 1)] # [doc = "An extrinsic failed."] ExtrinsicFailed { dispatch_error : runtime_types :: sp_runtime :: DispatchError , dispatch_info : runtime_types :: frame_support :: dispatch :: DispatchInfo , } , # [codec (index = 2)] # [doc = "`:code` was updated."] CodeUpdated , # [codec (index = 3)] # [doc = "A new account was created."] NewAccount { account : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , } , # [codec (index = 4)] # [doc = "An account was reaped."] KilledAccount { account : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , } , # [codec (index = 5)] # [doc = "On on-chain remark happened."] Remarked { sender : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , hash : :: subxt :: ext :: sp_core :: H256 , } , }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
        pub mod pallet_bags_list {
            use super::runtime_types;
            pub mod list {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Bag {
                    pub head: ::core::option::Option<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                    pub tail: ::core::option::Option<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub enum ListError {
                    #[codec(index = 0)]
                    Duplicate,
                    #[codec(index = 1)]
                    NotHeavier,
                    #[codec(index = 2)]
                    NotInSameBag,
                    #[codec(index = 3)]
                    NodeNotFound,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Node {
                    pub id: ::subxt::ext::sp_core::crypto::AccountId32,
                    pub prev: ::core::option::Option<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                    pub next: ::core::option::Option<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                    pub bag_upper: ::core::primitive::u64,
                    pub score: ::core::primitive::u64,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Declare that some `dislocated` account has, through rewards or penalties, sufficiently"]
                    #[doc = "changed its score that it should properly fall into a different bag than its current"]
                    #[doc = "one."]
                    #[doc = ""]
                    #[doc = "Anyone can call this function about any potentially dislocated account."]
                    #[doc = ""]
                    #[doc = "Will always update the stored score of `dislocated` to the correct score, based on"]
                    #[doc = "`ScoreProvider`."]
                    #[doc = ""]
                    #[doc = "If `dislocated` does not exists, it returns an error."]
                    rebag {
                        dislocated: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Move the caller's Id directly in front of `lighter`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and can only be called by the Id of"]
                    #[doc = "the account going in front of `lighter`."]
                    #[doc = ""]
                    #[doc = "Only works if"]
                    #[doc = "- both nodes are within the same bag,"]
                    #[doc = "- and `origin` has a greater `Score` than `lighter`."]
                    put_in_front_of {
                        lighter: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "A error in the list interface implementation."]
                    List(runtime_types::pallet_bags_list::list::ListError),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Moved an account from one bag to another."]
                    Rebagged {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        from: ::core::primitive::u64,
                        to: ::core::primitive::u64,
                    },
                    #[codec(index = 1)]
                    #[doc = "Updated the score of some account to the given amount."]
                    ScoreUpdated {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        new_score: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                    #[doc = "  types. See related functions below."]
                    #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                    #[doc = "  computation."]
                    #[doc = ""]
                    #[doc = "Related functions:"]
                    #[doc = ""]
                    #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                    #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                    #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                    #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                    #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                    #[doc = "    that the transfer will not kill the origin account."]
                    #[doc = "---------------------------------"]
                    #[doc = "- Origin account is already in memory, so no DB operations for them."]
                    #[doc = "# </weight>"]
                    transfer {
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the balances of a given account."]
                    #[doc = ""]
                    #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                    #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                    #[doc = "If the new free or reserved balance is below the existential deposit,"]
                    #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    set_balance {
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                    #[doc = "specified."]
                    #[doc = "# <weight>"]
                    #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                    #[doc = "  assumed to be in the overlay."]
                    #[doc = "# </weight>"]
                    force_transfer {
                        source: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                    #[doc = "origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true). # <weight>"]
                    #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                    #[doc = "  #</weight>"]
                    transfer_all {
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value"]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal"]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit"]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account"]
                    KeepAlive,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account"]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist"]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed MaxReserves"]
                    TooManyReserves,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    # [codec (index = 0)] # [doc = "An account was created with some free balance."] Endowed { account : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , free_balance : :: core :: primitive :: u128 , } , # [codec (index = 1)] # [doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"] # [doc = "resulting in an outright loss."] DustLost { account : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 2)] # [doc = "Transfer succeeded."] Transfer { from : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , to : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 3)] # [doc = "A balance was set by root."] BalanceSet { who : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , free : :: core :: primitive :: u128 , reserved : :: core :: primitive :: u128 , } , # [codec (index = 4)] # [doc = "Some balance was reserved (moved from free to reserved)."] Reserved { who : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 5)] # [doc = "Some balance was unreserved (moved from reserved to free)."] Unreserved { who : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 6)] # [doc = "Some balance was moved from the reserve of the first account to the second account."] # [doc = "Final argument indicates the destination balance type."] ReserveRepatriated { from : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , to : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , destination_status : runtime_types :: frame_support :: traits :: tokens :: misc :: BalanceStatus , } , # [codec (index = 7)] # [doc = "Some amount was deposited (e.g. for transaction fees)."] Deposit { who : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 8)] # [doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."] Withdraw { who : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 9)] # [doc = "Some amount was removed from the account (e.g. for misbehavior)."] Slashed { who : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_bridge_registry {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Set an account's identity information and reserve the appropriate deposit."] # [doc = ""] # [doc = "If the account already has identity information, the deposit is taken as part payment"] # [doc = "for the new deposit."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_."] # [doc = ""] # [doc = "- `info`: The identity information."] # [doc = ""] # [doc = "Emits `ResourceSet` if successful."] set_metadata { bridge_index : :: core :: primitive :: u32 , info : runtime_types :: pallet_bridge_registry :: types :: BridgeInfo , } , # [codec (index = 1)] force_reset_indices { resource_ids : :: std :: vec :: Vec < runtime_types :: webb_proposals :: header :: ResourceId > , bridge_index : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Parameters haven't been initialized"]
                    ParametersNotInitialized,
                    #[codec(index = 1)]
                    #[doc = "Error during verification"]
                    VerifyError,
                    #[codec(index = 2)]
                    #[doc = "Proposal is not signed and should not be processed"]
                    ProposalNotSigned,
                    #[codec(index = 3)]
                    #[doc = "Resources map to different bridge indices"]
                    BridgeIndexError,
                    #[codec(index = 4)]
                    #[doc = "Too many additional fields."]
                    TooManyFields,
                    #[codec(index = 5)]
                    #[doc = "Bridge does not exist."]
                    BridgeNotFound,
                    #[codec(index = 6)]
                    #[doc = "Too many resources."]
                    TooManyResources,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {}
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct BridgeInfo { pub additional : runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < (runtime_types :: pallet_identity :: types :: Data , runtime_types :: pallet_identity :: types :: Data ,) > , pub display : runtime_types :: pallet_identity :: types :: Data , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct BridgeMetadata { pub resource_ids : runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < runtime_types :: webb_proposals :: header :: ResourceId > , pub info : runtime_types :: pallet_bridge_registry :: types :: BridgeInfo , }
            }
        }
        pub mod pallet_dkg_metadata {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Set the pending signature threshold for the session following the next session."] # [doc = ""] # [doc = "We cannot assume that the next DKG has not already completed keygen."] # [doc = "After all, if we are in a new session the next DKG may have already completed."] # [doc = "Therefore, when we update the thresholds we are updating a threshold"] # [doc = "that will become the next threshold after the next session update."] # [doc = ""] # [doc = "* `origin` - The account origin."] # [doc = "* `new_threshold` - The new signature threshold for the DKG."] set_signature_threshold { new_threshold : :: core :: primitive :: u16 , } , # [codec (index = 1)] # [doc = "Set the pending keygen threshold for the session following the next session."] # [doc = ""] # [doc = "We cannot assume that the next DKG has not already completed keygen."] # [doc = "After all, if we are in a new session the next DKG may have already completed."] # [doc = "Therefore, when we update the thresholds we are updating a threshold"] # [doc = "that will become the next threshold after the next session update."] # [doc = ""] # [doc = "* `origin` - The account origin."] # [doc = "* `new_threshold` - The new keygen threshold for the DKG."] set_keygen_threshold { new_threshold : :: core :: primitive :: u16 , } , # [codec (index = 2)] # [doc = "Sets the delay when a unsigned `RefreshProposal` will be added to the unsigned"] # [doc = "proposal queue."] # [doc = ""] # [doc = "* `origin` - The account origin."] # [doc = "* `new_delay` - The percentage of elapsed session duration to wait before adding an"] # [doc = "  unsigned refresh proposal to the unsigned proposal queue."] set_refresh_delay { new_delay : :: core :: primitive :: u8 , } , # [codec (index = 3)] # [doc = "Submits and stores the active public key for the genesis session into the on-chain"] # [doc = "storage. This is primarily used to separate the genesis public key submission from"] # [doc = "non-genesis rounds."] # [doc = ""] # [doc = "Can only be submitted by the current authorities. It is also required that a"] # [doc = "`SignatureThreshold` of submissions is reached in order to successfully"] # [doc = "store the public key on-chain."] # [doc = ""] # [doc = "* `origin` - The account origin."] # [doc = "* `keys_and_signatures` - The aggregated public keys and signatures for possible current"] # [doc = "  DKG public keys."] submit_public_key { keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys , } , # [codec (index = 4)] # [doc = "Submits and stores the next public key for the next session into the on-chain storage."] # [doc = ""] # [doc = "Can only be submitted by the next authorities. It is also required that a"] # [doc = "`NextSignatureThreshold` of submissions is reached in order to successfully"] # [doc = "store the public key on-chain."] # [doc = ""] # [doc = "* `origin` - The account origin."] # [doc = "* `keys_and_signatures` - The aggregated public keys and signatures for possible next"] # [doc = "  DKG public keys."] submit_next_public_key { keys_and_signatures : runtime_types :: dkg_runtime_primitives :: AggregatedPublicKeys , } , # [codec (index = 5)] # [doc = "Submits the public key signature for the key refresh/rotation process."] # [doc = ""] # [doc = "The signature is the signature of the next public key `RefreshProposal`, signed by the"] # [doc = "current DKG. It is stored on-chain only if it verifies successfully against the current"] # [doc = "DKG's public key. Successful storage of this public key signature also removes"] # [doc = "the unsigned `RefreshProposal` from the unsigned queue."] # [doc = ""] # [doc = "For manual refreshes, after the signature is submitted and stored on-chain,"] # [doc = "the keys are immediately refreshed and the authority set is immediately rotated"] # [doc = "and incremented."] # [doc = ""] # [doc = "* `origin` - The account origin."] # [doc = "* `signature_proposal` - The signed refresh proposal containing the public key signature"] # [doc = "  and nonce."] submit_public_key_signature { signature_proposal : runtime_types :: dkg_runtime_primitives :: proposal :: RefreshProposalSigned , } , # [codec (index = 6)] # [doc = "Submits misbehaviour reports on chain. Signatures of the offending authority are"] # [doc = "verified against the current or next authorities depending on the type of misbehaviour."] # [doc = "- Keygen: Verifies against the next authorities, since they are doing keygen."] # [doc = "- Signing: Verifies against the current authorities, since they are doing signing."] # [doc = ""] # [doc = "Verifies the reports against the respective thresholds and if enough reports are met"] # [doc = "begins to jail and decrease the reputation of the offending authority."] # [doc = ""] # [doc = "The misbehaviour reputation update is:"] # [doc = "\tAUTHORITY_REPUTATION = DECAY_PERCENTAGE * AUTHORITY_REPUTATION"] # [doc = ""] # [doc = "If there are not enough unjailed keygen authorities to perform a keygen after the next"] # [doc = "session, then we deduct the pending keygen threshold (and pending signing threshold)"] # [doc = "accordingly."] # [doc = ""] # [doc = "* `origin` - The account origin."] # [doc = "* `reports` - The aggregated misbehaviour reports containing signatures of an offending"] # [doc = "  authority"] submit_misbehaviour_reports { reports : runtime_types :: dkg_runtime_primitives :: AggregatedMisbehaviourReports < runtime_types :: dkg_runtime_primitives :: crypto :: Public > , } , # [codec (index = 7)] # [doc = "Attempts to remove an authority from all possible jails (keygen & signing)."] # [doc = "This can only be called by the controller of the authority in jail. The"] # [doc = "origin must map directly to the authority in jail."] # [doc = ""] # [doc = "The authority's jail sentence for either keygen or signing must be elapsed"] # [doc = "for the authority to be removed from the jail."] # [doc = ""] # [doc = "* `origin` - The account origin."] unjail , # [codec (index = 8)] # [doc = "Force removes an authority from keygen jail."] # [doc = ""] # [doc = "Can only be called by DKG"] # [doc = "* `origin` - The account origin."] # [doc = "* `authority` - The authority to be removed from the keygen jail."] force_unjail_keygen { authority : runtime_types :: dkg_runtime_primitives :: crypto :: Public , } , # [codec (index = 9)] # [doc = "Force removes an authority from signing jail."] # [doc = ""] # [doc = "Can only be called by the root origin."] # [doc = ""] # [doc = "* `origin` - The account origin."] # [doc = "* `authority` - The authority to be removed from the signing jail."] force_unjail_signing { authority : runtime_types :: dkg_runtime_primitives :: crypto :: Public , } , # [codec (index = 10)] # [doc = "Forcefully rotate the DKG"] # [doc = ""] # [doc = "This forces the next authorities into the current authority spot and"] # [doc = "automatically increments the authority ID. It uses `change_authorities`"] # [doc = "to execute the rotation forcefully."] force_change_authorities , # [codec (index = 11)] # [doc = "Triggers an Emergency Keygen Porotocol."] # [doc = ""] # [doc = "The keygen protocol will then be executed and the result will be stored in the off chain"] # [doc = "storage, which will be picked up by the on chain worker and stored on chain."] # [doc = ""] # [doc = "Note that, this will clear the next public key and its signature, if any."] trigger_emergency_keygen , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "No mapped account to authority"]
                    NoMappedAccount,
                    #[codec(index = 1)]
                    #[doc = "Invalid threshold"]
                    InvalidThreshold,
                    #[codec(index = 2)]
                    #[doc = "Must be queued  to become an authority"]
                    MustBeAQueuedAuthority,
                    #[codec(index = 3)]
                    #[doc = "Must be an an authority"]
                    MustBeAnActiveAuthority,
                    #[codec(index = 4)]
                    #[doc = "Refresh delay should be in the range of 0% - 100%"]
                    InvalidRefreshDelay,
                    #[codec(index = 5)]
                    #[doc = "Invalid public key submission"]
                    InvalidPublicKeys,
                    #[codec(index = 6)]
                    #[doc = "Already submitted a public key"]
                    AlreadySubmittedPublicKey,
                    #[codec(index = 7)]
                    #[doc = "Already submitted a public key signature"]
                    AlreadySubmittedSignature,
                    #[codec(index = 8)]
                    #[doc = "Used signature from past sessions"]
                    UsedSignature,
                    #[codec(index = 9)]
                    #[doc = "Invalid public key signature submission"]
                    InvalidSignature,
                    #[codec(index = 10)]
                    #[doc = "Invalid Nonece used, must be greater than [`refresh_nonce`]."]
                    InvalidNonce,
                    #[codec(index = 11)]
                    #[doc = "Invalid misbehaviour reports"]
                    InvalidMisbehaviourReports,
                    #[codec(index = 12)]
                    #[doc = "DKG Refresh is already in progress."]
                    RefreshInProgress,
                    #[codec(index = 13)]
                    #[doc = "No NextPublicKey stored on-chain."]
                    NoNextPublicKey,
                    #[codec(index = 14)]
                    #[doc = "Must be calling from the controller account"]
                    InvalidControllerAccount,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    # [codec (index = 0)] # [doc = "Current public key submitted"] PublicKeySubmitted { compressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , uncompressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 1)] # [doc = "Next public key submitted"] NextPublicKeySubmitted { compressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , uncompressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] # [doc = "Next public key signature submitted"] NextPublicKeySignatureSubmitted { pub_key_sig : :: std :: vec :: Vec < :: core :: primitive :: u8 > , compressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , uncompressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , nonce : :: core :: primitive :: u32 , } , # [codec (index = 3)] # [doc = "Current Public Key Changed."] PublicKeyChanged { compressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , uncompressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 4)] # [doc = "Current Public Key Signature Changed."] PublicKeySignatureChanged { pub_key_sig : :: std :: vec :: Vec < :: core :: primitive :: u8 > , compressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , uncompressed_pub_key : :: std :: vec :: Vec < :: core :: primitive :: u8 > , nonce : :: core :: primitive :: u32 , } , # [codec (index = 5)] # [doc = "Misbehaviour reports submitted"] MisbehaviourReportsSubmitted { misbehaviour_type : runtime_types :: dkg_runtime_primitives :: MisbehaviourType , reporters : :: std :: vec :: Vec < runtime_types :: dkg_runtime_primitives :: crypto :: Public > , } , # [codec (index = 6)] # [doc = "Refresh DKG Keys Finished (forcefully)."] RefreshKeysFinished { next_authority_set_id : :: core :: primitive :: u64 , } , # [codec (index = 7)] # [doc = "NextKeygenThreshold updated"] NextKeygenThresholdUpdated { next_keygen_threshold : :: core :: primitive :: u16 , } , # [codec (index = 8)] # [doc = "NextSignatureThreshold updated"] NextSignatureThresholdUpdated { next_signature_threshold : :: core :: primitive :: u16 , } , # [codec (index = 9)] # [doc = "An Emergency Keygen Protocol was triggered."] EmergencyKeygenTriggered , }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    submit_signed_proposals {
                        props: ::std::vec::Vec<
                            runtime_types::webb_proposals::proposal::Proposal,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Force submit an unsigned proposal to the DKG"]
                    #[doc = ""]
                    #[doc = "There are certain proposals we'd like to be proposable only"]
                    #[doc = "through root actions. The currently supported proposals are"]
                    #[doc = "\t1. Updating"]
                    force_submit_unsigned_proposal {
                        prop: runtime_types::webb_proposals::proposal::Proposal,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Error names should be descriptive."]
                    NoneValue,
                    #[codec(index = 1)]
                    #[doc = "Errors should have helpful documentation associated with them."]
                    StorageOverflow,
                    #[codec(index = 2)]
                    #[doc = "Proposal format is invalid"]
                    ProposalFormatInvalid,
                    #[codec(index = 3)]
                    #[doc = "Proposal signature is invalid"]
                    ProposalSignatureInvalid,
                    #[codec(index = 4)]
                    #[doc = "No proposal with the ID was found"]
                    ProposalDoesNotExists,
                    #[codec(index = 5)]
                    #[doc = "Proposal with the ID has already been submitted"]
                    ProposalAlreadyExists,
                    #[codec(index = 6)]
                    #[doc = "Chain id is invalid"]
                    ChainIdInvalid,
                    #[codec(index = 7)]
                    #[doc = "Proposal length exceeds max allowed per batch"]
                    ProposalsLengthOverflow,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    # [codec (index = 0)] # [doc = "RuntimeEvent Emitted when we encounter a Proposal with invalid Signature."] InvalidProposalSignature { kind : runtime_types :: webb_proposals :: proposal :: ProposalKind , data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , invalid_signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , expected_public_key : :: core :: option :: Option < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , actual_public_key : :: core :: option :: Option < :: std :: vec :: Vec < :: core :: primitive :: u8 > > , } , # [codec (index = 1)] # [doc = "RuntimeEvent When a Proposal is added to UnsignedProposalQueue."] ProposalAdded { key : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , target_chain : runtime_types :: webb_proposals :: header :: TypedChainId , data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] # [doc = "RuntimeEvent When a Proposal is removed from UnsignedProposalQueue."] ProposalRemoved { key : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , target_chain : runtime_types :: webb_proposals :: header :: TypedChainId , } , # [codec (index = 3)] # [doc = "RuntimeEvent When a Proposal Gets Signed by DKG."] ProposalSigned { key : runtime_types :: dkg_runtime_primitives :: proposal :: DKGPayloadKey , target_chain : runtime_types :: webb_proposals :: header :: TypedChainId , data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
            }
        }
        pub mod pallet_dkg_proposals {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Sets the vote threshold for proposals."]
                    #[doc = ""]
                    #[doc = "This threshold is used to determine how many votes are required"]
                    #[doc = "before a proposal is executed."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1) lookup and insert"]
                    #[doc = "# </weight>"]
                    set_threshold { threshold: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "Stores a method name on chain under an associated resource ID."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1) write"]
                    #[doc = "# </weight>"]
                    set_resource {
                        id: runtime_types::webb_proposals::header::ResourceId,
                        method: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Removes a resource ID from the resource mapping."]
                    #[doc = ""]
                    #[doc = "After this call, bridge transfers with the associated resource ID"]
                    #[doc = "will be rejected."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1) removal"]
                    #[doc = "# </weight>"]
                    remove_resource {
                        id: runtime_types::webb_proposals::header::ResourceId,
                    },
                    #[codec(index = 3)]
                    #[doc = "Enables a chain ID as a source or destination for a bridge transfer."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1) lookup and insert"]
                    #[doc = "# </weight>"]
                    whitelist_chain {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                    },
                    #[codec(index = 4)]
                    #[doc = "Adds a new proposer to the proposer set."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1) lookup and insert"]
                    #[doc = "# </weight>"]
                    add_proposer {
                        native_account:
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        external_account:
                            ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Removes an existing proposer from the set."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1) lookup and removal"]
                    #[doc = "# </weight>"]
                    remove_proposer {
                        v: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Commits a vote in favour of the provided proposal."]
                    #[doc = ""]
                    #[doc = "If a proposal with the given nonce and source chain ID does not"]
                    #[doc = "already exist, it will be created with an initial vote in favour"]
                    #[doc = "from the caller."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- weight of proposed call, regardless of whether execution is performed"]
                    #[doc = "# </weight>"]
                    acknowledge_proposal {
                        nonce: runtime_types::webb_proposals::nonce::Nonce,
                        src_chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        r_id: runtime_types::webb_proposals::header::ResourceId,
                        prop: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Commits a vote against a provided proposal."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Fixed, since execution of proposal should not be included"]
                    #[doc = "# </weight>"]
                    reject_proposal {
                        nonce: runtime_types::webb_proposals::nonce::Nonce,
                        src_chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        r_id: runtime_types::webb_proposals::header::ResourceId,
                        prop: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    #[doc = "Evaluate the state of a proposal given the current vote threshold."]
                    #[doc = ""]
                    #[doc = "A proposal with enough votes will be either executed or cancelled,"]
                    #[doc = "and the status will be updated accordingly."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- weight of proposed call, regardless of whether execution is performed"]
                    #[doc = "# </weight>"]
                    eval_vote_state {
                        nonce: runtime_types::webb_proposals::nonce::Nonce,
                        src_chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        prop: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Account does not have correct permissions"]
                    InvalidPermissions,
                    #[codec(index = 1)]
                    #[doc = "Proposer threshold not set"]
                    ThresholdNotSet,
                    #[codec(index = 2)]
                    #[doc = "Provided chain Id is not valid"]
                    InvalidChainId,
                    #[codec(index = 3)]
                    #[doc = "Proposer threshold cannot be 0"]
                    InvalidThreshold,
                    #[codec(index = 4)]
                    #[doc = "Interactions with this chain is not permitted"]
                    ChainNotWhitelisted,
                    #[codec(index = 5)]
                    #[doc = "Chain has already been enabled"]
                    ChainAlreadyWhitelisted,
                    #[codec(index = 6)]
                    #[doc = "Resource ID provided isn't mapped to anything"]
                    ResourceDoesNotExist,
                    #[codec(index = 7)]
                    #[doc = "Proposer already in set"]
                    ProposerAlreadyExists,
                    #[codec(index = 8)]
                    #[doc = "Provided accountId is not a proposer"]
                    ProposerInvalid,
                    #[codec(index = 9)]
                    #[doc = "Protected operation, must be performed by proposer"]
                    MustBeProposer,
                    #[codec(index = 10)]
                    #[doc = "Proposer has already submitted some vote for this proposal"]
                    ProposerAlreadyVoted,
                    #[codec(index = 11)]
                    #[doc = "A proposal with these parameters has already been submitted"]
                    ProposalAlreadyExists,
                    #[codec(index = 12)]
                    #[doc = "No proposal with the ID was found"]
                    ProposalDoesNotExist,
                    #[codec(index = 13)]
                    #[doc = "Cannot complete proposal, needs more votes"]
                    ProposalNotComplete,
                    #[codec(index = 14)]
                    #[doc = "Proposal has either failed or succeeded"]
                    ProposalAlreadyComplete,
                    #[codec(index = 15)]
                    #[doc = "Lifetime of proposal has been exceeded"]
                    ProposalExpired,
                    #[codec(index = 16)]
                    #[doc = "Proposer Count is Zero"]
                    ProposerCountIsZero,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Vote threshold has changed (new_threshold)"]
                    ProposerThresholdChanged {
                        new_threshold: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Chain now available for transfers (chain_id)"]
                    ChainWhitelisted {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                    },
                    #[codec(index = 2)]
                    #[doc = "Proposer added to set"]
                    ProposerAdded {
                        proposer_id: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Proposer removed from set"]
                    ProposerRemoved {
                        proposer_id: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "Vote submitted in favour of proposal"]
                    VoteFor {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Vot submitted against proposal"]
                    VoteAgainst {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Voting successful for a proposal"]
                    ProposalApproved {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                    },
                    #[codec(index = 7)]
                    #[doc = "Voting rejected a proposal"]
                    ProposalRejected {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                    },
                    #[codec(index = 8)]
                    #[doc = "Execution of call succeeded"]
                    ProposalSucceeded {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                    },
                    #[codec(index = 9)]
                    #[doc = "Execution of call failed"]
                    ProposalFailed {
                        chain_id:
                            runtime_types::webb_proposals::header::TypedChainId,
                        proposal_nonce:
                            runtime_types::webb_proposals::nonce::Nonce,
                    },
                    #[codec(index = 10)]
                    #[doc = "Proposers have been reset"]
                    AuthorityProposersReset {
                        proposers: ::std::vec::Vec<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct ProposalVotes < _0 , _1 > { pub votes_for : :: std :: vec :: Vec < _0 > , pub votes_against : :: std :: vec :: Vec < _0 > , pub status : runtime_types :: pallet_dkg_proposals :: types :: ProposalStatus , pub expiry : _1 , }
            }
        }
        pub mod pallet_election_provider_multi_phase {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Submit a solution for the unsigned phase."] # [doc = ""] # [doc = "The dispatch origin fo this call must be __none__."] # [doc = ""] # [doc = "This submission is checked on the fly. Moreover, this unsigned solution is only"] # [doc = "validated when submitted to the pool from the **local** node. Effectively, this means"] # [doc = "that only active validators can submit this transaction when authoring a block (similar"] # [doc = "to an inherent)."] # [doc = ""] # [doc = "To prevent any incorrect solution (and thus wasted time/weight), this transaction will"] # [doc = "panic if the solution submitted by the validator is invalid in any way, effectively"] # [doc = "putting their authoring reward at risk."] # [doc = ""] # [doc = "No deposit or reward is associated with this submission."] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] # [doc = "Set a new value for `MinimumUntrustedScore`."] # [doc = ""] # [doc = "Dispatch origin must be aligned with `T::ForceOrigin`."] # [doc = ""] # [doc = "This check can be turned off by setting the value to `None`."] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < runtime_types :: sp_npos_elections :: ElectionScore > , } , # [codec (index = 2)] # [doc = "Set a solution in the queue, to be handed out to the client of this pallet in the next"] # [doc = "call to `ElectionProvider::elect`."] # [doc = ""] # [doc = "This can only be set by `T::ForceOrigin`, and only when the phase is `Emergency`."] # [doc = ""] # [doc = "The solution is not checked for any feasibility and is assumed to be trustworthy, as any"] # [doc = "feasibility check itself can in principle cause the election process to fail (due to"] # [doc = "memory/weight constrains)."] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) > , } , # [codec (index = 3)] # [doc = "Submit a solution for the signed phase."] # [doc = ""] # [doc = "The dispatch origin fo this call must be __signed__."] # [doc = ""] # [doc = "The solution is potentially queued, based on the claimed score and processed at the end"] # [doc = "of the signed phase."] # [doc = ""] # [doc = "A deposit is reserved and recorded for the solution. Based on the outcome, the solution"] # [doc = "might be rewarded, slashed, or get all or a part of the deposit back."] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: dkg_standalone_runtime :: NposSolution16 > > , } , # [codec (index = 4)] # [doc = "Trigger the governance fallback."] # [doc = ""] # [doc = "This can only be called when [`Phase::Emergency`] is enabled, as an alternative to"] # [doc = "calling [`Call::set_emergency_election_result`]."] governance_fallback { maybe_max_voters : :: core :: option :: Option < :: core :: primitive :: u32 > , maybe_max_targets : :: core :: option :: Option < :: core :: primitive :: u32 > , } , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Error of the pallet that can be returned in response to dispatches."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Submission was too early."]
                    PreDispatchEarlySubmission,
                    #[codec(index = 1)]
                    #[doc = "Wrong number of winners presented."]
                    PreDispatchWrongWinnerCount,
                    #[codec(index = 2)]
                    #[doc = "Submission was too weak, score-wise."]
                    PreDispatchWeakSubmission,
                    #[codec(index = 3)]
                    #[doc = "The queue was full, and the solution was not better than any of the existing ones."]
                    SignedQueueFull,
                    #[codec(index = 4)]
                    #[doc = "The origin failed to pay the deposit."]
                    SignedCannotPayDeposit,
                    #[codec(index = 5)]
                    #[doc = "Witness data to dispatchable is invalid."]
                    SignedInvalidWitness,
                    #[codec(index = 6)]
                    #[doc = "The signed submission consumes too much weight"]
                    SignedTooMuchWeight,
                    #[codec(index = 7)]
                    #[doc = "OCW submitted solution for wrong round"]
                    OcwCallWrongEra,
                    #[codec(index = 8)]
                    #[doc = "Snapshot metadata should exist but didn't."]
                    MissingSnapshotMetadata,
                    #[codec(index = 9)]
                    #[doc = "`Self::insert_submission` returned an invalid index."]
                    InvalidSubmissionIndex,
                    #[codec(index = 10)]
                    #[doc = "The call is not allowed at this point."]
                    CallNotAllowed,
                    #[codec(index = 11)]
                    #[doc = "The fallback failed"]
                    FallbackFailed,
                    #[codec(index = 12)]
                    #[doc = "Some bound not met"]
                    BoundNotMet,
                    #[codec(index = 13)]
                    #[doc = "Submitted solution has too many winners"]
                    TooManyWinners,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    # [codec (index = 0)] # [doc = "A solution was stored with the given compute."] # [doc = ""] # [doc = "If the solution is signed, this means that it hasn't yet been processed. If the"] # [doc = "solution is unsigned, this means that it has also been processed."] # [doc = ""] # [doc = "The `bool` is `true` when a previous solution was ejected to make room for this one."] SolutionStored { compute : runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , prev_ejected : :: core :: primitive :: bool , } , # [codec (index = 1)] # [doc = "The election has been finalized, with the given computation and score."] ElectionFinalized { compute : runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , score : runtime_types :: sp_npos_elections :: ElectionScore , } , # [codec (index = 2)] # [doc = "An election failed."] # [doc = ""] # [doc = "Not much can be said about which computes failed in the process."] ElectionFailed , # [codec (index = 3)] # [doc = "An account has been rewarded for their signed submission being finalized."] Rewarded { account : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , value : :: core :: primitive :: u128 , } , # [codec (index = 4)] # [doc = "An account has been slashed for submitting an invalid signed submission."] Slashed { account : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , value : :: core :: primitive :: u128 , } , # [codec (index = 5)] # [doc = "The signed phase of the given round has started."] SignedPhaseStarted { round : :: core :: primitive :: u32 , } , # [codec (index = 6)] # [doc = "The unsigned phase of the given round has started."] UnsignedPhaseStarted { round : :: core :: primitive :: u32 , } , }
            }
            pub mod signed {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct SignedSubmission < _0 , _1 , _2 > { pub who : _0 , pub deposit : _1 , pub raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < _2 > , pub call_fee : _1 , }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RawSolution<_0> {
                pub solution: _0,
                pub score: runtime_types::sp_npos_elections::ElectionScore,
                pub round: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ReadySolution { pub supports : runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > ,) > , pub score : runtime_types :: sp_npos_elections :: ElectionScore , pub compute : runtime_types :: pallet_election_provider_multi_phase :: ElectionCompute , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RoundSnapshot {
                pub voters: ::std::vec::Vec<(
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    ::core::primitive::u64,
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                )>,
                pub targets:
                    ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Report voter equivocation/misbehavior. This method will verify the"] # [doc = "equivocation proof and validate the given key ownership proof"] # [doc = "against the extracted offender. If both are valid, the offence"] # [doc = "will be reported."] report_equivocation { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: ext :: sp_core :: H256 , :: core :: primitive :: u32 > > , key_owner_proof : runtime_types :: sp_core :: Void , } , # [codec (index = 1)] # [doc = "Report voter equivocation/misbehavior. This method will verify the"] # [doc = "equivocation proof and validate the given key ownership proof"] # [doc = "against the extracted offender. If both are valid, the offence"] # [doc = "will be reported."] # [doc = ""] # [doc = "This extrinsic must be called unsigned and it is expected that only"] # [doc = "block authors will call it (validated in `ValidateUnsigned`), as such"] # [doc = "if the block author is defined it will be defined as the equivocation"] # [doc = "reporter."] report_equivocation_unsigned { equivocation_proof : :: std :: boxed :: Box < runtime_types :: sp_finality_grandpa :: EquivocationProof < :: subxt :: ext :: sp_core :: H256 , :: core :: primitive :: u32 > > , key_owner_proof : runtime_types :: sp_core :: Void , } , # [codec (index = 2)] # [doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."] # [doc = ""] # [doc = "This will trigger a forced authority set change at the beginning of the next session, to"] # [doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"] # [doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."] # [doc = "The block production rate (which may be slowed down because of finality lagging) should"] # [doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"] # [doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"] # [doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"] # [doc = "block of all validators of the new authority set."] # [doc = ""] # [doc = "Only callable by root."] note_stalled { delay : :: core :: primitive :: u32 , best_finalized_block_number : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                    #[doc = "(either paused or already pending pause)."]
                    PauseFailed,
                    #[codec(index = 1)]
                    #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                    #[doc = "(either live or already pending resume)."]
                    ResumeFailed,
                    #[codec(index = 2)]
                    #[doc = "Attempt to signal GRANDPA change with one already pending."]
                    ChangePending,
                    #[codec(index = 3)]
                    #[doc = "Cannot signal forced change so soon after last."]
                    TooSoon,
                    #[codec(index = 4)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New authority set has been applied."]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Current authority set has been paused."]
                    Paused,
                    #[codec(index = 2)]
                    #[doc = "Current authority set has been resumed."]
                    Resumed,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct StoredPendingChange < _0 > { pub scheduled_at : _0 , pub delay : _0 , pub next_authorities : runtime_types :: sp_core :: bounded :: weak_bounded_vec :: WeakBoundedVec < (runtime_types :: sp_finality_grandpa :: app :: Public , :: core :: primitive :: u64 ,) > , pub forced : :: core :: option :: Option < _0 > , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
        pub mod pallet_identity {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Identity pallet declaration."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Add a registrar to the system."] # [doc = ""] # [doc = "The dispatch origin for this call must be `T::RegistrarOrigin`."] # [doc = ""] # [doc = "- `account`: the account of the registrar."] # [doc = ""] # [doc = "Emits `RegistrarAdded` if successful."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(R)` where `R` registrar-count (governance-bounded and code-bounded)."] # [doc = "- One storage mutation (codec `O(R)`)."] # [doc = "- One event."] # [doc = "# </weight>"] add_registrar { account : :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 1)] # [doc = "Set an account's identity information and reserve the appropriate deposit."] # [doc = ""] # [doc = "If the account already has identity information, the deposit is taken as part payment"] # [doc = "for the new deposit."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_."] # [doc = ""] # [doc = "- `info`: The identity information."] # [doc = ""] # [doc = "Emits `IdentitySet` if successful."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(X + X' + R)`"] # [doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)"] # [doc = "  - where `R` judgements-count (registrar-count-bounded)"] # [doc = "- One balance reserve operation."] # [doc = "- One storage mutation (codec-read `O(X' + R)`, codec-write `O(X + R)`)."] # [doc = "- One event."] # [doc = "# </weight>"] set_identity { info : :: std :: boxed :: Box < runtime_types :: pallet_identity :: types :: IdentityInfo > , } , # [codec (index = 2)] # [doc = "Set the sub-accounts of the sender."] # [doc = ""] # [doc = "Payment: Any aggregate balance reserved by previous `set_subs` calls will be returned"] # [doc = "and an amount `SubAccountDeposit` will be reserved for each item in `subs`."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"] # [doc = "identity."] # [doc = ""] # [doc = "- `subs`: The identity's (new) sub-accounts."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(P + S)`"] # [doc = "  - where `P` old-subs-count (hard- and deposit-bounded)."] # [doc = "  - where `S` subs-count (hard- and deposit-bounded)."] # [doc = "- At most one balance operations."] # [doc = "- DB:"] # [doc = "  - `P + S` storage mutations (codec complexity `O(1)`)"] # [doc = "  - One storage read (codec complexity `O(P)`)."] # [doc = "  - One storage write (codec complexity `O(S)`)."] # [doc = "  - One storage-exists (`IdentityOf::contains_key`)."] # [doc = "# </weight>"] set_subs { subs : :: std :: vec :: Vec < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , runtime_types :: pallet_identity :: types :: Data ,) > , } , # [codec (index = 3)] # [doc = "Clear an account's identity info and all sub-accounts and return all deposits."] # [doc = ""] # [doc = "Payment: All reserved balances on the account are returned."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"] # [doc = "identity."] # [doc = ""] # [doc = "Emits `IdentityCleared` if successful."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(R + S + X)`"] # [doc = "  - where `R` registrar-count (governance-bounded)."] # [doc = "  - where `S` subs-count (hard- and deposit-bounded)."] # [doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."] # [doc = "- One balance-unreserve operation."] # [doc = "- `2` storage reads and `S + 2` storage deletions."] # [doc = "- One event."] # [doc = "# </weight>"] clear_identity , # [codec (index = 4)] # [doc = "Request a judgement from a registrar."] # [doc = ""] # [doc = "Payment: At most `max_fee` will be reserved for payment to the registrar if judgement"] # [doc = "given."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"] # [doc = "registered identity."] # [doc = ""] # [doc = "- `reg_index`: The index of the registrar whose judgement is requested."] # [doc = "- `max_fee`: The maximum fee that may be paid. This should just be auto-populated as:"] # [doc = ""] # [doc = "```nocompile"] # [doc = "Self::registrars().get(reg_index).unwrap().fee"] # [doc = "```"] # [doc = ""] # [doc = "Emits `JudgementRequested` if successful."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(R + X)`."] # [doc = "- One balance-reserve operation."] # [doc = "- Storage: 1 read `O(R)`, 1 mutate `O(X + R)`."] # [doc = "- One event."] # [doc = "# </weight>"] request_judgement { # [codec (compact)] reg_index : :: core :: primitive :: u32 , # [codec (compact)] max_fee : :: core :: primitive :: u128 , } , # [codec (index = 5)] # [doc = "Cancel a previous request."] # [doc = ""] # [doc = "Payment: A previously reserved deposit is returned on success."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"] # [doc = "registered identity."] # [doc = ""] # [doc = "- `reg_index`: The index of the registrar whose judgement is no longer requested."] # [doc = ""] # [doc = "Emits `JudgementUnrequested` if successful."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(R + X)`."] # [doc = "- One balance-reserve operation."] # [doc = "- One storage mutation `O(R + X)`."] # [doc = "- One event"] # [doc = "# </weight>"] cancel_request { reg_index : :: core :: primitive :: u32 , } , # [codec (index = 6)] # [doc = "Set the fee required for a judgement to be requested from a registrar."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"] # [doc = "of the registrar whose index is `index`."] # [doc = ""] # [doc = "- `index`: the index of the registrar whose fee is to be set."] # [doc = "- `fee`: the new fee."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(R)`."] # [doc = "- One storage mutation `O(R)`."] # [doc = "- Benchmark: 7.315 + R * 0.329 µs (min squares analysis)"] # [doc = "# </weight>"] set_fee { # [codec (compact)] index : :: core :: primitive :: u32 , # [codec (compact)] fee : :: core :: primitive :: u128 , } , # [codec (index = 7)] # [doc = "Change the account associated with a registrar."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"] # [doc = "of the registrar whose index is `index`."] # [doc = ""] # [doc = "- `index`: the index of the registrar whose fee is to be set."] # [doc = "- `new`: the new account ID."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(R)`."] # [doc = "- One storage mutation `O(R)`."] # [doc = "- Benchmark: 8.823 + R * 0.32 µs (min squares analysis)"] # [doc = "# </weight>"] set_account_id { # [codec (compact)] index : :: core :: primitive :: u32 , new : :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 8)] # [doc = "Set the field information for a registrar."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"] # [doc = "of the registrar whose index is `index`."] # [doc = ""] # [doc = "- `index`: the index of the registrar whose fee is to be set."] # [doc = "- `fields`: the fields that the registrar concerns themselves with."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(R)`."] # [doc = "- One storage mutation `O(R)`."] # [doc = "- Benchmark: 7.464 + R * 0.325 µs (min squares analysis)"] # [doc = "# </weight>"] set_fields { # [codec (compact)] index : :: core :: primitive :: u32 , fields : runtime_types :: pallet_identity :: types :: BitFlags < runtime_types :: pallet_identity :: types :: IdentityField > , } , # [codec (index = 9)] # [doc = "Provide a judgement for an account's identity."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"] # [doc = "of the registrar whose index is `reg_index`."] # [doc = ""] # [doc = "- `reg_index`: the index of the registrar whose judgement is being made."] # [doc = "- `target`: the account whose identity the judgement is upon. This must be an account"] # [doc = "  with a registered identity."] # [doc = "- `judgement`: the judgement of the registrar of index `reg_index` about `target`."] # [doc = "- `identity`: The hash of the [`IdentityInfo`] for that the judgement is provided."] # [doc = ""] # [doc = "Emits `JudgementGiven` if successful."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(R + X)`."] # [doc = "- One balance-transfer operation."] # [doc = "- Up to one account-lookup operation."] # [doc = "- Storage: 1 read `O(R)`, 1 mutate `O(R + X)`."] # [doc = "- One event."] # [doc = "# </weight>"] provide_judgement { # [codec (compact)] reg_index : :: core :: primitive :: u32 , target : :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , judgement : runtime_types :: pallet_identity :: types :: Judgement < :: core :: primitive :: u128 > , identity : :: subxt :: ext :: sp_core :: H256 , } , # [codec (index = 10)] # [doc = "Remove an account's identity and sub-account information and slash the deposits."] # [doc = ""] # [doc = "Payment: Reserved balances from `set_subs` and `set_identity` are slashed and handled by"] # [doc = "`Slash`. Verification request deposits are not returned; they should be cancelled"] # [doc = "manually using `cancel_request`."] # [doc = ""] # [doc = "The dispatch origin for this call must match `T::ForceOrigin`."] # [doc = ""] # [doc = "- `target`: the account whose identity the judgement is upon. This must be an account"] # [doc = "  with a registered identity."] # [doc = ""] # [doc = "Emits `IdentityKilled` if successful."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- `O(R + S + X)`."] # [doc = "- One balance-reserve operation."] # [doc = "- `S + 2` storage mutations."] # [doc = "- One event."] # [doc = "# </weight>"] kill_identity { target : :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 11)] # [doc = "Add the given account to the sender's subs."] # [doc = ""] # [doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"] # [doc = "to the sender."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"] # [doc = "sub identity of `sub`."] add_sub { sub : :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , data : runtime_types :: pallet_identity :: types :: Data , } , # [codec (index = 12)] # [doc = "Alter the associated name of the given sub-account."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"] # [doc = "sub identity of `sub`."] rename_sub { sub : :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , data : runtime_types :: pallet_identity :: types :: Data , } , # [codec (index = 13)] # [doc = "Remove the given account from the sender's subs."] # [doc = ""] # [doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"] # [doc = "to the sender."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"] # [doc = "sub identity of `sub`."] remove_sub { sub : :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 14)] # [doc = "Remove the sender as a sub-account."] # [doc = ""] # [doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"] # [doc = "to the sender (*not* the original depositor)."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"] # [doc = "super-identity."] # [doc = ""] # [doc = "NOTE: This should not normally be used, but is provided in the case that the non-"] # [doc = "controller of an account is maliciously registered as a sub-account."] quit_sub , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Too many subs-accounts."]
                    TooManySubAccounts,
                    #[codec(index = 1)]
                    #[doc = "Account isn't found."]
                    NotFound,
                    #[codec(index = 2)]
                    #[doc = "Account isn't named."]
                    NotNamed,
                    #[codec(index = 3)]
                    #[doc = "Empty index."]
                    EmptyIndex,
                    #[codec(index = 4)]
                    #[doc = "Fee is changed."]
                    FeeChanged,
                    #[codec(index = 5)]
                    #[doc = "No identity found."]
                    NoIdentity,
                    #[codec(index = 6)]
                    #[doc = "Sticky judgement."]
                    StickyJudgement,
                    #[codec(index = 7)]
                    #[doc = "Judgement given."]
                    JudgementGiven,
                    #[codec(index = 8)]
                    #[doc = "Invalid judgement."]
                    InvalidJudgement,
                    #[codec(index = 9)]
                    #[doc = "The index is invalid."]
                    InvalidIndex,
                    #[codec(index = 10)]
                    #[doc = "The target is invalid."]
                    InvalidTarget,
                    #[codec(index = 11)]
                    #[doc = "Too many additional fields."]
                    TooManyFields,
                    #[codec(index = 12)]
                    #[doc = "Maximum amount of registrars reached. Cannot add any more."]
                    TooManyRegistrars,
                    #[codec(index = 13)]
                    #[doc = "Account ID is already named."]
                    AlreadyClaimed,
                    #[codec(index = 14)]
                    #[doc = "Sender is not a sub-account."]
                    NotSub,
                    #[codec(index = 15)]
                    #[doc = "Sub-account isn't owned by sender."]
                    NotOwned,
                    #[codec(index = 16)]
                    #[doc = "The provided judgement was for a different identity."]
                    JudgementForDifferentIdentity,
                    #[codec(index = 17)]
                    #[doc = "Error that occurs when there is an issue paying for judgement."]
                    JudgementPaymentFailed,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A name was set or reset (which will remove all judgements)."]
                    IdentitySet {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "A name was cleared, and the given balance returned."]
                    IdentityCleared {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "A name was removed and the given balance slashed."]
                    IdentityKilled {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A judgement was asked from a registrar."]
                    JudgementRequested {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "A judgement request was retracted."]
                    JudgementUnrequested {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "A judgement was given by a registrar."]
                    JudgementGiven {
                        target: ::subxt::ext::sp_core::crypto::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "A registrar was added."]
                    RegistrarAdded {
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "A sub-identity was added to an identity and the deposit paid."]
                    SubIdentityAdded {
                        sub: ::subxt::ext::sp_core::crypto::AccountId32,
                        main: ::subxt::ext::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "A sub-identity was removed from an identity and the deposit freed."]
                    SubIdentityRemoved {
                        sub: ::subxt::ext::sp_core::crypto::AccountId32,
                        main: ::subxt::ext::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "A sub-identity was cleared, and the given deposit repatriated from the"]
                    #[doc = "main identity account to the sub-identity account."]
                    SubIdentityRevoked {
                        sub: ::subxt::ext::sp_core::crypto::AccountId32,
                        main: ::subxt::ext::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct BitFlags<_0>(
                    pub ::core::primitive::u64,
                    #[codec(skip)] pub ::core::marker::PhantomData<_0>,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub enum Data {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Raw0([::core::primitive::u8; 0usize]),
                    #[codec(index = 2)]
                    Raw1([::core::primitive::u8; 1usize]),
                    #[codec(index = 3)]
                    Raw2([::core::primitive::u8; 2usize]),
                    #[codec(index = 4)]
                    Raw3([::core::primitive::u8; 3usize]),
                    #[codec(index = 5)]
                    Raw4([::core::primitive::u8; 4usize]),
                    #[codec(index = 6)]
                    Raw5([::core::primitive::u8; 5usize]),
                    #[codec(index = 7)]
                    Raw6([::core::primitive::u8; 6usize]),
                    #[codec(index = 8)]
                    Raw7([::core::primitive::u8; 7usize]),
                    #[codec(index = 9)]
                    Raw8([::core::primitive::u8; 8usize]),
                    #[codec(index = 10)]
                    Raw9([::core::primitive::u8; 9usize]),
                    #[codec(index = 11)]
                    Raw10([::core::primitive::u8; 10usize]),
                    #[codec(index = 12)]
                    Raw11([::core::primitive::u8; 11usize]),
                    #[codec(index = 13)]
                    Raw12([::core::primitive::u8; 12usize]),
                    #[codec(index = 14)]
                    Raw13([::core::primitive::u8; 13usize]),
                    #[codec(index = 15)]
                    Raw14([::core::primitive::u8; 14usize]),
                    #[codec(index = 16)]
                    Raw15([::core::primitive::u8; 15usize]),
                    #[codec(index = 17)]
                    Raw16([::core::primitive::u8; 16usize]),
                    #[codec(index = 18)]
                    Raw17([::core::primitive::u8; 17usize]),
                    #[codec(index = 19)]
                    Raw18([::core::primitive::u8; 18usize]),
                    #[codec(index = 20)]
                    Raw19([::core::primitive::u8; 19usize]),
                    #[codec(index = 21)]
                    Raw20([::core::primitive::u8; 20usize]),
                    #[codec(index = 22)]
                    Raw21([::core::primitive::u8; 21usize]),
                    #[codec(index = 23)]
                    Raw22([::core::primitive::u8; 22usize]),
                    #[codec(index = 24)]
                    Raw23([::core::primitive::u8; 23usize]),
                    #[codec(index = 25)]
                    Raw24([::core::primitive::u8; 24usize]),
                    #[codec(index = 26)]
                    Raw25([::core::primitive::u8; 25usize]),
                    #[codec(index = 27)]
                    Raw26([::core::primitive::u8; 26usize]),
                    #[codec(index = 28)]
                    Raw27([::core::primitive::u8; 27usize]),
                    #[codec(index = 29)]
                    Raw28([::core::primitive::u8; 28usize]),
                    #[codec(index = 30)]
                    Raw29([::core::primitive::u8; 29usize]),
                    #[codec(index = 31)]
                    Raw30([::core::primitive::u8; 30usize]),
                    #[codec(index = 32)]
                    Raw31([::core::primitive::u8; 31usize]),
                    #[codec(index = 33)]
                    Raw32([::core::primitive::u8; 32usize]),
                    #[codec(index = 34)]
                    BlakeTwo256([::core::primitive::u8; 32usize]),
                    #[codec(index = 35)]
                    Sha256([::core::primitive::u8; 32usize]),
                    #[codec(index = 36)]
                    Keccak256([::core::primitive::u8; 32usize]),
                    #[codec(index = 37)]
                    ShaThree256([::core::primitive::u8; 32usize]),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub enum IdentityField {
                    #[codec(index = 1)]
                    Display,
                    #[codec(index = 2)]
                    Legal,
                    #[codec(index = 4)]
                    Web,
                    #[codec(index = 8)]
                    Riot,
                    #[codec(index = 16)]
                    Email,
                    #[codec(index = 32)]
                    PgpFingerprint,
                    #[codec(index = 64)]
                    Image,
                    #[codec(index = 128)]
                    Twitter,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct IdentityInfo { pub additional : runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < (runtime_types :: pallet_identity :: types :: Data , runtime_types :: pallet_identity :: types :: Data ,) > , pub display : runtime_types :: pallet_identity :: types :: Data , pub legal : runtime_types :: pallet_identity :: types :: Data , pub web : runtime_types :: pallet_identity :: types :: Data , pub riot : runtime_types :: pallet_identity :: types :: Data , pub email : runtime_types :: pallet_identity :: types :: Data , pub pgp_fingerprint : :: core :: option :: Option < [:: core :: primitive :: u8 ; 20usize] > , pub image : runtime_types :: pallet_identity :: types :: Data , pub twitter : runtime_types :: pallet_identity :: types :: Data , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub enum Judgement<_0> {
                    #[codec(index = 0)]
                    Unknown,
                    #[codec(index = 1)]
                    FeePaid(_0),
                    #[codec(index = 2)]
                    Reasonable,
                    #[codec(index = 3)]
                    KnownGood,
                    #[codec(index = 4)]
                    OutOfDate,
                    #[codec(index = 5)]
                    LowQuality,
                    #[codec(index = 6)]
                    Erroneous,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct RegistrarInfo<_0, _1> {
                    pub account: _1,
                    pub fee: _0,
                    pub fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Registration < _0 > { pub judgements : runtime_types :: sp_core :: bounded :: bounded_vec :: BoundedVec < (:: core :: primitive :: u32 , runtime_types :: pallet_identity :: types :: Judgement < _0 > ,) > , pub deposit : _0 , pub info : runtime_types :: pallet_identity :: types :: IdentityInfo , }
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "# <weight>"] # [doc = "- Complexity: `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is"] # [doc = "  length of `heartbeat.network_state.external_address`"] # [doc = "  - `O(K)`: decoding of length `K`"] # [doc = "  - `O(E)`: decoding/encoding of length `E`"] # [doc = "- DbReads: pallet_session `Validators`, pallet_session `CurrentIndex`, `Keys`,"] # [doc = "  `ReceivedHeartbeats`"] # [doc = "- DbWrites: `ReceivedHeartbeats`"] # [doc = "# </weight>"] heartbeat { heartbeat : runtime_types :: pallet_im_online :: Heartbeat < :: core :: primitive :: u32 > , signature : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Signature , } , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Non existent public key."]
                    InvalidKey,
                    #[codec(index = 1)]
                    #[doc = "Duplicated heartbeat."]
                    DuplicatedHeartbeat,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    # [codec (index = 0)] # [doc = "A new heartbeat was received from `AuthorityId`."] HeartbeatReceived { authority_id : runtime_types :: pallet_im_online :: sr25519 :: app_sr25519 :: Public , } , # [codec (index = 1)] # [doc = "At the end of the session, no offence was committed."] AllGood , # [codec (index = 2)] # [doc = "At the end of the session, at least one validator was found to be offline."] SomeOffline { offline : :: std :: vec :: Vec < (:: subxt :: ext :: sp_core :: crypto :: AccountId32 , runtime_types :: pallet_staking :: Exposure < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 > ,) > , } , }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct Public(
                        pub runtime_types::sp_core::sr25519::Public,
                    );
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct Signature(
                        pub runtime_types::sp_core::sr25519::Signature,
                    );
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct BoundedOpaqueNetworkState { pub peer_id : runtime_types :: sp_core :: bounded :: weak_bounded_vec :: WeakBoundedVec < :: core :: primitive :: u8 > , pub external_addresses : runtime_types :: sp_core :: bounded :: weak_bounded_vec :: WeakBoundedVec < runtime_types :: sp_core :: bounded :: weak_bounded_vec :: WeakBoundedVec < :: core :: primitive :: u8 > > , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state:
                    runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
        }
        pub mod pallet_indices {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Assign an previously unassigned index."]
                    #[doc = ""]
                    #[doc = "Payment: `Deposit` is reserved from the sender account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `index`: the index to be claimed. This must not be in use."]
                    #[doc = ""]
                    #[doc = "Emits `IndexAssigned` if successful."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`."]
                    #[doc = "- One storage mutation (codec `O(1)`)."]
                    #[doc = "- One reserve operation."]
                    #[doc = "- One event."]
                    #[doc = "-------------------"]
                    #[doc = "- DB Weight: 1 Read/Write (Accounts)"]
                    #[doc = "# </weight>"]
                    claim { index: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "Assign an index already owned by the sender to another account. The balance reservation"]
                    #[doc = "is effectively transferred to the new account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `index`: the index to be re-assigned. This must be owned by the sender."]
                    #[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
                    #[doc = ""]
                    #[doc = "Emits `IndexAssigned` if successful."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`."]
                    #[doc = "- One storage mutation (codec `O(1)`)."]
                    #[doc = "- One transfer operation."]
                    #[doc = "- One event."]
                    #[doc = "-------------------"]
                    #[doc = "- DB Weight:"]
                    #[doc = "   - Reads: Indices Accounts, System Account (recipient)"]
                    #[doc = "   - Writes: Indices Accounts, System Account (recipient)"]
                    #[doc = "# </weight>"]
                    transfer {
                        new: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Free up an index owned by the sender."]
                    #[doc = ""]
                    #[doc = "Payment: Any previous deposit placed for the index is unreserved in the sender account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must own the index."]
                    #[doc = ""]
                    #[doc = "- `index`: the index to be freed. This must be owned by the sender."]
                    #[doc = ""]
                    #[doc = "Emits `IndexFreed` if successful."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`."]
                    #[doc = "- One storage mutation (codec `O(1)`)."]
                    #[doc = "- One reserve operation."]
                    #[doc = "- One event."]
                    #[doc = "-------------------"]
                    #[doc = "- DB Weight: 1 Read/Write (Accounts)"]
                    #[doc = "# </weight>"]
                    free { index: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    #[doc = "Force an index to an account. This doesn't require a deposit. If the index is already"]
                    #[doc = "held, then any deposit is reimbursed to its current owner."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    #[doc = ""]
                    #[doc = "- `index`: the index to be (re-)assigned."]
                    #[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
                    #[doc = "- `freeze`: if set to `true`, will freeze the index so it cannot be transferred."]
                    #[doc = ""]
                    #[doc = "Emits `IndexAssigned` if successful."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`."]
                    #[doc = "- One storage mutation (codec `O(1)`)."]
                    #[doc = "- Up to one reserve operation."]
                    #[doc = "- One event."]
                    #[doc = "-------------------"]
                    #[doc = "- DB Weight:"]
                    #[doc = "   - Reads: Indices Accounts, System Account (original owner)"]
                    #[doc = "   - Writes: Indices Accounts, System Account (original owner)"]
                    #[doc = "# </weight>"]
                    force_transfer {
                        new: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        index: ::core::primitive::u32,
                        freeze: ::core::primitive::bool,
                    },
                    #[codec(index = 4)]
                    #[doc = "Freeze an index so it will always point to the sender account. This consumes the"]
                    #[doc = "deposit."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the signing account must have a"]
                    #[doc = "non-frozen account `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index to be frozen in place."]
                    #[doc = ""]
                    #[doc = "Emits `IndexFrozen` if successful."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`."]
                    #[doc = "- One storage mutation (codec `O(1)`)."]
                    #[doc = "- Up to one slash operation."]
                    #[doc = "- One event."]
                    #[doc = "-------------------"]
                    #[doc = "- DB Weight: 1 Read/Write (Accounts)"]
                    #[doc = "# </weight>"]
                    freeze { index: ::core::primitive::u32 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The index was not already assigned."]
                    NotAssigned,
                    #[codec(index = 1)]
                    #[doc = "The index is assigned to another account."]
                    NotOwner,
                    #[codec(index = 2)]
                    #[doc = "The index was not available."]
                    InUse,
                    #[codec(index = 3)]
                    #[doc = "The source and destination accounts are identical."]
                    NotTransfer,
                    #[codec(index = 4)]
                    #[doc = "The index is permanent and may not be freed/changed."]
                    Permanent,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A account index was assigned."]
                    IndexAssigned {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "A account index has been freed up (unassigned)."]
                    IndexFreed { index: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    #[doc = "A account index has been frozen to its current account ID."]
                    IndexFrozen {
                        index: ::core::primitive::u32,
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                }
            }
        }
        pub mod pallet_nomination_pools {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Stake funds with a pool. The amount to bond is transferred from the member to the"]
                    #[doc = "pools account and immediately increases the pools bond."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "* An account can only be a member of a single pool."]
                    #[doc = "* An account cannot join the same pool multiple times."]
                    #[doc = "* This call will *not* dust the member account, so the member must have at least"]
                    #[doc = "  `existential deposit + amount` in their account."]
                    #[doc = "* Only a pool with [`PoolState::Open`] can be joined"]
                    join {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        pool_id: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Bond `extra` more funds from `origin` into the pool to which they already belong."]
                    #[doc = ""]
                    #[doc = "Additional funds can come from either the free balance of the account, of from the"]
                    #[doc = "accumulated rewards, see [`BondExtra`]."]
                    #[doc = ""]
                    #[doc = "Bonding extra funds implies an automatic payout of all pending rewards as well."]
                    bond_extra {
                        extra:
                            runtime_types::pallet_nomination_pools::BondExtra<
                                ::core::primitive::u128,
                            >,
                    },
                    #[codec(index = 2)]
                    #[doc = "A bonded member can use this to claim their payout based on the rewards that the pool"]
                    #[doc = "has accumulated since their last claimed payout (OR since joining if this is there first"]
                    #[doc = "time claiming rewards). The payout will be transferred to the member's account."]
                    #[doc = ""]
                    #[doc = "The member will earn rewards pro rata based on the members stake vs the sum of the"]
                    #[doc = "members in the pools stake. Rewards do not \"expire\"."]
                    claim_payout,
                    #[codec(index = 3)]
                    #[doc = "Unbond up to `unbonding_points` of the `member_account`'s funds from the pool. It"]
                    #[doc = "implicitly collects the rewards one last time, since not doing so would mean some"]
                    #[doc = "rewards would be forfeited."]
                    #[doc = ""]
                    #[doc = "Under certain conditions, this call can be dispatched permissionlessly (i.e. by any"]
                    #[doc = "account)."]
                    #[doc = ""]
                    #[doc = "# Conditions for a permissionless dispatch."]
                    #[doc = ""]
                    #[doc = "* The pool is blocked and the caller is either the root or state-toggler. This is"]
                    #[doc = "  refereed to as a kick."]
                    #[doc = "* The pool is destroying and the member is not the depositor."]
                    #[doc = "* The pool is destroying, the member is the depositor and no other members are in the"]
                    #[doc = "  pool."]
                    #[doc = ""]
                    #[doc = "## Conditions for permissioned dispatch (i.e. the caller is also the"]
                    #[doc = "`member_account`):"]
                    #[doc = ""]
                    #[doc = "* The caller is not the depositor."]
                    #[doc = "* The caller is the depositor, the pool is destroying and no other members are in the"]
                    #[doc = "  pool."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "If there are too many unlocking chunks to unbond with the pool account,"]
                    #[doc = "[`Call::pool_withdraw_unbonded`] can be called to try and minimize unlocking chunks."]
                    #[doc = "The [`StakingInterface::unbond`] will implicitly call [`Call::pool_withdraw_unbonded`]"]
                    #[doc = "to try to free chunks if necessary (ie. if unbound was called and no unlocking chunks"]
                    #[doc = "are available). However, it may not be possible to release the current unlocking chunks,"]
                    #[doc = "in which case, the result of this call will likely be the `NoMoreChunks` error from the"]
                    #[doc = "staking system."]
                    unbond {
                        member_account: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        unbonding_points: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Call `withdraw_unbonded` for the pools account. This call can be made by any account."]
                    #[doc = ""]
                    #[doc = "This is useful if their are too many unlocking chunks to call `unbond`, and some"]
                    #[doc = "can be cleared by withdrawing. In the case there are too many unlocking chunks, the user"]
                    #[doc = "would probably see an error like `NoMoreChunks` emitted from the staking system when"]
                    #[doc = "they attempt to unbond."]
                    pool_withdraw_unbonded {
                        pool_id: ::core::primitive::u32,
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Withdraw unbonded funds from `member_account`. If no bonded funds can be unbonded, an"]
                    #[doc = "error is returned."]
                    #[doc = ""]
                    #[doc = "Under certain conditions, this call can be dispatched permissionlessly (i.e. by any"]
                    #[doc = "account)."]
                    #[doc = ""]
                    #[doc = "# Conditions for a permissionless dispatch"]
                    #[doc = ""]
                    #[doc = "* The pool is in destroy mode and the target is not the depositor."]
                    #[doc = "* The target is the depositor and they are the only member in the sub pools."]
                    #[doc = "* The pool is blocked and the caller is either the root or state-toggler."]
                    #[doc = ""]
                    #[doc = "# Conditions for permissioned dispatch"]
                    #[doc = ""]
                    #[doc = "* The caller is the target and they are not the depositor."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "If the target is the depositor, the pool will be destroyed."]
                    withdraw_unbonded {
                        member_account: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Create a new delegation pool."]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = "* `amount` - The amount of funds to delegate to the pool. This also acts of a sort of"]
                    #[doc = "  deposit since the pools creator cannot fully unbond funds until the pool is being"]
                    #[doc = "  destroyed."]
                    #[doc = "* `index` - A disambiguation index for creating the account. Likely only useful when"]
                    #[doc = "  creating multiple pools in the same extrinsic."]
                    #[doc = "* `root` - The account to set as [`PoolRoles::root`]."]
                    #[doc = "* `nominator` - The account to set as the [`PoolRoles::nominator`]."]
                    #[doc = "* `state_toggler` - The account to set as the [`PoolRoles::state_toggler`]."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "In addition to `amount`, the caller will transfer the existential deposit; so the caller"]
                    #[doc = "needs at have at least `amount + existential_deposit` transferrable."]
                    create {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        root: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        nominator: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        state_toggler: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 7)]
                    #[doc = "Create a new delegation pool with a previously used pool id"]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = "same as `create` with the inclusion of"]
                    #[doc = "* `pool_id` - `A valid PoolId."]
                    create_with_pool_id {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        root: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        nominator: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        state_toggler: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        pool_id: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Nominate on behalf of the pool."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed by the pool nominator or the pool"]
                    #[doc = "root role."]
                    #[doc = ""]
                    #[doc = "This directly forward the call to the staking pallet, on behalf of the pool bonded"]
                    #[doc = "account."]
                    nominate {
                        pool_id: ::core::primitive::u32,
                        validators: ::std::vec::Vec<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    },
                    #[codec(index = 9)]
                    #[doc = "Set a new state for the pool."]
                    #[doc = ""]
                    #[doc = "If a pool is already in the `Destroying` state, then under no condition can its state"]
                    #[doc = "change again."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be either:"]
                    #[doc = ""]
                    #[doc = "1. signed by the state toggler, or the root role of the pool,"]
                    #[doc = "2. if the pool conditions to be open are NOT met (as described by `ok_to_be_open`), and"]
                    #[doc = "   then the state of the pool can be permissionlessly changed to `Destroying`."]
                    set_state {
                        pool_id: ::core::primitive::u32,
                        state:
                            runtime_types::pallet_nomination_pools::PoolState,
                    },
                    #[codec(index = 10)]
                    #[doc = "Set a new metadata for the pool."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed by the state toggler, or the root role"]
                    #[doc = "of the pool."]
                    set_metadata {
                        pool_id: ::core::primitive::u32,
                        metadata: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 11)]
                    #[doc = "Update configurations for the nomination pools. The origin for this call must be"]
                    #[doc = "Root."]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = "* `min_join_bond` - Set [`MinJoinBond`]."]
                    #[doc = "* `min_create_bond` - Set [`MinCreateBond`]."]
                    #[doc = "* `max_pools` - Set [`MaxPools`]."]
                    #[doc = "* `max_members` - Set [`MaxPoolMembers`]."]
                    #[doc = "* `max_members_per_pool` - Set [`MaxPoolMembersPerPool`]."]
                    set_configs {
                        min_join_bond:
                            runtime_types::pallet_nomination_pools::ConfigOp<
                                ::core::primitive::u128,
                            >,
                        min_create_bond:
                            runtime_types::pallet_nomination_pools::ConfigOp<
                                ::core::primitive::u128,
                            >,
                        max_pools:
                            runtime_types::pallet_nomination_pools::ConfigOp<
                                ::core::primitive::u32,
                            >,
                        max_members:
                            runtime_types::pallet_nomination_pools::ConfigOp<
                                ::core::primitive::u32,
                            >,
                        max_members_per_pool:
                            runtime_types::pallet_nomination_pools::ConfigOp<
                                ::core::primitive::u32,
                            >,
                    },
                    #[codec(index = 12)]
                    #[doc = "Update the roles of the pool."]
                    #[doc = ""]
                    #[doc = "The root is the only entity that can change any of the roles, including itself,"]
                    #[doc = "excluding the depositor, who can never change."]
                    #[doc = ""]
                    #[doc = "It emits an event, notifying UIs of the role change. This event is quite relevant to"]
                    #[doc = "most pool members and they should be informed of changes to pool roles."]
                    update_roles {
                        pool_id: ::core::primitive::u32,
                        new_root:
                            runtime_types::pallet_nomination_pools::ConfigOp<
                                ::subxt::ext::sp_core::crypto::AccountId32,
                            >,
                        new_nominator:
                            runtime_types::pallet_nomination_pools::ConfigOp<
                                ::subxt::ext::sp_core::crypto::AccountId32,
                            >,
                        new_state_toggler:
                            runtime_types::pallet_nomination_pools::ConfigOp<
                                ::subxt::ext::sp_core::crypto::AccountId32,
                            >,
                    },
                    #[codec(index = 13)]
                    #[doc = "Chill on behalf of the pool."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be signed by the pool nominator or the pool"]
                    #[doc = "root role, same as [`Pallet::nominate`]."]
                    #[doc = ""]
                    #[doc = "This directly forward the call to the staking pallet, on behalf of the pool bonded"]
                    #[doc = "account."]
                    chill { pool_id: ::core::primitive::u32 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub enum DefensiveError {
                    #[codec(index = 0)]
                    NotEnoughSpaceInUnbondPool,
                    #[codec(index = 1)]
                    PoolNotFound,
                    #[codec(index = 2)]
                    RewardPoolNotFound,
                    #[codec(index = 3)]
                    SubPoolsNotFound,
                    #[codec(index = 4)]
                    BondedStashKilledPrematurely,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    # [codec (index = 0)] # [doc = "A (bonded) pool id does not exist."] PoolNotFound , # [codec (index = 1)] # [doc = "An account is not a member."] PoolMemberNotFound , # [codec (index = 2)] # [doc = "A reward pool does not exist. In all cases this is a system logic error."] RewardPoolNotFound , # [codec (index = 3)] # [doc = "A sub pool does not exist."] SubPoolsNotFound , # [codec (index = 4)] # [doc = "An account is already delegating in another pool. An account may only belong to one"] # [doc = "pool at a time."] AccountBelongsToOtherPool , # [codec (index = 5)] # [doc = "The member is fully unbonded (and thus cannot access the bonded and reward pool"] # [doc = "anymore to, for example, collect rewards)."] FullyUnbonding , # [codec (index = 6)] # [doc = "The member cannot unbond further chunks due to reaching the limit."] MaxUnbondingLimit , # [codec (index = 7)] # [doc = "None of the funds can be withdrawn yet because the bonding duration has not passed."] CannotWithdrawAny , # [codec (index = 8)] # [doc = "The amount does not meet the minimum bond to either join or create a pool."] # [doc = ""] # [doc = "The depositor can never unbond to a value less than"] # [doc = "`Pallet::depositor_min_bond`. The caller does not have nominating"] # [doc = "permissions for the pool. Members can never unbond to a value below `MinJoinBond`."] MinimumBondNotMet , # [codec (index = 9)] # [doc = "The transaction could not be executed due to overflow risk for the pool."] OverflowRisk , # [codec (index = 10)] # [doc = "A pool must be in [`PoolState::Destroying`] in order for the depositor to unbond or for"] # [doc = "other members to be permissionlessly unbonded."] NotDestroying , # [codec (index = 11)] # [doc = "The caller does not have nominating permissions for the pool."] NotNominator , # [codec (index = 12)] # [doc = "Either a) the caller cannot make a valid kick or b) the pool is not destroying."] NotKickerOrDestroying , # [codec (index = 13)] # [doc = "The pool is not open to join"] NotOpen , # [codec (index = 14)] # [doc = "The system is maxed out on pools."] MaxPools , # [codec (index = 15)] # [doc = "Too many members in the pool or system."] MaxPoolMembers , # [codec (index = 16)] # [doc = "The pools state cannot be changed."] CanNotChangeState , # [codec (index = 17)] # [doc = "The caller does not have adequate permissions."] DoesNotHavePermission , # [codec (index = 18)] # [doc = "Metadata exceeds [`Config::MaxMetadataLen`]"] MetadataExceedsMaxLen , # [codec (index = 19)] # [doc = "Some error occurred that should never happen. This should be reported to the"] # [doc = "maintainers."] Defensive (runtime_types :: pallet_nomination_pools :: pallet :: DefensiveError ,) , # [codec (index = 20)] # [doc = "Partial unbonding now allowed permissionlessly."] PartialUnbondNotAllowedPermissionlessly , # [codec (index = 21)] # [doc = "Pool id currently in use."] PoolIdInUse , # [codec (index = 22)] # [doc = "Pool id provided is not correct/usable."] InvalidPoolId , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Events of this pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A pool has been created."]
                    Created {
                        depositor: ::subxt::ext::sp_core::crypto::AccountId32,
                        pool_id: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "A member has became bonded in a pool."]
                    Bonded {
                        member: ::subxt::ext::sp_core::crypto::AccountId32,
                        pool_id: ::core::primitive::u32,
                        bonded: ::core::primitive::u128,
                        joined: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    #[doc = "A payout has been made to a member."]
                    PaidOut {
                        member: ::subxt::ext::sp_core::crypto::AccountId32,
                        pool_id: ::core::primitive::u32,
                        payout: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A member has unbonded from their pool."]
                    #[doc = ""]
                    #[doc = "- `balance` is the corresponding balance of the number of points that has been"]
                    #[doc = "  requested to be unbonded (the argument of the `unbond` transaction) from the bonded"]
                    #[doc = "  pool."]
                    #[doc = "- `points` is the number of points that are issued as a result of `balance` being"]
                    #[doc = "dissolved into the corresponding unbonding pool."]
                    #[doc = "- `era` is the era in which the balance will be unbonded."]
                    #[doc = "In the absence of slashing, these values will match. In the presence of slashing, the"]
                    #[doc = "number of points that are issued in the unbonding pool will be less than the amount"]
                    #[doc = "requested to be unbonded."]
                    Unbonded {
                        member: ::subxt::ext::sp_core::crypto::AccountId32,
                        pool_id: ::core::primitive::u32,
                        balance: ::core::primitive::u128,
                        points: ::core::primitive::u128,
                        era: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "A member has withdrawn from their pool."]
                    #[doc = ""]
                    #[doc = "The given number of `points` have been dissolved in return of `balance`."]
                    #[doc = ""]
                    #[doc = "Similar to `Unbonded` event, in the absence of slashing, the ratio of point to balance"]
                    #[doc = "will be 1."]
                    Withdrawn {
                        member: ::subxt::ext::sp_core::crypto::AccountId32,
                        pool_id: ::core::primitive::u32,
                        balance: ::core::primitive::u128,
                        points: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "A pool has been destroyed."]
                    Destroyed { pool_id: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    #[doc = "The state of a pool has changed"]
                    StateChanged {
                        pool_id: ::core::primitive::u32,
                        new_state:
                            runtime_types::pallet_nomination_pools::PoolState,
                    },
                    #[codec(index = 7)]
                    #[doc = "A member has been removed from a pool."]
                    #[doc = ""]
                    #[doc = "The removal can be voluntary (withdrawn all unbonded funds) or involuntary (kicked)."]
                    MemberRemoved {
                        pool_id: ::core::primitive::u32,
                        member: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 8)]
                    #[doc = "The roles of a pool have been updated to the given new roles. Note that the depositor"]
                    #[doc = "can never change."]
                    RolesUpdated {
                        root: ::core::option::Option<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                        state_toggler: ::core::option::Option<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                        nominator: ::core::option::Option<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    },
                    #[codec(index = 9)]
                    #[doc = "The active balance of pool `pool_id` has been slashed to `balance`."]
                    PoolSlashed {
                        pool_id: ::core::primitive::u32,
                        balance: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "The unbond pool at `era` of pool `pool_id` has been slashed to `balance`."]
                    UnbondingPoolSlashed {
                        pool_id: ::core::primitive::u32,
                        era: ::core::primitive::u32,
                        balance: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub enum BondExtra<_0> {
                #[codec(index = 0)]
                FreeBalance(_0),
                #[codec(index = 1)]
                Rewards,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct BondedPoolInner {
                pub points: ::core::primitive::u128,
                pub state: runtime_types::pallet_nomination_pools::PoolState,
                pub member_counter: ::core::primitive::u32,
                pub roles: runtime_types::pallet_nomination_pools::PoolRoles<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub enum ConfigOp<_0> {
                #[codec(index = 0)]
                Noop,
                #[codec(index = 1)]
                Set(_0),
                #[codec(index = 2)]
                Remove,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct PoolMember { pub pool_id : :: core :: primitive :: u32 , pub points : :: core :: primitive :: u128 , pub last_recorded_reward_counter : runtime_types :: sp_arithmetic :: fixed_point :: FixedU128 , pub unbonding_eras : runtime_types :: sp_core :: bounded :: bounded_btree_map :: BoundedBTreeMap < :: core :: primitive :: u32 , :: core :: primitive :: u128 > , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct PoolRoles<_0> {
                pub depositor: _0,
                pub root: ::core::option::Option<_0>,
                pub nominator: ::core::option::Option<_0>,
                pub state_toggler: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub enum PoolState {
                #[codec(index = 0)]
                Open,
                #[codec(index = 1)]
                Blocked,
                #[codec(index = 2)]
                Destroying,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RewardPool {
                pub last_recorded_reward_counter:
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                pub last_recorded_total_payouts: ::core::primitive::u128,
                pub total_rewards_claimed: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct SubPools { pub no_era : runtime_types :: pallet_nomination_pools :: UnbondPool , pub with_era : runtime_types :: sp_core :: bounded :: bounded_btree_map :: BoundedBTreeMap < :: core :: primitive :: u32 , runtime_types :: pallet_nomination_pools :: UnbondPool > , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct UnbondPool {
                pub points: ::core::primitive::u128,
                pub balance: ::core::primitive::u128,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Sets the session key(s) of the function caller to `keys`."] # [doc = "Allows an account to set its session key prior to becoming a validator."] # [doc = "This doesn't take effect until the next session."] # [doc = ""] # [doc = "The dispatch origin of this function must be signed."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"] # [doc = "  `T::Keys::key_ids()` which is fixed."] # [doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"] # [doc = "- DbWrites: `origin account`, `NextKeys`"] # [doc = "- DbReads per key id: `KeyOwner`"] # [doc = "- DbWrites per key id: `KeyOwner`"] # [doc = "# </weight>"] set_keys { keys : runtime_types :: dkg_standalone_runtime :: opaque :: SessionKeys , proof : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 1)] # [doc = "Removes any session key(s) of the function caller."] # [doc = ""] # [doc = "This doesn't take effect until the next session."] # [doc = ""] # [doc = "The dispatch origin of this function must be Signed and the account must be either be"] # [doc = "convertible to a validator ID using the chain's typical addressing system (this usually"] # [doc = "means being a controller account) or directly convertible into a validator ID (which"] # [doc = "usually means being a stash account)."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"] # [doc = "  of `T::Keys::key_ids()` which is fixed."] # [doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"] # [doc = "- DbWrites: `NextKeys`, `origin account`"] # [doc = "- DbWrites per key id: `KeyOwner`"] # [doc = "# </weight>"] purge_keys , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Error for the session pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Invalid ownership proof."]
                    InvalidProof,
                    #[codec(index = 1)]
                    #[doc = "No associated validator ID for account."]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    #[doc = "Registered duplicate key."]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    #[doc = "No keys are associated with this account."]
                    NoKeys,
                    #[codec(index = 4)]
                    #[doc = "Key setting account is not live, so it's impossible to associate keys."]
                    NoAccount,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New session has happened. Note that the argument is the session index, not the"]
                    #[doc = "block number as the type might suggest."]
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
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                    pub enum Call {
                        # [codec (index = 0)] # [doc = "Take the origin account as a stash and lock up `value` of its balance. `controller` will"] # [doc = "be the account that controls it."] # [doc = ""] # [doc = "`value` must be more than the `minimum_balance` specified by `T::Currency`."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ by the stash account."] # [doc = ""] # [doc = "Emits `Bonded`."] # [doc = "# <weight>"] # [doc = "- Independent of the arguments. Moderate complexity."] # [doc = "- O(1)."] # [doc = "- Three extra DB entries."] # [doc = ""] # [doc = "NOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned"] # [doc = "unless the `origin` falls below _existential deposit_ and gets removed as dust."] # [doc = "------------------"] # [doc = "# </weight>"] bond { controller : :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , # [codec (compact)] value : :: core :: primitive :: u128 , payee : runtime_types :: pallet_staking :: RewardDestination < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 1)] # [doc = "Add some extra amount that have appeared in the stash `free_balance` into the balance up"] # [doc = "for staking."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ by the stash, not the controller."] # [doc = ""] # [doc = "Use this if there are additional funds in your stash account that you wish to bond."] # [doc = "Unlike [`bond`](Self::bond) or [`unbond`](Self::unbond) this function does not impose"] # [doc = "any limitation on the amount that can be added."] # [doc = ""] # [doc = "Emits `Bonded`."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- Independent of the arguments. Insignificant complexity."] # [doc = "- O(1)."] # [doc = "# </weight>"] bond_extra { # [codec (compact)] max_additional : :: core :: primitive :: u128 , } , # [codec (index = 2)] # [doc = "Schedule a portion of the stash to be unlocked ready for transfer out after the bond"] # [doc = "period ends. If this leaves an amount actively bonded less than"] # [doc = "T::Currency::minimum_balance(), then it is increased to the full amount."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."] # [doc = ""] # [doc = "Once the unlock period is done, you can call `withdraw_unbonded` to actually move"] # [doc = "the funds out of management ready for transfer."] # [doc = ""] # [doc = "No more than a limited number of unlocking chunks (see `MaxUnlockingChunks`)"] # [doc = "can co-exists at the same time. If there are no unlocking chunks slots available"] # [doc = "[`Call::withdraw_unbonded`] is called to remove some of the chunks (if possible)."] # [doc = ""] # [doc = "If a user encounters the `InsufficientBond` error when calling this extrinsic,"] # [doc = "they should call `chill` first in order to free up their bonded funds."] # [doc = ""] # [doc = "Emits `Unbonded`."] # [doc = ""] # [doc = "See also [`Call::withdraw_unbonded`]."] unbond { # [codec (compact)] value : :: core :: primitive :: u128 , } , # [codec (index = 3)] # [doc = "Remove any unlocked chunks from the `unlocking` queue from our management."] # [doc = ""] # [doc = "This essentially frees up that balance to be used by the stash account to do"] # [doc = "whatever it wants."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ by the controller."] # [doc = ""] # [doc = "Emits `Withdrawn`."] # [doc = ""] # [doc = "See also [`Call::unbond`]."] # [doc = ""] # [doc = "# <weight>"] # [doc = "Complexity O(S) where S is the number of slashing spans to remove"] # [doc = "NOTE: Weight annotation is the kill scenario, we refund otherwise."] # [doc = "# </weight>"] withdraw_unbonded { num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 4)] # [doc = "Declare the desire to validate for the origin controller."] # [doc = ""] # [doc = "Effects will be felt at the beginning of the next era."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."] validate { prefs : runtime_types :: pallet_staking :: ValidatorPrefs , } , # [codec (index = 5)] # [doc = "Declare the desire to nominate `targets` for the origin controller."] # [doc = ""] # [doc = "Effects will be felt at the beginning of the next era."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- The transaction's complexity is proportional to the size of `targets` (N)"] # [doc = "which is capped at CompactAssignments::LIMIT (T::MaxNominations)."] # [doc = "- Both the reads and writes follow a similar pattern."] # [doc = "# </weight>"] nominate { targets : :: std :: vec :: Vec < :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , } , # [codec (index = 6)] # [doc = "Declare no desire to either validate or nominate."] # [doc = ""] # [doc = "Effects will be felt at the beginning of the next era."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- Independent of the arguments. Insignificant complexity."] # [doc = "- Contains one read."] # [doc = "- Writes are limited to the `origin` account key."] # [doc = "# </weight>"] chill , # [codec (index = 7)] # [doc = "(Re-)set the payment target for a controller."] # [doc = ""] # [doc = "Effects will be felt instantly (as soon as this function is completed successfully)."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- Independent of the arguments. Insignificant complexity."] # [doc = "- Contains a limited number of reads."] # [doc = "- Writes are limited to the `origin` account key."] # [doc = "---------"] # [doc = "- Weight: O(1)"] # [doc = "- DB Weight:"] # [doc = "    - Read: Ledger"] # [doc = "    - Write: Payee"] # [doc = "# </weight>"] set_payee { payee : runtime_types :: pallet_staking :: RewardDestination < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 8)] # [doc = "(Re-)set the controller of a stash."] # [doc = ""] # [doc = "Effects will be felt instantly (as soon as this function is completed successfully)."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ by the stash, not the controller."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- Independent of the arguments. Insignificant complexity."] # [doc = "- Contains a limited number of reads."] # [doc = "- Writes are limited to the `origin` account key."] # [doc = "----------"] # [doc = "Weight: O(1)"] # [doc = "DB Weight:"] # [doc = "- Read: Bonded, Ledger New Controller, Ledger Old Controller"] # [doc = "- Write: Bonded, Ledger New Controller, Ledger Old Controller"] # [doc = "# </weight>"] set_controller { controller : :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > , } , # [codec (index = 9)] # [doc = "Sets the ideal number of validators."] # [doc = ""] # [doc = "The dispatch origin must be Root."] # [doc = ""] # [doc = "# <weight>"] # [doc = "Weight: O(1)"] # [doc = "Write: Validator Count"] # [doc = "# </weight>"] set_validator_count { # [codec (compact)] new : :: core :: primitive :: u32 , } , # [codec (index = 10)] # [doc = "Increments the ideal number of validators upto maximum of"] # [doc = "`ElectionProviderBase::MaxWinners`."] # [doc = ""] # [doc = "The dispatch origin must be Root."] # [doc = ""] # [doc = "# <weight>"] # [doc = "Same as [`Self::set_validator_count`]."] # [doc = "# </weight>"] increase_validator_count { # [codec (compact)] additional : :: core :: primitive :: u32 , } , # [codec (index = 11)] # [doc = "Scale up the ideal number of validators by a factor upto maximum of"] # [doc = "`ElectionProviderBase::MaxWinners`."] # [doc = ""] # [doc = "The dispatch origin must be Root."] # [doc = ""] # [doc = "# <weight>"] # [doc = "Same as [`Self::set_validator_count`]."] # [doc = "# </weight>"] scale_validator_count { factor : runtime_types :: sp_arithmetic :: per_things :: Percent , } , # [codec (index = 12)] # [doc = "Force there to be no new eras indefinitely."] # [doc = ""] # [doc = "The dispatch origin must be Root."] # [doc = ""] # [doc = "# Warning"] # [doc = ""] # [doc = "The election process starts multiple blocks before the end of the era."] # [doc = "Thus the election process may be ongoing when this is called. In this case the"] # [doc = "election will continue until the next era is triggered."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- No arguments."] # [doc = "- Weight: O(1)"] # [doc = "- Write: ForceEra"] # [doc = "# </weight>"] force_no_eras , # [codec (index = 13)] # [doc = "Force there to be a new era at the end of the next session. After this, it will be"] # [doc = "reset to normal (non-forced) behaviour."] # [doc = ""] # [doc = "The dispatch origin must be Root."] # [doc = ""] # [doc = "# Warning"] # [doc = ""] # [doc = "The election process starts multiple blocks before the end of the era."] # [doc = "If this is called just before a new era is triggered, the election process may not"] # [doc = "have enough blocks to get a result."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- No arguments."] # [doc = "- Weight: O(1)"] # [doc = "- Write ForceEra"] # [doc = "# </weight>"] force_new_era , # [codec (index = 14)] # [doc = "Set the validators who cannot be slashed (if any)."] # [doc = ""] # [doc = "The dispatch origin must be Root."] set_invulnerables { invulnerables : :: std :: vec :: Vec < :: subxt :: ext :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 15)] # [doc = "Force a current staker to become completely unstaked, immediately."] # [doc = ""] # [doc = "The dispatch origin must be Root."] force_unstake { stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 16)] # [doc = "Force there to be a new era at the end of sessions indefinitely."] # [doc = ""] # [doc = "The dispatch origin must be Root."] # [doc = ""] # [doc = "# Warning"] # [doc = ""] # [doc = "The election process starts multiple blocks before the end of the era."] # [doc = "If this is called just before a new era is triggered, the election process may not"] # [doc = "have enough blocks to get a result."] force_new_era_always , # [codec (index = 17)] # [doc = "Cancel enactment of a deferred slash."] # [doc = ""] # [doc = "Can be called by the `T::SlashCancelOrigin`."] # [doc = ""] # [doc = "Parameters: era and indices of the slashes for that era to kill."] cancel_deferred_slash { era : :: core :: primitive :: u32 , slash_indices : :: std :: vec :: Vec < :: core :: primitive :: u32 > , } , # [codec (index = 18)] # [doc = "Pay out all the stakers behind a single validator for a single era."] # [doc = ""] # [doc = "- `validator_stash` is the stash account of the validator. Their nominators, up to"] # [doc = "  `T::MaxNominatorRewardedPerValidator`, will also receive their rewards."] # [doc = "- `era` may be any era between `[current_era - history_depth; current_era]`."] # [doc = ""] # [doc = "The origin of this call must be _Signed_. Any account can call this function, even if"] # [doc = "it is not one of the stakers."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- Time complexity: at most O(MaxNominatorRewardedPerValidator)."] # [doc = "- Contains a limited number of reads and writes."] # [doc = "-----------"] # [doc = "N is the Number of payouts for the validator (including the validator)"] # [doc = "Weight:"] # [doc = "- Reward Destination Staked: O(N)"] # [doc = "- Reward Destination Controller (Creating): O(N)"] # [doc = ""] # [doc = "  NOTE: weights are assuming that payouts are made to alive stash account (Staked)."] # [doc = "  Paying even a dead controller is cheaper weight-wise. We don't do any refunds here."] # [doc = "# </weight>"] payout_stakers { validator_stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , era : :: core :: primitive :: u32 , } , # [codec (index = 19)] # [doc = "Rebond a portion of the stash scheduled to be unlocked."] # [doc = ""] # [doc = "The dispatch origin must be signed by the controller."] # [doc = ""] # [doc = "# <weight>"] # [doc = "- Time complexity: O(L), where L is unlocking chunks"] # [doc = "- Bounded by `MaxUnlockingChunks`."] # [doc = "- Storage changes: Can't increase storage, only decrease it."] # [doc = "# </weight>"] rebond { # [codec (compact)] value : :: core :: primitive :: u128 , } , # [codec (index = 20)] # [doc = "Remove all data structures concerning a staker/stash once it is at a state where it can"] # [doc = "be considered `dust` in the staking system. The requirements are:"] # [doc = ""] # [doc = "1. the `total_balance` of the stash is below existential deposit."] # [doc = "2. or, the `ledger.total` of the stash is below existential deposit."] # [doc = ""] # [doc = "The former can happen in cases like a slash; the latter when a fully unbonded account"] # [doc = "is still receiving staking rewards in `RewardDestination::Staked`."] # [doc = ""] # [doc = "It can be called by anyone, as long as `stash` meets the above requirements."] # [doc = ""] # [doc = "Refunds the transaction fees upon successful execution."] reap_stash { stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , num_slashing_spans : :: core :: primitive :: u32 , } , # [codec (index = 21)] # [doc = "Remove the given nominations from the calling validator."] # [doc = ""] # [doc = "Effects will be felt at the beginning of the next era."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_ by the controller, not the stash."] # [doc = ""] # [doc = "- `who`: A list of nominator stash accounts who are nominating this validator which"] # [doc = "  should no longer be nominating this validator."] # [doc = ""] # [doc = "Note: Making this call only makes sense if you first set the validator preferences to"] # [doc = "block any further nominations."] kick { who : :: std :: vec :: Vec < :: subxt :: ext :: sp_runtime :: MultiAddress < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , } , # [codec (index = 22)] # [doc = "Update the various staking configurations ."] # [doc = ""] # [doc = "* `min_nominator_bond`: The minimum active bond needed to be a nominator."] # [doc = "* `min_validator_bond`: The minimum active bond needed to be a validator."] # [doc = "* `max_nominator_count`: The max number of users who can be a nominator at once. When"] # [doc = "  set to `None`, no limit is enforced."] # [doc = "* `max_validator_count`: The max number of users who can be a validator at once. When"] # [doc = "  set to `None`, no limit is enforced."] # [doc = "* `chill_threshold`: The ratio of `max_nominator_count` or `max_validator_count` which"] # [doc = "  should be filled in order for the `chill_other` transaction to work."] # [doc = "* `min_commission`: The minimum amount of commission that each validators must maintain."] # [doc = "  This is checked only upon calling `validate`. Existing validators are not affected."] # [doc = ""] # [doc = "RuntimeOrigin must be Root to call this function."] # [doc = ""] # [doc = "NOTE: Existing nominators and validators will not be affected by this update."] # [doc = "to kick people under the new limits, `chill_other` should be called."] set_staking_configs { min_nominator_bond : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < :: core :: primitive :: u128 > , min_validator_bond : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < :: core :: primitive :: u128 > , max_nominator_count : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < :: core :: primitive :: u32 > , max_validator_count : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < :: core :: primitive :: u32 > , chill_threshold : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < runtime_types :: sp_arithmetic :: per_things :: Percent > , min_commission : runtime_types :: pallet_staking :: pallet :: pallet :: ConfigOp < runtime_types :: sp_arithmetic :: per_things :: Perbill > , } , # [codec (index = 23)] # [doc = "Declare a `controller` to stop participating as either a validator or nominator."] # [doc = ""] # [doc = "Effects will be felt at the beginning of the next era."] # [doc = ""] # [doc = "The dispatch origin for this call must be _Signed_, but can be called by anyone."] # [doc = ""] # [doc = "If the caller is the same as the controller being targeted, then no further checks are"] # [doc = "enforced, and this function behaves just like `chill`."] # [doc = ""] # [doc = "If the caller is different than the controller being targeted, the following conditions"] # [doc = "must be met:"] # [doc = ""] # [doc = "* `controller` must belong to a nominator who has become non-decodable,"] # [doc = ""] # [doc = "Or:"] # [doc = ""] # [doc = "* A `ChillThreshold` must be set and checked which defines how close to the max"] # [doc = "  nominators or validators we must reach before users can start chilling one-another."] # [doc = "* A `MaxNominatorCount` and `MaxValidatorCount` must be set which is used to determine"] # [doc = "  how close we are to the threshold."] # [doc = "* A `MinNominatorBond` and `MinValidatorBond` must be set and checked, which determines"] # [doc = "  if this is a person that should be chilled because they have not met the threshold"] # [doc = "  bond required."] # [doc = ""] # [doc = "This can be helpful if bond requirements are updated, and we need to remove old users"] # [doc = "who do not satisfy these requirements."] chill_other { controller : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , } , # [codec (index = 24)] # [doc = "Force a validator to have at least the minimum commission. This will not affect a"] # [doc = "validator who already has a commission greater than or equal to the minimum. Any account"] # [doc = "can call this."] force_apply_min_commission { validator_stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , } , }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub enum ConfigOp<_0> {
                        #[codec(index = 0)]
                        Noop,
                        #[codec(index = 1)]
                        Set(_0),
                        #[codec(index = 2)]
                        Remove,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                    pub enum Error {
                        #[codec(index = 0)]
                        #[doc = "Not a controller account."]
                        NotController,
                        #[codec(index = 1)]
                        #[doc = "Not a stash account."]
                        NotStash,
                        #[codec(index = 2)]
                        #[doc = "Stash is already bonded."]
                        AlreadyBonded,
                        #[codec(index = 3)]
                        #[doc = "Controller is already paired."]
                        AlreadyPaired,
                        #[codec(index = 4)]
                        #[doc = "Targets cannot be empty."]
                        EmptyTargets,
                        #[codec(index = 5)]
                        #[doc = "Duplicate index."]
                        DuplicateIndex,
                        #[codec(index = 6)]
                        #[doc = "Slash record index out of bounds."]
                        InvalidSlashIndex,
                        #[codec(index = 7)]
                        #[doc = "Cannot have a validator or nominator role, with value less than the minimum defined by"]
                        #[doc = "governance (see `MinValidatorBond` and `MinNominatorBond`). If unbonding is the"]
                        #[doc = "intention, `chill` first to remove one's role as validator/nominator."]
                        InsufficientBond,
                        #[codec(index = 8)]
                        #[doc = "Can not schedule more unlock chunks."]
                        NoMoreChunks,
                        #[codec(index = 9)]
                        #[doc = "Can not rebond without unlocking chunks."]
                        NoUnlockChunk,
                        #[codec(index = 10)]
                        #[doc = "Attempting to target a stash that still has funds."]
                        FundedTarget,
                        #[codec(index = 11)]
                        #[doc = "Invalid era to reward."]
                        InvalidEraToReward,
                        #[codec(index = 12)]
                        #[doc = "Invalid number of nominations."]
                        InvalidNumberOfNominations,
                        #[codec(index = 13)]
                        #[doc = "Items are not sorted and unique."]
                        NotSortedAndUnique,
                        #[codec(index = 14)]
                        #[doc = "Rewards for this era have already been claimed for this validator."]
                        AlreadyClaimed,
                        #[codec(index = 15)]
                        #[doc = "Incorrect previous history depth input provided."]
                        IncorrectHistoryDepth,
                        #[codec(index = 16)]
                        #[doc = "Incorrect number of slashing spans provided."]
                        IncorrectSlashingSpans,
                        #[codec(index = 17)]
                        #[doc = "Internal state has become somehow corrupted and the operation cannot continue."]
                        BadState,
                        #[codec(index = 18)]
                        #[doc = "Too many nomination targets supplied."]
                        TooManyTargets,
                        #[codec(index = 19)]
                        #[doc = "A nomination target was supplied that was blocked or otherwise not a validator."]
                        BadTarget,
                        #[codec(index = 20)]
                        #[doc = "The user has enough bond and thus cannot be chilled forcefully by an external person."]
                        CannotChillOther,
                        #[codec(index = 21)]
                        #[doc = "There are too many nominators in the system. Governance needs to adjust the staking"]
                        #[doc = "settings to keep things safe for the runtime."]
                        TooManyNominators,
                        #[codec(index = 22)]
                        #[doc = "There are too many validator candidates in the system. Governance needs to adjust the"]
                        #[doc = "staking settings to keep things safe for the runtime."]
                        TooManyValidators,
                        #[codec(index = 23)]
                        #[doc = "Commission is too low. Must be at least `MinCommission`."]
                        CommissionTooLow,
                        #[codec(index = 24)]
                        #[doc = "Some bound is not met."]
                        BoundNotMet,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                    pub enum Event {
                        # [codec (index = 0)] # [doc = "The era payout has been set; the first balance is the validator-payout; the second is"] # [doc = "the remainder from the maximum amount of reward."] EraPaid { era_index : :: core :: primitive :: u32 , validator_payout : :: core :: primitive :: u128 , remainder : :: core :: primitive :: u128 , } , # [codec (index = 1)] # [doc = "The nominator has been rewarded by this amount."] Rewarded { stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 2)] # [doc = "A staker (validator or nominator) has been slashed by the given amount."] Slashed { staker : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 3)] # [doc = "A slash for the given validator, for the given percentage of their stake, at the given"] # [doc = "era as been reported."] SlashReported { validator : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , fraction : runtime_types :: sp_arithmetic :: per_things :: Perbill , slash_era : :: core :: primitive :: u32 , } , # [codec (index = 4)] # [doc = "An old slashing report from a prior era was discarded because it could"] # [doc = "not be processed."] OldSlashingReportDiscarded { session_index : :: core :: primitive :: u32 , } , # [codec (index = 5)] # [doc = "A new set of stakers was elected."] StakersElected , # [codec (index = 6)] # [doc = "An account has bonded this amount. \\[stash, amount\\]"] # [doc = ""] # [doc = "NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,"] # [doc = "it will not be emitted for staking rewards when they are added to stake."] Bonded { stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 7)] # [doc = "An account has unbonded this amount."] Unbonded { stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 8)] # [doc = "An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`"] # [doc = "from the unlocking queue."] Withdrawn { stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , amount : :: core :: primitive :: u128 , } , # [codec (index = 9)] # [doc = "A nominator has been kicked from a validator."] Kicked { nominator : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , } , # [codec (index = 10)] # [doc = "The election failed. No new era is planned."] StakingElectionFailed , # [codec (index = 11)] # [doc = "An account has stopped participating as either a validator or nominator."] Chilled { stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , } , # [codec (index = 12)] # [doc = "The stakers' rewards are getting paid."] PayoutStarted { era_index : :: core :: primitive :: u32 , validator_stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , } , # [codec (index = 13)] # [doc = "A validator has set their preferences."] ValidatorPrefsSet { stash : :: subxt :: ext :: sp_core :: crypto :: AccountId32 , prefs : runtime_types :: pallet_staking :: ValidatorPrefs , } , }
                }
            }
            pub mod slashing {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct SlashingSpans {
                    pub span_index: ::core::primitive::u32,
                    pub last_start: ::core::primitive::u32,
                    pub last_nonzero_slash: ::core::primitive::u32,
                    pub prior: ::std::vec::Vec<::core::primitive::u32>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct SpanRecord<_0> {
                    pub slashed: _0,
                    pub paid_out: _0,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual:
                    ::subxt::utils::KeyedVec<_0, ::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct IndividualExposure<_0, _1> {
                pub who: _0,
                #[codec(compact)]
                pub value: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Nominations {
                pub targets:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                pub submitted_in: ::core::primitive::u32,
                pub suppressed: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                #[codec(index = 8)]
                V9_0_0,
                #[codec(index = 9)]
                V10_0_0,
                #[codec(index = 10)]
                V11_0_0,
                #[codec(index = 11)]
                V12_0_0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct StakingLedger {
                pub stash: ::subxt::ext::sp_core::crypto::AccountId32,
                #[codec(compact)]
                pub total: ::core::primitive::u128,
                #[codec(compact)]
                pub active: ::core::primitive::u128,
                pub unlocking:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        runtime_types::pallet_staking::UnlockChunk<
                            ::core::primitive::u128,
                        >,
                    >,
                pub claimed_rewards:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u32,
                    >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct UnappliedSlash<_0, _1> {
                pub validator: _0,
                pub own: _1,
                pub others: ::std::vec::Vec<(_0, _1)>,
                pub reporters: ::std::vec::Vec<_0>,
                pub payout: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct UnlockChunk<_0> {
                #[codec(compact)]
                pub value: _0,
                #[codec(compact)]
                pub era: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"]
                    sudo {
                        call: ::std::boxed::Box<
                            runtime_types::dkg_standalone_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- The weight of this call is defined by the caller."]
                    #[doc = "# </weight>"]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<
                            runtime_types::dkg_standalone_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB change."]
                    #[doc = "# </weight>"]
                    set_key {
                        new: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"]
                    sudo_as {
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                        call: ::std::boxed::Box<
                            runtime_types::dkg_standalone_runtime::RuntimeCall,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Error for the Sudo pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account"]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    Sudid {
                        sudo_result: ::core::result::Result<
                            (),
                            runtime_types::sp_runtime::DispatchError,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "A sudo just took place. \\[result\\]"]
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "`MinimumPeriod`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Inherent`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    #[doc = "# </weight>"]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."]
                    TransactionFeePaid {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ChargeTransactionPayment(
                #[codec(compact)] pub ::core::primitive::u128,
            );
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct PerU16(pub ::core::primitive::u16);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Percent(pub ::core::primitive::u8);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
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
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
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
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod bounded {
                use super::runtime_types;
                pub mod bounded_btree_map {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct BoundedBTreeMap<_0, _1>(
                        pub ::subxt::utils::KeyedVec<_0, _1>,
                    );
                }
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct OpaqueMultiaddr(
                    pub ::std::vec::Vec<::core::primitive::u8>,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses: ::std::vec::Vec<
                        runtime_types::sp_core::offchain::OpaqueMultiaddr,
                    >,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Signature(
                    pub runtime_types::sp_core::ed25519::Signature,
                );
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ElectionScore {
                pub minimal_stake: ::core::primitive::u128,
                pub sum_stake: ::core::primitive::u128,
                pub sum_stake_squared: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
                    )]
                    pub struct Digest { pub logs : :: std :: vec :: Vec < runtime_types :: sp_runtime :: generic :: digest :: DigestItem > , }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
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
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
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
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Clone,
                        Debug,
                        Eq,
                        PartialEq,
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
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
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
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Clone,
                Debug,
                Eq,
                PartialEq,
            )]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
        pub mod webb_proposals {
            use super::runtime_types;
            pub mod header {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct ResourceId(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
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
                    #[codec(index = 8)]
                    Ink(::core::primitive::u32),
                }
            }
            pub mod nonce {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub struct Nonce(pub ::core::primitive::u32);
            }
            pub mod proposal {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
                )]
                pub enum Proposal {
                    # [codec (index = 0)] Signed { kind : runtime_types :: webb_proposals :: proposal :: ProposalKind , data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , signature : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 1)] Unsigned { kind : runtime_types :: webb_proposals :: proposal :: ProposalKind , data : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Clone,
                    Debug,
                    Eq,
                    PartialEq,
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
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue,"]
    #[doc = r" exposed here for ease of use."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn indices(&self) -> indices::constants::ConstantsApi {
            indices::constants::ConstantsApi
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
        pub fn bags_list(&self) -> bags_list::constants::ConstantsApi {
            bags_list::constants::ConstantsApi
        }
        pub fn nomination_pools(
            &self,
        ) -> nomination_pools::constants::ConstantsApi {
            nomination_pools::constants::ConstantsApi
        }
        pub fn staking(&self) -> staking::constants::ConstantsApi {
            staking::constants::ConstantsApi
        }
        pub fn bridge_registry(
            &self,
        ) -> bridge_registry::constants::ConstantsApi {
            bridge_registry::constants::ConstantsApi
        }
        pub fn identity(&self) -> identity::constants::ConstantsApi {
            identity::constants::ConstantsApi
        }
        pub fn im_online(&self) -> im_online::constants::ConstantsApi {
            im_online::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn indices(&self) -> indices::storage::StorageApi {
            indices::storage::StorageApi
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi {
            randomness_collective_flip::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn aura(&self) -> aura::storage::StorageApi {
            aura::storage::StorageApi
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn dkg(&self) -> dkg::storage::StorageApi {
            dkg::storage::StorageApi
        }
        pub fn dkg_proposals(&self) -> dkg_proposals::storage::StorageApi {
            dkg_proposals::storage::StorageApi
        }
        pub fn dkg_proposal_handler(
            &self,
        ) -> dkg_proposal_handler::storage::StorageApi {
            dkg_proposal_handler::storage::StorageApi
        }
        pub fn transaction_payment(
            &self,
        ) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::storage::StorageApi {
            election_provider_multi_phase::storage::StorageApi
        }
        pub fn bags_list(&self) -> bags_list::storage::StorageApi {
            bags_list::storage::StorageApi
        }
        pub fn nomination_pools(
            &self,
        ) -> nomination_pools::storage::StorageApi {
            nomination_pools::storage::StorageApi
        }
        pub fn staking(&self) -> staking::storage::StorageApi {
            staking::storage::StorageApi
        }
        pub fn session(&self) -> session::storage::StorageApi {
            session::storage::StorageApi
        }
        pub fn historical(&self) -> historical::storage::StorageApi {
            historical::storage::StorageApi
        }
        pub fn bridge_registry(&self) -> bridge_registry::storage::StorageApi {
            bridge_registry::storage::StorageApi
        }
        pub fn identity(&self) -> identity::storage::StorageApi {
            identity::storage::StorageApi
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi {
            im_online::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn indices(&self) -> indices::calls::TransactionApi {
            indices::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
            grandpa::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn dkg(&self) -> dkg::calls::TransactionApi {
            dkg::calls::TransactionApi
        }
        pub fn dkg_proposals(&self) -> dkg_proposals::calls::TransactionApi {
            dkg_proposals::calls::TransactionApi
        }
        pub fn dkg_proposal_handler(
            &self,
        ) -> dkg_proposal_handler::calls::TransactionApi {
            dkg_proposal_handler::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::calls::TransactionApi {
            election_provider_multi_phase::calls::TransactionApi
        }
        pub fn bags_list(&self) -> bags_list::calls::TransactionApi {
            bags_list::calls::TransactionApi
        }
        pub fn nomination_pools(
            &self,
        ) -> nomination_pools::calls::TransactionApi {
            nomination_pools::calls::TransactionApi
        }
        pub fn staking(&self) -> staking::calls::TransactionApi {
            staking::calls::TransactionApi
        }
        pub fn session(&self) -> session::calls::TransactionApi {
            session::calls::TransactionApi
        }
        pub fn bridge_registry(
            &self,
        ) -> bridge_registry::calls::TransactionApi {
            bridge_registry::calls::TransactionApi
        }
        pub fn identity(&self) -> identity::calls::TransactionApi {
            identity::calls::TransactionApi
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi {
            im_online::calls::TransactionApi
        }
    }
    #[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
    pub fn validate_codegen<
        T: ::subxt::Config,
        C: ::subxt::client::OfflineClientT<T>,
    >(
        client: &C,
    ) -> Result<(), ::subxt::error::MetadataError> {
        let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
        if runtime_metadata_hash
            != [
                188u8, 90u8, 177u8, 23u8, 160u8, 236u8, 92u8, 192u8, 7u8,
                163u8, 212u8, 249u8, 6u8, 223u8, 157u8, 154u8, 152u8, 16u8,
                146u8, 100u8, 135u8, 134u8, 138u8, 56u8, 28u8, 84u8, 192u8,
                117u8, 203u8, 178u8, 38u8, 30u8,
            ]
        {
            Err(::subxt::error::MetadataError::IncompatibleMetadata)
        } else {
            Ok(())
        }
    }
}
