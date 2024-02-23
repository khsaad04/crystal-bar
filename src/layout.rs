use crate::modules::{
    clock::ClockModule, sysinfo::SysinfoModule, window::WindowModule, workspaces::WorkspacesModule,
    Module,
};

use gtk::prelude::*;

pub fn layout() -> gtk::CenterBox {
    let clock = ClockModule::default().into_widget();
    let workspaces = WorkspacesModule::default().into_widget();
    let window = WindowModule::default().into_widget();
    let sysinfo = SysinfoModule::default().into_widget();

    // Box

    let start_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    start_widgets.append(&workspaces);
    start_widgets.append(&window);

    let center_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    center_widgets.append(&clock);

    let end_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    end_widgets.append(&sysinfo);

    gtk::CenterBox::builder()
        .start_widget(&start_widgets)
        .center_widget(&center_widgets)
        .end_widget(&end_widgets)
        .build()
}
