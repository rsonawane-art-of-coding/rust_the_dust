/* Enums and match statement*/

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


impl CustomerDetails {
    fn display_user(self) {
        match self {
            CustomerDetails::BasicUser(details) => {
                println!("Name : {} card_number {} mobile {} active {}",
                    details.name,
                    details.card_number,
                    details.linked_mobile,
                    details.is_active,
                    );
            }, 
            CustomerDetails::ProLiteSubscribedUser(details) => {
                println!("Name : {} card_number {} mobile {} active {}",
                    details.name,
                    details.card_number,
                    details.linked_mobile,
                    details.is_active,
                    );
            }, 
            CustomerDetails::ProSubscribedUser(details) => {
                println!("Name : {} card_number {} mobile {} active {}",
                    details.name,
                    details.card_number,
                    details.linked_mobile,
                    details.is_active,
                    );
            }, 
            _ => println!("Invalid User Account"), 

        };
    }

}


impl PaymentMethod {

    fn show_payment_details (self) {
        match self {
            PaymentMethod::Upi(upi_id) => {
                println!("Your UPI ID :{upi_id:?}")
            },
            PaymentMethod::CreditCard(card_number, bank, is_active) => {
                println!("You Credit Card Details Number : {:?} Bank : {:?}  Active : {:?}",
                    card_number, bank, is_active);
            },
            PaymentMethod::DebitCard(card_number, bank, is_active) => {
                println!("You Credit Card Details Number : {:?} Bank : {:?}  Active : {:?}",
                    card_number, bank, is_active);
            },
            _ => println!("Payment method None"),
        };
    }

}


fn main()
{
    let payment_method = PaymentMethod::Upi(String::from("example@paytm"));
    let payment_method = PaymentMethod::DebitCard(
                            String::from("1234-1234-1234"), 
                            String::from("American Express"),
                            true
                            );

    payment_method.show_payment_details();

    let netflix_user = CustomerDetails::BasicUser (
        UserStatusDetails {
            name:String::from("Netflix User"),
            card_number:String::from("1231-2342-5672-0999"), 
            linked_mobile:String::from("9876543210"), 
            is_active:true,
        }
    );

    netflix_user.display_user();

}


