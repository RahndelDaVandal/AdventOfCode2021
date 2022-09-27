use day01::{count_increase, load_input};

fn main() {
    let data = load_input();
    println!("Input: {:?}\n", data);
    println!("{} measurements greater than previous measurements", count_increase(data));
}
