/* Slice */
fn main()
{
    let favorite_places:String = String::from("Itali, France, Finland, Alaska");
    let first_place : &str = &favorite_places[0..5];
    let second_place : &str = &favorite_places[7..13];
    let third_place : &str = &favorite_places[15..22];

    println!(" 1st {first_place}");
    println!(" 2nd {second_place}");
    println!(" 3rd {third_place}");

    let all_places = &favorite_places[..];
    println!("I want to visit {all_places}");



    let favorite_gadgets:&str = "GalaxyS25 PS5 RaspberryPi STM32 Bose";
    let phone = &favorite_gadgets[..10];

    println!("I own {phone}");
    println!("I have collections of gadgets :  {}", favorite_gadgets);


    let favorite_car = {
        let my_cars_list = "Porshe, BMW, Mercedis VW Thar GVagon";
        &my_cars_list[8..11]
    };

    /* 
     * Even though my_cars_list variable is out of scope. 
     * Its reference still valid.
     * Because its string belongs to RO section
     **/
    println!("my first car {}", favorite_car);
    
    /* 
    
    my_cars_list is dropped here and references to it no longer valid
    
    let _favorite_car = {
        let my_cars_list = String ::from("Porshe, BMW, Mercedis VW Thar GVagon");
        &my_cars_list[8..11]
    };

    println!("my first car {}", _favorite_car);

    */

    let natural_number:[i32;6] = [0, 1, 2, 3, 4, 5];
    
    let first_two_numbers = &natural_number[0..2];

    println!("First two natural numbers  : {first_two_numbers:?}");


    let primary_colors : (&str, &str, &str) = ("red", "blue", "yellow");
    
    let first_prim_color = &primary_colors.0;

    println!("First Primary color {}", first_prim_color);
}
