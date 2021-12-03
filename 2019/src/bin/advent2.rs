const SMALL_INPUT: &str = "1,1,1,4,99,5,6,0,99";

const DESIRED_OUTPUT: usize = 19690720;

use advent2019::intcode::Intcode;

fn main() {
    let input = advent2019::load_input("02.txt");
    let ic = Intcode::new(&input[0]);
    println!("{}", ic.run_copy(12, 2));
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = ic.run_copy(noun, verb);
            if result == DESIRED_OUTPUT {
                println!("{} {}", noun, verb)
            }
        }
    }
}
