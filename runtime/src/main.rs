use std::env;
use std::time::Instant;

use anyhow::{anyhow, Result};
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

use crate::runtime::{Host, Time};

bindgen!({
    path: "../wit/script.wit",
    // Required by WASI probably
    async: true,
});

#[pollster::main]
async fn main() -> Result<()> {
    let component_file = env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing script name"))?;
    let now = Instant::now();

    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, component_file)?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::preview2::command::add_to_linker(&mut linker)?;
    Example::add_to_linker(&mut linker, |state: &mut State| state)?;

    let mut wasi_table = Table::new();
    let wasi_context = WasiCtxBuilder::new().build(&mut wasi_table)?;
    let mut store = Store::new(
        &engine,
        State {
            start_time: now,
            wasi_table,
            wasi_context,
        },
    );
    let (bindings, _) = Example::instantiate_async(&mut store, &component, &linker).await?;

    let result = bindings.call_run(&mut store).await?;
    result.expect("script should exit with ok result");
    Ok(())
}

struct State {
    start_time: Instant,
    wasi_table: Table,
    wasi_context: WasiCtx,
}

#[async_trait::async_trait]
impl Host for State {
    async fn get_current_time(&mut self) -> Result<Time> {
        let now = Instant::now();
        let duration = now.duration_since(self.start_time);
        Ok(Time {
            seconds: duration.as_secs(),
            milliseconds: duration.subsec_millis(),
        })
    }

    async fn print(&mut self, message: String) -> Result<()> {
        println!("{message}");
        Ok(())
    }
}

impl WasiView for State {
    fn table(&self) -> &Table {
        &self.wasi_table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.wasi_table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.wasi_context
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_context
    }
}
