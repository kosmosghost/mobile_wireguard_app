mod window;
mod app;

use gtk::prelude::*;
use gtk::{gio, Application};
use window::Window;

fn main() {
    // Register and include resources
    gio::resources_register_include!("mobilewireguardapp.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder()
        .application_id("org.kosmosghost.mobilewireguardapp")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.present();
}