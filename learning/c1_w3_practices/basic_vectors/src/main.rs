fn get_item(index: usize){
    let vec = vec![1, 2, 3, 4, 5];

    let value = vec.get(index).unwrap();

    println!("Value at index {}: {}", index, value);
}

fn main() {

    let vec = vec![1,2,3,4,5];

    if vec.is_empty() {
        println!("The vector is empty!");
        return;
    } else {
        println!("The vector is not empty!");
    }

    get_item(3);

    let third_value = vec[2];
    println!("The third value in the vector is: {}", third_value);

    let last_value = vec.last().unwrap();

    println!("The last value in the vector is: {}", last_value);

    //Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }

    let mut sum: i32 = 0;

    for elem in & vec {
        sum += elem;
    }

    println!("The sum of the elements in the vector is: {}", sum);


}
