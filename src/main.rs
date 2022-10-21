
pub mod renderer;

use renderer::Renderer;

fn main() {
    let mut r = Renderer::initialize(1024, 768, "Hello");

    while r.update() {
        // do something
    }

    println!("Shutting down engine!");

    r.shutdown();

}
