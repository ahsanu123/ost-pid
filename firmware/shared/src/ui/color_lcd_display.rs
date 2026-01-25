use crate::{models::ui_state_model::UiState, ui::display_trait::DisplayTrait};
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::{
    Drawable as _,
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point, RgbColor as _, Size},
    text::Text,
};

pub struct ColoredLcdDisplay<D>
where
    D: DrawTarget<Color = Rgb565>,
{
    pub display: D,
}

impl<D> ColoredLcdDisplay<D>
where
    D: DrawTarget<Color = Rgb565>,
{
    pub fn new(device: D) -> Self {
        Self { display: device }
    }
}

impl<D> DisplayTrait for ColoredLcdDisplay<D>
where
    D: DrawTarget<Color = Rgb565>,
{
    fn update(&mut self, state: UiState) {
        let small_green = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(20, 30))
            .digit_spacing(10)
            .segment_width(6)
            .segment_color(Rgb565::GREEN)
            .build();

        let large_red = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(20, 40))
            .digit_spacing(10)
            .segment_width(6)
            .segment_color(Rgb565::RED)
            .inactive_segment_color(Rgb565::new(0x30, 0x00, 0x00))
            .build();

        Text::new("-----\n12:42\n13°C\n-----", Point::new(10, 30), small_green)
            .draw(&mut self.display)
            .ok();

        Text::new("3.141", Point::new(150, 130), large_red)
            .draw(&mut self.display)
            .ok();
    }
}
