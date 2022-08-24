pub use yearn_lens_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod yearn_lens_oracle {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "YearnLensOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static YEARNLENSORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_managementListAddress\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"address\", \"name\": \"_usdcAddress\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"tokenAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"tokenAliasAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"TokenAliasAdded\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"tokenAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"TokenAliasRemoved\",\n    \"type\": \"event\"\n  },\n  { \"stateMutability\": \"nonpayable\", \"type\": \"fallback\" },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"tokenAddress\", \"type\": \"address\" },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenAliasAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addTokenAlias\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"tokenAddress\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"tokenAliasAddress\",\n            \"type\": \"address\"\n          }\n        ],\n        \"internalType\": \"struct Oracle.TokenAlias[]\",\n        \"name\": \"_tokenAliases\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"addTokenAliases\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"calculations\",\n    \"outputs\": [\n      { \"internalType\": \"address[]\", \"name\": \"\", \"type\": \"address[]\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"tokenAddress\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"getNormalizedValueUsdc\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"tokenAddress\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"priceUsdc\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"getNormalizedValueUsdc\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"tokenAddress\", \"type\": \"address\" }\n    ],\n    \"name\": \"getPriceUsdcRecommended\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"managementList\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ManagementList\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"tokenAddress\", \"type\": \"address\" }\n    ],\n    \"name\": \"removeTokenAlias\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"calculationAddresses\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"setCalculations\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"name\": \"tokenAliases\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"usdcAddress\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    pub struct YearnLensOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for YearnLensOracle<M> {
        fn clone(&self) -> Self {
            YearnLensOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for YearnLensOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for YearnLensOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(YearnLensOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> YearnLensOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), YEARNLENSORACLE_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `addTokenAlias` (0x6a4cb36c) function"]
        pub fn add_token_alias(
            &self,
            token_address: ethers::core::types::Address,
            token_alias_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 76, 179, 108], (token_address, token_alias_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addTokenAliases` (0x88dea52a) function"]
        pub fn add_token_aliases(
            &self,
            token_aliases: ::std::vec::Vec<TokenAlias>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 222, 165, 42], token_aliases)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculations` (0x3d71473b) function"]
        pub fn calculations(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([61, 113, 71, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNormalizedValueUsdc` (0x8a7f6680) function"]
        pub fn get_normalized_value_usdc(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([138, 127, 102, 128], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNormalizedValueUsdc` (0xd02d20aa) function"]
        pub fn get_normalized_value_usdc_with_price_usdc(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            price_usdc: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([208, 45, 32, 170], (token_address, amount, price_usdc))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriceUsdcRecommended` (0x482ba306) function"]
        pub fn get_price_usdc_recommended(
            &self,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([72, 43, 163, 6], token_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `managementList` (0x58443a3b) function"]
        pub fn management_list(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([88, 68, 58, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeTokenAlias` (0xf3ae3077) function"]
        pub fn remove_token_alias(
            &self,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 174, 48, 119], token_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setCalculations` (0x255aacf1) function"]
        pub fn set_calculations(
            &self,
            calculation_addresses: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 90, 172, 241], calculation_addresses)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenAliases` (0x41394ced) function"]
        pub fn token_aliases(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([65, 57, 76, 237], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `usdcAddress` (0x02d45457) function"]
        pub fn usdc_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([2, 212, 84, 87], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `TokenAliasAdded` event"]
        pub fn token_alias_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenAliasAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenAliasRemoved` event"]
        pub fn token_alias_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenAliasRemovedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, YearnLensOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for YearnLensOracle<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "TokenAliasAdded", abi = "TokenAliasAdded(address,address)")]
    pub struct TokenAliasAddedFilter {
        pub token_address: ethers::core::types::Address,
        pub token_alias_address: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "TokenAliasRemoved", abi = "TokenAliasRemoved(address)")]
    pub struct TokenAliasRemovedFilter {
        pub token_address: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum YearnLensOracleEvents {
        TokenAliasAddedFilter(TokenAliasAddedFilter),
        TokenAliasRemovedFilter(TokenAliasRemovedFilter),
    }
    impl ethers::contract::EthLogDecode for YearnLensOracleEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = TokenAliasAddedFilter::decode_log(log) {
                return Ok(YearnLensOracleEvents::TokenAliasAddedFilter(decoded));
            }
            if let Ok(decoded) = TokenAliasRemovedFilter::decode_log(log) {
                return Ok(YearnLensOracleEvents::TokenAliasRemovedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for YearnLensOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                YearnLensOracleEvents::TokenAliasAddedFilter(element) => element.fmt(f),
                YearnLensOracleEvents::TokenAliasRemovedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addTokenAlias` function with signature `addTokenAlias(address,address)` and selector `[106, 76, 179, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addTokenAlias", abi = "addTokenAlias(address,address)")]
    pub struct AddTokenAliasCall {
        pub token_address: ethers::core::types::Address,
        pub token_alias_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addTokenAliases` function with signature `addTokenAliases((address,address)[])` and selector `[136, 222, 165, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addTokenAliases", abi = "addTokenAliases((address,address)[])")]
    pub struct AddTokenAliasesCall {
        pub token_aliases: ::std::vec::Vec<TokenAlias>,
    }
    #[doc = "Container type for all input parameters for the `calculations` function with signature `calculations()` and selector `[61, 113, 71, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "calculations", abi = "calculations()")]
    pub struct CalculationsCall;
    #[doc = "Container type for all input parameters for the `getNormalizedValueUsdc` function with signature `getNormalizedValueUsdc(address,uint256)` and selector `[138, 127, 102, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "getNormalizedValueUsdc",
        abi = "getNormalizedValueUsdc(address,uint256)"
    )]
    pub struct GetNormalizedValueUsdcCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getNormalizedValueUsdc` function with signature `getNormalizedValueUsdc(address,uint256,uint256)` and selector `[208, 45, 32, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "getNormalizedValueUsdc",
        abi = "getNormalizedValueUsdc(address,uint256,uint256)"
    )]
    pub struct GetNormalizedValueUsdcWithPriceUsdcCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub price_usdc: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getPriceUsdcRecommended` function with signature `getPriceUsdcRecommended(address)` and selector `[72, 43, 163, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "getPriceUsdcRecommended",
        abi = "getPriceUsdcRecommended(address)"
    )]
    pub struct GetPriceUsdcRecommendedCall {
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `managementList` function with signature `managementList()` and selector `[88, 68, 58, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "managementList", abi = "managementList()")]
    pub struct ManagementListCall;
    #[doc = "Container type for all input parameters for the `removeTokenAlias` function with signature `removeTokenAlias(address)` and selector `[243, 174, 48, 119]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeTokenAlias", abi = "removeTokenAlias(address)")]
    pub struct RemoveTokenAliasCall {
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setCalculations` function with signature `setCalculations(address[])` and selector `[37, 90, 172, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setCalculations", abi = "setCalculations(address[])")]
    pub struct SetCalculationsCall {
        pub calculation_addresses: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `tokenAliases` function with signature `tokenAliases(address)` and selector `[65, 57, 76, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tokenAliases", abi = "tokenAliases(address)")]
    pub struct TokenAliasesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `usdcAddress` function with signature `usdcAddress()` and selector `[2, 212, 84, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "usdcAddress", abi = "usdcAddress()")]
    pub struct UsdcAddressCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum YearnLensOracleCalls {
        AddTokenAlias(AddTokenAliasCall),
        AddTokenAliases(AddTokenAliasesCall),
        Calculations(CalculationsCall),
        GetNormalizedValueUsdc(GetNormalizedValueUsdcCall),
        GetNormalizedValueUsdcWithPriceUsdc(GetNormalizedValueUsdcWithPriceUsdcCall),
        GetPriceUsdcRecommended(GetPriceUsdcRecommendedCall),
        ManagementList(ManagementListCall),
        RemoveTokenAlias(RemoveTokenAliasCall),
        SetCalculations(SetCalculationsCall),
        TokenAliases(TokenAliasesCall),
        UsdcAddress(UsdcAddressCall),
    }
    impl ethers::core::abi::AbiDecode for YearnLensOracleCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddTokenAliasCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnLensOracleCalls::AddTokenAlias(decoded));
            }
            if let Ok(decoded) =
                <AddTokenAliasesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnLensOracleCalls::AddTokenAliases(decoded));
            }
            if let Ok(decoded) =
                <CalculationsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnLensOracleCalls::Calculations(decoded));
            }
            if let Ok(decoded) =
                <GetNormalizedValueUsdcCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnLensOracleCalls::GetNormalizedValueUsdc(decoded));
            }
            if let Ok(decoded) =
                <GetNormalizedValueUsdcWithPriceUsdcCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(YearnLensOracleCalls::GetNormalizedValueUsdcWithPriceUsdc(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetPriceUsdcRecommendedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnLensOracleCalls::GetPriceUsdcRecommended(decoded));
            }
            if let Ok(decoded) =
                <ManagementListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnLensOracleCalls::ManagementList(decoded));
            }
            if let Ok(decoded) =
                <RemoveTokenAliasCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnLensOracleCalls::RemoveTokenAlias(decoded));
            }
            if let Ok(decoded) =
                <SetCalculationsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnLensOracleCalls::SetCalculations(decoded));
            }
            if let Ok(decoded) =
                <TokenAliasesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnLensOracleCalls::TokenAliases(decoded));
            }
            if let Ok(decoded) =
                <UsdcAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnLensOracleCalls::UsdcAddress(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for YearnLensOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                YearnLensOracleCalls::AddTokenAlias(element) => element.encode(),
                YearnLensOracleCalls::AddTokenAliases(element) => element.encode(),
                YearnLensOracleCalls::Calculations(element) => element.encode(),
                YearnLensOracleCalls::GetNormalizedValueUsdc(element) => element.encode(),
                YearnLensOracleCalls::GetNormalizedValueUsdcWithPriceUsdc(element) => {
                    element.encode()
                }
                YearnLensOracleCalls::GetPriceUsdcRecommended(element) => element.encode(),
                YearnLensOracleCalls::ManagementList(element) => element.encode(),
                YearnLensOracleCalls::RemoveTokenAlias(element) => element.encode(),
                YearnLensOracleCalls::SetCalculations(element) => element.encode(),
                YearnLensOracleCalls::TokenAliases(element) => element.encode(),
                YearnLensOracleCalls::UsdcAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for YearnLensOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                YearnLensOracleCalls::AddTokenAlias(element) => element.fmt(f),
                YearnLensOracleCalls::AddTokenAliases(element) => element.fmt(f),
                YearnLensOracleCalls::Calculations(element) => element.fmt(f),
                YearnLensOracleCalls::GetNormalizedValueUsdc(element) => element.fmt(f),
                YearnLensOracleCalls::GetNormalizedValueUsdcWithPriceUsdc(element) => {
                    element.fmt(f)
                }
                YearnLensOracleCalls::GetPriceUsdcRecommended(element) => element.fmt(f),
                YearnLensOracleCalls::ManagementList(element) => element.fmt(f),
                YearnLensOracleCalls::RemoveTokenAlias(element) => element.fmt(f),
                YearnLensOracleCalls::SetCalculations(element) => element.fmt(f),
                YearnLensOracleCalls::TokenAliases(element) => element.fmt(f),
                YearnLensOracleCalls::UsdcAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddTokenAliasCall> for YearnLensOracleCalls {
        fn from(var: AddTokenAliasCall) -> Self {
            YearnLensOracleCalls::AddTokenAlias(var)
        }
    }
    impl ::std::convert::From<AddTokenAliasesCall> for YearnLensOracleCalls {
        fn from(var: AddTokenAliasesCall) -> Self {
            YearnLensOracleCalls::AddTokenAliases(var)
        }
    }
    impl ::std::convert::From<CalculationsCall> for YearnLensOracleCalls {
        fn from(var: CalculationsCall) -> Self {
            YearnLensOracleCalls::Calculations(var)
        }
    }
    impl ::std::convert::From<GetNormalizedValueUsdcCall> for YearnLensOracleCalls {
        fn from(var: GetNormalizedValueUsdcCall) -> Self {
            YearnLensOracleCalls::GetNormalizedValueUsdc(var)
        }
    }
    impl ::std::convert::From<GetNormalizedValueUsdcWithPriceUsdcCall> for YearnLensOracleCalls {
        fn from(var: GetNormalizedValueUsdcWithPriceUsdcCall) -> Self {
            YearnLensOracleCalls::GetNormalizedValueUsdcWithPriceUsdc(var)
        }
    }
    impl ::std::convert::From<GetPriceUsdcRecommendedCall> for YearnLensOracleCalls {
        fn from(var: GetPriceUsdcRecommendedCall) -> Self {
            YearnLensOracleCalls::GetPriceUsdcRecommended(var)
        }
    }
    impl ::std::convert::From<ManagementListCall> for YearnLensOracleCalls {
        fn from(var: ManagementListCall) -> Self {
            YearnLensOracleCalls::ManagementList(var)
        }
    }
    impl ::std::convert::From<RemoveTokenAliasCall> for YearnLensOracleCalls {
        fn from(var: RemoveTokenAliasCall) -> Self {
            YearnLensOracleCalls::RemoveTokenAlias(var)
        }
    }
    impl ::std::convert::From<SetCalculationsCall> for YearnLensOracleCalls {
        fn from(var: SetCalculationsCall) -> Self {
            YearnLensOracleCalls::SetCalculations(var)
        }
    }
    impl ::std::convert::From<TokenAliasesCall> for YearnLensOracleCalls {
        fn from(var: TokenAliasesCall) -> Self {
            YearnLensOracleCalls::TokenAliases(var)
        }
    }
    impl ::std::convert::From<UsdcAddressCall> for YearnLensOracleCalls {
        fn from(var: UsdcAddressCall) -> Self {
            YearnLensOracleCalls::UsdcAddress(var)
        }
    }
    #[doc = "Container type for all return fields from the `calculations` function with signature `calculations()` and selector `[61, 113, 71, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CalculationsReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getNormalizedValueUsdc` function with signature `getNormalizedValueUsdc(address,uint256)` and selector `[138, 127, 102, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNormalizedValueUsdcReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getNormalizedValueUsdc` function with signature `getNormalizedValueUsdc(address,uint256,uint256)` and selector `[208, 45, 32, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNormalizedValueUsdcWithPriceUsdcReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPriceUsdcRecommended` function with signature `getPriceUsdcRecommended(address)` and selector `[72, 43, 163, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPriceUsdcRecommendedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `managementList` function with signature `managementList()` and selector `[88, 68, 58, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ManagementListReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `tokenAliases` function with signature `tokenAliases(address)` and selector `[65, 57, 76, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TokenAliasesReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `usdcAddress` function with signature `usdcAddress()` and selector `[2, 212, 84, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UsdcAddressReturn(pub ethers::core::types::Address);
    #[doc = "`TokenAlias(address,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TokenAlias {
        pub token_address: ethers::core::types::Address,
        pub token_alias_address: ethers::core::types::Address,
    }
}
