use restaurant::{
    self as res,
    back_of_house::{Appetizer, Breakfast},
    hosting, serving,
};

fn main() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    serving::take_payment();
    serving::serve_order();

    let mut breakfast = Breakfast::summer("rye");
    breakfast.toast = String::from("wheat");

    // // private, will break the code...
    // breakfast.seasonal_fruit = String::from("apple");

    let ap1 = Appetizer::Soup;
    let ap2 = Appetizer::Salad;

    println!("{:?}", breakfast);
    println!("{:?}", ap1);
    println!("{:?}", ap2);

    res::eat_at_restaurant();
    res::pay_restaurant();
}
