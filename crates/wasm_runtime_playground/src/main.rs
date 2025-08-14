// https://github.com/bytecodealliance/wasmtime/tree/7380932631f7784d944cb0326a6ffaaf5dac29fc/examples/component

// use anyhow;
use wasmtime::component::{Component, HasSelf, Linker, ResourceTable};
use wasmtime::*;
use wasmtime_wasi::p2::bindings::sync::Command;
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};

component::bindgen!("app" in "../app/wit");

pub struct ComponentRunStates {
    // These two are required basically as a standard way to enable the impl of IoView and
    // WasiView.
    // impl of WasiView is required by [`wasmtime_wasi::p2::add_to_linker_sync`]
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
    // You can add other custom host states if needed
    pub custom_host_state: CustomHostState,
}

pub struct CustomHostState {
    value: u32,
}

impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

impl host::Host for ComponentRunStates {
    fn call_host(&mut self) -> () {
        self.custom_host_state.value += 1;
        println!("Host function called! value={}", self.custom_host_state.value);
    }
}

fn main() -> wasmtime::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

    host::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;

    let args = ["foo", "bar"];
    let state = ComponentRunStates {
        wasi_ctx: WasiCtx::builder().inherit_stdout().args(&args).build(),
        resource_table: ResourceTable::new(),

        custom_host_state: CustomHostState { value: 42 },
    };

    let mut store = Store::new(&engine, state);
    let component: Component =
        Component::from_file(&engine, "./target/wasm32-wasip2/debug/app.wasm")?;
    let command = Command::instantiate(&mut store, &component, &linker)?;
    let program_result = command.wasi_cli_run().call_run(&mut store)?;
    if program_result.is_err() {
        std::process::exit(1)
    }

    Ok(())
}
