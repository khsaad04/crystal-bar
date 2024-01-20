mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct ClockButton(ObjectSubclass<imp::ClockButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ClockButton {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for ClockButton {
    fn default() -> Self {
        Self::new()
    }
}
