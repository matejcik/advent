use advent2019;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn main() {
    let input = &advent2019::load_input("08.txt")[0];
    let mut idx = 0;
    let mut min_zeroes = LAYER_SIZE + 1;
    let mut mul_result = 0;
    while idx < input.len() {
        let mut digits = [0, 0, 0];
        let slice = &input[idx..idx + LAYER_SIZE];
        for c in slice.chars() {
            let digit_index = c as usize - '0' as usize;
            if digit_index > 2 { continue; }
            digits[digit_index] += 1;
        }
        if digits[0] < min_zeroes {
            min_zeroes = digits[0];
            mul_result = digits[1] * digits[2];
        }
        idx += LAYER_SIZE;
    }
    println!("{}", mul_result);

    let mut image_data = vec!['_'; LAYER_SIZE];
    idx = 0;
    while idx < input.len() {
        let slice = &input[idx..idx + LAYER_SIZE];
        for (i, ch) in (0..LAYER_SIZE).zip(slice.chars()) {
            if image_data[i] == '_' {
                image_data[i] = match ch {
                    '0' => 'X',
                    '1' => ' ',
                    '2' => '_',
                    _ => panic!("unrecognized character"),
                };
            }
        }
        idx += LAYER_SIZE;
    }

    for ptr in 0..LAYER_SIZE {
        if ptr % WIDTH == 0 {
            print!("\n");
        }
        print!("{}", image_data[ptr]);
    }
    print!("\n");
}
