use gtk::glib;
use gtk::Label;
use hyprland::event_listener::EventListener;
use hyprland::shared::HyprDataActiveOptional;
use tokio::sync::broadcast;

use crate::modules::Module;
use crate::RUNTIME;

pub struct WindowModule {}

impl Module<Label> for WindowModule {
    fn new() -> Self {
        Self {}
    }

    fn into_widget(self) -> Label {
        let window = Label::new(None);
        let current_active_window = hyprland::data::Client::get_active();
        window.set_label(&current_active_window.unwrap().unwrap().title);

        {
            let mut listener = EventListener::new();
            let (tx, mut rx) = broadcast::channel(10);

            RUNTIME.spawn(async move {
                listener.add_active_window_change_handler(move |id| {
                    let _ = tx.send(id);
                });
                let _ = listener.start_listener();
            });

            let window = window.clone();
            glib::spawn_future_local(async move {
                while let Ok(response) = rx.recv().await {
                    window.set_label(&response.unwrap().window_title);
                }
            });
        }

        window
    }
}
