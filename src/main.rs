use chrono::Local;
use glib::once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use hyprland::{async_closure, event_listener::AsyncEventListener};
use tokio::{runtime::Runtime, sync::mpsc, time::sleep};

const APP_ID: &str = "dev.khsaad04.bar";
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Failed to exec tokio runtime"));

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    // Time module
    let time_widget = gtk::Label::default();
    time_widget.set_margin_top(5);
    time_widget.set_margin_bottom(5);

    let current_time_str = current_time();
    time_widget.set_text(&current_time_str);

    let (tx, mut rx) = mpsc::channel(1);

    RUNTIME.spawn(async move {
        loop {
            let _ = tx.send(current_time()).await;
            sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });

    let time = time_widget.clone();
    glib::spawn_future_local(async move {
        while let Some(response) = rx.recv().await {
            time.set_text(&response);
        }
    });

    // Workspace module
    let workspace_widget = gtk::Label::default();
    workspace_widget.set_margin_top(5);
    workspace_widget.set_margin_bottom(5);

    let mut listener = AsyncEventListener::new();

    let (tx_ws, mut rx_ws) = mpsc::channel(1);

    listener.add_workspace_change_handler(async_closure!(move |id| {
        tx_ws.send(&id.to_string()).await;
    }));

    RUNTIME.spawn(async move {
        let _ = listener.start_listener_async().await;
    });

    let workspace = workspace_widget.clone();
    glib::spawn_future_local(async move {
        while let Some(response) = rx_ws.recv().await {
            workspace.set_text(&response);
        }
    });

    // Box
    let center_box = gtk::CenterBox::default();
    center_box.set_center_widget(Some(&time_widget));
    center_box.set_start_widget(Some(&workspace_widget));

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
