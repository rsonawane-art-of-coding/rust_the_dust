fn apply_to_jobs(number:i32, title:&str)
{
    println!("I'm applying to {number} {title} jobs");
}

fn is_even(number:i32) -> bool
{
    if number % 2 == 0
    {
        true
    }
    else 
    {
        false
    }
}

fn alphabets(text:&str) -> (bool, bool)
{
    let mut a_to_z : (bool, bool) = (false, false);

    for ch in text.chars() {
        if ch == 'a' || ch == 'A' {
            a_to_z.0 = true;
        } 
        else if ch == 'z' || ch == 'Z'
        {
            a_to_z.1 = true;
        }
    } 
   
    a_to_z
}

fn alphabets_v1(text:&str) -> (bool, bool)
{
    let mut a_to_z : (bool, bool) = (false, false);

    a_to_z.0 = text.contains("a");
    a_to_z.1 = text.contains("z");
 
    a_to_z
}

fn main()
{
    apply_to_jobs(17, "Rust Developer");
    println!("20 is even number : {}", is_even(20));
    println!("25 is even number : {}", is_even(25));
    
    println!("{:?}", alphabets("There are Elephants in Zoo"));
    println!("{:?}", alphabets("The Rust Developer"));
    println!("{:?}", alphabets("Space"));
    
    println!("{:?}", alphabets_v1("There are Elephants in Zoo"));
    println!("{:?}", alphabets_v1("The Rust Developer"));
    println!("{:?}", alphabets_v1("Space"));
}
