fn own_vec(mut vector: Vec<i32>){
    vector.push(10);
    println!("Vector inside function: {:?}", vector);
}

fn own_integer(mut num: i32){

    num+1;
}


fn own_string(s: &String){
    println!("{}", s);
}

fn main() {
    
    let mut my_vec = vec![1, 2, 3, 4 ,5];

    let my_int = 10;

    let my_string = String::from("Hello, Rust!");

    own_integer(my_int);

    println!("{}", my_int);

    own_string(&my_string);
    //println!("{}", my_string);

    own_vec(my_vec);
    //println!("Vector in main: {:?}", my_vec);

}
