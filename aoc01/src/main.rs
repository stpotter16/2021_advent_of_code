use std::fs;

fn main() {
    let input_file = "./input/input.txt";
    let contents = fs::read_to_string(input_file).expect("Should be able to read file");
    let (increases, _ )= contents.lines().map(str::parse::<i64>).map(Result::unwrap).fold(
        (0, i64::MIN), |(increases, prev), curr| {
            (
                if prev < curr {
                    increases + 1
                } else {
                    increases
                },
                curr,
            )
        });
                
    println!("Iter count: {}", increases - 1);
}
