pub mod clock;
pub mod workspaces;

use gtk::glib::IsA;
use gtk::Widget;

pub trait Module<W>
where
    W: IsA<Widget>,
{
    fn into_widget(self) -> W;
    fn new() -> Self;
}
