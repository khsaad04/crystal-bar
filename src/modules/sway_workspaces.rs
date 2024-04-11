use futures_lite::StreamExt;
use gtk::glib;
use gtk::prelude::*;
use gtk::Box;
use gtk::Button;
use gtk::Orientation;
use swayipc_async::{Connection, Event, EventType};
use tokio::sync::broadcast;

use crate::modules::Module;
use crate::RUNTIME;

#[derive(Default)]
pub struct SwayWorkspacesModule {}

impl Module<Box> for SwayWorkspacesModule {
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
                RUNTIME.spawn(async move {
                    let connection = Connection::new().await;
                    connection
                        .unwrap()
                        .run_command(format!("workspace number {i}"))
                        .await
                        .unwrap();
                });
            });
            workspaces_box.add(&button);
            buttons.push(button);
        }

        {
            let (tx, mut rx) = broadcast::channel(10);

            RUNTIME.spawn(async move {
                let connection = Connection::new().await;
                let workspaces = connection.unwrap().get_workspaces().await.unwrap();
                let subs = [EventType::Workspace];
                let mut events = Connection::new()
                    .await
                    .unwrap()
                    .subscribe(subs)
                    .await
                    .unwrap();
                while let Some(event) = events.next().await {
                    if let Event::Workspace(event) = event.unwrap() {
                        let _ = tx.send(format!("a:{}", event.current.unwrap().name.unwrap()));
                    }
                }
                for ws in workspaces.iter() {
                    let id = ws.name.parse::<usize>().expect("Couldn't parse to usize");
                    let _ = tx.send(format!("o:{id}"));
                }
            });

            glib::spawn_future_local(async move {
                while let Ok(response) = rx.recv().await {
                    if response.starts_with('a') {
                        for (id, btn) in buttons.clone().into_iter().enumerate() {
                            if (id + 1).to_string() == response[2..] {
                                btn.set_widget_name("active");
                            } else {
                                btn.set_widget_name("");
                            }
                        }
                    } else if response.starts_with('o') {
                        for (id, btn) in buttons.clone().into_iter().enumerate() {
                            if (id + 1).to_string() == response[2..] {
                                btn.set_widget_name("occupied");
                            }
                        }
                    }
                }
            });
        }

        workspaces_box
    }
}
