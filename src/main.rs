mod app;
mod dice;
mod score;
mod ui;

use crate::app::{AppState, Event, InputMode};
use crate::score::Score;
use crate::ui::render_app;
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::terminal::disable_raw_mode;
use crossterm::{
    event::{self, Event as CEvent},
    terminal::enable_raw_mode,
};
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SendError};
use std::thread;
use std::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

fn event_loop(tick_rate: Duration) -> Receiver<Event<KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || -> Result<(), SendError<Event<KeyEvent>>> {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).unwrap_or(false) {
                if let Ok(CEvent::Key(key)) = event::read() {
                    tx.send(Event::Input(key))?;
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });
    rx
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let rx = event_loop(Duration::from_millis(200));

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut app_state = AppState::new();

    loop {
        terminal.draw(|rect| render_app(&app_state, rect))?;

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
