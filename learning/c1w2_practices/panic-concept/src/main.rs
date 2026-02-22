use core::panic;

fn loop_and_panic(numbers: Vec<i32>)
{
    for num in numbers {
        if num < 0 {
            panic!("Negative number encountered: {}", num);
        }

        println!("Processing number: {}", num);
    }
}

fn main() {
    loop_and_panic(vec![1,2,3,4,5]);
}
