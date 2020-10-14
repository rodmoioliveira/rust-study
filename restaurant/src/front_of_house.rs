pub mod hosting {
    pub fn add_to_waitlist() {
        println!("add_to_waitlist");
    }

    pub fn seat_at_table() {
        println!("seat_at_table");
    }
}

pub mod serving {
    fn hide() {
        println!("private!!!");
    }

    pub fn take_order() {
        println!("take_order");
    }

    pub fn serve_order() {
        println!("serve_order");
        crate::front_of_house::serving::hide();
        super::serving::hide();
    }

    pub fn take_payment() {
        println!("take_payment");
    }
}
