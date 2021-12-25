use std::env;
use std::error::Error;
use std::str::FromStr;

enum Degrees {
    C,
    F,
}

impl FromStr for Degrees {
    type Err = ();

    fn from_str(s: &str) -> Result<Degrees, Self::Err> {
        match s {
            "c" => Ok(Degrees::C),
            "f" => Ok(Degrees::F),
            _ => Err(()),
        }
    }
}

pub struct Config {
    temperature: f64,
    degrees: Degrees,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let temperature = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Missing temperature. Please call c2f with a floating point argument");
            }
        };

        let temperature = match temperature.parse::<f64>() {
            Ok(f) => f,
            Err(_) => return Err("Could not parse floating point number."),
        };

        let degrees = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Missing degrees. Please call c2f with a 'c' or 'f'");
            }
        };

        let degrees = match Degrees::from_str(&degrees) {
            Ok(degrees) => degrees,
            Err(_) => return Err("Wrong degrees. Please call c2f with a 'c' or 'f'"),
        };

        Ok(Config {
            temperature,
            degrees,
        })
    }
}

fn c2f(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

fn f2c(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    match config.degrees {
        Degrees::C => println!("{}", c2f(config.temperature)),
        Degrees::F => println!("{}", f2c(config.temperature)),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOILING_POINT_C: f64 = 100.0;
    const FREEZING_POINT_C: f64 = 0.0;
    const BOILING_POINT_F: f64 = 212.0;
    const FREEZING_POINT_F: f64 = 32.0;

    #[test]
    fn c2f_works() {
        assert_eq!(c2f(BOILING_POINT_C), BOILING_POINT_F);
        assert_eq!(c2f(FREEZING_POINT_C), FREEZING_POINT_F);
    }

    #[test]
    fn f2c_works() {
        assert_eq!(BOILING_POINT_C, f2c(BOILING_POINT_F));
        assert_eq!(FREEZING_POINT_C, f2c(FREEZING_POINT_F));
    }
}
