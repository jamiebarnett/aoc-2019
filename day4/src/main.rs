use std::str;

fn main() {
    let mut input = "271973-785961".split("-").map(|i| i.trim().parse::<i32>().unwrap()); // create an iterator on the split

    let mut count: usize = 0;

    for n in (input.next().unwrap()..=input.next().unwrap())
        .map(|i| i.to_string().bytes().collect::<Vec<_>>())
        {
            let mut rule1 = false;
            let mut rule2 = false;

            // println!("{}", str::from_utf8(&n).unwrap());

            if n.windows(2).all(|i| i[0] <= i[1]) {
                rule1 = true
            }

            // nothing to see here please move on
            if n.windows(6).any(
                { |i|
                    i[0] == i[1] && i[1] != i[2] ||
                    i[0] != i[1] && i[1] == i[2] && i[2] != i[3] ||
                        i[1] != i[2] && i[2] == i[3] && i[3] != i[4] ||
                        i[2] != i[3] && i[3] == i[4] && i[4] != i[5] ||
                        i[3] != i[4] && i[4] == i[5]
            }){
                rule2 = true
            }

            if rule1 && rule2 {
                count += 1;
            }
        };

    println!("{:?}", count);
}
