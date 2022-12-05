use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("That file really should be there");
    let mut input_string = String::new();

    file.read_to_string(&mut input_string).expect("Uh oh, something very wrong happened");

    let input_split = input_string.trim().split("\n\n");

    // take the list of lists and map to their sums (via folding over the parsing)
    let mut input_sums = input_split.map(|list| {
        list.split("\n").fold(0, |acc, x| {
            x.parse::<i32>().expect("Why is there a bad fmt in here") + acc
        })
    }).collect::<Vec<i32>>();

    // sort, then reverse so the largest are at front
    input_sums.sort();
    input_sums.reverse();

    // our results
    println!("max: {:?}", input_sums.iter().max().unwrap());
    println!("top 3: {:?}", &input_sums[0..3]);
    println!("sum top 3: {}", &input_sums[0..3].iter().sum::<i32>());
}
