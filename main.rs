/* Structure */

#[derive(Debug)]
struct Coffee {
    price   : f64,
    name    : String,
    is_hot  : bool,
}

fn main()
{

    let order_coffee : Coffee  = Coffee {
        price   : 6.5,
        name    : String::from("Capecinno"),
        is_hot  : true, 
    };

    println!("{:?}", order_coffee);
    
    println!(
        "Your coffee {} is ready please pay {}$ and it serve hot {}", 
        order_coffee.name,
        order_coffee.price,
        order_coffee.is_hot);
    
    /* At this point order_coffee.name ownership is transferred to last_order_coffee 
     * and order_coffee.name is no longer valid */
    let last_order_coffee = order_coffee.name;

    println!("{last_order_coffee}");

    /* Below line will give compile time error */
   // println!("{:?}", order_coffee);
   
    /* We can only make whole structure object mutable. Not individual field within 
     * struct */
    let mut next_order_coffee : Coffee  = Coffee {
        price   : 7.5,
        name    : String::from("moccha"),
        is_hot  : true, 
    };

    println!("{:?}", next_order_coffee);
    next_order_coffee.name = String::from("Creamy Latte");
    next_order_coffee.price = 9.0;
    next_order_coffee.is_hot = false;
    println!("{:?}", next_order_coffee);


    let my_coffee = make_coffee("Dark Chocolate Latte".to_string(), 15.5, true);

    println!("{my_coffee:?}");
}

fn make_coffee(name:String, price:f64, is_hot:bool) -> Coffee {
    Coffee {
       name: name,
       price:price,
       is_hot:is_hot,
    }
}


