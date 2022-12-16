mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("Whole wheat"),
                cheese: String::from("Gouda"),
                topping: String::from(topping),
            }
        }
    }
    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }
        // Child functions need to specify if public or not
        pub fn take_order() {
            seat_at_table();
            let cust_pizza: super::Pizza = 
                super::Pizza::lunch("veggies");
            serve_customer(cust_pizza);
        }
        fn serve_customer(cust_pizza: super::Pizza){
            println!("Customer served pizza with {}", cust_pizza.topping);
        }
    }
}

// Need a function that allows other files to use the above functions
pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}