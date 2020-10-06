use restaurant::front_of_house;

fn main() {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
    front_of_house::serving::take_order();
    front_of_house::serving::take_payment();
    front_of_house::serving::serve_order();
}
