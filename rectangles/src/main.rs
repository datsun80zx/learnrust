// use of a struct allows for additional meaning to be added to the data through the use of labeling 

#[derive(Debug)]
struct Rectangle {
    width_2: u32,
    height_2: u32,
}

impl Rectangle {
    fn area_3(&self) -> u32 {
        self.width_2 * self.height_2
    }
}

fn main() {
    // overly verbose and not really well written as it doesn't clearly show anwhere that the parameters are related
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.", 
        area(width1, height1)
    );

    // same program as above but refactored to use touples in the function area_1. However the use of tuples in area_1 makes it difficult to know which index is which. 
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_1(rect1)
    );

    // final way to illustrate the same operation by using struct Rectangles. This makes the operation more clear and readable. 
    let rect_2 = Rectangle {
        width_2: 30,
        height_2: 50,
    };

    dbg!(&rect_2);

    println!("rect_2 is {rect_2:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(&rect_2)
    );

    let rect_3 = Rectangle {
        width_2: 30, 
        height_2: 50,
    }; 

    println!(
        "The area of the rectangle is {} square pixels.",
        rect_3.area_3()
    );
}

fn area_2(rectangle: &Rectangle) -> u32 {
    rectangle.width_2 * rectangle.height_2
}

fn area_1(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}