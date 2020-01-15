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
        if i == 34 {
            break;
        }


        let instruction = cloned_program[i];
        let instruction_bytes = get_bytes(&instruction);
        println!("\nvalue at 224 {}", cloned_program[224]);
        println!("index {}, value {}, bytes {:?}", i, instruction, instruction_bytes);

        if instruction == 1 {
            let res = add_code(&mut cloned_program, i);
            cloned_program[res.index as usize] = res.value;
            i += 4;
            continue;
        }

        if instruction == 2 {
            let res = multiply_code(&mut cloned_program, i);
            cloned_program[res.index as usize] = res.value;
            i += 4;
            continue;
        }

        if instruction == 3 {
            let loc = cloned_program[i + 1];
            cloned_program[loc as usize] = input;
            i += 2;
            continue;
        }

        if instruction == 4 {
            let loc = cloned_program[i + 1];
            println!("param : {}, value : {}", loc, cloned_program[loc as usize]);
            println!("output instruction : {}", cloned_program[loc as usize]);
            i += 2;
            continue;
        }

        if instruction == 99 {
            println!("ended diagnostic with value {}", cloned_program[0]);
            break;
        }


        if is_opcode_with_parameters(&instruction_bytes) {
            let opcode = instruction_bytes[instruction_bytes.len() - 1];
            if opcode == Some(4) {
                let mode = instruction_bytes[instruction_bytes.len() - 3].unwrap();
                let mut val = 0;
                if mode == 0 {
                    let input_loc = cloned_program[i + 1];
                    val = cloned_program[input_loc as usize];
                } else {
                    val = cloned_program[i + 1]
                }

                println!("output instruction : {}", val);

                i += 2;
            } else {
                let mut modes = [0, 0, 0];

                // loop through the parameter modes - we know
                // the first two are the opcode so skip those
                let mut j = instruction_bytes.len() - 2;
                let mut k = 0;
                // TODO stuck in this loop - panics if j is subtracted when 0
                // + when k is added to when 2 but the loop get's stuck and doesn't
                // complete when checks are in place

                // have implemented fix to subtract from j but now stuck on a value of 41 which is incorrect
                while j > 0 {
//                    println!("byte : {}", j - 1);
                    if instruction_bytes[j - 1] == Some(1) {
                        modes[k] = 1
                    }
                    if j > 0 {
                        j -= 1;
                    }

                    if k < 2 {
                        k += 1
                    }
                }

                println!("modes : {:?}", modes);

                let mut vals: Vec<isize> = Vec::new();
                let mut l = 1;
                while l <= 3 {
                    let mut val = 0;
                    if modes[l - 1] == 0 {
                        let input_loc = cloned_program[i + l];
                        val = cloned_program[input_loc as usize];
                    } else {
                        val = cloned_program[i + l]
                    }

                    vals.push(val);
                    println!("param : {}, val : {}", cloned_program[i + l], val);

                    l += 1;
                }

                let mut result: isize = 0;
                if opcode == Some(1) {
                    result = vals[0] + vals[1];
                    println!("{} + {} = {}", vals[0], vals[1], result)
                }

                if opcode == Some(2) {
                    result = vals[0] * vals[1]
                }

                cloned_program[vals[2] as usize] = result;

                i += 4;
            }
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
        if input[input.len() - 1] == Some(1) || input[input.len() - 1] == Some(2) || input[input.len() - 1] == Some(4) {
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

    println!("param : {}, value : {}", program[i + 1], program[value1 as usize]);
    println!("param : {}, value : {}", program[i + 2], program[value2 as usize]);
    println!("param : {}, value : {}", program[i + 3], program[store as usize]);

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

    println!("param : {}, value : {}", program[i + 1], program[value1 as usize]);
    println!("param : {}, value : {}", program[i + 2], program[value2 as usize]);
    println!("param : {}, value : {}", program[i + 3], program[store as usize]);

    Res {
        index: store,
        value: result,
    }
}