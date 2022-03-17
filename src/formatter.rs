use druid::text::{Formatter, Selection, Validation, ValidationError};

pub struct TemperatureFormatter;

impl Formatter<f64> for TemperatureFormatter {
    fn format(&self, value: &f64) -> String {
        format!("{value}")
    }

    fn format_for_editing(&self, value: &f64) -> String {
        self.format(value)
    }

    fn value(&self, input: &str) -> Result<f64, ValidationError> {
        Ok(match input.trim().replace(',', ".").parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        })
    }

    fn validate_partial_input(&self, _input: &str, _sel: &Selection) -> Validation {
        return Validation::success();
    }
}
