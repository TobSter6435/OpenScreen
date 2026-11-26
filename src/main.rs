use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
fn main() {
    let mut count: i32 = 0;
    let app = app::App::default();
    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    let mut flex = Flex::default().with_size(120, 140).center_of_parent().column();
    let mut but_inc = Button::default().with_label("+");
    let mut frame = Frame::default().with_label("0");
    let mut but_dec = Button::default().with_label("-");

    but_inc.set_color(Color::LightGreen);
    but_dec.set_color(Color::Light1);
    flex.add(&but_inc);
    flex.add(&frame);
    flex.add(&but_dec);
    flex.end();
    wind.end();
    wind.show();
    app.run().unwrap();
}