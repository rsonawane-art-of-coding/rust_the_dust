/* Mutable Slice */
fn main()
{
    let cities : String = String :: from("Mumbai Hyd Munich Tokyo");

    let favorite_city : &str = &cities[17..];

    print_string_len(&cities);
    print_string_len(favorite_city);


    let mut number = [10, 15, 17, 23, 92, 93];
    let array_slice = &mut number[..3];

    array_slice[0] = 23;

    print_array(array_slice);
    print_array(&number);

    let string : String = String::from("I am a great!");
    
    /* This just creates a new string with replace sub string */
    let new_string = string.replace("I am", "You");

    println!("{}", new_string);
    println!("{}", string);
    
}

fn print_array(array:&[i32]) {
    println!("{:?}", array.len());
}

fn print_string_len(string:&str) {
    println!("{}", string.len()); 
}
