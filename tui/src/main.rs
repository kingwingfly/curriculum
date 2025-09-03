mod app;

use std::io;

use app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let res = App::new().run(&mut terminal);
    ratatui::restore();
    res
}
