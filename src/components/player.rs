use color_eyre::eyre::Result;
use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    context::BuildContext,
    models::{self},
};

use super::Component;

pub struct Player<'a> {
    pub player: &'a models::player::Player,
}

impl<'a> Player<'a> {
    pub fn new(player: &'a models::player::Player)->Self{
        Self{player}
    }


    pub fn display_status(&self) -> Vec<Line> {
        let mut spans: Vec<Line> = Vec::new();

        // 添加表格头部

        let style = Style::default();
        spans.push(Line::from(vec![
            Span::styled(format!("{:^8}", "式神"), style.magenta()),
            Span::styled(format!("{:^6}", "等级"), style.yellow()),
            Span::styled(format!("{:^6}", "力量"), style.green()),
            Span::styled(format!("{:^6}", "血量"), style.red()),
        ]));

        // 添加每个角色的状态
        for ch in &self.player.characters {
            spans.push(Line::from(vec![
                Span::from(format!("{:^8}", ch.name)),
                Span::from(format!("{:^8}", ch.level)),
                Span::from(format!("{:^8}", ch.atk)),
                Span::from(format!("{:^8}", ch.health)),
            ]));
        }

        spans.push(Line::from("\n\n"));

        spans
    }
}

impl<'a> Component for Player<'a> {
    fn draw(&mut self, context: BuildContext, frame: &mut Frame, area: Rect) -> Result<()> {
        let player = &self.player;
        let p = Paragraph::new(self.display_status())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("牌手: {}   血量: {}", player.name, player.health)),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black));
        frame.render_widget(p, area);
        Ok(())
    }
}
