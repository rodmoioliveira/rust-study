use restaurant::back_of_house;
use restaurant::eat_at_restaurant;
use restaurant::front_of_house;
use restaurant::pay_restaurant;
// use restaurant::*;

fn main() {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
    front_of_house::serving::take_order();
    front_of_house::serving::take_payment();
    front_of_house::serving::serve_order();

    let mut breakfast = back_of_house::Breakfast::summer("rye");
    breakfast.toast = String::from("wheat");

    // // private, will break the code...
    // breakfast.seasonal_fruit = String::from("apple");

    let ap1 = back_of_house::Appetizer::Soup;
    let ap2 = back_of_house::Appetizer::Salad;

    println!("{:?}", breakfast);
    println!("{:?}", ap1);
    println!("{:?}", ap2);

    eat_at_restaurant();
    pay_restaurant();
}
