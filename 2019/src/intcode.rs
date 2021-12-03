use num;
use std::collections::VecDeque;

#[derive(Clone, FromPrimitive)]
enum Mode {
    Pos = 0,
    Immed = 1,
    Rel = 2,
}

type MemoryItem = i64;
type InputOutput = VecDeque<MemoryItem>;

#[derive(Clone)]
pub struct Intcode {
    ip: usize,
    modes: Vec<Mode>,
    memory: Vec<MemoryItem>,
    rel_base: MemoryItem,
}

enum InstructionResult {
    Jmp(usize),
    StoreIn(usize),
    LoadOut(usize),
    Continue,
    Halt,
}

enum RegAccess {
    MemoryAt(usize),
    Value(MemoryItem),
}

use InstructionResult::{Jmp, Continue, StoreIn, LoadOut, Halt};

struct Instruction {
    args: usize,
    call: fn(&mut Intcode) -> InstructionResult,
}

fn get_instruction(op: u8) -> Result<Instruction, &'static str> {
    match op {
        1 => Ok(Instruction { /* ADD */
            args: 3,
            call: |ic| ic.store(2, ic.load(0) + ic.load(1))
        }),
        2 => Ok(Instruction { /* MUL */
            args: 3,
            call: |ic| ic.store(2, ic.load(0) * ic.load(1))
        }),
        3 => Ok(Instruction { /* IN */
            args: 1,
            call: |_ic| StoreIn(0)
        }),
        4 => Ok(Instruction { /* OUT */
            args: 1,
            call: |_ic| LoadOut(0)
        }),
        5 => Ok(Instruction { /* JNZ */
            args: 2,
            call: |ic| if ic.load(0) != 0 {
                let dst = ic.load(1);
                if dst < 0 { panic!("negative JNZ destination") }
                Jmp(dst as usize)
            } else { Continue }
        }),
        6 => Ok(Instruction { /* JZ */
            args: 2,
            call: |ic| if ic.load(0) == 0 {
                let dst = ic.load(1);
                if dst < 0 { panic!("negative JZ destination") }
                Jmp(dst as usize)
            } else { Continue }
        }),
        7 => Ok(Instruction { /* LT */
            args: 3,
            call: |ic| ic.store(2, if ic.load(0) < ic.load(1) { 1 } else { 0 })
        }),
        8 => Ok(Instruction { /* EQ */
            args: 3,
            call: |ic| ic.store(2, if ic.load(0) == ic.load(1) { 1 } else { 0 })
        }),
        9 => Ok(Instruction { /* BASE */
            args: 1,
            call: |ic| { ic.rel_base += ic.load(0); Continue }
        }),
        99 => Ok(Instruction { /* HALT */
            args: 0,
            call: |_ic| Halt
        }),
        _ => Err("no such instruction")
    }
}

fn parse_code(code: &String) -> Vec<MemoryItem> {
    code.split(",")
        .map(|x| x.parse::<MemoryItem>().unwrap())
        .collect::<Vec<_>>()
}

impl Intcode {
    pub fn new<'a>(code: &String) -> Intcode {
        Intcode {
            ip: 0,
            modes: vec![],
            memory: parse_code(code),
            rel_base: 0,
        }
    }

    pub fn run(&mut self, input: &mut InputOutput, output: &mut InputOutput) -> bool {
        loop {
            let instr = self.memory[self.ip];
            if instr < 0 { panic!("negative instruction code") }
            let instr_no = (instr % 100) as u8;
            let instr_code = get_instruction(instr_no).unwrap();

            self.modes.clear();
            let mut modes: usize = (instr / 100) as usize;
            for i in 0..instr_code.args {
                let mode = num::FromPrimitive::from_usize(modes % 10);
                self.modes.push(mode.unwrap());
                modes /= 10;
                self.ensure_arg_available(i);
            }

            let result = (instr_code.call)(self);

            let next_instr = self.ip + instr_code.args + 1;

            self.ip = match result {
                Halt => return false,
                Jmp(dest) => dest,
                StoreIn(reg) => match input.pop_front() {
                    None => return true,
                    Some(input_value) => {
                        self.store(reg, input_value);
                        next_instr
                    }
                },
                LoadOut(reg) => {
                    output.push_back(self.load(reg));
                    next_instr
                },
                Continue => next_instr,
            }
        }
    }

    fn reg_access(&self, arg: usize) -> RegAccess {
        let val = self.memory[self.ip + 1 + arg];
        let pos = match self.modes[arg] {
            Mode::Pos => val,
            Mode::Rel => val + self.rel_base,
            Mode::Immed => { return RegAccess::Value(val); }
        };
        if pos < 0 {
            panic!("Negative memory access");
        }
        RegAccess::MemoryAt(pos as usize)
    }

    fn ensure_arg_available(&mut self, arg: usize) {
        if let RegAccess::MemoryAt(pos) = self.reg_access(arg) {
            if pos >= self.memory.len() {
                self.memory.resize(pos + 1, 0);
            }
        }
    }

    fn load(&self, arg: usize) -> MemoryItem {
        match self.reg_access(arg) {
            RegAccess::Value(val) => val,
            RegAccess::MemoryAt(pos) => self.memory[pos],
        }
    }

    fn store(&mut self, arg: usize, value: MemoryItem) -> InstructionResult {
        match self.reg_access(arg) {
            RegAccess::Value(_) => panic!("Cannot write in immediate mode"),
            RegAccess::MemoryAt(pos) => self.memory[pos] = value,
        }
        Continue
    }
}
