use std::time::Instant;

use anyhow::Result;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

use crate::runtime::{Host, Time};

bindgen!(in "../wit" );

fn main() -> Result<()> {
    let now = Instant::now();

    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "script-component.wasm")?;

    let mut linker = Linker::new(&engine);
    Example::add_to_linker(&mut linker, |state: &mut State| state)?;

    let mut store = Store::new(&engine, State { start_time: now });
    let (bindings, _) = Example::instantiate(&mut store, &component, &linker)?;

    let result = bindings.call_run(&mut store)?;
    result.expect("script should exit with ok result");
    Ok(())
}

struct State {
    start_time: Instant,
}

impl Host for State {
    fn get_current_time(&mut self) -> Result<Time> {
        let now = Instant::now();
        let duration = now.duration_since(self.start_time);
        Ok(Time {
            seconds: duration.as_secs(),
            milliseconds: duration.subsec_millis(),
        })
    }

    fn print(&mut self, message: String) -> Result<()> {
        println!("{message}");
        Ok(())
    }
}
