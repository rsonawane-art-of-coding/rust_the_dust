const MAX_PIZZA_STORE : i32 = 5;


fn square(number:i32) -> i32
{
    number * number
}

fn mystery () {}


fn main() {
    open_store(1, "Dubai UAE");
    open_store(2, "Mumbai India");
    open_store(3, "Hampi India");
    open_store(6, "Jerusalem Israel");
    bake_pizza();
    swim_in_profit();

    println!("Square of number {}", square(5));
    
    println!("Return Value of function {:?}", mystery());


    /* Block In functions */
    let multiplier = 3;

    let calculation = {
        let value  = 2345;
        multiplier * value
    };

    println!("Return Value of function block {calculation}");

}

fn open_store (store_num:i32, city:&str) {
    if store_num < MAX_PIZZA_STORE {
        println!("Pizza Store No : {store_num} Opened in {city}.");
    }
    else {
        println!("Can't Open More Pizza Stores!");

    } 
}

fn bake_pizza() {
    println!("Pizza is baked and Ready !");
}

fn swim_in_profit() {
    println!("So much $$$, So Little time");
}
