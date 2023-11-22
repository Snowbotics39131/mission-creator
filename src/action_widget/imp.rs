use gtk::glib;
use gtk::subclass::prelude::*;
#[derive(Default)]
pub struct ActionWidget;
#[glib::object_subclass]
impl ObjectSubclass for ActionWidget {
    const NAME: &'static str = "MissionCreatorActionWidget";
    type Type = super::ActionWidget;
    type ParentType = gtk::Box;
}
impl ObjectImpl for ActionWidget {}
impl WidgetImpl for ActionWidget {}
impl BoxImpl for ActionWidget {}
