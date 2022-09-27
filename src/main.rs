use nescht::prelude::*;

fn main() {
    let instances = wgpu::Instance::new(wgpu::Backends::all());
    for adapter in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }

    let scheduler = Scheduler::new(20, update, 60, render, shutdown);
    let game = Game::create(scheduler, "Nescht Demo", 1280, 720);

    #[cfg(not(target_os = "linux"))]
    {
        nescht::window::start(game);
    }
    #[cfg(target_os = "linux")]
    {
        nescht::window::start(game);
    }
}

fn update() {
    println!("UPDATE");
}

fn render() {
    println!("RENDER");
}

fn shutdown() {
    println!("peepoBye");
}
