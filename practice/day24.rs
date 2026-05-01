/* Vectors */

fn main() {
    let mut numbers_vector = Vec::<i32>::new();

    numbers_vector.push(0);
    numbers_vector.push(1);
    numbers_vector.push(2);
    numbers_vector.push(3);
    numbers_vector.push(4);

    println!("{numbers_vector:#?}");

    numbers_vector.insert(0,  -1);
    numbers_vector.insert(3,  -3);
    
    println!("{numbers_vector:#?}");

    numbers_vector.pop();
    numbers_vector.pop();
    numbers_vector.pop();
    println!("{numbers_vector:#?}");
    
    numbers_vector.remove(0);
    println!("{numbers_vector:#?}");
}
