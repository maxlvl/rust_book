mod back_of_house;

pub use crate::back_of_house::back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
}
