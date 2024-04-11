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
pub struct HyprWorkspacesModule {}

impl Module<Box> for HyprWorkspacesModule {
    fn callback(self) -> Box {
        let workspaces_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(5)
            .name("workspaces")
            .build();

        let mut buttons: Vec<Button> = vec![];

        for i in 1..11 {
            let button = Button::builder()
                .label(i.to_string())
                .name(i.to_string())
                .build();
            button.connect_clicked(move |_btn| {
                change_workspace(i);
            });
            workspaces_box.add(&button);
            buttons.push(button);
        }

        // Init
        let workspaces = Workspaces::get().expect("Failed to get workspaces");
        for ws in workspaces.to_vec().iter() {
            let id = ws.id;
            buttons[id as usize].set_widget_name("occupied");
        }

        let active_workspace = Workspace::get_active().expect("Missing active workspace button");
        buttons[active_workspace.id as usize].set_widget_name("active");

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

            glib::spawn_future_local(async move {
                while let Ok(response) = rx.recv().await {
                    if response.starts_with('c') {
                        for (id, btn) in buttons.clone().into_iter().enumerate() {
                            if id.to_string() == response[2..] {
                                btn.set_widget_name("active");
                            }
                        }
                    } else {
                        for (id, btn) in buttons.clone().into_iter().enumerate() {
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

fn change_workspace(id: i32) {
    let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
        id,
    )));
}
