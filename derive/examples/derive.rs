use derive::Hello;

trait Hello {
    fn say_hello(&self);
}

#[derive(Hello)]
struct World;

// impl Hello for World {
//     fn say_hello(&self) {
//         println!("Hello rustaceans from World");
//     }
// }

fn main() {
    // Derive procedural macro
    //
    let world = World {};
    world.say_hello();
}
