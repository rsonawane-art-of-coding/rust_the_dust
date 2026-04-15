/* Mutable Slice */
fn main()
{
    let mut cities : String = String :: from("Mumbai Hyd Munich Tokyo");

    let favorite_city : &str = &mut cities[17..];

    println!("{}", favorite_city);

    /* Not Valid */
    // favorite_city.push_str("Japan");
     
    println!("{}", favorite_city);

}
