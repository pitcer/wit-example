wit_bindgen::generate!({
    path: "../wit",

    exports: {
        world: Example,
    }
});

struct Example;

impl Guest for Example {
    fn run() -> Result<(), ()> {
        let mut time = runtime::get_current_time();
        time.seconds += 1;
        time.milliseconds /= 2;
        runtime::print(format!("{time:?}").as_str());
        Ok(())
    }
}
