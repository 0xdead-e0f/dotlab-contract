use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Addr, Binary, Coin, ContractResult, Decimal, OwnedDeps,
    Querier, QuerierResult, QueryRequest, StdError, StdResult, SystemError, SystemResult, Uint128,
    WasmQuery, Empty,
};
use dotlabs::registrar::{
    GetBaseNodeResponse, GetRegistryResponse, IsAvailableResponse, QueryMsg as RegistrarQueryMsg,
};
use schemars::JsonSchema;
use sei_cosmwasm::SeiQueryWrapper;
use std::collections::HashMap;
/// mock_dependencies is a drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// this uses our CustomQuerier.
pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier =
        WasmMockQuerier::new(MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: std::marker::PhantomData,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<SeiQueryWrapper>,
    // tax_querier: TaxQuerier,
}

// #[derive(Clone, Default)]
// pub struct TaxQuerier {
//     rate: Decimal,
//     // this lets us iterate over all pairs that match the first string
//     caps: HashMap<String, Uint128>,
// }

// impl TaxQuerier {
//     pub fn new(rate: Decimal, caps: &[(&String, &Uint128)]) -> Self {
//         TaxQuerier {
//             rate,
//             caps: caps_to_map(caps),
//         }
//     }
// }

pub(crate) fn caps_to_map(caps: &[(&String, &Uint128)]) -> HashMap<String, Uint128> {
    let mut owner_map: HashMap<String, Uint128> = HashMap::new();
    for (denom, cap) in caps.iter() {
        owner_map.insert(denom.to_string(), **cap);
    }
    owner_map
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<SeiQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<SeiQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: _,
                msg,
            }) => match from_binary::<RegistrarQueryMsg<Empty>>(msg) {
                Ok(RegistrarQueryMsg::GetRegistry {}) => SystemResult::Ok(ContractResult::Ok(
                    to_binary(&GetRegistryResponse {
                        registry: Addr::unchecked("registry_address"),
                    })
                    .unwrap(),
                )),
                Ok(RegistrarQueryMsg::GetBaseNode {}) => SystemResult::Ok(ContractResult::Ok(
                    to_binary(&GetBaseNodeResponse {
                        base_node: String::from(
                            "749f2b479b45e5da8e4cbecd926ee9a6f78db5424fa6993b6ecababa5d736b12",
                        ),
                    })
                    .unwrap(),
                )),
                Ok(RegistrarQueryMsg::IsAvailable { .. }) => SystemResult::Ok(ContractResult::Ok(
                    to_binary(&IsAvailableResponse { available: true }).unwrap(),
                )),
                _ => {
                    panic!("DO NOT ENTER HERE")
                }
                Err(_) => panic!("DO NOT ENTER HERE"),
            },
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    pub fn new(base: MockQuerier<SeiQueryWrapper>) -> Self {
        WasmMockQuerier {
            base,
            // tax_querier: TaxQuerier::default(),
        }
    }
}
