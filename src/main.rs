extern crate cgmath;

mod mesh;

use mesh::Mesh;

fn main() {
    println!("Hello, world!");

    let mut m = Mesh::new();
    m.load_obj("/Users/jeremy/Repositories/dust3d/gourd.obj");
    println!("Mesh debug info: {:?}", m);
}
