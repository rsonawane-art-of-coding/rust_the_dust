/* Enums Implementation */

#![allow(unused)]

/*Tuple variant*/
#[derive(Debug)]
enum PaymentMethod {
    Upi(String),
    CreditCard(String, String, bool),
    DebitCard(String, String, bool),
}

/*Structure variant*/
#[derive(Debug)]
struct UserStatusDetails {
    name:String,
    card_number:String, 
    linked_mobile:String, 
    is_active:bool
}

#[derive(Debug)]
enum CustomerDetails {
    BasicUser(UserStatusDetails),
    ProLiteSubscribedUser(UserStatusDetails),
    ProSubscribedUser(UserStatusDetails),
}

fn main()
{
    let payment_method1 = PaymentMethod::Upi(String::from("example@upiid"));
    println!("\nO'reilly payment method is {:?}", payment_method1);
    
    let payment_method2 = PaymentMethod::CreditCard
        (
            String::from("1234-5678-0134"),
            String::from("9087665151"),
            true
        );

    println!("\nUdemy payment method is {:?}", payment_method2);
    
    let payment_method3 = PaymentMethod::DebitCard
        (
            String::from("1234-5678-0134"),
            String::from("9087665151"),
            true
        );

    println!("\nApple TV payment method is {:?}", payment_method3);

    let netflix_customer:CustomerDetails = CustomerDetails :: 
        BasicUser(
            UserStatusDetails {
                name:String::from("Bob Ryne"),
                card_number:String::from("1234-1234"), 
                linked_mobile:String::from("123333466373"), 
                is_active:true 
            }
        );

    println!("\nNetflix User :: {:#?}", netflix_customer);
}


