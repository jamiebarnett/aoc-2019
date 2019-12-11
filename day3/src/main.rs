use std::fs::File;
use std::io::Read;

#[derive(Debug, Copy, Clone)]
struct Coords {
    x: isize,
    y: isize,
    total_steps: isize,
}

fn main() {
    let mut wire1: Vec<Coords> = Vec::new();
    let mut wire2: Vec<Coords> = Vec::new();

    let mut f1 = File::open("wire1.txt").unwrap();
    let mut f2 = File::open("wire2.txt").unwrap();

    let mut wire_1_inputs = String::new();
    let mut wire_2_inputs = String::new();
    f1.read_to_string(&mut wire_1_inputs).unwrap();
    f2.read_to_string(&mut wire_2_inputs).unwrap();

    let mut wire1loc = Coords {
        x: 0,
        y: 0,
        total_steps: 0,
    };


    for s in wire_1_inputs.split(",") {
        let sub = &s[1..];
        let direction: isize = sub.parse().unwrap();

        if s.starts_with("U") {
            let mut i = 0;
            while i < direction {
                wire1loc.y += 1;
                wire1loc.total_steps += 1;
                wire1.push(wire1loc);
                i += 1
            }
        }
        if s.starts_with("R") {
            let mut i = 0;
            while i < direction {
                wire1loc.x += 1;
                wire1loc.total_steps += 1;
                wire1.push(wire1loc);
                i += 1
            }
        }
        if s.starts_with("D") {
            let mut i = 0;
            while i < direction {
                wire1loc.y -= 1;
                wire1loc.total_steps += 1;
                wire1.push(wire1loc);
                i += 1
            }
        }
        if s.starts_with("L") {
            let mut i = 0;
            while i < direction {
                wire1loc.x -= 1;
                wire1loc.total_steps += 1;
                wire1.push(wire1loc);
                i += 1
            }
        }

        wire1.push(wire1loc);
    }

    let mut wire2loc = Coords {
        x: 0,
        y: 0,
        total_steps: 0,
    };

    for s in wire_2_inputs.split(",") {
        let sub = &s[1..];
        let direction: isize = sub.parse().unwrap();

        if s.starts_with("U") {
            let mut i = 0;
            while i < direction {
                wire2loc.y += 1;
                wire2loc.total_steps += 1;
                wire2.push(wire2loc);
                i += 1
            }
        }
        if s.starts_with("R") {
            let mut i = 0;
            while i < direction {
                wire2loc.x += 1;
                wire2loc.total_steps += 1;
                wire2.push(wire2loc);
                i += 1
            }
        }
        if s.starts_with("D") {
            let mut i = 0;
            while i < direction {
                wire2loc.y -= 1;
                wire2loc.total_steps += 1;
                wire2.push(wire2loc);
                i += 1
            }
        }
        if s.starts_with("L") {
            let mut i = 0;
            while i < direction {
                wire2loc.x -= 1;
                wire2loc.total_steps += 1;
                wire2.push(wire2loc);
                i += 1
            }
        }
    }

    let mut i = 0;
    while i < wire1.len() {
//        println! {"wire 1 : {:?}", wire1[i]}
        let mut j = 0;
        while j < wire2.len() {
            // println! {"wire 2 : {:?}", wire2[j]}
            if wire1[i].y == wire2[j].y && wire1[i].x == wire2[j].x {
                let mut m_distance = 0;
                let mut x = wire1[i].x;
                let mut y = wire1[i].y;

                if x < 0 {
                    x *= -1
                }

                if y < 0 {
                    y *= -1
                }

                m_distance = y + x;

                println!("MATCH at x : {} y : {} m distance : {}", wire1[i].x, wire1[i].y, m_distance);
                println!("STEPS TAKEN x : {} y : {}", wire1[i].total_steps, wire2[j].total_steps)
            }
            j += 1
        }
        i += 1
    }
}
