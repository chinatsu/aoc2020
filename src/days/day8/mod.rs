pub mod parser;

pub const INPUT: &str = include_str!("resources/input.txt");
pub const TEST: &str = include_str!("resources/test.txt");


fn solve_one(program: &Vec<Instruction>, patch: Option<Instruction>) -> Output {
    let mut visited: Vec<isize> = Vec::new();
    let mut pointer: isize = 0;
    let mut register: isize = 0;
    while !visited.contains(&pointer) {
        let instruction: Instruction = program[pointer as usize];
        visited.push(pointer);
        if instruction.r#type == InstructionType::Nop {
            if patch.is_some() && patch.unwrap() == instruction {
                pointer += instruction.value;
            } else {
                pointer += 1;
            }
        } else if instruction.r#type == InstructionType::Acc {
            register += instruction.value;
            pointer += 1
        } else if instruction.r#type == InstructionType::Jmp {
            if patch.is_some() && patch.unwrap() == instruction {
                pointer += 1
            } else {
                pointer += instruction.value;
            }
        }
        if pointer >= program.len() as isize {
            return Output {
                infinite: false, 
                value: register
            }
        }
    }
    Output {
        infinite: true, 
        value: register
    }
}

fn solve_two(program: &Vec<Instruction>) -> isize {
    let potential_patches: Vec<&Instruction> = program.iter()
        .filter(|ins| ins.r#type == InstructionType::Nop || ins.r#type == InstructionType::Jmp)
        .collect();
    for patch in potential_patches {
        let result = solve_one(&program, Some(*patch));
        if !result.infinite {
            return result.value
        }
    }
    0
}

pub fn one(program: &Vec<Instruction>) -> String {
    let answer = solve_one(&program, None);
    if answer.infinite {
        format!("Day 8-1:  {}", answer.value)
    } else {
        format!("Day 8-1:  Something's weird, but {}", answer.value)
    }
}

pub fn two(program: &Vec<Instruction>) -> String {
    let answer = solve_two(&program);
    format!("Day 8-2:  {}", answer)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InstructionType {
    Nop, Acc, Jmp
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Instruction {
    r#type: InstructionType,
    value: isize
}

struct Output {
    infinite: bool,
    value: isize
}

#[test]
pub fn solve_one_test() {
    assert_eq!(5, solve_one(&parser::parse(TEST), None).value)
}


#[test]
pub fn solve_two_test() {
     assert_eq!(8, solve_two(&parser::parse(TEST)))
}

#[test]
pub fn regression() {
    let input = parser::parse(INPUT);
    assert_eq!(1217, solve_one(&input, None).value);
    assert_eq!(501, solve_two(&input));
}
