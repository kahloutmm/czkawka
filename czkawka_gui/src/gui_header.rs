use crate::fl;
use gtk4::prelude::*;use gtk4::Inhibit;

#[derive(Clone)]
pub struct GuiHeader {
    pub button_settings: gtk4::Button,
    pub button_app_info: gtk4::Button,
}

impl GuiHeader {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let button_settings: gtk4::Button = builder.object("button_settings").unwrap();
        let button_app_info: gtk4::Button = builder.object("button_app_info").unwrap();

        Self { button_settings, button_app_info }
    }

    pub fn update_language(&self) {
        self.button_settings.set_tooltip_text(Some(&fl!("header_setting_button_tooltip")));
        self.button_app_info.set_tooltip_text(Some(&fl!("header_about_button_tooltip")));
    }
}
