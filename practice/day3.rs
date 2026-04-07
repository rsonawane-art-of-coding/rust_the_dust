fn even_or_odd(number:i32) -> String {
    
    let result:String =
    {
        if number % 2 == 0 {
            "even".to_string()
        } else {
            "odd".to_string()
        }
    };

    result
}


fn main()
{
    if true {
        println!("This message will be always printed !");
    }

    if false {
        println!("This message will not be printed !");
    }

    /* If always required bool in condition not integer
    if 1 {
        println!("Opps!");
    }*/
    
    let season :&str = "mansoon";

    if season == "summer" {
        println!("Too Hot");
    } else if season == "winter" {
        println!("Too cold !");
    } else {
        println!("Oh ! Water everywhere");
    }

    println!("2 is {}", even_or_odd(2));
    println!("7 is {}", even_or_odd(7));
    println!("10 is {}", even_or_odd(10));
    println!("113 is {}", even_or_odd(113));
}
