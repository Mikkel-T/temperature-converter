use druid::Data;

/// The scales that can be converted to and from
///
/// Data and scales taken from https://en.wikipedia.org/wiki/Conversion_of_scales_of_temperature
#[derive(Data, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scales {
    /// The celcius scale which is also the default
    Celcius,
    /// The fahrenheit scale
    Fahrenheit,
    /// The kelvin scale
    Kelvin,
    /// The rankine scale
    Rankine,
    /// The delisle scale
    Delisle,
    /// The newton scale
    Newton,
    /// The réaumur scale
    Reaumur,
    /// The rømer scale
    Romer,
}

impl Scales {
    /// An array of the scales
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

    /// Get the symbol for a scale
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

    /// Get the name of a scale
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

    /// Get the name and symbol for a scale
    pub fn short_and_name(&self) -> String {
        format!("{} ({})", self.short(), self.name())
    }

    /// Convert a scale to the others
    pub fn convert_to(&self, convert_to: Scales, num: f64) -> f64 {
        match self {
            Scales::Celcius => match convert_to {
                Scales::Celcius => num,
                Scales::Fahrenheit => num * (9. / 5.) + 32.,
                Scales::Kelvin => num + 273.15,
                Scales::Rankine => (num + 273.15) * (9. / 5.),
                Scales::Delisle => (100. - num) * (3. / 2.),
                Scales::Newton => num * (33. / 100.),
                Scales::Reaumur => num * (4. / 5.),
                Scales::Romer => num * (21. / 40.) + 7.5,
            },
            Scales::Fahrenheit => match convert_to {
                Scales::Celcius => (num - 32.) * (5. / 9.),
                Scales::Fahrenheit => num,
                Scales::Kelvin => (num + 459.67) * (5. / 9.),
                Scales::Rankine => num + 459.67,
                Scales::Delisle => (212. - num) * (5. / 6.),
                Scales::Newton => (num - 32.) * (11. / 60.),
                Scales::Reaumur => (num - 32.) * (4. / 9.),
                Scales::Romer => (num - 32.) * (7. / 24.) + 7.5,
            },
            Scales::Kelvin => match convert_to {
                Scales::Celcius => num - 273.15,
                Scales::Fahrenheit => num * (9. / 5.) - 459.67,
                Scales::Kelvin => num,
                Scales::Rankine => num * (9. / 5.),
                Scales::Delisle => (373.15 - num) * (3. / 2.),
                Scales::Newton => (num - 273.15) * (33. / 100.),
                Scales::Reaumur => (num - 273.15) * (4. / 5.),
                Scales::Romer => (num - 273.15) * (21. / 40.) + 7.5,
            },
            Scales::Rankine => match convert_to {
                Scales::Celcius => (num - 491.67) * (5. / 9.),
                Scales::Fahrenheit => num - 459.67,
                Scales::Kelvin => num * (5. / 9.),
                Scales::Rankine => num,
                Scales::Delisle => (671.67 - num) * (5. / 6.),
                Scales::Newton => (num - 491.67) * (11. / 60.),
                Scales::Reaumur => (num - 491.67) * (4. / 9.),
                Scales::Romer => (num - 491.67) * (7. / 24.) + 7.5,
            },
            Scales::Delisle => match convert_to {
                Scales::Celcius => 100. - num * (2. / 3.),
                Scales::Fahrenheit => 212. - num * (6. / 5.),
                Scales::Kelvin => 373.15 - num * (2. / 3.),
                Scales::Rankine => 671.67 - num * (6. / 5.),
                Scales::Delisle => num,
                Scales::Newton => 33. - num * (11. / 50.),
                Scales::Reaumur => 80. - num * (8. / 15.),
                Scales::Romer => 60. - num * (7. / 20.),
            },
            Scales::Newton => match convert_to {
                Scales::Celcius => num * (100. / 33.),
                Scales::Fahrenheit => num * (60. / 11.) + 32.,
                Scales::Kelvin => num * (100. / 33.) + 273.15,
                Scales::Rankine => num * (60. / 11.) + 491.67,
                Scales::Delisle => (33. - num) * (50. / 11.),
                Scales::Newton => num,
                Scales::Reaumur => num * (80. / 33.),
                Scales::Romer => num * (35. / 22.) + 7.5,
            },
            Scales::Reaumur => match convert_to {
                Scales::Celcius => num * (5. / 4.),
                Scales::Fahrenheit => num * (9. / 4.) + 32.,
                Scales::Kelvin => num * (5. / 4.) + 273.15,
                Scales::Rankine => num * (9. / 4.) + 491.67,
                Scales::Delisle => (80. - num) * (15. / 8.),
                Scales::Newton => num * (33. / 80.),
                Scales::Reaumur => num,
                Scales::Romer => num * (21. / 32.) + 7.5,
            },
            Scales::Romer => match convert_to {
                Scales::Celcius => (num - 7.5) * (40. / 21.),
                Scales::Fahrenheit => (num - 7.5) * (24. / 7.) + 32.,
                Scales::Kelvin => (num - 7.5) * (40. / 21.) + 273.15,
                Scales::Rankine => (num - 7.5) * (24. / 7.) + 491.67,
                Scales::Delisle => (60. - num) * (20. / 7.),
                Scales::Newton => (num - 7.5) * (22. / 35.),
                Scales::Reaumur => (num - 7.5) * (32. / 21.),
                Scales::Romer => num,
            },
        }
    }
}

// Set default scale to celcius
impl Default for Scales {
    fn default() -> Scales {
        Scales::Celcius
    }
}
