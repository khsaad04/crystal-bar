use crate::modules::Module;
use crate::RUNTIME;
use chrono::Local;
use gtk::glib;
use gtk::prelude::*;
use gtk::Button;
use tokio::sync::mpsc;
use tokio::time::sleep;

pub struct ClockModule {}

impl Module<Button> for ClockModule {
    fn new() -> Self {
        Self {}
    }

    fn into_widget(self) -> Button {
        let label = Button::default();
        label.set_margin_top(5);
        label.set_margin_bottom(5);

        let current_time_str = current_time();
        label.set_label(&current_time_str);

        let (tx, mut rx) = mpsc::channel(1);

        RUNTIME.spawn(async move {
            loop {
                let _ = tx.send(current_time()).await;
                sleep(tokio::time::Duration::from_millis(500)).await;
            }
        });

        let time = label.clone();
        glib::spawn_future_local(async move {
            while let Some(response) = rx.recv().await {
                time.set_label(&response);
            }
        });
        label
    }
}

fn current_time() -> String {
    format!("{}", Local::now().format("󰥔 %I:%M %p"))
}
