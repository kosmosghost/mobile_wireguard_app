mod imp;

use crate::app;

use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use adw::Application;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_callbacks(&self) {
        self.imp().connect_disconnect_button.connect_clicked(
            clone!(@weak self as window => move |_| {
                window.connect_disconnect_callback()
            }),
        );

        self.imp()
            .reconnect_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.reconnect_callback()
            }));
    }

    fn connect_disconnect_callback(&self) {
        if self.imp().connected.get() == false {
            let interface = app::connect_wg();
            let button = &self.imp().connect_disconnect_button;
            button.set_label("Disconnect");
            self.imp().connected.set(true);
            let wireguard_conf_label = &self.imp().wireguard_conf;
            wireguard_conf_label.set_text(&interface);
        } else {
            app::disconnect_wg();
            self.imp().connect_disconnect_button.set_label("Connect");
            self.imp().connected.set(false);
            let wireguard_conf_label = &self.imp().wireguard_conf;
            wireguard_conf_label.set_text("");
        }
    }

    fn reconnect_callback(&self) {
        let interface = app::refresh_wg();
        let wireguard_conf_label = &self.imp().wireguard_conf;
        wireguard_conf_label.set_text(&interface);
    }
}
