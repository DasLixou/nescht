use nescht::prelude::*;

fn main() {
    Game::create("Nescht Demo", 1280, 720)
        .start(update, shutdown);
}

fn update() {
    println!("UPDATE");
}

fn shutdown() {
    println!("peepoBye");
}