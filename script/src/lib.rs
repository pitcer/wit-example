wit_bindgen::generate!({
    path: "../wit",

    exports: {
        world: Example,
    }
});

struct Example;

impl Guest for Example {
    fn run() -> Result<(), ()> {
        let time = runtime::get_current_time();
        runtime::print(format!("time: {time:?}").as_str());
        let fibonacci = fibonacci(32);
        let fibonacci_time = runtime::get_current_time();
        runtime::print(format!("Fibonacci: {fibonacci}, time: {fibonacci_time:?}").as_str());
        Ok(())
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
