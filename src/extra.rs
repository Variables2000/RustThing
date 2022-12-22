#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub fn gref() {
    println!("Deadbeat.")
}
pub fn sys_init() {
    let int:i64 = get_number();
    println!("{} was recorded.", int);
}
fn get_number() -> i64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let i: i64 = rng.gen_range(-65535..=65536);
    return i;
}