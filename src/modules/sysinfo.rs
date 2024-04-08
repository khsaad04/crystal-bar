use crate::modules::Module;
use crate::RUNTIME;
use gtk::glib;
use gtk::prelude::LabelExt;
use gtk::Label;
use sysinfo::System;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[derive(Default)]
pub struct SysinfoModule {}

impl Module<Label> for SysinfoModule {
    fn into_widget(self) -> Label {
        let mut sys = System::new_all();
        let label = Label::builder()
            .margin_top(5)
            .margin_bottom(5)
            .label(get_cpu_usage(&mut sys))
            .build();

        let (tx, mut rx) = mpsc::channel(1);

        RUNTIME.spawn(async move {
            loop {
                let _ = tx.send(get_sysinfo(&mut sys)).await;
                sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        {
            let label = label.clone();
            glib::spawn_future_local(async move {
                while let Some(response) = rx.recv().await {
                    label.set_label(&response);
                }
            });
        }

        label
    }
}

fn get_sysinfo(sys: &mut System) -> String {
    let cpu_usage = get_cpu_usage(sys);
    let memory_usage = get_memory_usage(sys);
    format!(" {}%  {}%", cpu_usage, memory_usage)
}

fn get_cpu_usage(sys: &mut System) -> String {
    sys.refresh_cpu();

    let mut cpus = vec![];
    for cpu in sys.cpus() {
        cpus.push(cpu.cpu_usage());
    }

    mean(&cpus).to_string()
}

fn get_memory_usage(sys: &mut System) -> String {
    sys.refresh_memory();

    let used_memory = sys.used_memory();
    let total_memory = sys.total_memory();
    let used_memory_percentage = used_memory as f32 / total_memory as f32 * 100_f32;

    format!("{:.0}", used_memory_percentage)
}

fn mean(arr: &[f32]) -> String {
    format!("{:.0}", arr.iter().sum::<f32>() / arr.len() as f32)
}
