use std::fs;

fn main() {
    let input_file = "./input/input.txt";
    let contents = fs::read_to_string(input_file).expect("Should be able to read file");
    let depths = contents.lines().map(str::parse::<i64>).map(Result::unwrap).collect();
    let count = find_increasing_depths(depths);

    fn find_increasing_depths(depths: Vec<i64>) -> i64 {
        let mut count: i64 = 0;
        for i in 0..depths.len() - 1 {
            let j = i + 1;
            let cur = depths[i];
            let next = depths[j];
            if next > cur {
                count += 1;
            }
        }
        count
    }
    
    println!("Final count: {}", count);
}
