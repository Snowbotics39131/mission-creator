use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Orientation, Button};
use glib::clone;
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
    let add_action_button = Button::builder()
        .label("+")
        .build();
    add_action_button.connect_clicked(clone!(@weak main_box => move |_| {
        main_box.append(&action_widget::ActionWidget::new());
    }));
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
