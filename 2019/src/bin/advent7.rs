use advent2019;
use advent2019::intcode::Intcode;

use std::cell::Cell;
use std::collections::VecDeque;

struct Permutation {
    stack: Vec<u32>,
    i: usize,
    permutation: Vec<u32>,
    first_run: bool,
}

impl Permutation {
    pub fn new(vector: Vec<u32>) -> Permutation {
        Permutation {
            stack: vec![0; vector.len()],
            i: 0,
            permutation: vector,
            first_run: true,
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let tmp = self.permutation[a];
        self.permutation[a] = self.permutation[b];
        self.permutation[b] = tmp;
    }

    pub fn next(&mut self) -> bool {
        if self.first_run {
            self.first_run = false;
            return true;
        }
        // from wikipedia: https://en.wikipedia.org/wiki/Heap%27s_algorithm
        loop {
            if self.i >= self.stack.len() {
                return false;
            }
            if self.stack[self.i] as usize >= self.i {
                self.stack[self.i] = 0;
                self.i += 1;
            } else {
                break;
            }
        }

        if self.i % 2 == 0 {
            self.swap(0, self.i)
        } else {
            self.swap(self.stack[self.i] as usize, self.i)
        }
        self.stack[self.i] += 1;
        self.i = 0;
        true
    }

    pub fn perm(&self) -> &Vec<u32> {
        &self.permutation
    }
}

fn main() {
    let input = advent2019::load_input("07.txt");
    //let input = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");

    let mut p = Permutation::new((0..=4).collect());

    let mut max_output = 0;
    let mut sequence_vector = vec![0; 5];
    let mut input_vector = VecDeque::new();
    let mut output_vector = VecDeque::new();

    while p.next() {
        output_vector.push_back(0);
        for phase in p.perm() {
            input_vector.push_back(*phase as i32);
            input_vector.push_back(output_vector.pop_back().unwrap());
            let mut machine = Intcode::new(&input[0]);
            machine.run(&mut input_vector, &mut output_vector);
        }

        let final_output = output_vector.pop_back().unwrap();
        if final_output > max_output {
            sequence_vector.copy_from_slice(p.perm());
            max_output = final_output;
        }
    }
    println!("maximum output: {}", max_output);
    println!("using sequence: {:?}", sequence_vector);

    println!("======== LOOPBACK MODE ==========");
    p = Permutation::new((5..=9).collect());
    max_output = 0;
    while p.next() {
        let mut machines = (0..5).map(|_| Intcode::new(&input[0]))
            .collect::<Vec<Intcode>>();
        let mut buffers: Vec<Cell<Option<Box<VecDeque<i32>>>>> = p.perm().iter()
            .map(|x| {
                let mut vd: VecDeque<i32> = VecDeque::new();
                vd.push_back(*x as i32);
                Cell::new(Some(Box::new(vd)))
            })
            .collect();
        buffers[0].get_mut().as_mut().map(|b| b.push_back(0));
        let mut continue_loop = true;
        while continue_loop {
            for i in 0..5 {
                let machine = &mut machines[i];
                let next_i = (i + 1) % 5;
                
                let mut input = buffers[i].take().unwrap();
                let mut output = buffers[next_i].take().unwrap();
                if !machine.run(&mut input, &mut output) {
                    continue_loop = false;
                }
                buffers[i].set(Some(input));
                buffers[next_i].set(Some(output));
            }
        }
        let mut last_buffer = buffers[0].take().unwrap();
        let final_output = last_buffer.pop_back().unwrap();
        if final_output > max_output {
            max_output = final_output;
        }
    }
    println!("Maximum output: {}", max_output);
}
