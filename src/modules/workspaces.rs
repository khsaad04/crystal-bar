use std::collections::HashMap;

use gtk::prelude::*;
use gtk::Box;
use gtk::Button;
use gtk::Orientation;
use hyprland::data::Workspace;
use hyprland::data::Workspaces;
use hyprland::dispatch::Dispatch;
use hyprland::dispatch::DispatchType;
use hyprland::dispatch::WorkspaceIdentifierWithSpecial;
use hyprland::event_listener::EventListener;
use hyprland::shared::HyprDataActive;
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

        let mut button_map: HashMap<i32, Button> = HashMap::new();

        for i in 1..11 {
            let button = Button::builder().label("").name(i.to_string()).build();
            button.connect_clicked(move |_btn| {
                let _ = Dispatch::call(DispatchType::Workspace(
                    WorkspaceIdentifierWithSpecial::Id(i),
                ));
            });
            workspaces_box.append(&button);
            button_map.insert(i, button);
        }

        {
            let workspace = Workspaces::get();
            for ws in workspace.unwrap().to_vec().iter() {
                let id = ws.id;
                button_map.get(&id).unwrap().add_css_class("occupied");
            }

            let active_workspace = Workspace::get_active().unwrap();
            button_map
                .get(&active_workspace.id)
                .unwrap()
                .add_css_class("active");
        }

        {
            let mut listener = EventListener::new();
            let (tx, mut rx) = broadcast::channel(1);

            RUNTIME.spawn(async move {
                listener.add_workspace_change_handler(move |id| {
                    let _ = tx.send(id.to_string());
                });
                let _ = listener.start_listener();
            });

            let button_map = button_map.clone();
            glib::spawn_future_local(async move {
                while let Ok(response) = rx.recv().await {
                    for (id, btn) in &button_map {
                        if *id.to_string() == response {
                            btn.add_css_class("active");
                            btn.add_css_class("occupied");
                        } else {
                            btn.remove_css_class("active");
                        }
                    }
                }
            });
        }

        {
            let mut listener = EventListener::new();
            let (tx, mut rx) = broadcast::channel(1);

            RUNTIME.spawn(async move {
                listener.add_workspace_destroy_handler(move |id| {
                    let _ = tx.send(id.to_string());
                });
                let _ = listener.start_listener();
            });

            let button_map = button_map.clone();
            glib::spawn_future_local(async move {
                while let Ok(response) = rx.recv().await {
                    for (id, btn) in &button_map {
                        if *id.to_string() == response {
                            btn.remove_css_class("occupied");
                        }
                    }
                }
            });
        }
        workspaces_box
    }
}
