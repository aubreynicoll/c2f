use std::env;
use std::error::Error;

pub struct Config {
    temperature_celsius: f64,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let temperature_celsius = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(
                    "Missing degrees celsius. Please call c2f with a floating point argument",
                );
            }
        };

        let temperature_celsius = match temperature_celsius.parse::<f64>() {
            Ok(f) => f,
            Err(_) => return Err("Could not parse floating point number."),
        };

        Ok(Config {
            temperature_celsius,
        })
    }
}

fn c2f(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let temperature_fahrenheit = c2f(config.temperature_celsius);
    println!("{}", temperature_fahrenheit);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_works() {
        const BOILING_POINT_C: f64 = 100.0;
        const FREEZING_POINT_C: f64 = 0.0;
        const BOILING_POINT_F: f64 = 212.0;
        const FREEZING_POINT_F: f64 = 32.0;

        assert_eq!(c2f(BOILING_POINT_C), BOILING_POINT_F);
        assert_eq!(c2f(FREEZING_POINT_C), FREEZING_POINT_F);
    }
}
