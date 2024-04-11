pub mod clock;
pub mod hypr_window;
pub mod hypr_workspaces;
pub mod sway_workspaces;
pub mod sysinfo;

use gtk::glib::IsA;
use gtk::Widget;

pub trait Module<W>
where
    W: IsA<Widget>,
{
    fn callback(self) -> W;
}
