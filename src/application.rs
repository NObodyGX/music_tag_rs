use crate::config::{APP_ID, APP_NAME, APP_URL, VERSION};
use crate::gui::MainPreferences;
use crate::gui::MainWindow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::{gio, glib};

mod imp {

    use super::*;

    #[derive(Debug, Default)]
    pub struct MTApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for MTApplication {
        const NAME: &'static str = "MTApplication";
        type Type = super::MTApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for MTApplication {
        fn constructed(&self) {
            let obj = self.obj();
            self.parent_constructed();

            obj.setup_gactions();
        }
    }
    impl ApplicationImpl for MTApplication {
        fn activate(&self) {
            self.parent_activate();

            let obj = self.obj();
            let app = obj.downcast_ref::<super::MTApplication>().unwrap();
            let window = app.create_window();
            window.set_default_size(1280, 720);
            // todo: read settings
            window.set_title(Some(APP_NAME));
            window.set_icon_name(Some(APP_NAME));
            window.present();
        }
    }
    impl AdwApplicationImpl for MTApplication {}
    impl GtkApplicationImpl for MTApplication {}
}

glib::wrapper! {
    pub struct MTApplication(ObjectSubclass<imp::MTApplication>)
        @extends adw::Application, gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl MTApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }
    fn create_window(&self) -> MainWindow {
        let window = MainWindow::new(&self.clone());

        window.present();
        window
    }

    fn setup_gactions(&self) {
        let preferences_action = gio::SimpleAction::new("preferences", None);
        preferences_action.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_action, _parameter| {
                app.show_prefrerences();
            }
        ));
        self.add_action(&preferences_action);

        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_action, _parameter| {
                app.quit();
            }
        ));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_action, _parameter| {
                app.show_about();
            }
        ));
        self.add_action(&about_action);
    }

    fn show_prefrerences(&self) {
        let window = self.active_window().unwrap();
        let preferences = MainPreferences::new();
        preferences.set_modal(true);
        preferences.set_transient_for(Some(&window));
        preferences.connect_font_changed(clone!(
            #[weak]
            window,
            move |_| {
                window
                    .activate_action("win.refresh_text_view_font", None)
                    .unwrap();
            },
        ));
        preferences.present();
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let dialog = gtk::AboutDialog::builder()
            .transient_for(&window)
            .modal(true)
            .program_name(APP_NAME)
            .logo_icon_name(APP_NAME)
            .version(VERSION)
            .authors(vec!["nobodygx"])
            .website(APP_URL)
            .license_type(gtk::License::MitX11)
            .build();

        dialog.present();
    }
}

impl Default for MTApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .build()
    }
}
