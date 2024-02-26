use crate::modules::Module;
use crate::RUNTIME;
use chrono::Local;
use gtk::glib;
use gtk::prelude::*;
use gtk::Button;
use tokio::sync::mpsc;
use tokio::time::sleep;

pub struct ClockModule {
    format: String,
}

impl Default for ClockModule {
    fn default() -> Self {
        Self {
            format: "󰥔 %I:%M %p".to_string(),
        }
    }
}

impl Module<Button> for ClockModule {
    fn into_widget(self) -> Button {
        let button = Button::builder()
            .margin_top(5)
            .margin_bottom(5)
            .label(get_current_time(&self))
            .build();

        let (tx, mut rx) = mpsc::channel(1);

        RUNTIME.spawn(async move {
            loop {
                let _ = tx.send(get_current_time(&self)).await;
                sleep(tokio::time::Duration::from_millis(500)).await;
            }
        });

        let time = button.clone();
        glib::spawn_future_local(async move {
            while let Some(response) = rx.recv().await {
                time.set_label(&response);
            }
        });
        button
    }
}

fn get_current_time(module: &ClockModule) -> String {
    format!("{}", Local::now().format(&module.format))
}
