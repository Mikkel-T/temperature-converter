/// The scales that can be converted to and from
///
/// Data and scales taken from <https://en.wikipedia.org/wiki/Conversion_of_scales_of_temperature>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Scales {
    /// The celcius scale which is also the default
    #[default]
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
    pub const ALL: [Self; 8] = [
        Self::Celcius,
        Self::Fahrenheit,
        Self::Kelvin,
        Self::Rankine,
        Self::Delisle,
        Self::Newton,
        Self::Reaumur,
        Self::Romer,
    ];

    /// Get the symbol for a scale
    pub fn short(self) -> String {
        match self {
            Self::Celcius => "°C".to_string(),
            Self::Fahrenheit => "°F".to_string(),
            Self::Kelvin => "K".to_string(),
            Self::Rankine => "°R".to_string(),
            Self::Delisle => "°De".to_string(),
            Self::Newton => "°N".to_string(),
            Self::Reaumur => "°Ré".to_string(),
            Self::Romer => "°Rø".to_string(),
        }
    }

    /// Get the name of a scale
    pub fn name(self) -> String {
        match self {
            Self::Celcius => "Celsius".to_string(),
            Self::Fahrenheit => "Fahrenheit".to_string(),
            Self::Kelvin => "Kelvin".to_string(),
            Self::Rankine => "Rankine".to_string(),
            Self::Delisle => "Delisle".to_string(),
            Self::Newton => "Newton".to_string(),
            Self::Reaumur => "Réaumur".to_string(),
            Self::Romer => "Rømer".to_string(),
        }
    }

    /// Convert a scale to the others
    pub fn convert_to(self, convert_to: Self, num: f64) -> f64 {
        match self {
            Self::Celcius => match convert_to {
                Self::Celcius => num,
                Self::Fahrenheit => num.mul_add(9. / 5., 32.),
                Self::Kelvin => num + 273.15,
                Self::Rankine => (num + 273.15) * (9. / 5.),
                Self::Delisle => (100. - num) * (3. / 2.),
                Self::Newton => num * (33. / 100.),
                Self::Reaumur => num * (4. / 5.),
                Self::Romer => num.mul_add(21. / 40., 7.5),
            },
            Self::Fahrenheit => match convert_to {
                Self::Celcius => (num - 32.) * (5. / 9.),
                Self::Fahrenheit => num,
                Self::Kelvin => (num + 459.67) * (5. / 9.),
                Self::Rankine => num + 459.67,
                Self::Delisle => (212. - num) * (5. / 6.),
                Self::Newton => (num - 32.) * (11. / 60.),
                Self::Reaumur => (num - 32.) * (4. / 9.),
                Self::Romer => (num - 32.).mul_add(7. / 24., 7.5),
            },
            Self::Kelvin => match convert_to {
                Self::Celcius => num - 273.15,
                Self::Fahrenheit => num.mul_add(9. / 5., -459.67),
                Self::Kelvin => num,
                Self::Rankine => num * (9. / 5.),
                Self::Delisle => (373.15 - num) * (3. / 2.),
                Self::Newton => (num - 273.15) * (33. / 100.),
                Self::Reaumur => (num - 273.15) * (4. / 5.),
                Self::Romer => (num - 273.15).mul_add(21. / 40., 7.5),
            },
            Self::Rankine => match convert_to {
                Self::Celcius => (num - 491.67) * (5. / 9.),
                Self::Fahrenheit => num - 459.67,
                Self::Kelvin => num * (5. / 9.),
                Self::Rankine => num,
                Self::Delisle => (671.67 - num) * (5. / 6.),
                Self::Newton => (num - 491.67) * (11. / 60.),
                Self::Reaumur => (num - 491.67) * (4. / 9.),
                Self::Romer => (num - 491.67).mul_add(7. / 24., 7.5),
            },
            Self::Delisle => match convert_to {
                Self::Celcius => num.mul_add(-(2. / 3.), 100.),
                Self::Fahrenheit => num.mul_add(-(6. / 5.), 212.),
                Self::Kelvin => num.mul_add(-(2. / 3.), 373.15),
                Self::Rankine => num.mul_add(-(6. / 5.), 671.67),
                Self::Delisle => num,
                Self::Newton => num.mul_add(-(11. / 50.), 33.),
                Self::Reaumur => num.mul_add(-(8. / 15.), 80.),
                Self::Romer => num.mul_add(-(7. / 20.), 60.),
            },
            Self::Newton => match convert_to {
                Self::Celcius => num * (100. / 33.),
                Self::Fahrenheit => num.mul_add(60. / 11., 32.),
                Self::Kelvin => num.mul_add(100. / 33., 273.15),
                Self::Rankine => num.mul_add(60. / 11., 491.67),
                Self::Delisle => (33. - num) * (50. / 11.),
                Self::Newton => num,
                Self::Reaumur => num * (80. / 33.),
                Self::Romer => num.mul_add(35. / 22., 7.5),
            },
            Self::Reaumur => match convert_to {
                Self::Celcius => num * (5. / 4.),
                Self::Fahrenheit => num.mul_add(9. / 4., 32.),
                Self::Kelvin => num.mul_add(5. / 4., 273.15),
                Self::Rankine => num.mul_add(9. / 4., 491.67),
                Self::Delisle => (80. - num) * (15. / 8.),
                Self::Newton => num * (33. / 80.),
                Self::Reaumur => num,
                Self::Romer => num.mul_add(21. / 32., 7.5),
            },
            Self::Romer => match convert_to {
                Self::Celcius => (num - 7.5) * (40. / 21.),
                Self::Fahrenheit => (num - 7.5).mul_add(24. / 7., 32.),
                Self::Kelvin => (num - 7.5).mul_add(40. / 21., 273.15),
                Self::Rankine => (num - 7.5).mul_add(24. / 7., 491.67),
                Self::Delisle => (60. - num) * (20. / 7.),
                Self::Newton => (num - 7.5) * (22. / 35.),
                Self::Reaumur => (num - 7.5) * (32. / 21.),
                Self::Romer => num,
            },
        }
    }
}

impl std::fmt::Display for Scales {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.short(), self.name())
    }
}
