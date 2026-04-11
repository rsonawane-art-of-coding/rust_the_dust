/* References and Borrowing*/

fn main()
{

    let number : i32 = 2;
    let number_ref = &number;

    /* This work because references has implemented display trait*/
    
    println!("number reference {number_ref}");
    /* or */
    println!("number reference {}", *number_ref);

    let text:String = String :: from("This is dynamic string");
    let text_ref : &String = &text;

    println!("{}", text_ref);
    /* or */
    println!("{}", text_ref);


    let text = "This is read only string"; // type is &str
    let text_ref: &str = &text;

    println!("{}", text_ref);
    /* or */
    println!("{}", text_ref);


    println!("{}", text);

    /*
     *  String : Dynamic string with memory allocated on Heap
     *
     *  &String : Reference to String pointing to same location as String
     *  
     *  str : Read only string stored in text section of program.
     *
     *  &str : Reference to str 
     *
     * */






}
