pub struct LogExecution;

impl sp_wasm_interface::Function for LogExecution {
	fn name(&self) -> &str {
		"log_execution"
	}

	fn signature(&self) -> sp_wasm_interface::Signature {
		sp_wasm_interface::Signature::new_with_args(&[sp_wasm_interface::ValueType::I32][..])
	}

	fn execute(
		&self,
		_: &mut dyn sp_wasm_interface::FunctionContext,
		args: &mut dyn Iterator<Item = sp_wasm_interface::Value>,
	) -> Result<Option<sp_wasm_interface::Value>, String> {
		let number = args.next().unwrap().as_i32().unwrap();
		println!("LOG_EXECUTION: {}", number);
		Ok(None)
	}
}

impl sp_wasm_interface::HostFunctions for LogExecution {
	fn host_functions() -> Vec<&'static dyn sp_wasm_interface::Function> {
		vec![&LogExecution]
	}
}

fn dispatch(_: &str, _: &[u8]) -> Option<Vec<u8>> {
	unimplemented!("Not required")
}

fn native_version() -> sp_version::NativeVersion {
	sp_version::NativeVersion {
		runtime_version: Default::default(),
		can_author_with: Default::default(),
	}
}

sc_executor::native_executor_instance!(
	pub Executor,
	dispatch,
	native_version,
	LogExecution,
);

fn main() {
	let mut builder = env_logger::Builder::from_default_env();
	builder.parse_filters("runtime=debug");
	builder.init();

	let wasm_file_path = std::env::args().nth(1).expect("Please give wasm file!");
	let wasm_file = std::fs::read(wasm_file_path).expect("Read wasm file");

	let fetcher = sp_core::traits::WrappedRuntimeCode(wasm_file.into());
	let runtime_code = sp_core::traits::RuntimeCode {
		code_fetcher: &fetcher,
		heap_pages: None,
		hash: Vec::new(),
	};
	let spawner = sp_core::testing::TaskExecutor::new();
	let mut overlay = sp_state_machine::OverlayedChanges::default();
	let executor = sc_executor::NativeExecutor::<Executor>::new(Default::default(), None, 10);

	let res = sp_state_machine::execution_proof_check::<sp_runtime::traits::BlakeTwo256, u32, _, _>(
		codec::Decode::decode(&mut &include_bytes!("../res/storage_root")[..]).unwrap(),
		codec::Decode::decode(&mut &include_bytes!("../res/proof")[..]).unwrap(),
		&mut overlay,
		&executor,
		spawner,
		"validate_block",
		&include_bytes!("../res/validation_params")[..],
		&runtime_code,
	);
	println!("RES: {:?}", res);
}
