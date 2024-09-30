
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{log::Log, player::Player, Component};
use crate::{
    action::Action,
    config::Config,
    context::{BuildContext},
    models::game::{Game, GameState},
};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    fn render_middle_area(&mut self, f: &mut Frame, area: Rect, game: &Game) {
        let middle = Paragraph::new(Text::raw(format!("Round: {}", game.round)))
            .block(Block::bordered().title("Status"))
            .style(Style::new())
            .alignment(Alignment::Center);
        f.render_widget(middle, area);
    }

    fn render_action_area(&mut self, f: &mut Frame, area: Rect, game: &Game) {
        let state = &game.state;


        let text = match state {
            GameState::WaitingLevelUp => vec![Line::styled("等待选择升级角色".to_string(), Style::new().yellow())],
            GameState::WaitingPlayerInput =>  vec![Line::styled("等待玩家出击".to_string(), Style::new().yellow())],
        };

        let p = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("操作"))
            .style(Style::default().fg(Color::White).bg(Color::Black));
        f.render_widget(p, area);
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, context: BuildContext, frame: &mut Frame, area: Rect) -> Result<()> {
        let [main_area, log_area, action_area] = Layout::vertical([
            Constraint::Min(8),
            Constraint::Percentage(100),
            Constraint::Min(8),
        ])
        .areas(area);

        // 创建一个布局，垂直方向
        let [player_a_area, middle_area, player_b_area] = Layout::horizontal([
            Constraint::Fill(50),
            Constraint::Min(32),
            Constraint::Fill(50),
        ])
        .areas(main_area);

        let game = &context.borrow().game;
        let mut log_component = Log{};
        let mut player_a_component = Player::new(&game.player_a);
        let mut player_b_component = Player::new(&game.player_b);


        // 渲染玩家状态
        let _ = player_a_component.draw(context.clone(), frame, player_a_area);
        let _ = player_b_component.draw(context.clone(), frame, player_b_area);


        self.render_middle_area(frame, middle_area, game);
        // self.render_log_area(frame, log_area, game);
        let _ = log_component.draw(context.clone(), frame, log_area);
        self.render_action_area(frame, action_area, game);

        Ok(())
    }
}
