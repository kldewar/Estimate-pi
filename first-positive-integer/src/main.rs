fn first_missing_positive(numbers: Vec<i32> ) -> i32 {
    for (count, number) in numbers.iter().enumerate() {
        println!("{}: {}", count, number);
        // TODO: implement algorithm.
    }
    return 0
}

fn main() {
    let my_vector = vec!(1,35,64,36,26);
    first_missing_positive(my_vector);
}