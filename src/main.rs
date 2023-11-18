use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
const APP_ID: &str = "org.Snowbotics39131.mission_creator";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Snowbotics Mission Creator")
        .build();
    window.present();
}
