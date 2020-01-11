use std::collections::HashMap;

#[derive(Debug)]
struct Res {
    index: isize,
    value: isize,
}


fn main() {
    let input = 1;

    let program: [isize; 678] = [3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 27, 28, 225, 1, 113, 14, 224, 1001, 224, -34, 224, 4, 224, 102, 8, 223, 223, 101, 7, 224, 224, 1, 224, 223, 223, 1102, 52, 34, 224, 101, -1768, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 223, 224, 223, 1002, 187, 14, 224, 1001, 224, -126, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 224, 223, 223, 1102, 54, 74, 225, 1101, 75, 66, 225, 101, 20, 161, 224, 101, -54, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 7, 224, 1, 224, 223, 223, 1101, 6, 30, 225, 2, 88, 84, 224, 101, -4884, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 2, 224, 224, 1, 224, 223, 223, 1001, 214, 55, 224, 1001, 224, -89, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224, 1, 224, 223, 223, 1101, 34, 69, 225, 1101, 45, 67, 224, 101, -112, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223, 224, 223, 1102, 9, 81, 225, 102, 81, 218, 224, 101, -7290, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1101, 84, 34, 225, 1102, 94, 90, 225, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 329, 101, 1, 223, 223, 1108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 344, 101, 1, 223, 223, 1008, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 359, 101, 1, 223, 223, 8, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 374, 101, 1, 223, 223, 108, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 389, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 404, 1001, 223, 1, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 419, 101, 1, 223, 223, 1107, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 434, 1001, 223, 1, 223, 1107, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 449, 101, 1, 223, 223, 1108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 464, 101, 1, 223, 223, 8, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 479, 101, 1, 223, 223, 8, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 494, 1001, 223, 1, 223, 1007, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 509, 1001, 223, 1, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 524, 1001, 223, 1, 223, 1108, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 539, 101, 1, 223, 223, 1008, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 554, 101, 1, 223, 223, 107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 569, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 584, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 599, 101, 1, 223, 223, 1008, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 614, 1001, 223, 1, 223, 107, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 629, 101, 1, 223, 223, 7, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 644, 1001, 223, 1, 223, 1007, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 659, 101, 1, 223, 223, 108, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226];
    let mut cloned_program = program.clone();

    let mut i: usize = 0;
    while i < cloned_program.len() {
        if i == 3 {
            break;
        }

        let instruction = cloned_program[i];

        if instruction == 1 {
            let res = add_code(&mut cloned_program, i);
            cloned_program[res.index as usize] = res.value;
            i += 4;
        }

        if instruction == 2 {
            let res = multiply_code(&mut cloned_program, i);
            cloned_program[res.index as usize] = res.value;
            i += 4;
        }

        if instruction == 3 {
            let loc = cloned_program[i + 1];
            cloned_program[loc as usize] = input;
            i += 2;
        }

        if instruction == 4 {
            let loc = cloned_program[i + 1];
            println!("output instruction : {}", cloned_program[loc as usize]);
            i += 2;
        }

        if instruction == 99 {
            println!("ended diagnostic with value {}", cloned_program[0]);
            break;
        }

        let instruction_bytes = get_bytes(&instruction);

        println!("index {}, value {}, bytes {:?}", i, instruction, instruction_bytes);

        if is_opcode_with_parameters(&instruction_bytes) {

            let opcode = Some(instruction_bytes[instruction_bytes.len() - 1]);

            let mut modes = [0, 0, 0];

            // loop through the parameter modes - we know
            // the first two are the opcode so skip those
            let mut j = instruction_bytes.len() - 2;
            while j != 0 {
                if instruction_bytes[j] == Some(1) {
                    modes[k] = 1
                }
                j -= 1
            }

            let mut j = 1;
            while j <= 3 {
                println!("param : {}", cloned_program[i + j]);

                let mut val = 0;
                if modes[j - 1] == 0 {
                    let input_loc = cloned_program[i + j];
                    val = cloned_program[input_loc];
                } else {
                    val = cloned_program[i + j]
                }

                j += 1;
            }

            // TODO parse parameter modes and action
            // the length parameter modes does not reflect the number of parameters
            // as there can be more parameter than modes defined. "Any missing modes are 0"
            // figure out how many parameters there are without using the parameter codes.

            // we know there are at least 2 params so start with 2
//            let mut no_of_params = 2;
//            let mut j: usize = i + 3;
//            let mut is_op = false;
//            while !is_op {
//                let bytes = get_bytes(&cloned_program[j]);
//                if is_opcode_with_parameters(&bytes) || is_opcode(&cloned_program[j]) {
//                    is_op = true;
//                }
//                println!("{} is opcode : {}", cloned_program[j], is_op);
//                no_of_params += 1;
//                j += 1;
//            }
//            println!("total params : {}", no_of_params);
//            i += no_of_params;
            break;
        }
    }
}

fn get_bytes(input: &isize) -> Vec<Option<u32>> {
    input.to_string().chars().map(|d| {
        match d.to_digit(10) {
            Some(isize) => Some(isize),
            None => None
        }
    }).collect()
}

fn is_opcode_with_parameters(input: &Vec<Option<u32>>) -> bool {
    if input.len() > 1 && input[0] != None && input[input.len() - 2] == Some(0) {
        if input[input.len() - 1] == Some(1) || input[input.len() - 1] == Some(2) {
            return true;
        }
    }
    false
}

fn is_opcode(input: &isize) -> (bool) {
    match input {
        1 => true,
        2 => true,
        3 => true,
        4 => true,
        99 => true,
        _ => false,
    }
}

fn add_code(program: &mut [isize; 678], i: usize) -> Res {
    let value1 = program[i + 1];
    let value2 = program[i + 2];
    let store = program[i + 3];
    let result = program[value1 as usize] + program[value2 as usize];

    Res {
        index: store,
        value: result,
    }
}

fn multiply_code(program: &mut [isize; 678], i: usize) -> Res {
    let value1 = program[i + 1];
    let value2 = program[i + 2];
    let store = program[i + 3];
    let result = program[value1 as usize] * program[value2 as usize];

    Res {
        index: store,
        value: result,
    }
}