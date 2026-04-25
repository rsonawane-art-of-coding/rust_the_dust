/* Structure and implementation */
#![allow(unused)]

#[derive(Debug)]
struct SocialProfile {
    name:String,
    age:u32,
    profession:String,
}

impl SocialProfile {
    fn new (name:String, age:u32, profession:String) -> Self {
        Self {
            name:name,
            age:age,
            profession:profession
        }
    }    

    fn update_age (&mut self, age:u32) {
        self.age = age;
    }
}

fn number_to_string(number:&i32) -> &str {
    match number {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        _ => "invalid",
    }
}

fn main()
{
    let mut profile = SocialProfile {
        name:String::from("Steve jobs"),
        age: 70,
        profession:String::from("Apple founder"),
    };

    println!("{:?}", profile);
    
    profile.update_age(71);
    
    println!("{:?}", profile);


    let profile =  SocialProfile::new (
        String::from("Larry Page"),
        70,
        String::from("Google founder"),
    );

    println!("{:?}", profile);

    let number_array:[i32;8] = [1, 2, 3, 4, 5, 6, 7, 8];

    for num in number_array {
        println!("{}", number_to_string(&num));
    }
}

