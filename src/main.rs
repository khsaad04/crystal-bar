use chrono::Local;
use gio::prelude::*;
use gtk::{glib, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

const APP_ID: &str = "org.khs.bar";

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let bar = gtk::ApplicationWindow::builder()
        .application(app)
        .title("top_bar")
        .build();

    bar.init_layer_shell();
    bar.set_layer(Layer::Overlay);
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
    let time = current_time();
    let label = gtk::Label::default();
    label.set_text(&time);

    bar.set_child(Some(&label));

    bar.present();

    let tick = move || {
        let time = current_time();
        label.set_text(&time);
        glib::ControlFlow::Continue
    };

    glib::timeout_add_seconds_local(1, tick);
}

fn current_time() -> String {
    format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
}
