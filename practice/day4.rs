fn num_to_string (number:i32) -> String {

    match number {
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        _ => "None".to_string(),
    }
}


fn main()
{
    let evalution:bool = true;

    match evalution {
        true => {
            println!{"The is value true"};
        }
        false => {
            println!{"The is value false"};
        }
    }

    println!("{}", num_to_string(1));
    println!("{}", num_to_string(2));
    println!("{}", num_to_string(10));
    println!("{}", num_to_string(3));
    println!("{}", num_to_string(5));


    /* Match with multiple value */
    let value = 8;

    match value {
        2 | 4 | 6 | 8 => println!("Even"),
        1 | 3 | 5 | 7 => println!("Odd"),
        _ => ()
    }

}
