/* Result enum */
#![allow(unused)]

fn divide(numerator:f64, denominator:f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Divide by zero error".to_string())
    } else {
        Ok(numerator/denominator)
    }
}

#[derive(Debug)]
enum Coin {
    Head(String),
    Tail(i32)
}

#[derive(Debug)]
enum GenericCoin<H, T> {
    Head(H),
    Tail(T)
}

fn main()
{
    let numerator : f64 = 256.0;
    let denominator : f64 = 0.0;
    
    let result = divide(numerator, denominator);

    match result {
        Ok(value) => println!("Result is {}", value),
        Err(error) => println!("{error}"),
    }


    let coin_flip = Coin::Head(String::from("Head"));
    println!("{:?}", coin_flip);
    
    let coin_flip = Coin::Tail(20);
    println!("{:?}", coin_flip);
    
    let coin_flip = GenericCoin::<String, i32>::Head(String::from("Head"));
    println!("{:?}", coin_flip);
    
    let coin_flip = GenericCoin::<String, i32>::Tail(20);
    println!("{:?}", coin_flip);

}

