pub use treasury_contract::*;
#[doc = r" This module was auto-generated with ethers-rs Abigen."]
#[doc = r" More information at: <https://github.com/gakonst/ethers-rs>"]
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod treasury_contract {
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_treasuryHandler\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToRescue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rescueTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newHandler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setHandler\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]" ;
    #[doc = "The parsed JSON ABI of the contract."]
    pub static TREASURYCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    # [rustfmt :: skip] const __BYTECODE : & [u8] = & [96 , 128 , 96 , 64 , 82 , 52 , 128 , 21 , 97 , 0 , 16 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 64 , 81 , 97 , 8 , 7 , 56 , 3 , 128 , 97 , 8 , 7 , 131 , 57 , 129 , 1 , 96 , 64 , 129 , 144 , 82 , 97 , 0 , 47 , 145 , 97 , 0 , 84 , 86 , 91 , 96 , 1 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 146 , 144 , 146 , 22 , 145 , 144 , 145 , 23 , 144 , 85 , 97 , 0 , 132 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 0 , 102 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 129 , 20 , 97 , 0 , 125 , 87 , 96 , 0 , 128 , 253 , 91 , 147 , 146 , 80 , 80 , 80 , 86 , 91 , 97 , 7 , 116 , 128 , 97 , 0 , 147 , 96 , 0 , 57 , 96 , 0 , 243 , 254 , 96 , 128 , 96 , 64 , 82 , 96 , 4 , 54 , 16 , 97 , 0 , 67 , 87 , 96 , 0 , 53 , 96 , 224 , 28 , 128 , 99 , 11 , 39 , 251 , 154 , 20 , 97 , 0 , 79 , 87 , 128 , 99 , 98 , 44 , 119 , 217 , 20 , 97 , 0 , 114 , 87 , 128 , 99 , 114 , 193 , 173 , 3 , 20 , 97 , 0 , 148 , 87 , 128 , 99 , 204 , 60 , 116 , 161 , 20 , 97 , 0 , 180 , 87 , 96 , 0 , 128 , 253 , 91 , 54 , 97 , 0 , 74 , 87 , 0 , 91 , 96 , 0 , 128 , 253 , 91 , 52 , 128 , 21 , 97 , 0 , 91 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 0 , 84 , 91 , 96 , 64 , 81 , 144 , 129 , 82 , 96 , 32 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 243 , 91 , 52 , 128 , 21 , 97 , 0 , 126 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 0 , 146 , 97 , 0 , 141 , 54 , 96 , 4 , 97 , 5 , 73 , 86 , 91 , 97 , 0 , 202 , 86 , 91 , 0 , 91 , 52 , 128 , 21 , 97 , 0 , 160 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 0 , 146 , 97 , 0 , 175 , 54 , 96 , 4 , 97 , 5 , 154 , 86 , 91 , 97 , 4 , 60 , 86 , 91 , 52 , 128 , 21 , 97 , 0 , 192 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 0 , 96 , 96 , 0 , 84 , 129 , 86 , 91 , 96 , 1 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 0 , 253 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 17 , 86 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 253 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 0 , 84 , 16 , 97 , 1 , 37 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 96 , 86 , 91 , 96 , 0 , 84 , 97 , 1 , 51 , 144 , 96 , 1 , 97 , 7 , 0 , 86 , 91 , 129 , 17 , 21 , 97 , 1 , 82 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 163 , 86 , 91 , 96 , 0 , 129 , 144 , 85 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 97 , 1 , 187 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 37 , 96 , 36 , 130 , 1 , 82 , 127 , 67 , 97 , 110 , 110 , 111 , 116 , 32 , 115 , 101 , 110 , 100 , 32 , 108 , 105 , 113 , 117 , 105 , 100 , 105 , 116 , 121 , 32 , 116 , 111 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 96 , 68 , 130 , 1 , 82 , 100 , 100 , 114 , 101 , 115 , 115 , 96 , 216 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 0 , 244 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 48 , 20 , 21 , 97 , 2 , 20 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 27 , 96 , 36 , 130 , 1 , 82 , 127 , 67 , 97 , 110 , 110 , 111 , 116 , 32 , 114 , 101 , 115 , 99 , 117 , 101 , 32 , 119 , 114 , 97 , 112 , 112 , 101 , 100 , 32 , 97 , 115 , 115 , 101 , 116 , 0 , 0 , 0 , 0 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 0 , 244 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 97 , 2 , 164 , 87 , 71 , 131 , 129 , 16 , 97 , 2 , 102 , 87 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 144 , 133 , 21 , 97 , 8 , 252 , 2 , 144 , 134 , 144 , 96 , 0 , 129 , 129 , 129 , 133 , 136 , 136 , 241 , 147 , 80 , 80 , 80 , 80 , 21 , 128 , 21 , 97 , 2 , 96 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 97 , 2 , 158 , 86 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 144 , 130 , 21 , 97 , 8 , 252 , 2 , 144 , 131 , 144 , 96 , 0 , 129 , 129 , 129 , 133 , 136 , 136 , 241 , 147 , 80 , 80 , 80 , 80 , 21 , 128 , 21 , 97 , 2 , 156 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 91 , 80 , 97 , 4 , 53 , 86 , 91 , 96 , 64 , 81 , 99 , 112 , 160 , 130 , 49 , 96 , 224 , 27 , 129 , 82 , 48 , 96 , 4 , 130 , 1 , 82 , 96 , 0 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 135 , 22 , 144 , 99 , 112 , 160 , 130 , 49 , 144 , 96 , 36 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 134 , 128 , 59 , 21 , 128 , 21 , 97 , 2 , 230 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 90 , 250 , 21 , 128 , 21 , 97 , 2 , 250 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 3 , 30 , 145 , 144 , 97 , 5 , 248 , 86 , 91 , 144 , 80 , 131 , 129 , 16 , 97 , 3 , 175 , 87 , 96 , 64 , 81 , 99 , 169 , 5 , 156 , 187 , 96 , 224 , 27 , 129 , 82 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 129 , 22 , 96 , 4 , 131 , 1 , 82 , 96 , 36 , 130 , 1 , 134 , 144 , 82 , 135 , 22 , 144 , 99 , 169 , 5 , 156 , 187 , 144 , 96 , 68 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 96 , 0 , 135 , 128 , 59 , 21 , 128 , 21 , 97 , 3 , 113 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 90 , 241 , 21 , 128 , 21 , 97 , 3 , 133 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 3 , 169 , 145 , 144 , 97 , 5 , 207 , 86 , 91 , 80 , 97 , 4 , 51 , 86 , 91 , 96 , 64 , 81 , 99 , 169 , 5 , 156 , 187 , 96 , 224 , 27 , 129 , 82 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 129 , 22 , 96 , 4 , 131 , 1 , 82 , 96 , 36 , 130 , 1 , 131 , 144 , 82 , 135 , 22 , 144 , 99 , 169 , 5 , 156 , 187 , 144 , 96 , 68 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 96 , 0 , 135 , 128 , 59 , 21 , 128 , 21 , 97 , 3 , 249 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 90 , 241 , 21 , 128 , 21 , 97 , 4 , 13 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 4 , 49 , 145 , 144 , 97 , 5 , 207 , 86 , 91 , 80 , 91 , 80 , 91 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 1 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 4 , 102 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 17 , 86 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 0 , 84 , 16 , 97 , 4 , 142 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 96 , 86 , 91 , 96 , 0 , 84 , 97 , 4 , 156 , 144 , 96 , 1 , 97 , 7 , 0 , 86 , 91 , 129 , 17 , 21 , 97 , 4 , 187 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 163 , 86 , 91 , 96 , 0 , 129 , 144 , 85 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 97 , 5 , 12 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 19 , 96 , 36 , 130 , 1 , 82 , 114 , 4 , 134 , 22 , 230 , 70 , 198 , 87 , 34 , 6 , 54 , 22 , 230 , 230 , 247 , 66 , 6 , 38 , 82 , 3 , 96 , 108 , 27 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 0 , 244 , 86 , 91 , 80 , 80 , 96 , 1 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 146 , 144 , 146 , 22 , 145 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 128 , 53 , 99 , 255 , 255 , 255 , 255 , 129 , 22 , 129 , 20 , 97 , 5 , 68 , 87 , 96 , 0 , 128 , 253 , 91 , 145 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 128 , 96 , 128 , 133 , 135 , 3 , 18 , 21 , 97 , 5 , 95 , 87 , 96 , 0 , 128 , 253 , 91 , 132 , 53 , 97 , 5 , 106 , 129 , 97 , 7 , 38 , 86 , 91 , 147 , 80 , 96 , 32 , 133 , 1 , 53 , 97 , 5 , 122 , 129 , 97 , 7 , 38 , 86 , 91 , 146 , 80 , 96 , 64 , 133 , 1 , 53 , 145 , 80 , 97 , 5 , 143 , 96 , 96 , 134 , 1 , 97 , 5 , 48 , 86 , 91 , 144 , 80 , 146 , 149 , 145 , 148 , 80 , 146 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 5 , 173 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 97 , 5 , 184 , 129 , 97 , 7 , 38 , 86 , 91 , 145 , 80 , 97 , 5 , 198 , 96 , 32 , 132 , 1 , 97 , 5 , 48 , 86 , 91 , 144 , 80 , 146 , 80 , 146 , 144 , 80 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 5 , 225 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 81 , 128 , 21 , 21 , 129 , 20 , 97 , 5 , 241 , 87 , 96 , 0 , 128 , 253 , 91 , 147 , 146 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 6 , 10 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 81 , 145 , 144 , 80 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 47 , 144 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 99 , 116 , 105 , 111 , 110 , 32 , 99 , 97 , 110 , 32 , 111 , 110 , 108 , 121 , 32 , 98 , 101 , 32 , 99 , 97 , 108 , 108 , 101 , 100 , 32 , 98 , 121 , 32 , 116 , 96 , 64 , 130 , 1 , 82 , 110 , 57 , 50 , 176 , 185 , 186 , 185 , 60 , 144 , 52 , 48 , 183 , 50 , 54 , 50 , 185 , 96 , 137 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 35 , 144 , 130 , 1 , 82 , 127 , 80 , 114 , 111 , 112 , 111 , 115 , 97 , 108 , 78 , 111 , 110 , 99 , 101 , 84 , 114 , 97 , 99 , 107 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 110 , 111 , 96 , 64 , 130 , 1 , 82 , 98 , 110 , 99 , 101 , 96 , 232 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 58 , 144 , 130 , 1 , 82 , 127 , 80 , 114 , 111 , 112 , 111 , 115 , 97 , 108 , 78 , 111 , 110 , 99 , 101 , 84 , 114 , 97 , 99 , 107 , 101 , 114 , 58 , 32 , 78 , 111 , 110 , 99 , 101 , 32 , 109 , 117 , 115 , 116 , 96 , 64 , 130 , 1 , 82 , 127 , 32 , 110 , 111 , 116 , 32 , 105 , 110 , 99 , 114 , 101 , 109 , 101 , 110 , 116 , 32 , 109 , 111 , 114 , 101 , 32 , 116 , 104 , 97 , 110 , 32 , 49 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 0 , 130 , 25 , 130 , 17 , 21 , 97 , 7 , 33 , 87 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 17 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 80 , 1 , 144 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 129 , 20 , 97 , 7 , 59 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 86 , 254 , 162 , 100 , 105 , 112 , 102 , 115 , 88 , 34 , 18 , 32 , 167 , 93 , 32 , 138 , 217 , 231 , 124 , 77 , 27 , 107 , 244 , 101 , 100 , 193 , 85 , 76 , 233 , 140 , 74 , 101 , 169 , 222 , 104 , 69 , 186 , 160 , 244 , 226 , 82 , 145 , 180 , 88 , 100 , 115 , 111 , 108 , 99 , 67 , 0 , 8 , 5 , 0 , 51] ;
    #[doc = "The bytecode of the contract."]
    pub static TREASURYCONTRACT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    # [rustfmt :: skip] const __DEPLOYED_BYTECODE : & [u8] = & [96 , 128 , 96 , 64 , 82 , 96 , 4 , 54 , 16 , 97 , 0 , 67 , 87 , 96 , 0 , 53 , 96 , 224 , 28 , 128 , 99 , 11 , 39 , 251 , 154 , 20 , 97 , 0 , 79 , 87 , 128 , 99 , 98 , 44 , 119 , 217 , 20 , 97 , 0 , 114 , 87 , 128 , 99 , 114 , 193 , 173 , 3 , 20 , 97 , 0 , 148 , 87 , 128 , 99 , 204 , 60 , 116 , 161 , 20 , 97 , 0 , 180 , 87 , 96 , 0 , 128 , 253 , 91 , 54 , 97 , 0 , 74 , 87 , 0 , 91 , 96 , 0 , 128 , 253 , 91 , 52 , 128 , 21 , 97 , 0 , 91 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 0 , 84 , 91 , 96 , 64 , 81 , 144 , 129 , 82 , 96 , 32 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 243 , 91 , 52 , 128 , 21 , 97 , 0 , 126 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 0 , 146 , 97 , 0 , 141 , 54 , 96 , 4 , 97 , 5 , 73 , 86 , 91 , 97 , 0 , 202 , 86 , 91 , 0 , 91 , 52 , 128 , 21 , 97 , 0 , 160 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 0 , 146 , 97 , 0 , 175 , 54 , 96 , 4 , 97 , 5 , 154 , 86 , 91 , 97 , 4 , 60 , 86 , 91 , 52 , 128 , 21 , 97 , 0 , 192 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 97 , 0 , 96 , 96 , 0 , 84 , 129 , 86 , 91 , 96 , 1 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 0 , 253 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 17 , 86 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 253 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 0 , 84 , 16 , 97 , 1 , 37 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 96 , 86 , 91 , 96 , 0 , 84 , 97 , 1 , 51 , 144 , 96 , 1 , 97 , 7 , 0 , 86 , 91 , 129 , 17 , 21 , 97 , 1 , 82 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 163 , 86 , 91 , 96 , 0 , 129 , 144 , 85 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 97 , 1 , 187 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 37 , 96 , 36 , 130 , 1 , 82 , 127 , 67 , 97 , 110 , 110 , 111 , 116 , 32 , 115 , 101 , 110 , 100 , 32 , 108 , 105 , 113 , 117 , 105 , 100 , 105 , 116 , 121 , 32 , 116 , 111 , 32 , 122 , 101 , 114 , 111 , 32 , 97 , 100 , 96 , 68 , 130 , 1 , 82 , 100 , 100 , 114 , 101 , 115 , 115 , 96 , 216 , 27 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 97 , 0 , 244 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 48 , 20 , 21 , 97 , 2 , 20 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 27 , 96 , 36 , 130 , 1 , 82 , 127 , 67 , 97 , 110 , 110 , 111 , 116 , 32 , 114 , 101 , 115 , 99 , 117 , 101 , 32 , 119 , 114 , 97 , 112 , 112 , 101 , 100 , 32 , 97 , 115 , 115 , 101 , 116 , 0 , 0 , 0 , 0 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 0 , 244 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 97 , 2 , 164 , 87 , 71 , 131 , 129 , 16 , 97 , 2 , 102 , 87 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 144 , 133 , 21 , 97 , 8 , 252 , 2 , 144 , 134 , 144 , 96 , 0 , 129 , 129 , 129 , 133 , 136 , 136 , 241 , 147 , 80 , 80 , 80 , 80 , 21 , 128 , 21 , 97 , 2 , 96 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 97 , 2 , 158 , 86 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 22 , 144 , 130 , 21 , 97 , 8 , 252 , 2 , 144 , 131 , 144 , 96 , 0 , 129 , 129 , 129 , 133 , 136 , 136 , 241 , 147 , 80 , 80 , 80 , 80 , 21 , 128 , 21 , 97 , 2 , 156 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 91 , 80 , 97 , 4 , 53 , 86 , 91 , 96 , 64 , 81 , 99 , 112 , 160 , 130 , 49 , 96 , 224 , 27 , 129 , 82 , 48 , 96 , 4 , 130 , 1 , 82 , 96 , 0 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 135 , 22 , 144 , 99 , 112 , 160 , 130 , 49 , 144 , 96 , 36 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 134 , 128 , 59 , 21 , 128 , 21 , 97 , 2 , 230 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 90 , 250 , 21 , 128 , 21 , 97 , 2 , 250 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 3 , 30 , 145 , 144 , 97 , 5 , 248 , 86 , 91 , 144 , 80 , 131 , 129 , 16 , 97 , 3 , 175 , 87 , 96 , 64 , 81 , 99 , 169 , 5 , 156 , 187 , 96 , 224 , 27 , 129 , 82 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 129 , 22 , 96 , 4 , 131 , 1 , 82 , 96 , 36 , 130 , 1 , 134 , 144 , 82 , 135 , 22 , 144 , 99 , 169 , 5 , 156 , 187 , 144 , 96 , 68 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 96 , 0 , 135 , 128 , 59 , 21 , 128 , 21 , 97 , 3 , 113 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 90 , 241 , 21 , 128 , 21 , 97 , 3 , 133 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 3 , 169 , 145 , 144 , 97 , 5 , 207 , 86 , 91 , 80 , 97 , 4 , 51 , 86 , 91 , 96 , 64 , 81 , 99 , 169 , 5 , 156 , 187 , 96 , 224 , 27 , 129 , 82 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 134 , 129 , 22 , 96 , 4 , 131 , 1 , 82 , 96 , 36 , 130 , 1 , 131 , 144 , 82 , 135 , 22 , 144 , 99 , 169 , 5 , 156 , 187 , 144 , 96 , 68 , 1 , 96 , 32 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 96 , 0 , 135 , 128 , 59 , 21 , 128 , 21 , 97 , 3 , 249 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 90 , 241 , 21 , 128 , 21 , 97 , 4 , 13 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 96 , 64 , 81 , 61 , 96 , 31 , 25 , 96 , 31 , 130 , 1 , 22 , 130 , 1 , 128 , 96 , 64 , 82 , 80 , 129 , 1 , 144 , 97 , 4 , 49 , 145 , 144 , 97 , 5 , 207 , 86 , 91 , 80 , 91 , 80 , 91 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 1 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 4 , 102 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 17 , 86 , 91 , 128 , 99 , 255 , 255 , 255 , 255 , 22 , 128 , 96 , 0 , 84 , 16 , 97 , 4 , 142 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 96 , 86 , 91 , 96 , 0 , 84 , 97 , 4 , 156 , 144 , 96 , 1 , 97 , 7 , 0 , 86 , 91 , 129 , 17 , 21 , 97 , 4 , 187 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 4 , 1 , 97 , 0 , 244 , 144 , 97 , 6 , 163 , 86 , 91 , 96 , 0 , 129 , 144 , 85 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 131 , 22 , 97 , 5 , 12 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 19 , 96 , 36 , 130 , 1 , 82 , 114 , 4 , 134 , 22 , 230 , 70 , 198 , 87 , 34 , 6 , 54 , 22 , 230 , 230 , 247 , 66 , 6 , 38 , 82 , 3 , 96 , 108 , 27 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 0 , 244 , 86 , 91 , 80 , 80 , 96 , 1 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 146 , 144 , 146 , 22 , 145 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 128 , 53 , 99 , 255 , 255 , 255 , 255 , 129 , 22 , 129 , 20 , 97 , 5 , 68 , 87 , 96 , 0 , 128 , 253 , 91 , 145 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 128 , 96 , 128 , 133 , 135 , 3 , 18 , 21 , 97 , 5 , 95 , 87 , 96 , 0 , 128 , 253 , 91 , 132 , 53 , 97 , 5 , 106 , 129 , 97 , 7 , 38 , 86 , 91 , 147 , 80 , 96 , 32 , 133 , 1 , 53 , 97 , 5 , 122 , 129 , 97 , 7 , 38 , 86 , 91 , 146 , 80 , 96 , 64 , 133 , 1 , 53 , 145 , 80 , 97 , 5 , 143 , 96 , 96 , 134 , 1 , 97 , 5 , 48 , 86 , 91 , 144 , 80 , 146 , 149 , 145 , 148 , 80 , 146 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 5 , 173 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 97 , 5 , 184 , 129 , 97 , 7 , 38 , 86 , 91 , 145 , 80 , 97 , 5 , 198 , 96 , 32 , 132 , 1 , 97 , 5 , 48 , 86 , 91 , 144 , 80 , 146 , 80 , 146 , 144 , 80 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 5 , 225 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 81 , 128 , 21 , 21 , 129 , 20 , 97 , 5 , 241 , 87 , 96 , 0 , 128 , 253 , 91 , 147 , 146 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 6 , 10 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 81 , 145 , 144 , 80 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 47 , 144 , 130 , 1 , 82 , 127 , 70 , 117 , 110 , 99 , 116 , 105 , 111 , 110 , 32 , 99 , 97 , 110 , 32 , 111 , 110 , 108 , 121 , 32 , 98 , 101 , 32 , 99 , 97 , 108 , 108 , 101 , 100 , 32 , 98 , 121 , 32 , 116 , 96 , 64 , 130 , 1 , 82 , 110 , 57 , 50 , 176 , 185 , 186 , 185 , 60 , 144 , 52 , 48 , 183 , 50 , 54 , 50 , 185 , 96 , 137 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 35 , 144 , 130 , 1 , 82 , 127 , 80 , 114 , 111 , 112 , 111 , 115 , 97 , 108 , 78 , 111 , 110 , 99 , 101 , 84 , 114 , 97 , 99 , 107 , 101 , 114 , 58 , 32 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 110 , 111 , 96 , 64 , 130 , 1 , 82 , 98 , 110 , 99 , 101 , 96 , 232 , 27 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 32 , 128 , 130 , 82 , 96 , 58 , 144 , 130 , 1 , 82 , 127 , 80 , 114 , 111 , 112 , 111 , 115 , 97 , 108 , 78 , 111 , 110 , 99 , 101 , 84 , 114 , 97 , 99 , 107 , 101 , 114 , 58 , 32 , 78 , 111 , 110 , 99 , 101 , 32 , 109 , 117 , 115 , 116 , 96 , 64 , 130 , 1 , 82 , 127 , 32 , 110 , 111 , 116 , 32 , 105 , 110 , 99 , 114 , 101 , 109 , 101 , 110 , 116 , 32 , 109 , 111 , 114 , 101 , 32 , 116 , 104 , 97 , 110 , 32 , 49 , 0 , 0 , 0 , 0 , 0 , 0 , 96 , 96 , 130 , 1 , 82 , 96 , 128 , 1 , 144 , 86 , 91 , 96 , 0 , 130 , 25 , 130 , 17 , 21 , 97 , 7 , 33 , 87 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 17 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 80 , 1 , 144 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 129 , 20 , 97 , 7 , 59 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 86 , 254 , 162 , 100 , 105 , 112 , 102 , 115 , 88 , 34 , 18 , 32 , 167 , 93 , 32 , 138 , 217 , 231 , 124 , 77 , 27 , 107 , 244 , 101 , 100 , 193 , 85 , 76 , 233 , 140 , 74 , 101 , 169 , 222 , 104 , 69 , 186 , 160 , 244 , 226 , 82 , 145 , 180 , 88 , 100 , 115 , 111 , 108 , 99 , 67 , 0 , 8 , 5 , 0 , 51] ;
    #[doc = "The deployed bytecode of the contract."]
    pub static TREASURYCONTRACT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct TreasuryContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TreasuryContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TreasuryContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TreasuryContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TreasuryContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(TreasuryContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TreasuryContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers` client at"]
        #[doc = r" `address`. The contract derefs to a `ethers::Contract` object."]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                TREASURYCONTRACT_ABI.clone(),
                client,
            ))
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" - If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" - The default poll duration is 7 seconds."]
        #[doc = r" - The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter, "../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                TREASURYCONTRACT_ABI.clone(),
                TREASURYCONTRACT_BYTECODE.clone(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getProposalNonce` (0x0b27fb9a) function"]
        pub fn get_proposal_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([11, 39, 251, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposalNonce` (0xcc3c74a1) function"]
        pub fn proposal_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([204, 60, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rescueTokens` (0x622c77d9) function"]
        pub fn rescue_tokens(
            &self,
            token_address: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount_to_rescue: ::ethers::core::types::U256,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [98, 44, 119, 217],
                    (token_address, to, amount_to_rescue, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHandler` (0x72c1ad03) function"]
        pub fn set_handler(
            &self,
            new_handler: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 193, 173, 3], (new_handler, nonce))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for TreasuryContract<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[doc = "Container type for all input parameters for the `getProposalNonce` function with signature `getProposalNonce()` and selector `0x0b27fb9a`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getProposalNonce", abi = "getProposalNonce()")]
    pub struct GetProposalNonceCall;
    #[doc = "Container type for all input parameters for the `proposalNonce` function with signature `proposalNonce()` and selector `0xcc3c74a1`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "proposalNonce", abi = "proposalNonce()")]
    pub struct ProposalNonceCall;
    #[doc = "Container type for all input parameters for the `rescueTokens` function with signature `rescueTokens(address,address,uint256,uint32)` and selector `0x622c77d9`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "rescueTokens",
        abi = "rescueTokens(address,address,uint256,uint32)"
    )]
    pub struct RescueTokensCall {
        pub token_address: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount_to_rescue: ::ethers::core::types::U256,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `setHandler` function with signature `setHandler(address,uint32)` and selector `0x72c1ad03`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setHandler", abi = "setHandler(address,uint32)")]
    pub struct SetHandlerCall {
        pub new_handler: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all of the contract's call "]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum TreasuryContractCalls {
        GetProposalNonce(GetProposalNonceCall),
        ProposalNonce(ProposalNonceCall),
        RescueTokens(RescueTokensCall),
        SetHandler(SetHandlerCall),
    }
    impl ::ethers::core::abi::AbiDecode for TreasuryContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <GetProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <ProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <RescueTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RescueTokens(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetHandler(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TreasuryContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RescueTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TreasuryContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::GetProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RescueTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetProposalNonceCall> for TreasuryContractCalls {
        fn from(value: GetProposalNonceCall) -> Self {
            Self::GetProposalNonce(value)
        }
    }
    impl ::core::convert::From<ProposalNonceCall> for TreasuryContractCalls {
        fn from(value: ProposalNonceCall) -> Self {
            Self::ProposalNonce(value)
        }
    }
    impl ::core::convert::From<RescueTokensCall> for TreasuryContractCalls {
        fn from(value: RescueTokensCall) -> Self {
            Self::RescueTokens(value)
        }
    }
    impl ::core::convert::From<SetHandlerCall> for TreasuryContractCalls {
        fn from(value: SetHandlerCall) -> Self {
            Self::SetHandler(value)
        }
    }
    #[doc = "Container type for all return fields from the `getProposalNonce` function with signature `getProposalNonce()` and selector `0x0b27fb9a`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetProposalNonceReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `proposalNonce` function with signature `proposalNonce()` and selector `0xcc3c74a1`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProposalNonceReturn(pub ::ethers::core::types::U256);
}
