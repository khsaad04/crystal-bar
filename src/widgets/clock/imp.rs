use chrono::Local;
use gtk::glib;
use gtk::prelude::{ButtonExt, WidgetExt};
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct ClockButton;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ClockButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::ClockButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for ClockButton {
    fn constructed(&self) {
        self.parent_constructed();

        self.obj().set_label("CLOCK");
        self.obj().set_margin_top(5);
        self.obj().set_margin_bottom(5);

        // let time = current_time();
        // let label = gtk::Label::default();
        //
        // label.set_text(&time);
        //
        // let tick = move || {
        //     let time = current_time();
        //     self.obj().set_label(&time);
        //     glib::ControlFlow::Continue
        // };
        //
        // glib::timeout_add_seconds_local(1, tick);
    }
}

// fn current_time() -> String {
//     format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
// }

// Trait shared by all widgets
impl WidgetImpl for ClockButton {}

// Trait shared by all buttons
impl ButtonImpl for ClockButton {}
