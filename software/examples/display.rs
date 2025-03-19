use ssd1306::{I2CDisplayInterface, Ssd1306, mode::BufferedGraphicsMode, size::DisplaySize128x64};
use embedded_graphics::mono_font::{ascii::FONT_5X8, MonoTextStyleBuilder};
use embedded_graphics::text::Text;
use embedded_graphics::prelude::*;
use core::fmt::Write;

pub fn update_display(
    display: &mut Ssd1306<I2CDisplayInterface, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>, 
    time: u32
) {
    let text_style = MonoTextStyleBuilder::new().font(&FONT_5X8).build();
    Text::new(&format_args!("Time: {}s", time).to_string(), Point::new(10, 10), text_style)
        .draw(display)
        .unwrap();
}
