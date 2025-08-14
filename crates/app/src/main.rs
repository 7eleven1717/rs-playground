mod bindings {
    wit_bindgen::generate!();

    use super::HelloComponent;
    export!(HelloComponent);
}

use bindings::host::call_host;

struct HelloComponent;

impl bindings::exports::guest::Guest for HelloComponent {
    fn call_guest() -> () {
        println!("Hello from the guest component!");
    }
}

fn main() {
    call_host();
}
