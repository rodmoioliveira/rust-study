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
