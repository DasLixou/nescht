mod surface;

use crate::surface::Surface;

fn main() {
    let surface = Surface::create("Nescht Demo", 1280, 720);
    surface.start(update, shutdown);
}

fn update() {
    println!("UPDATE");
}

fn shutdown() {
    println!("peepoBye");
}