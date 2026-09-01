pub fn describe() {
    let temperature = Temperature::Celcius(35);

    match temperature {
        Temperature::Celcius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celcius(t) => println!("{}C is equal to or below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }
}

enum Temperature {
    Celcius(i32),
    Fahrenheit(i32),
}
