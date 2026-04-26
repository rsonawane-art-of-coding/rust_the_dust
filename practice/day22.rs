/* Options and impl, Result enum*/
#![allow(unused)]

#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Opps Panic !"), 
        }
    }

    fn unwrap_or(self, fallback_value:i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value, 
        }
    }
}

fn main()
{
    let some_number : MyOption =  MyOption::Some(13);
    let number = some_number.unwrap();  
    println!("{}", number);
    
   // let none_number : MyOption =  MyOption::None;
   // let number = None_number.unwrap();  
   // println!("{}", number);

    let some_number : MyOption =  MyOption::Some(13);
    let number = some_number.unwrap_or(0);  
    println!("{}", number);
    
    let none_number : MyOption =  MyOption::None;
    let number = none_number.unwrap_or(-1);  
    println!("{}", number);

    /* Result Enum*/
    let ok:Result<i8, &str>= Result::Ok(5);
    println!("{:?}", ok);

    let error:Result<i8, &str>= Result::Err("This is an error fix it !");
    println!("{:?}", error);

}

