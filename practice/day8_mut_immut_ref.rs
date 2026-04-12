/* Mutable and Immutable References */

fn main()
{
    let mut meal:String = String::from("Chiken Pulao");

    add_desert(&mut meal);
    add_sides(&mut meal);
    
    show_meal(&meal);
    
    println!("{}", meal);


    /* Multiple reference mutable and immutable */
    let mut car:String = String::from("Red");
    
    /* This is going to work */
    let _ref_3 = &mut car;
    let _ref_1 = &car;
    let _ref_2 = &car;
    let _ref_4 = &car;
    
    /* But if we use _ref_3 here program will not compile*/
    //let _ref_3 = &mut car;

    let mut bikes:String = String::from("KTM");

    /* In this case both ref are valid */
    let _ref_7 = &mut bikes;


    /* Here _ref_7 ownership is moved to _ref_8 */
    let _ref_8 = _ref_7;

    let _ref_5 = &bikes;
    let _ref_6 = _ref_5;



    println!("{_ref_1} and {_ref_2} and {_ref_4}");

    let mut sport_car:String = String::from("Mat Black");

    let _sc_ref1 = &mut sport_car;
    
    println!("{_sc_ref1}");
   
    /* First reference is borrowed here and no longer valid*/
    let _sc_ref2 = &mut sport_car;
    
    println!("{_sc_ref2}");

    /* Mutable reference does not implement copy trait */
    /* Immutable reference does implement copy trait */

    /* Ownership with Array */

    let languages:[String;3] = [
        String::from("rust"),
        String::from("c/c++"),
        String::from("Javascripts"),
    ];

    /* This is not allowed because partial ownership is moving from array
     * to variable */
    //let first :String = languages[0];
    
    let first :&String = &languages[0];

    println!("{first} and {languages:?}");
}

fn show_meal(meal:&String) {
    println!("{}", meal);
}

fn add_sides(meal:&mut String) {
    meal.push_str(" salan");
}

fn add_desert(meal:&mut String) {
    meal.push_str(" Gulab Jamun");
}
