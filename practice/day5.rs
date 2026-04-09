fn main()
{
    let number :i32 = 6;

    match number {
        x if x % 2 == 0 => println!("{x} is even"),
        y if y % 2 == 0 => println!("{y} is even"),
        _ => println!("Unknown"),
    }

    let mut seconds = 7;

    loop {
        if seconds == 0 {
            println!("Times up !");
            seconds = 10;
            break;
        }

        if seconds % 2 != 0 {
            println!("Its odd time");
            seconds -=1;
            continue;
        }

        println!("{seconds} to go");
        seconds -= 1;
    }

    while seconds > 0 {
        if seconds % 2 != 0 {
            println!("Its odd time");
            seconds -=1;
            continue;
        }

        println!("{seconds} to go");
        seconds -= 1;
    }
}
