use ratatui::{style::{Color, Style}, text::Line, widgets::{Block, Borders, Paragraph}};

use super::Component;

pub struct Log;

impl Component for Log {
    fn draw(&mut self, context: crate::context::BuildContext, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> color_eyre::eyre::Result<()> {
        let game = &context.borrow().game;
        let player = if game.round % 2 == 0 {
            &game.player_a
        } else {
            &game.player_b
        };

        let text = vec![
            Line::styled(format!("第{}回合", game.round), Style::new().bg(Color::Green)),
            Line::raw(format!("等待玩家{}升级", player.name)),
        ];

        let p = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("日志"))
            .style(Style::default().fg(Color::White).bg(Color::Black));
        frame.render_widget(p, area);
        Ok(())
    }
}