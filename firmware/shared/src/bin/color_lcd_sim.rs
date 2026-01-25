use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use frr_shared::DisplayTrait;
use frr_shared::{ColoredLcdDisplay, UiState};
use std::time::Duration;

fn main() {
    let display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 170));
    let mut window = Window::new("Digital clock", &OutputSettings::default());

    let mut lcd = ColoredLcdDisplay::new(display);
    lcd.update(UiState::default());
    // window.show_static(&lcd.device);

    loop {
        lcd.display.clear(Rgb565::default()).unwrap();
        lcd.update(UiState::default());
        window.update(&lcd.display);

        if window.events().any(|event| event == SimulatorEvent::Quit) {
            break;
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}
