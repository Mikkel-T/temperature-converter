#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod formatter;
mod scales;
use scales::Scales;

use druid::widget::{Align, Flex, Label, Scroll, TextBox};
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc};
use druid_widget_nursery::DropdownSelect;

/// The state for the temperature converter
#[derive(Clone, Data, Lens)]
struct TempConverterState {
    /// The current temperature inputted
    temperature: f64,
    /// The scale that is currently being used
    scale: Scales,
}

fn main() {
    // Create main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("Temperature converter")
        .window_size((400.0, 470.0));

    // Initialize state
    let initial_state = TempConverterState {
        temperature: 0.0,
        scale: Scales::Celcius,
    };

    // Launch app
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<TempConverterState> {
    // Header at the top
    let header = Label::new("Temperature converter").with_text_size(30.0);

    // Make a column for all the temperatures
    let mut temperatures = Flex::column();

    // Add all the temperatures to the column
    for scale in Scales::ALL {
        temperatures.add_child(
            Label::new(move |data: &TempConverterState, _env: &Env| {
                format!(
                    "{} {}",
                    data.scale.convert_to(scale, data.temperature),
                    scale.short()
                )
            })
            .with_text_size(25.0),
        );
    }

    // Temperature input
    let textbox = TextBox::new()
        .with_formatter(formatter::TemperatureFormatter)
        .lens(TempConverterState::temperature);

    // Label for the textbox that shows the current temperature
    let temp_label = Flex::row().with_child(textbox).with_child(Label::new(
        |data: &TempConverterState, _env: &Env| format!("{}", data.scale.short()),
    ));

    // Dropdown to select the temperature scale to convert from
    let dropdown = DropdownSelect::new(Scales::ALL.map(|i| (i.short_and_name(), i)))
        .lens(TempConverterState::scale);

    // Collect all the widgets
    let layout = Flex::column()
        .with_child(header)
        .with_child(temperatures)
        .with_spacer(10.)
        .with_child(temp_label)
        .with_spacer(10.)
        .with_child(dropdown);

    // Return all the widgets in a scrollable centered widget
    Align::centered(Scroll::new(layout))
}
