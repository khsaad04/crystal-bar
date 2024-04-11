use crate::modules::{
    clock::ClockModule, hypr_window::WindowModule, hypr_workspaces::HyprWorkspacesModule,
    sway_workspaces::SwayWorkspacesModule, sysinfo::SysinfoModule, Module,
};

use gtk::prelude::*;

pub fn layout() -> gtk::Box {
    let clock = ClockModule::default().callback();
    let workspaces = SwayWorkspacesModule::default().callback();
    // let window = WindowModule::default().callback();
    let sysinfo = SysinfoModule::default().callback();

    // Box

    let start_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    start_widgets.set_valign(gtk::Align::Center);
    start_widgets.set_halign(gtk::Align::Start);
    start_widgets.add(&workspaces);
    // start_widgets.add(&window);

    let center_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    center_widgets.set_valign(gtk::Align::Center);
    center_widgets.set_halign(gtk::Align::Center);
    center_widgets.set_hexpand(true);
    center_widgets.add(&clock);

    let end_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    end_widgets.set_valign(gtk::Align::Center);
    end_widgets.set_halign(gtk::Align::End);
    end_widgets.add(&sysinfo);

    gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_start(5)
        .margin_end(5)
        .child(&start_widgets)
        .child(&center_widgets)
        .child(&end_widgets)
        .build()
}
