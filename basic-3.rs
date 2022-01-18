#[allow(unused_variables)]
#[allow(unused_mut)]

fn main() {

    let s: String = "Hello world".to_string();

    let say: &str = &s;

    // println!("{}",say);

    let four_ints: [i32; 4] = [1,2,3,4];

    let mut vector: Vec<i32> = vec![1,2,3,4];

    println!("{:?}",vector);
}