use std::str;

fn main() {
    let mut input = "271973-785961".split("-").map(|i| i.trim().parse::<i32>().unwrap()); // create an iterator on the split

    println!("{:?}", input);

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

            if n.windows(2).any(|i| i[0] == i[1]) {
                rule2 = true
            }

            if rule1 && rule2 {
                count += 1;
            }

        };

    println!("{:?}", count);
}

//fn main() {
//    let input = "271973-785961".split("-").map(|i| i.trim().parse::<i32>().unwrap()); // create an iterator on the split
//
//    println!("{:?}", input);
//
//    let matches = (input.next().unwrap()..=input.next().unwrap())
//        .map(|i| i.to_string().bytes().collect::<Vec<_>>()) // map the iteration into a vector so filter an window methods can be used
//        .filter(|i| {
//            i.windows(2).any(|i| {
//                i[0] == i[1]
//            }) && i.windows(2).all(|i| {
//                i[0] <= i[1]
//            }) && find_duplicates(i)
//
//        }); // filter in elements that
//    // match in window and preceding element isn't greater than the following element
//
//    println!("{:?}", matches.count());
//}
//
//struct Match {
//    key: [usize],
//    value: [usize],
//}
//
//fn find_duplicates(i: Vec<u8> ) -> bool{
//    let n = 0;
//    let mut matches: Vec<Match> = vec![];
//    while n < i.len() {
//        if n > 1 {
//            if i[n] == i[n - 1] && i[n - 1] == i[n - 2] {
//                matches.push(Match {
//                    key: k,
//                    value: v,
//                })
//            }
//        }
//    }
//
//    if matches.len() > 1 {
//        false
//    }
//    true
//}