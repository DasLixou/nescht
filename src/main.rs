use nescht::prelude::*;

fn main() {
    let instances = wgpu::Instance::new(wgpu::Backends::all());
    for adapter in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }

    Game::create("Nescht Demo", 1280, 720)
        .start(update, shutdown);
}

fn update() {
    println!("UPDATE");
}

fn shutdown() {
    println!("peepoBye");
}