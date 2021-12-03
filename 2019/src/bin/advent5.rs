use advent2019::intcode::Intcode;

fn main() {
    let input = advent2019::load_input("05.txt");
    let mut ic = Intcode::new(&input[0]);
    let mut output: Vec<i32> = vec![];
    let res = ic.run(&vec![5], &mut output);
    println!("{} {:?}", res, output);
}
