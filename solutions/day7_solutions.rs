fn main()
{
    let is_concert:bool = true;
    let is_event:bool = is_concert;
    
    /* Here ownership will not move because bool implements copy trait */
    
    println!("Is Concert {}", is_concert);
    println!("Is Event {}", is_event);

    let sushi = "Salmon"; // same as let sushi:&str = "Salmon";
    let dinner = sushi; // same as let dinner:&str = &sushi;

    /* Here ownership will not move, ref is type and it implements copy strai*/
    println!("Sushi {}", sushi); 
    println!("Dinner  {}", dinner);

    let sushi = String::from("Salmon"); 
    let dinner = sushi; // Ownership moved to dinner and sushi is invalid

    //println!("Sushi {}", sushi); 
    println!("Dinner  {}", dinner);

    let meal:String = String :: from("Panjabi Thali");

    /* 

    // meal ownership is passed to eat_meal_v0 
    eat_meal_v0(meal);

    // meal variable no longer valid
    println!("Meal is {:?}", meal);
    
    */
 
    eat_meal_v1(meal.clone());
    println!("Meal is {:?}", meal);

}

fn eat_meal_v0(_meal:String) {
    /* Doing nothing, but taking ownership of variable meal*/
}

fn eat_meal_v1(mut meal: String) {
    meal.clear();
}
