use std::process::Stdio;

use crate::modules::Module;
use crate::RUNTIME;
use chrono::Local;
use gtk::glib;
use gtk::prelude::*;
use gtk::Label;
use tokio::sync::mpsc;
use tokio::time::sleep;

pub struct ClockModule {}

impl Module<Label> for ClockModule {
    fn new() -> Self {
        Self {}
    }

    fn into_widget(self) -> Label {
        let label = gtk::Label::default();
        label.set_tooltip_text(Some(current_date()[..].as_ref()));
        label.set_margin_top(5);
        label.set_margin_bottom(5);

        let current_time_str = current_time();
        label.set_text(&current_time_str);

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
                time.set_text(&response);
            }
        });
        label
    }
}

fn current_time() -> String {
    format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
}

fn current_date() -> String {
    let output = std::process::Command::new("cal")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}
