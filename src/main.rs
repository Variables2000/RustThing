fn main() {
    println!("Hello World!");
    update();
    let a:i128 = 256;
    println!("{} + 1 = {}", a , add_one(a));
    update_two();
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