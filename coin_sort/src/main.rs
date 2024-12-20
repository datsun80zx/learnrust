#[derive(Debug)] // This lets us print the state enum
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    // Let's add a few more for demonstration
    California,
    NewYork,
}

#[derive(Debug)] // This lets us print the coin enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    // Let's create a collection of different coins
    let coins = vec![
        Coin::Penny,
        Coin::Quarter(UsState::Alaska),
        Coin::Quarter(UsState::California),
        Coin::Dime,
        Coin::Quarter(UsState::NewYork),
        Coin::Nickel,
    ];

    // Let's process each coin and keep a total
    let mut total_cents = 0;
    
    for coin in coins {
        // Process each coin and add its value to our total
        total_cents += value_in_cents(coin);
    }

    println!("\nTotal value: {} cents", total_cents);
}