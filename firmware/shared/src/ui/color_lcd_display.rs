use crate::{models::ui_state_model::UiState, ui::display_trait::DisplayTrait};
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::{
    Drawable as _,
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point, RgbColor, Size, WebColors},
    text::Text,
};
use heapless::format;

pub struct ColoredLcdDisplay<D>
where
    D: DrawTarget<Color = Rgb565>,
    D::Error: core::fmt::Debug,
{
    pub display: D,
}

impl<D> ColoredLcdDisplay<D>
where
    D: DrawTarget<Color = Rgb565>,
    D::Error: core::fmt::Debug,
{
    pub fn new(device: D) -> Self {
        Self { display: device }
    }
}

impl<D> DisplayTrait for ColoredLcdDisplay<D>
where
    D: DrawTarget<Color = Rgb565>,
    D::Error: core::fmt::Debug,
{
    fn update(&mut self, state: UiState) {
        const CHAR_HEIGHT: u32 = 30;
        const CHAR_WIDTH: u32 = 20;
        const START_POINT_X: i32 = 25;
        const START_POINT_Y: i32 = 50;
        const LINE_SPACING: i32 = 40;

        let small_green = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(CHAR_WIDTH, CHAR_HEIGHT))
            .digit_spacing(10)
            .segment_width(6)
            .segment_color(Rgb565::CSS_ORANGE)
            .build();

        let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);

        let temp = Text::new(
            "temp: ",
            Point::new(START_POINT_X, START_POINT_Y),
            text_style,
        )
        .draw(&mut self.display)
        .unwrap();

        let temp_value = Text::new(
            format!(10; "{} C", state.temperature).unwrap().as_str(),
            temp,
            small_green,
        )
        .draw(&mut self.display)
        .unwrap();

        let outval = Text::new(
            "outval: ",
            Point::new(START_POINT_X, temp_value.y + LINE_SPACING),
            text_style,
        )
        .draw(&mut self.display)
        .unwrap();

        let outval_value = Text::new(
            format!(10; "{}", state.output_val).unwrap().as_str(),
            outval,
            small_green,
        )
        .draw(&mut self.display)
        .unwrap();

        let setpoint = Text::new(
            "setpoint: ",
            Point::new(START_POINT_X, outval_value.y + LINE_SPACING),
            text_style,
        )
        .draw(&mut self.display)
        .unwrap();

        let _ = Text::new(
            format!(10; "{}", state.setpoint).unwrap().as_str(),
            setpoint,
            small_green,
        )
        .draw(&mut self.display)
        .unwrap();
    }
}
