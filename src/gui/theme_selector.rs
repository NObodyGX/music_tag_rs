use gtk::subclass::prelude::*;
use gtk::{gdk, glib, CompositeTemplate, CssProvider};
use gtk::{prelude::*, style_context_add_provider_for_display};

glib::wrapper! {
    pub struct ThemeSelector(ObjectSubclass<imp::ThemeSelector>)
        @extends gtk::Widget,
        @implements gtk::Accessible;
}

impl Default for ThemeSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeSelector {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/musictag/ui/theme_selector.ui")]
    pub struct ThemeSelector {
        #[template_child(id = "box")]
        pub gbox: TemplateChild<gtk::Box>,
        #[template_child(id = "system")]
        pub system: TemplateChild<gtk::ToggleButton>,
        #[template_child(id = "dark")]
        pub dark: TemplateChild<gtk::ToggleButton>,
        #[template_child(id = "light")]
        pub light: TemplateChild<gtk::ToggleButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ThemeSelector {
        const NAME: &'static str = "ThemeSelector";
        type Type = super::ThemeSelector;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            load_css();
            klass.set_css_name("themeselector");
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ThemeSelector {
        fn dispose(&self) {
            self.gbox.unparent();
        }
    }
    impl WidgetImpl for ThemeSelector {}
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/com/github/nobodygx/musictag/themes/theme_selector.css");

    // Add the provider to the default screen
    style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
