use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let mut fuel_total: isize = 0;
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let mass: isize = l.parse().unwrap();

        // calculate module fuel
        let module_fuel = (mass / 3) - 2;

        // calculate fuel needed for fuel weight
        let excess_fuel = calculate_fuel(module_fuel);

        fuel_total += (module_fuel + excess_fuel);
    }

    print!("total fuel needed : {}", fuel_total)
}


fn calculate_fuel(mass: isize) -> isize {
    let mut fuel = (mass / 3) - 2;
    if fuel > 0 {
        fuel += calculate_fuel(fuel)
    } else {
        fuel = 0
    };

    fuel
}