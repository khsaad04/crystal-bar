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
    }
}

// Trait shared by all widgets
impl WidgetImpl for ClockButton {}

// Trait shared by all buttons
impl ButtonImpl for ClockButton {}
