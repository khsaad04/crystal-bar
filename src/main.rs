mod modules;
use modules::workspaces::WorkspacesModule;
use modules::{clock::ClockModule, Module};

use glib::once_cell::sync::Lazy;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::CssProvider;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use tokio::runtime::Runtime;

const APP_ID: &str = "dev.khsaad04.bar";
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Failed to exec tokio runtime"));

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let clock = ClockModule::new().into_widget();
    let workspaces = WorkspacesModule::new().into_widget();

    // Box
    let center_box = gtk::CenterBox::default();
    center_box.set_center_widget(Some(&clock));
    center_box.set_start_widget(Some(&workspaces));

    let bar = gtk::ApplicationWindow::builder()
        .application(app)
        .title("top_bar")
        .child(&center_box)
        .build();

    bar.set_layer(Layer::Overlay);
    bar.init_layer_shell();
    bar.auto_exclusive_zone_enable();

    // bar.set_margin(Edge::Left, 5);
    // bar.set_margin(Edge::Right, 5);
    // bar.set_margin(Edge::Top, 5);

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

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}
