fn main() {

    //let maybe_number: Option<Option<()>>= Some(None);
    //let maybe_number = Some(42);

    //let maybe_number: Option<()> = None;

    let maybe_number = Some(Some(42));

    if let Some(Some(number)) = maybe_number {
        println!("The number has 2 nested Some and value is {:?}", number)
    }
    else if let Some(number) = maybe_number {
        println!("The number is {:?}", number)
    }
    else {
        println!("There is no number")
    }
}
