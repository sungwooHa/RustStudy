mod test1;

use front_of_house2::*;

mod front_of_house1 {
    mod hosting { // `pub` mod hosting
        fn add_to_waitlist() {} //`pub` fn add_to_waitlist()
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    front_of_house2::hosting::add_to_waitlist(); //hosting이 pub이 아님, add_to_waitlist()도 pub아님

    // Relative path
    //front_of_house::hosting::add_to_waitlist(); //hosting이 pub이 아님, add_to_waitlist()도 pub아님
}