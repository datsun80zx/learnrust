use std::io;

fn main() {
    
    loop {

        let mut units = String::new();

        
        println!("Please enter units: C or F");

        io::stdin()
                .read_line(&mut units)
                .expect("Failed to read line");
        
        let units: String = units.trim().parse().expect("REASON");

        if units == "C" {
            println!("You entered Celcius!");
        } else if units == "F" {
            println!("You entered Farenheight!");
        } else {
            println!("You entered an invalid unit!")
        }


        println!("Please enter value to convert:");

        let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");

            let temp: f32 = temp.trim().parse().expect("REASON");

        println!("You entered: {}^{}", temp, units);


        if units == "C" {
            let f = c_to_f(temp);
            println!("The temperature is: {} degrees Fahrenheit!", f);
        } else {
            let c = f_to_c(temp);
            println!("The temperature is: {} degrees Celsius!", c);
        }

    }

    fn c_to_f (temp: f32) -> f32 {
        let f = (1.8 * temp) + 32.0;
        return f;
    }

    fn f_to_c (temp: f32) -> f32 {
        let c = (temp - 32.0) * (5.0 / 9.0);
        return c;
    }
}