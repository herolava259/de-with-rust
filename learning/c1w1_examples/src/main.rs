// this is a command to calculate average price of a fruit in two different regions


fn average_price() -> f64{
    let fruit_name = "pineapple";

    let quantity: i32 = 2;

    let thai_price: f64 = 3.50;
    let vn_price: f64 = 4.50;

    let price = if quantity > 20 {
        vn_price
    }else {
        thai_price
    };

    


    return price * quantity as f64;

}

// create a function that finds out the average of several numbers and returns it
fn average(numbers: &[u64]) -> f64 {
    let mut sum: f64 = 0.0;

    for num in numbers{

        sum += *num as f64;

        println!("{}", num);
    }


    
    return sum / numbers.len() as f64;
}

fn main() {
    println!("Hello, world!");

    println!("I am a children in the Rustapia city");

    let item = "mango";

    let price: f64 = 2.50;

    let quantity: i32 = 10;

    println!("{} {} for {} dorllars", quantity, item, price);


}
