mod app;
mod dice;
mod score;
mod ui;

use crate::app::{AppState, Event, InputMode};
use crate::score::Score;
use crate::ui::MenuItem;
use crossterm::event::KeyCode;
use crossterm::terminal::disable_raw_mode;
use crossterm::{
    event::{self, Event as CEvent},
    terminal::enable_raw_mode,
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs};
use tui::Terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["New game", "Roll dice", "Quit"];
    let active_menu_item = MenuItem::NewGame;
    let mut app_state = AppState::new();

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(2),
                    ]
                    .as_ref(),
                )
                .split(size);

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.clone().into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            let dice_paragraph = Paragraph::new(
                app_state
                    .dice_state
                    .dice
                    .0
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            )
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title(format!(
                "Dice - roll {:?} / {:?}",
                app_state.dice_state.number_of_rolls, app_state.dice_state.max_number_of_rolls
            )));

            let input = Paragraph::new(app_state.selection_input.as_ref())
                .style(match app_state.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Selecting => Style::default().fg(Color::Yellow),
                })
                .block(Block::default().borders(Borders::ALL).title("Selection"));

            rect.render_widget(tabs, chunks[0]);
            rect.render_widget(dice_paragraph, chunks[1]);
            rect.render_widget(input, chunks[2]);
            rect.render_widget(render_score(&app_state.score), chunks[3]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match app_state.input_mode {
                InputMode::Normal => match event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        quit_app(terminal)?;
                        break;
                    }
                    KeyCode::Char('n') => app_state.new_game(),
                    KeyCode::Char('e') => app_state.select_dice_to_reroll(),
                    KeyCode::Char('r') => app_state.roll_all_dice(),
                    KeyCode::Char('1') => app_state.set_aces(),
                    KeyCode::Char('2') => app_state.set_twos(),
                    KeyCode::Char('3') => app_state.set_threes(),
                    KeyCode::Char('4') => app_state.set_fours(),
                    KeyCode::Char('5') => app_state.set_fives(),
                    KeyCode::Char('6') => app_state.set_sixes(),
                    KeyCode::Char('t') => app_state.set_three_of_a_kind(),
                    KeyCode::Char('f') => app_state.set_four_of_a_kind(),
                    KeyCode::Char('h') => app_state.set_full_house(),
                    KeyCode::Char('s') => app_state.set_small_straight(),
                    KeyCode::Char('l') => app_state.set_large_straight(),
                    KeyCode::Char('y') => app_state.set_yahtzee(),
                    KeyCode::Char('c') => app_state.set_chance(),
                    _ => {}
                },
                InputMode::Selecting => match event.code {
                    KeyCode::Enter => app_state.reroll_selected_dice(),
                    KeyCode::Char(c) => {
                        app_state.selection_input.push(c);
                    }
                    KeyCode::Backspace => {
                        app_state.selection_input.pop();
                    }
                    KeyCode::Esc => app_state.cancel_selection_mode(),
                    _ => {}
                },
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn quit_app<B: Backend>(mut terminal: Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    terminal.show_cursor()?;
    terminal.clear()?;
    Ok(())
}

fn render_score<'a>(score: &Score) -> List<'a> {
    let scores = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Score")
        .border_type(BorderType::Plain);

    let mut items: Vec<_> = vec![];
    items.extend(vec![
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Aces: {}",
                score
                    .upper_section
                    .aces
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Twos: {}",
                score
                    .upper_section
                    .twos
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Threes: {}",
                score
                    .upper_section
                    .threes
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Fours: {}",
                score
                    .upper_section
                    .fours
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Fives: {}",
                score
                    .upper_section
                    .fives
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Sixes: {}",
                score
                    .upper_section
                    .sixes
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Total without bonus: {}",
                score.upper_section.score_without_bonus().to_string(),
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Bonus: {}",
                score
                    .upper_section
                    .bonus
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Total upper section: {}",
                score.upper_section.total_score().to_string(),
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Three of a kind: {}",
                score
                    .lower_section
                    .three_of_a_kind
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Four of a kind: {}",
                score
                    .lower_section
                    .four_of_a_kind
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Full house: {}",
                score
                    .lower_section
                    .full_house
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Small straight: {}",
                score
                    .lower_section
                    .small_straight
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Large straight: {}",
                score
                    .lower_section
                    .large_straight
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Yahtzee: {}",
                score
                    .lower_section
                    .yahtzee
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Chance: {}",
                score
                    .lower_section
                    .chance
                    .clone()
                    .map_or("-".to_string(), |v| v.to_string())
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!(
                "Total lower section: {}",
                score.lower_section.total_score().to_string(),
            ),
            Style::default(),
        )])),
        ListItem::new(Spans::from(vec![Span::styled(
            format!("TOTAL SCORE: {}", score.total_score().to_string(),),
            Style::default(),
        )])),
    ]);

    List::new(items).block(scores).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    )
}
