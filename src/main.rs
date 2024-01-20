mod widgets;
use widgets::clock::ClockButton;

use gio::prelude::*;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

const APP_ID: &str = "org.khs.bar";

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let clock_button = ClockButton::default();
    let bar = gtk::ApplicationWindow::builder()
        .application(app)
        .title("top_bar")
        .child(&clock_button)
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
