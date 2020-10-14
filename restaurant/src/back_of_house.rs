#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// Because the toast field in the back_of_house::Breakfast struct is public, in eat_at_restaurant
// we can write and read to the toast field using dot notation. Notice that we can’t use the
// seasonal_fruit field in eat_at_restaurant because seasonal_fruit is private. Try uncommenting
// the line modifying the seasonal_fruit field value to see what error you get!

// Also, note that because back_of_house::Breakfast has a private field, the struct needs to
// provide a public associated function that constructs an instance of Breakfast (we’ve named it
// summer here). If Breakfast didn’t have such a function, we couldn’t create an instance of
// Breakfast in eat_at_restaurant because we couldn’t set the value of the private seasonal_fruit
// field in eat_at_restaurant.

// In contrast, if we make an enum public, all of its variants are then public. We only need the
// pub before the enum keyword, as shown in Listing 7-10.

#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

// Because we made the Appetizer enum public, we can use the Soup and Salad variants in
// eat_at_restaurant. Enums aren’t very useful unless their variants are public; it would be
// annoying to have to annotate all enum variants with pub in every case, so the default for
// enum variants is to be public. Structs are often useful without their fields being public,
// so struct fields follow the general rule of everything being private by default unless
// annotated with pub.

// There’s one more situation involving pub that we haven’t covered, and that is our last module
// system feature: the use keyword. We’ll cover use by itself first, and then we’ll show how to
// combine pub and use.
