use std::collections::HashMap;

use gtk::glib;
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

#[derive(Default)]
pub struct WorkspacesModule {}

impl Module<Box> for WorkspacesModule {
    fn into_widget(self) -> Box {
        let workspaces_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(5)
            .name("workspaces")
            .build();

        let mut button_map: HashMap<i32, Button> = HashMap::new();

        for i in 1..11 {
            let button = Button::builder().label("").name(i.to_string()).build();
            button.connect_clicked(move |_btn| {
                let _ = Dispatch::call(DispatchType::Workspace(
                    WorkspaceIdentifierWithSpecial::Id(i),
                ));
            });
            workspaces_box.add(&button);
            button_map.insert(i, button);
        }

        // Init
        let workspaces = Workspaces::get();
        for ws in workspaces.unwrap().to_vec().iter() {
            let id = ws.id;
            button_map.get(&id).unwrap().set_widget_name("occupied");
        }

        let active_workspace = Workspace::get_active().unwrap();
        button_map
            .get(&active_workspace.id)
            .unwrap()
            .set_widget_name("active");

        {
            let mut listener = EventListener::new();
            let (tx, mut rx) = broadcast::channel(10);

            RUNTIME.spawn(async move {
                let tx_1 = tx.clone();
                listener.add_workspace_change_handler(move |id| {
                    let _ = tx_1.send(format!("c:{}", id));
                });

                let tx_2 = tx.clone();
                listener.add_workspace_destroy_handler(move |id| {
                    let _ = tx_2.send(format!("d:{}", id));
                });
                let _ = listener.start_listener();
            });

            let button_map = button_map.clone();

            glib::spawn_future_local(async move {
                while let Ok(response) = rx.recv().await {
                    if response.starts_with('c') {
                        for (id, btn) in &button_map {
                            if id.to_string() == response[2..] {
                                btn.set_widget_name("active");
                                btn.set_widget_name("occupied");
                            } else {
                                btn.set_widget_name("");
                            }
                        }
                    } else {
                        for (id, btn) in &button_map {
                            if id.to_string() == response[2..] {
                                btn.set_widget_name("");
                            }
                        }
                    }
                }
            });
        }

        workspaces_box
    }
}
