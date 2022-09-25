use day01::{num_increased, load_input};

fn main() {
    let data = load_input();
    println!("Input: {:?}\n", data);
    println!("{} measurements greater than previous measurements", num_increased(data));
}
