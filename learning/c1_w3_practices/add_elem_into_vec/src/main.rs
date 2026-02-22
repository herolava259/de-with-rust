

fn add_two_side(vec: &mut Vec<i32>, elem: i32){
    vec.push(elem);
    vec.insert(0, elem);
}

fn concat_two_vec(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>){
    for elem in vec2 {
        vec1.push(*elem);
    }
}

fn main() {
    let mut v = vec![1,2,3];

    v.push(4);

    let more_numbers = vec![5,6,7];

    v.extend(more_numbers);

    println!("{:?}", v);


    let mut other_numbers = vec![8,9,10];

    v.append(&mut other_numbers);

    println!("{:?}", v);

    v.insert(0, 0);

    println!("{:?}", v);

    // test concate fn 
    let mut vec1 = vec![1,2,3];
    let mut vec2 = vec![4,5,6];
    concat_two_vec(&mut vec1, &mut vec2);
    println!("{:?}", vec1);

    // test add_two_side fn
    let mut vec3 = vec![1,2,3];
    add_two_side(&mut vec3, 0);
    println!("{:?}", vec3);


}
