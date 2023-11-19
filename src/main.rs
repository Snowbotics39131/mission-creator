use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Orientation, Button, Label, Arrow};
const APP_ID: &str = "org.Snowbotics39131.mission_creator";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let hor_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    let add_button = Button::builder()
        .label("+")
        .build();
    let action_label = Label::builder()
        .label("ExampleAction")
        .build();
    let up_down_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    /*let up_button = Button::builder()
        .label("^")
        .build();
    let down_button = Button::builder()
        .label("v")
        .build();*/
    let up_button = Arrow::new()
        .build();
    let down_button = Arrow::new()
        .build();
    let delete_button = Button::builder()
        .label("X")
        .build();
    up_down_box.append(&up_button);
    up_down_box.append(&down_button);
    hor_box.append(&add_button);
    hor_box.append(&action_label);
    hor_box.append(&up_down_box);
    hor_box.append(&delete_button);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Snowbotics Mission Creator")
        .child(&hor_box)
        .build();
    window.present();
}
