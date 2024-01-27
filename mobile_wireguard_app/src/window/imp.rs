use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Entry, Label};

use std::cell::Cell;

use crate::app;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/kosmosghost/mobilewireguardapp/window.xml")]
pub struct Window {
    #[template_child]
    pub connect_disconnect_button: TemplateChild<Button>,
    #[template_child]
    pub reconnect_button: TemplateChild<Button>,
    #[template_child]
    pub wireguard_conf: TemplateChild<Label>,

    pub connected: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "mobile_wireguard_app";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_callbacks();
        obj.setup_connect_button();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
