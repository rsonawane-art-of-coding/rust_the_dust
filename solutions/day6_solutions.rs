/* RECURSSION */

fn countdown (seconds : i32)
{
    if seconds == 0
    {
        println!("Blastoff..");
    }
    else {
        println!("{seconds} seconds to blastoff..");
        countdown(seconds-1);
    }
}

fn color_to_number(color : &str) -> i32 {
    if color == "red"
    {
        1
    }
    else if color == "green"
    {
        2
    }
    else if color == "blue"
    {
        3
    } 
    else {
        0
    }
}

/* number must be less than 20 */
fn factorial_v0(number:i32) -> i64 {
    let mut factorial:i64 = 1;
    let mut tmp_num:i64= number as i64;

    loop {
        if tmp_num == 0 {
            break;
        }
        else if tmp_num > 20 { 
           println!("{number} to large !"); 
           factorial = -1;
           break;
        }
        else {
            factorial *= tmp_num;
        }
        tmp_num -= 1;
    }

    factorial
}


/* number must be less than 20 */
fn factorial_v1(number:i32) -> i32 {
    if number == 1
    {
        return 1;
    }
    
    number * factorial_v1(number - 1)   
}

fn main()
{
    countdown(10);
    println!("red color code  is {}", color_to_number("red"));
    println!("green color code  is {}", color_to_number("green"));
    println!("blue color code  is {}", color_to_number("blue"));
    println!("purple color code  is {}", color_to_number("purple"));

    println!("factorial of 20 {}", factorial_v0(20));
    println!("factorial of 21 {}", factorial_v0(21));
    
    println!("factorial of 5 {}", factorial_v1(5));
}
