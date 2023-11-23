use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Orientation, Button};
mod action_widget;
const APP_ID: &str = "org.Snowbotics39131.mission_creator";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    let compile_button = Button::builder()
        .label("Compile to Python")
        .build();
    /*let action_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();*/
    /*let action_box = action_widget::ActionWidget::with_orientation(Orientation::Horizontal);
    let add_button = Button::builder()
        .label("+")
        .build();
    let action_label = DropDown::from_strings(&actions.as_slice());
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
        .build();*/
    let add_action_button = Button::builder()
        .label("+")
        .build();
    /*up_down_box.append(&up_button);
    up_down_box.append(&down_button);
    action_box.append(&add_button);
    action_box.append(&action_label);
    action_box.append(&up_down_box);
    action_box.append(&delete_button);*/
    let action_box = action_widget::ActionWidget::new();
    main_box.append(&compile_button);
    main_box.append(&action_box);
    main_box.append(&add_action_button);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Snowbotics Mission Creator")
        .child(&main_box)
        .build();
    window.present();
}
