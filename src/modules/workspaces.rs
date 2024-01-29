use gtk::prelude::*;
use gtk::Box;
use gtk::Button;
use gtk::Orientation;
use hyprland::data::Workspaces;
use hyprland::dispatch::Dispatch;
use hyprland::dispatch::DispatchType;
use hyprland::dispatch::WorkspaceIdentifierWithSpecial;
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
        workspaces_box.set_widget_name("workspaces");

        for i in 1..11 {
            let button = Button::builder().label("").name(i.to_string()).build();
            button.connect_clicked(move |_btn| {
                let _ = Dispatch::call(DispatchType::Workspace(
                    WorkspaceIdentifierWithSpecial::Id(i),
                ));
            });
            workspaces_box.append(&button);
        }

        let mut listener = EventListener::new();
        let (tx, mut rx) = broadcast::channel(16);

        RUNTIME.spawn(async move {
            listener.add_workspace_change_handler(move |id| {
                let _ = tx.send(id.to_string());
            });
            // let _ = listener.start_listener();
        });

        glib::spawn_future_local(async move {
            while let Ok(response) = rx.recv().await {
                println!("{}", response);
            }
        });
        workspaces_box
    }
}
