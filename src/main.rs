mod application;
mod config;
mod core;
mod gui;
use application::MTApplication;
use gtk::prelude::*;
use gtk::{gio, glib};

use config::{APP_ID, PKGDATA_DIR};

fn main() -> glib::ExitCode {
    // Register and include resources
    let resources = gio::Resource::load(PKGDATA_DIR.to_owned() + "/musictag.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new application
    let app = MTApplication::new(APP_ID, &gio::ApplicationFlags::empty());

    app.connect_startup(|app| {
        setup_shortcuts(app);
    });

    app.run()
}

fn setup_shortcuts(app: &MTApplication) {
    app.set_accels_for_action("app.quit", &["<Ctrl>q"]);
    app.set_accels_for_action("win.execute", &["<Ctrl>r"]);
    app.set_accels_for_action("win.clear_all", &["<Ctrl>BackSpace"]);
    app.set_accels_for_action("win.switch_tab", &["<Ctrl>h"]);
}
