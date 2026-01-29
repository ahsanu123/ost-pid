use crate::{displays::display_trait::DisplayTrait, models::ui_state_model::UiState};
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::{
    Drawable as _,
    mono_font::{MonoTextStyle, MonoTextStyleBuilder, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point, Primitive as _, RgbColor, Size, WebColors},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Triangle},
    text::Text,
};
use embedded_layout::{
    View as _,
    align::{Align, horizontal, vertical},
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
        // https://rgbcolorpicker.com/565
        const CHAR_HEIGHT: u32 = 70;
        const CHAR_WIDTH: u32 = 50;
        const START_POINT_X: i32 = 15;
        const START_POINT_Y: i32 = 20;

        let display_area = self.display.bounding_box();

        let active_screen = state.screen.to_string();
        let pinkist_color = Rgb565::new(31, 26, 22);
        let greenist_color = Rgb565::new(29, 62, 27);

        let bg_style = PrimitiveStyleBuilder::new()
            .fill_color(greenist_color)
            .build();

        Rectangle::new(Point::new(0, 30), Size::new(200, 100))
            .into_styled(bg_style)
            .draw(&mut self.display)
            .unwrap();

        let segment_style = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(CHAR_WIDTH, CHAR_HEIGHT))
            .digit_spacing(6)
            .segment_width(12)
            .segment_color(pinkist_color)
            .build();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(Rgb565::WHITE)
            .build();

        let active_screen_text = Text::new(
            active_screen,
            Point::new(START_POINT_X, START_POINT_Y),
            text_style,
        )
        .draw(&mut self.display)
        .unwrap();

        let sp_text = Text::new(
            format!(10; "sp -> {}", state.setpoint).unwrap().as_str(),
            Point::new(20, 20),
            text_style,
        )
        .align_to(&display_area, horizontal::Right, vertical::Center)
        .translate(Point::new(-10, -25))
        .draw(&mut self.display)
        .unwrap();

        let sp_text = Text::new(
            format!(10; "e -> {}", state.setpoint).unwrap().as_str(),
            Point::new(20, 20),
            text_style,
        )
        .align_to(&display_area, horizontal::Right, vertical::Center)
        .translate(Point::new(-10, -10))
        .draw(&mut self.display)
        .unwrap();

        let sp_text = Text::new(
            format!(10; "p -> {}", state.output_val).unwrap().as_str(),
            sp_text,
            text_style,
        )
        .align_to(&display_area, horizontal::Right, vertical::Center)
        .translate(Point::new(-10, 5))
        .draw(&mut self.display)
        .unwrap();

        let sp_text = Text::new(
            format!(10; "i -> {}", state.setpoint).unwrap().as_str(),
            sp_text,
            text_style,
        )
        .align_to(&display_area, horizontal::Right, vertical::Center)
        .translate(Point::new(-10, 20))
        .draw(&mut self.display)
        .unwrap();

        let sp_text = Text::new(
            format!(10; "d -> {}", state.setpoint).unwrap().as_str(),
            sp_text,
            text_style,
        )
        .align_to(&display_area, horizontal::Right, vertical::Center)
        .translate(Point::new(-10, 35))
        .draw(&mut self.display)
        .unwrap();

        let value_text = Text::new(
            format!(10; "{}", state.temperature).unwrap().as_str(),
            active_screen_text,
            segment_style,
        )
        .align_to(&display_area, horizontal::Left, vertical::Center)
        .translate(Point::new(15, 0))
        .draw(&mut self.display)
        .unwrap();

        let triangle_style = PrimitiveStyleBuilder::new().fill_color(Rgb565::RED).build();

        Triangle::new(Point::new(40, 40), Point::new(30, 50), Point::new(40, 60))
            .translate(Point::new(0, value_text.y - 20))
            .into_styled(triangle_style)
            .draw(&mut self.display)
            .unwrap();

        Triangle::new(Point::new(40, 40), Point::new(50, 50), Point::new(40, 60))
            .translate(Point::new(40, value_text.y - 20))
            .into_styled(triangle_style)
            .draw(&mut self.display)
            .unwrap();

        Triangle::new(Point::new(40, 40), Point::new(30, 50), Point::new(50, 50))
            .translate(Point::new(80, value_text.y - 20))
            .into_styled(triangle_style)
            .draw(&mut self.display)
            .unwrap();

        Triangle::new(Point::new(40, 60), Point::new(30, 50), Point::new(50, 50))
            .translate(Point::new(120, value_text.y - 20))
            .into_styled(triangle_style)
            .draw(&mut self.display)
            .unwrap();
    }
}
