#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
struct pos {
    x: i64,
    y: i64
}
struct pos3d {
    x: i64,
    y: i64,
    z: i64
}
fn main() {
    println!("Hello World!");
    update();
    let a:i128 = 256;
    println!("{} + 1 = {}", a , add_one(a));
    update_two();
    let _position = pos {x: 32, y: 43};
    let _pos3d = pos3d {x: 64, y: 72, z: 12};

}
fn update() {
    let my_name: &str = "Twelvetican";
    let my_rating: f32 = 4.7;
    let is_liking: bool = true;
    println!("My name is: {}", my_name);
    println!("My rating is: {}", my_rating);
    println!("I like this: {}", is_liking);
}
fn add_one(a: i128) -> i128 {
    return a+1;
}
fn update_two() {
    let my_dream: &str = "be good at coding";
    println!("My dream is to {}", my_dream);
}