mod imp;

use std::process::{Command, Stdio};

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

    fn setup_connect_button(&self) {
        let mut string_buffer = String::new();
        let connection = app::check_connection();
        match connection {
            Some(x) => {
                self.imp().connected.set(true);
                self.imp().connect_disconnect_button.set_label("Disconnect");
                self.imp().wireguard_conf.set_text(&x);
            },
            None => (),
        }
    }

    fn connect_disconnect_callback(&self) {
        if self.imp().connected.get() == false {
            let interface = Command::new("mobile_wireguard_app_helper").arg("connect").output().unwrap();

            let button = &self.imp().connect_disconnect_button;
            button.set_label("Disconnect");
            self.imp().connected.set(true);
            let wireguard_conf_label = &self.imp().wireguard_conf;
            let buffer = String::from_utf8(interface.stdout).unwrap();
            wireguard_conf_label.set_text(&buffer.as_str());
        } else {
            let _interface = Command::new("mobile_wireguard_app_helper").arg("disconnect").output().unwrap();
            self.imp().connect_disconnect_button.set_label("Connect");
            self.imp().connected.set(false);
            let wireguard_conf_label = &self.imp().wireguard_conf;
            wireguard_conf_label.set_text("");
        }
    }

    fn reconnect_callback(&self) {
        let interface = Command::new("mobile_wireguard_app_helper").arg("refresh").output().unwrap();
        let wireguard_conf_label = &self.imp().wireguard_conf;
        let buffer = String::from_utf8(interface.stdout).unwrap();
        wireguard_conf_label.set_text(&buffer);
    }
}
