use {
    crossterm::event::{
        self,
        Event,
    },
    ratatui::{
        prelude::*,
        widgets::Block,
    },
};


fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    
    loop {
        terminal.draw(draw_frame)?;

        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }

    ratatui::restore();

    Ok(())
}

fn draw_frame(frame: &mut Frame) {
    frame.render_widget(Block::bordered().title("plotrat"), frame.area());
}
