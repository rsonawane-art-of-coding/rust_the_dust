/* Options */

fn play_musical_instrument(instrument:Option<&String>) -> String {
    match instrument {
       // Option::Some(value) if value == "Guitar" => String::from("Let's Play Guitar"),
       // Option::Some(value) if value == "Base" => String::from("Let's Play Base"),
       // Option::Some(value) if value == "Drums" => String::from("Let's Play Guitar"),
       // Option::None => String::from("Go vocal !"),
       // Option::Some(&_) => String::from("No Music show"),
        
       Some(value) if value == "Guitar" => String::from("Let's Play Guitar"),
       Some(value) if value == "Base" => String::from("Let's Play Base"),
       Some(value) if value == "Drums" => String::from("Let's Play Guitar"),
       Some(&_) => String::from("No Music show"),
       None => String::from("Go vocal !"),
    }
}

fn item_available_for_sell(in_stock:bool, in_system:bool) -> Option<bool> {
    if in_stock && in_system {
        Option::Some(true)
    }
    else if in_system{
        Option::Some(false)
    } else {
        Option::None
    }
}

fn main()
{

    let some_number:Option<i32> = Option::Some(12);

    println!("{:?}", some_number);

    let number = some_number.unwrap();
    
    println!("{}", number);

    let some_string:Option<String> = Option::Some(String::from("options examples"));

    let text = some_string.expect("String is None");

    println!("{}", text);


    let musical_instruments:[String;3] = [
        String::from("Guitar"),
        String::from("Base"),
        String::from("Drums"),
    ];
    
    let instrument:Option<&String> =  musical_instruments.get(1);

    println!("{}", play_musical_instrument(instrument));
    
    println!("{}", play_musical_instrument(musical_instruments.get(0)));
    println!("{}", play_musical_instrument(musical_instruments.get(1)));
    
    println!("{:?}", item_available_for_sell(true, true));
    println!("{:?}", item_available_for_sell(false, true));
    println!("{:?}", item_available_for_sell(false, false));

    let numbers:[i32;4] = [1,   2,  3,  4];
    
    let mut i:usize = 0;

    loop {
        let number:Option<&i32> = numbers.get(i);

        match number {
            Some(_) => println!("{}", number.unwrap()),
            None => break,
        }

        i += 1;
    }

}

