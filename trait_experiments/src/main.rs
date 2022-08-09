fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = [1, 200, 3, 4];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = ['e', 'a', 'z', 'A'];
    let char_result = largest(&char_list);
    println!("The largest char is {}", char_result);
}
