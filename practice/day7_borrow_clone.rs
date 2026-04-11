
fn main()
{
    let is_possible:bool = true;

    println!("Becoming a billionaire is possible {is_possible}");

    {
        /* is_billionaire is valid inside this block only*/
        let is_billionaire:bool = false;
        println!("Am I billionaire : {is_billionaire}");
    }
    
    /* is_billionaire is invalid */
    // println!("Am I billionaire : {is_billionaire}");
    

    /* COPY TRAIT */
    let time = 2026;
    let year = time;

    println!("\nThe time is {time} and The year is year {year}");

    let numbers : [i32;4] = [1, 2, 3, 4];
    let copy_of_number = numbers;
    
    println!("Numbers {:?}\n", copy_of_number);

    /* STRING TYPE */

    /* 
     * This is directly goes to text section of code
     * it's like constant string
     * 
     * */
    let _text_0:&str = "string literal"; 

    let _text_1:String = String :: new(); // Varible string created on Heap
    
    /*  text_2 varible store 3 info on stack "reference, length and capacity" */
    let mut text_2:String = String :: from("Initial String "); // Variable string with initial value

    println!("{text_2}");
                       
    text_2.push_str("changed to Text 2");


    /* text_2 is invalid and ownership move to _text_3 */
    let mut _text_3 = text_2;

    // drop(text_2); not allowed as value moved already

    println!("{_text_3}");
    let mut _text_4 = _text_3.clone();
    
    _text_4.push_str(" Text 4");
    println!("{_text_4}");

    _text_3.push_str(" Text 3");
    println!("{_text_3}");
    drop(_text_3);    
}
