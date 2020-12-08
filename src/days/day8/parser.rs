use super::{Instruction, InstructionType};

pub fn parse(input: &str) -> Vec<Instruction> {
    input.split('\n').filter(|line| line.len() > 0).map(|line| {
        let (type_str, value_str) = line.split_once(' ').unwrap();
        let instruction_type = match type_str {
            "jmp" => InstructionType::Jmp,
            "acc" => InstructionType::Acc,
            _ => InstructionType::Nop
        };
        let value = value_str.parse::<isize>().unwrap();

        Instruction {
            r#type: instruction_type,
            value: value
        }
    }).collect::<Vec<Instruction>>()
}

#[test]
pub fn parse_test() {
    use super::TEST;
    
    assert_eq!(InstructionType::Nop, parse(&TEST)[0].r#type)
}
