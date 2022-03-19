use druid::text::{Formatter, Selection, Validation, ValidationError};

pub struct TemperatureFormatter;

impl Formatter<f64> for TemperatureFormatter {
    // What will be shown in the input field when the user isn't editing it
    fn format(&self, value: &f64) -> String {
        format!("{value}")
    }

    // What will be shown in the input field when the user is editing it
    fn format_for_editing(&self, value: &f64) -> String {
        self.format(value)
    }

    // The actual value of the variable
    fn value(&self, input: &str) -> Result<f64, ValidationError> {
        Ok(match input.trim().replace(',', ".").parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        })
    }

    // Determine whether the newly edited text is valid
    fn validate_partial_input(&self, _input: &str, _sel: &Selection) -> Validation {
        return Validation::success();
    }
}
