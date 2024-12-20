fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurments(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("the value of y is: {}", y);

}

fn print_labeled_measurments(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}