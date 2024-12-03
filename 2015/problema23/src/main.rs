enum Instruction {
    HLF,
    TPL,
    INC,
    JMP,
    JIE,
    JIO,
}

fn parse_instruction(i: &str, t: Instruction) -> Vec<String> {
    match t {
        Instruction::HLF => Vec::new(),
        Instruction::TPL => Vec::new(),
        Instruction::INC => Vec::new(),
        Instruction::JMP => Vec::new(),
        Instruction::JIE => i
            .split(' ')
            .map(|x| x.replace(",", ""))
            .filter(|d| *d != "")
            .collect(),
        Instruction::JIO => Vec::new(),
    }
}

// hlf r sets register r to half its current value, then continues with the next instruction.
// tpl r sets register r to triple its current value, then continues with the next instruction.
// inc r increments register r, adding 1 to it, then continues with the next instruction.
// jmp offset is a jump; it continues with the instruction offset away relative to itself.
// jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
// jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
fn jio(rval: i32, offset: i32) -> i32 {
    if rval % 2 == 0 {
        return offset;
    }
    1
}

fn main() {
    let inst_string: String = std::fs::read_to_string("input.txt").expect("Cannot open file");
    let inst_arr: Vec<&str> = inst_string.as_str().split('\n').collect();
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut pos: usize = 0;
    let mut it: i32 = 0;
    while pos < inst_arr.len() && it < 100 {
        let instruction = inst_arr[pos];
        let subinst = &instruction[0..3];
        match subinst {
            "jio" => {
                let args = parse_instruction(instruction, Instruction::JIO);
                match args[1] {
                    String::new("a") => {
                        pos = (jio(a, args[2].parse().unwrap()) + pos as i32) as usize
                    }
                    String::new("b") => 0,
                }
                pos = jio() + pos;
                println!("1{}", subinst);
            }
            _ => println!("{}", subinst),
        }
        // Get the sub instruction
    }
}
