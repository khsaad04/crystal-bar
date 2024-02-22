use gtk::glib;
use gtk::Label;
use hyprland::event_listener::EventListener;
use hyprland::shared::HyprDataActiveOptional;
use tokio::sync::broadcast;

use crate::modules::Module;
use crate::RUNTIME;

#[derive(Default)]
pub struct WindowModule {}

impl Module<Label> for WindowModule {
    fn into_widget(self) -> Label {
        let window = Label::new(None);
        window.set_label(&get_current_active_window().unwrap_or("".to_string()));

        {
            let mut listener = EventListener::new();
            let (tx, mut rx) = broadcast::channel(10);

            RUNTIME.spawn(async move {
                let tx_1 = tx.clone();
                listener.add_active_window_change_handler(move |id| {
                    let _ = tx_1.send(id.unwrap().window_title);
                });

                let tx_2 = tx.clone();
                listener.add_workspace_change_handler(move |id| {
                    let _ = tx_2.send(format!("c:{}", id));
                });

                let _ = listener.start_listener();
            });

            let window = window.clone();
            glib::spawn_future_local(async move {
                while let Ok(response) = rx.recv().await {
                    if !response.starts_with('c') {
                        window.set_label(&response);
                    } else {
                        window
                            .clone()
                            .set_label(&get_current_active_window().unwrap_or("".to_string()));
                    }
                }
            });
        }

        window
    }
}

fn get_current_active_window() -> Option<String> {
    let client = hyprland::data::Client::get_active().unwrap();
    match client {
        Some(w) => Some(w.title),
        None => None,
    }
}
