use gtk::prelude::*;
use gtk::Orientation;
use gtk::{Box, Label};
use hyprland::data::Workspaces;
use hyprland::event_listener::EventListener;
use hyprland::shared::{HyprData, HyprDataVec};
use tokio::sync::broadcast;

use crate::modules::Module;
use crate::RUNTIME;

pub struct WorkspacesModule {}

impl Module<Box> for WorkspacesModule {
    fn new() -> Self {
        Self {}
    }

    fn into_widget(self) -> Box {
        let workspaces_box = Box::new(Orientation::Horizontal, 5);

        let workspace = Workspaces::get();
        workspace.unwrap().to_vec().iter().for_each(|wp| {
            let label = Label::new(Some(&wp.name));
            workspaces_box.append(&label);
        });

        let label = gtk::Label::default();
        label.set_margin_top(5);
        label.set_margin_bottom(5);

        let mut listener = EventListener::new();
        let (tx, mut rx) = broadcast::channel(10);

        RUNTIME.spawn(async move {
            listener.add_workspace_change_handler(move |id| {
                let _ = tx.send(id.to_string());
            });
            let _ = listener.start_listener();
        });

        let label_clone = label.clone();
        glib::spawn_future_local(async move {
            while let Ok(response) = rx.recv().await {
                label_clone.set_text(&response);
            }
        });
        workspaces_box
    }
}
