use gtk::glib;
use gtk::prelude::LabelExt;
use gtk::Label;
use hyprland::event_listener::EventListener;
use hyprland::shared::HyprDataActiveOptional;
use tokio::sync::broadcast;

use crate::modules::Module;
use crate::RUNTIME;

#[derive(Default)]
pub struct WindowModule {}

impl Module<Label> for WindowModule {
    fn callback(self) -> Label {
        let window = Label::builder().label(get_active_window()).build();

        {
            let mut listener = EventListener::new();
            let (tx, mut rx) = broadcast::channel(10);

            RUNTIME.spawn(async move {
                let tx_1 = tx.clone();
                listener.add_active_window_change_handler(move |id| {
                    let _ = tx_1.send(
                        id.expect("Failed to get window id")
                            .window_title
                            .to_string(),
                    );
                });

                let tx_2 = tx.clone();
                listener.add_workspace_change_handler(move |id| {
                    let _ = tx_2.send(id.to_string());
                });

                let _ = listener.start_listener();
            });

            let window = window.clone();
            glib::spawn_future_local(async move {
                while let Ok(_response) = rx.recv().await {
                    window.set_label(&get_active_window());
                }
            });
        }

        window
    }
}

fn get_active_window() -> String {
    let client = hyprland::data::Client::get_active().expect("Failed to get active window");
    match client {
        Some(w) => truncate(w.title),
        None => "".to_string(),
    }
}

fn truncate(title: String) -> String {
    let n = title.len();
    if n > 25 {
        format!("{}...", &title[0..22])
    } else {
        title
    }
}
