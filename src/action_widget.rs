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
use gtk::{glib, Orientation, Button, prelude::BoxExt, DropDown, prelude::ButtonExt, prelude::WidgetExt};
use glib::{clone, Object};
glib::wrapper! {
    pub struct ActionWidget(ObjectSubclass<imp::ActionWidget>)
        @extends gtk::Box, gtk::Widget;
}
const ACTIONS: [&str; 3] = ["DriveStraightAction", "DriveTurnAction", "DriveCurveAction"];
impl ActionWidget {
    pub fn new() -> Self {
        let output: Self = Object::builder().property("orientation", Orientation::Horizontal).build();
        let add_button = Button::builder()
            .label("+")
            .build();
        let action_drop_down = DropDown::from_strings(&ACTIONS.as_slice());
        let up_down_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();
        let up_button = Button::builder()
            .label("^")
            .build();
        let down_button = Button::builder()
            .label("v")
            .build();
        let delete_button = Button::builder()
            .label("X")
            .build();
        delete_button.connect_clicked(clone!(@weak output => move |_| {
            output.set_visible(false); 
        }));
        up_down_box.append(&up_button);
        up_down_box.append(&down_button);
        output.append(&add_button);
        output.append(&action_drop_down);
        output.append(&up_down_box);
        output.append(&delete_button);
        output
    }
}
impl Default for ActionWidget {
    fn default() -> Self {
        Self::new()
    }
}
