mod imp {
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
}
use glib::Object;
use gtk::glib;
use gtk::Orientation;
use gtk::Label;
use gtk::Button;
use gtk::prelude::BoxExt;
glib::wrapper! {
    pub struct ActionWidget(ObjectSubclass<imp::ActionWidget>)
        @extends gtk::Box, gtk::Widget;
}
impl ActionWidget {
    pub fn new() -> Self {
        let output: Self = Object::builder().property("orientation", Orientation::Horizontal).build();
        let add_button = Button::builder()
            .label("+")
            .build();
        let label = Label::builder()
            .label("example action")
            .build();
        output.append(&add_button);
        output.append(&label);
        output
    }
    /*pub fn with_orientation(orientation: Orientation) -> Self {
        Object::builder().property("orientation", orientation).build()
    }*/
}
impl Default for ActionWidget {
    fn default() -> Self {
        Self::new()
    }
}
