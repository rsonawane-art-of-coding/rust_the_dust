/* Mutable Slice */
fn main()
{
    let mut cities : String = String :: from("Mumbai Hyd Munich Tokyo");

    let favorite_city : &str = &mut cities[17..];

    println!("{}", favorite_city);

    /* Not Valid */
    //favorite_city.push_str("Japan");
     

    let mut number = [10, 15, 17, 23, 92, 93];
    let array_slice = &mut number[..3];

    array_slice[0] = 23;

    print_array(array_slice);
    print_array(&number);

}

fn print_array(array:&[i32]) {
    println!("{:?}", array.len());
}
