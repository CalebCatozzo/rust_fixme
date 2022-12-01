use std::fmt::{self, Formatter, Display};

/* Demonstrates printing of a user defined struct using println! macro.*/
/// this is the struct for the city variables 
struct City {
    name: &'static str,
    // Latitude
    /// The variable for Latitude and is a 32 bit float
    lat: f32,
    // Longitude
    ///The variable for Longitude and is a 32 bit float 
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, this method must write the formatted string into it
    ///this method will determine if the logitude is ether east or west and if the latitude is north or south 
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string into a buffer (the first argument)
        /// this will take the values for the three variables name,latitude and logitude formatte them  
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}
/// this is the struct for the color variables 
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    /// this will pirnt the values out using the a for loop 
    for city in [
        City { name: "Glassboro", lat: 39.702892, lon: -75.111839 },
        City { name: "Mullica Hill", lat: 39.73928, lon: -75.224072 },
        City { name: "Swedesboro", lat: 39.747616, lon: -75.310463 },
    ].iter() {
        println!("{}", *city);
    }
    /// this will print out the valuse using a for loop and the print line statement 
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Hint : Fix the code so you can print it using {}
        println!("red: {}, green: {}, blue: {}",color.red,color.green,color.blue);
    }
}
