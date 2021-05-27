mod controller;
pub mod view;
use crate::controller::connect;

fn main() {
    view::tui::tui::start_client();
    connect::con();
}
