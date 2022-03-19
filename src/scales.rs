use druid::Data;

#[derive(Data, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scales {
    Celcius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Delisle,
    Newton,
    Reaumur,
    Romer,
}

impl Scales {
    // An array of the scales
    pub const ALL: [Scales; 8] = [
        Scales::Celcius,
        Scales::Fahrenheit,
        Scales::Kelvin,
        Scales::Rankine,
        Scales::Delisle,
        Scales::Newton,
        Scales::Reaumur,
        Scales::Romer,
    ];

    // Get the symbol for a scale
    pub fn short(&self) -> String {
        match self {
            Scales::Celcius => format!("°C"),
            Scales::Fahrenheit => format!("°F"),
            Scales::Kelvin => format!("K"),
            Scales::Rankine => format!("°R"),
            Scales::Delisle => format!("°De"),
            Scales::Newton => format!("°N"),
            Scales::Reaumur => format!("°Ré"),
            Scales::Romer => format!("°Rø"),
        }
    }

    // Get the name of a scale
    pub fn name(&self) -> String {
        match self {
            Scales::Celcius => format!("Celsius"),
            Scales::Fahrenheit => format!("Fahrenheit"),
            Scales::Kelvin => format!("Kelvin"),
            Scales::Rankine => format!("Rankine"),
            Scales::Delisle => format!("Delisle"),
            Scales::Newton => format!("Newton"),
            Scales::Reaumur => format!("Réaumur"),
            Scales::Romer => format!("Rømer"),
        }
    }

    // Get the name and symbol for a scale
    pub fn short_and_name(&self) -> String {
        format!("{} ({})", self.short(), self.name())
    }

    // Convert a scale to the others
    pub fn convert_to(&self, convert_to: Scales, num: f64) -> f64 {
        match self {
            Scales::Celcius => match convert_to {
                Scales::Celcius => num,
                Scales::Fahrenheit => num * (9.0 / 5.0) + 32.0,
                Scales::Kelvin => num + 273.15,
                Scales::Rankine => (num + 273.15) * (9.0 / 5.0),
                Scales::Delisle => (100.0 - num) * (3.0 / 2.0),
                Scales::Newton => num * (33.0 / 100.0),
                Scales::Reaumur => num * (4.0 / 5.0),
                Scales::Romer => num * (21.0 / 40.0) + 7.5,
            },
            Scales::Fahrenheit => match convert_to {
                Scales::Celcius => (num - 32.0) * (5.0 / 9.0),
                Scales::Fahrenheit => num,
                Scales::Kelvin => (num + 459.67) * (5.0 / 9.0),
                Scales::Rankine => num + 459.67,
                Scales::Delisle => (212.0 - num) * (5.0 / 6.0),
                Scales::Newton => (num - 32.0) * (11.0 / 60.0),
                Scales::Reaumur => (num - 32.0) * (4.0 / 9.0),
                Scales::Romer => (num - 32.0) * (7.0 / 24.0) + 7.5,
            },
            Scales::Kelvin => match convert_to {
                Scales::Celcius => num - 273.15,
                Scales::Fahrenheit => num * (9.0 / 5.0) - 459.67,
                Scales::Kelvin => num,
                Scales::Rankine => num * (9.0 / 5.0),
                Scales::Delisle => (373.15 - num) * (3.0 / 2.0),
                Scales::Newton => (num - 273.15) * (33.0 / 100.0),
                Scales::Reaumur => (num - 273.15) * (4.0 / 5.0),
                Scales::Romer => (num - 273.15) * (21.0 / 40.0) + 7.5,
            },
            Scales::Rankine => match convert_to {
                Scales::Celcius => (num - 491.67) * (5.0 / 9.0),
                Scales::Fahrenheit => num - 459.67,
                Scales::Kelvin => num * (5.0 / 9.0),
                Scales::Rankine => num,
                Scales::Delisle => (671.67 - num) * (5.0 / 6.0),
                Scales::Newton => (num - 491.67) * (11.0 / 60.0),
                Scales::Reaumur => (num - 491.67) * (4.0 / 9.0),
                Scales::Romer => (num - 491.67) * (7.0 / 24.0) + 7.5,
            },
            Scales::Delisle => match convert_to {
                Scales::Celcius => 100.0 - num * (2.0 / 3.0),
                Scales::Fahrenheit => 212.0 - num * (6.0 / 5.0),
                Scales::Kelvin => 373.15 - num * (2.0 / 3.0),
                Scales::Rankine => 671.67 - num * (6.0 / 5.0),
                Scales::Delisle => num,
                Scales::Newton => 33.0 - num * (11.0 / 50.0),
                Scales::Reaumur => 80.0 - num * (8.0 / 15.0),
                Scales::Romer => 60.0 - num * (7.0 / 20.0),
            },
            Scales::Newton => match convert_to {
                Scales::Celcius => num * (100.0 / 33.0),
                Scales::Fahrenheit => num * (60.0 / 11.0) + 32.0,
                Scales::Kelvin => num * (100.0 / 33.0) + 273.15,
                Scales::Rankine => num * (60.0 / 11.0) + 491.67,
                Scales::Delisle => (33.0 - num) * (50.0 / 11.0),
                Scales::Newton => num,
                Scales::Reaumur => num * (80.0 / 33.0),
                Scales::Romer => num * (35.0 / 22.0) + 7.5,
            },
            Scales::Reaumur => match convert_to {
                Scales::Celcius => num * (5.0 / 4.0),
                Scales::Fahrenheit => num * (9.0 / 4.0) + 32.0,
                Scales::Kelvin => num * (5.0 / 4.0) + 273.15,
                Scales::Rankine => num * (9.0 / 4.0) + 491.67,
                Scales::Delisle => (80.0 - num) * (15.0 / 8.0),
                Scales::Newton => num * (33.0 / 80.0),
                Scales::Reaumur => num,
                Scales::Romer => num * (21.0 / 32.0) + 7.5,
            },
            Scales::Romer => match convert_to {
                Scales::Celcius => (num - 7.5) * (40.0 / 21.0),
                Scales::Fahrenheit => (num - 7.5) * (24.0 / 7.0) + 32.0,
                Scales::Kelvin => (num - 7.5) * (40.0 / 21.0) + 273.15,
                Scales::Rankine => (num - 7.5) * (24.0 / 7.0) + 491.67,
                Scales::Delisle => (60.0 - num) * (20.0 / 7.0),
                Scales::Newton => (num - 7.5) * (22.0 / 35.0),
                Scales::Reaumur => (num - 7.5) * (32.0 / 21.0),
                Scales::Romer => num,
            },
        }
    }
}

impl Default for Scales {
    fn default() -> Scales {
        Scales::Celcius
    }
}
