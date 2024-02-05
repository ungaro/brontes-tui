#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::Result;
pub use app::*;
pub use colors::*;
pub use root::*;
pub use term::*;
pub use theme::*;
pub use misc::*;

mod app;
mod colors;
mod root;
mod tabs;
mod term;
mod theme;
mod misc;
fn main() -> Result<()> {
    App::run()
}
