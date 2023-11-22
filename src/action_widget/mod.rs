mod imp;

use glib::Object;
use gtk::glib;
use gtk::Orientation;
use glib::object::ObjectBuilder;

glib::wrapper! {
    pub struct ActionWidget(ObjectSubclass<imp::ActionWidget>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl<O> ActionWidget {
    pub fn builder() -> ObjectBuilder<'static, O> {
        Object::builder()
    }
    pub fn orientation(orientation: Orientation) -> ObjectBuilder<'static, O> {
        Object::builder().orientation(orientation)
    }
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for ActionWidget {
    fn default() -> Self {
        Self::new()
    }
}
