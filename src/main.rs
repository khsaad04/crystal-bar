use chrono::Local;
use gio::{glib::once_cell::sync::Lazy, prelude::*};
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use tokio::runtime::Runtime;
use tokio::time::sleep;

const APP_ID: &str = "dev.khsaad04.bar";
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Failed to exec tokio runtime"));

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let time_widget = gtk::Label::default();
    let current_time_str = current_time();
    time_widget.set_text(&current_time_str);

    let (sender, receiver) = async_channel::bounded(1);

    RUNTIME.spawn(async move {
        loop {
            let _ = sender.send(current_time()).await;
            sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let time = time_widget.clone();
    gtk::glib::spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            time.set_text(&response);
        }
    });

    let center_box = gtk::CenterBox::default();
    center_box.set_center_widget(Some(&time_widget));

    let bar = gtk::ApplicationWindow::builder()
        .application(app)
        .title("top_bar")
        .child(&center_box)
        .build();

    bar.set_layer(Layer::Overlay);
    bar.init_layer_shell();
    bar.auto_exclusive_zone_enable();

    bar.set_margin(Edge::Left, 5);
    bar.set_margin(Edge::Right, 5);
    bar.set_margin(Edge::Top, 5);

    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        bar.set_anchor(anchor, state);
    }

    bar.present();
}

fn current_time() -> String {
    format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
}
