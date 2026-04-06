fn main() {

    let month_days:std::ops::Range<i8> = 1..31;
    println!("{month_days:?}");

    let var : i32 = 1_3_3_7; 
    dbg!(var);

    let var_2_byte : i16 = var as i16;
    dbg!(var_2_byte);

    let area_of_circle : f32 = 3.12345677;
    println!("Area of Circle {area_of_circle:.6}");

    let with_milk : bool =  true;
    
    let with_sugar : bool =  false;

    let is_my_type_of_coffee : bool = with_milk && with_sugar;
    
    println!("is my type of coffee {is_my_type_of_coffee}");
        
    let is_acceptable_coffee : bool = with_milk || with_sugar;
    
    println!("is acceptable coffee {is_acceptable_coffee}");

    let four_point : [i8;4] = [0, 4, 7, 8];
    println!("{four_point:#?}");

    let misc_tuple : (i32, f32, bool, [i8;4]) = (12, 3.14, true, four_point);

    println!("{misc_tuple:?}");

}
