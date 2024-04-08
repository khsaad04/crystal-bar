use crate::modules::{
    clock::ClockModule, sysinfo::SysinfoModule, window::WindowModule, workspaces::WorkspacesModule,
    Module,
};

use gtk::prelude::*;

pub fn layout() -> gtk::Box {
    let clock = ClockModule::default().into_widget();
    // let workspaces = WorkspacesModule::default().into_widget();
    // let window = WindowModule::default().into_widget();
    let sysinfo = SysinfoModule::default().into_widget();

    // Box

    let start_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    start_widgets.set_valign(gtk::Align::Center);
    start_widgets.set_halign(gtk::Align::Start);
    // start_widgets.append(&workspaces);
    // start_widgets.append(&window);

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
