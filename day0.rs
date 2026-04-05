use std::ops::Range;
use std::ops::RangeInclusive;

fn main() {
    
    let  first_initial :char ='B';
    println!{
        "{} {}\n",
        first_initial.is_alphabetic(),
        first_initial.is_lowercase()
    }

    /* Compound type */
    let numbers:[i32;4] = [4, 8, 16, 32];

    println!("{:#?}\n", numbers);
    
    let colors:[&str;4] = ["red", "blue", "green", "yellow"];

    
    println!(
        "We have total {} colors available\n", 
        colors.len()
    );
    
    //dbg!(colors);

    
    println!("First color is  : {}", colors[0]);
    println!("Second color is : {}", colors[1]);
    println!("Third color is  : {}", colors[2]);
    println!("Fourth color is : {}", colors[3]);



    /* Tuple Type */
    let employee : (&str, i32, i32, &str) = ("John", 5, 200000, "SparX");
    
    //dbg!(employee);

    println!(
        "\nEmployee details
        \rName            : {}
        \rYear of Service : {}
        \rSalary          : {}
        \rCode            : {}
        ",
        employee.0,
        employee.1,
        employee.2,
        employee.3
    );


    let (name, yos, salary, code) = employee;

    println!("
        \rName :{name},
        \rYears of Service : {yos},
        \rSalary : {salary},
        \rCode Name : {code}"
    );

    /* Range */
    let month_days : Range<i32> = 1..3;
    println!("{month_days:#?}");
    
    let month_days : RangeInclusive<i32> = 1..=3;
    println!("{month_days:#?}");

    for day in month_days
    {
        println!("{day}");
    }


    let letters:Range<char> = 'b'..'g';
    
    //dbg!(letters);
    
    for ch in letters
    {
        println!("{ch}");
    }
    
    let letters:Range<char> = 'b'..'g';
    dbg!(letters.start);
    dbg!(letters.end);
   

    let months:[&str;3]=["Jan", "Feb", "March"];
    
    for month in months
    {

        println!("{month}");    
    }

}
