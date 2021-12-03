use advent2019;
use std::collections::VecDeque;

fn main() {
    let input = advent2019::load_input("09.txt");
    let mut ic = advent2019::intcode::Intcode::new(&input[0]);
    let mut input = VecDeque::<i64>::new();
    input.push_back(2);
    let mut output = VecDeque::<i64>::new();
    ic.run(&mut input, &mut output);
    println!("{:?}", output);
}
