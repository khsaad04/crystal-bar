mod layout;
mod modules;

use layout::layout;

use gtk::glib::once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk_layer_shell::{Edge, Layer, LayerShell};
use tokio::runtime::Runtime;

const APP_ID: &str = "dev.khsaad04.bar";
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Failed to exec tokio runtime"));

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_startup(|app| {
        let provider = gtk::CssProvider::new();
        provider
            .load_from_path("style.css")
            .expect("Failed to load CSS");
        gtk::StyleContext::add_provider_for_screen(
            &gtk::gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        build_ui(app);
    });
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("window")
        .child(&layout())
        .build();

    window.set_layer(Layer::Overlay);
    window.init_layer_shell();
    window.auto_exclusive_zone_enable();

    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    app.connect_activate(move |_| {
        window.show_all();
    });
}
