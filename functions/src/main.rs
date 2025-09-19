// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

fn five() -> i32 {
    5
}

fn six() -> i32 {
    return 6
}

fn main() {
    let x = five();
    let y = six();

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}