#[cfg(test)]
mod tests {

    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        DEFAULT_RUN_GENESIS_REQUEST,
    };
    use casper_execution_engine::{
        core::{engine_state::Error as CoreError, execution::Error as ExecError},
        storage::global_state::in_memory::InMemoryGlobalState,
    };
    use casper_types::{runtime_args, system::mint, ApiError, ContractHash, RuntimeArgs, U512};

    const CONTRACT_WASM: &str = "contract.wasm";
    const GOOD_WASM: &str = "good.wasm";
    const BAD_WASM: &str = "bad.wasm";

    fn deploy_market_contract(builder: &mut WasmTestBuilder<InMemoryGlobalState>) -> ContractHash {
        let exec_request_1 =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, CONTRACT_WASM, runtime_args! {})
                .build();

        builder
            .run_genesis(&DEFAULT_RUN_GENESIS_REQUEST)
            .exec(exec_request_1)
            .expect_success()
            .commit();
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account");

        let contract_hash = account
            .named_keys()
            .get("contract1")
            .expect("should have contract1")
            .into_hash()
            .map(ContractHash::new)
            .expect("should be contracthash");

        contract_hash
    }
    #[test]
    fn good() {
        let mut builder = InMemoryWasmTestBuilder::default();

        let contract_hash = deploy_market_contract(&mut builder);

        let exec_request_2 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            GOOD_WASM,
            runtime_args! {
                "amount" => U512::from(100000u64),
                "marketplace_contract" => contract_hash,
            },
        )
        .build();

        builder.exec(exec_request_2).expect_success().commit();
    }

    #[test]
    fn bad() {
        let mut builder = InMemoryWasmTestBuilder::default();

        let contract_hash = deploy_market_contract(&mut builder);

        let exec_request_2 = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            BAD_WASM,
            runtime_args! {
                "amount" => U512::from(100000u64),
                "marketplace_contract" => contract_hash,
            },
        )
        .build();

        builder.exec(exec_request_2);

        let error = builder.get_error().expect("should have returned an error");

        assert!(
            matches!(error, CoreError::Exec(ExecError::Revert(ApiError::Mint(
            auction_error,
        ))) if auction_error == mint::Error::InvalidAccessRights as u8)
        );
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
