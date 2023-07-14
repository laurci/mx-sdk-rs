#![allow(non_snake_case)]

use rust_snippets_generator_test::ProxyTrait as _;
use rust_snippets_generator_test::*;

use multiversx_sc_snippets::{
    env_logger,
    erdrs::wallet::Wallet,
    multiversx_sc::{codec::multi_types::*, types::*},
    multiversx_sc_scenario::{
        api::StaticApi,
        bech32,
        scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext},
        scenario_model::*,
        ContractInfo,
    },
    sdk, tokio, Interactor,
};


const GATEWAY: &str = sdk::blockchain::DEVNET_GATEWAY;
const PEM: &str = "alice.pem";
const SC_ADDRESS: &str = "";

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const DEFAULT_ADDRESS_EXPR: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
const TOKEN_ISSUE_COST: u64 = 50_000_000_000_000_000;

type ContractType = ContractInfo<rust_snippets_generator_test::Proxy<StaticApi>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut state = State::new().await;
    match cmd.as_str() {
        "deploy" => state.deploy().await,
        "no_arg_no_result_endpoint" => state.no_arg_no_result_endpoint().await,
        "no_arg_one_result_endpoint" => state.no_arg_one_result_endpoint().await,
        "one_arg_no_result_endpoint" => state.one_arg_no_result_endpoint().await,
        "one_arg_one_result_endpoint" => state.one_arg_one_result_endpoint().await,
        "multi_result" => state.multi_result().await,
        "nested_result" => state.nested_result().await,
        "custom_struct" => state.custom_struct().await,
        "optional_type" => state.optional_type().await,
        "option_type" => state.option_type().await,
        "esdt_token_payment" => state.esdt_token_payment().await,
        "egld_or_esdt_payment" => state.egld_or_esdt_payment().await,
        "payable_endpoint" => state.payable_endpoint().await,
        "managed_buffer" => state.managed_buffer().await,
        "multi_value_2" => state.multi_value_2().await,
        "multi_value_4" => state.multi_value_4().await,
        "complex_multi_values" => state.complex_multi_values().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

struct State {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    contract: ContractType,
}

impl State {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let sc_addr_expr = if SC_ADDRESS == "" {
            DEFAULT_ADDRESS_EXPR.to_string()
        } else {
            "bech32:".to_string() + SC_ADDRESS
        };
        let contract_code = BytesValue::interpret_from(
            "file:../output/rust-snippets-generator-test.wasm",
            &InterpreterContext::default(),
        );
        let contract = ContractType::new(sc_addr_expr);

        State {
            interactor,
            wallet_address,
            contract_code,
            contract,
        }
    }

    async fn deploy(&mut self) {
        self.interactor
            .sc_deploy_use_result(
                ScDeployStep::new()
                    .call(self.contract.init())
                    .from(&self.wallet_address)
                    .code(&self.contract_code),
                |new_address, tr: TypedResponse<()>| {
                    tr.result.unwrap_or_else(|err| {
                        panic!(
                            "deploy failed: status: {}, message: {}",
                            err.status, err.message
                        )
                    });

                    let new_address_bech32 = bech32::encode(&new_address);
                    println!("new address: {new_address_bech32}");
                },
            )
            .await;
    }

    async fn no_arg_no_result_endpoint(&mut self) {
        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.no_arg_no_result_endpoint())
                    .from(&self.wallet_address),
                |tr: TypedResponse<()>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn no_arg_one_result_endpoint(&mut self) {
        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.no_arg_one_result_endpoint())
                    .from(&self.wallet_address),
                |tr: TypedResponse<u64>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn one_arg_no_result_endpoint(&mut self) {
        let _arg = 0u64;

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.one_arg_no_result_endpoint(_arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<()>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn one_arg_one_result_endpoint(&mut self) {
        let _arg = 0u64;

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.one_arg_one_result_endpoint(_arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<BigUint<StaticApi>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn multi_result(&mut self) {
        let _arg = TokenIdentifier::from_esdt_bytes(&b""[..]);

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.multi_result(_arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<MultiValueVec<BigUint<StaticApi>>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn nested_result(&mut self) {
        let _arg = TokenIdentifier::from_esdt_bytes(&b""[..]);

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.nested_result(_arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<ManagedVec<StaticApi, ManagedVec<StaticApi, BigUint<StaticApi>>>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn custom_struct(&mut self) {
        let _arg = PlaceholderInput;

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.custom_struct(_arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<MyCoolStruct<StaticApi>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn optional_type(&mut self) {
        let _arg = OptionalValue::Some(BigUint::<StaticApi>::from(0u128));

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.optional_type(_arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<OptionalValue<TokenIdentifier<StaticApi>>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn option_type(&mut self) {
        let _arg = Option::Some(ManagedVec::from_single_item(TokenIdentifier::from_esdt_bytes(&b""[..])));

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.option_type(_arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<Option<u64>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn esdt_token_payment(&mut self) {
        let _arg = OptionalValue::Some(EsdtTokenPayment::new(
            TokenIdentifier::from_esdt_bytes(&b""[..]),
            0u64,
            BigUint::from(0u128),
        ));

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.esdt_token_payment(_arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<EsdtTokenPayment<StaticApi>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn egld_or_esdt_payment(&mut self) {
        let arg = EgldOrEsdtTokenPayment::new(
            EgldOrEsdtTokenIdentifier::esdt(&b""[..]),
            0u64,
            BigUint::from(0u128),
        );

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.egld_or_esdt_payment(arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<EgldOrEsdtTokenIdentifier<StaticApi>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn payable_endpoint(&mut self) {
        let token_id = b"";
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.payable_endpoint())
                    .from(&self.wallet_address)
                    .esdt_transfer(token_id.to_vec(), token_nonce, token_amount),
                |tr: TypedResponse<()>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn managed_buffer(&mut self) {
        let _arg = Option::Some(ManagedBuffer::new_from_bytes(&b""[..]));

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.managed_buffer(_arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<MultiValueVec<ManagedVec<StaticApi, MyCoolStruct<StaticApi>>>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn multi_value_2(&mut self) {
        let arg = MultiValue2::from((0u64, BigUint::<StaticApi>::from(0u128)));

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.multi_value_2(arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<MultiValue2<u64, BigUint<StaticApi>>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn multi_value_4(&mut self) {
        let arg = PlaceholderInput;

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.multi_value_4(arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<MultiValue4<u64, BigUint<StaticApi>, MyCoolStruct<StaticApi>, TokenIdentifier<StaticApi>>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

    async fn complex_multi_values(&mut self) {
        let arg = MultiValueVec::from(vec![MultiValue3::from((TokenIdentifier::from_esdt_bytes(&b""[..]), 0u64, BigUint::<StaticApi>::from(0u128)))]);

        self.interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.complex_multi_values(arg))
                    .from(&self.wallet_address),
                |tr: TypedResponse<MultiValueVec<MultiValue3<TokenIdentifier<StaticApi>, u64, BigUint<StaticApi>>>>| {
                    match tr.result {
                        Ok(result) => {
                            println!("Result: {result:?}");
                        },
                        Err(err) => panic!(
                            "SC call failed: status: {}, message: {}",
                            err.status, err.message
                        ),
                    };
                },
            )
            .await;
    }

}
