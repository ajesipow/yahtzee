use crate::{AppState, InputMode, Score};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs};
use tui::Frame;

pub(crate) fn render_app<B: Backend>(app_state: &AppState, rect: &mut Frame<B>) {
    let menu_titles = vec!["New game", "Roll dice", "Quit"];

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
}

fn render_score<'a>(score: &Score) -> List<'a> {
    let scores = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Score")
        .border_type(BorderType::Plain);

    let mut items: Vec<_> = vec![];
    items.extend(vec![
        list_item(format!(
            "Aces: {}",
            score
                .upper_section
                .aces
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Twos: {}",
            score
                .upper_section
                .twos
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Threes: {}",
            score
                .upper_section
                .threes
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Fours: {}",
            score
                .upper_section
                .fours
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Fives: {}",
            score
                .upper_section
                .fives
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Sixes: {}",
            score
                .upper_section
                .sixes
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Total upper section without bonus: {}",
            score.upper_section.score_without_bonus(),
        )),
        list_item(format!(
            "Bonus: {}",
            score
                .upper_section
                .bonus
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Total upper section: {}",
            score.upper_section.total_score(),
        )),
        list_item(format!(
            "Three of a kind: {}",
            score
                .lower_section
                .three_of_a_kind
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Four of a kind: {}",
            score
                .lower_section
                .four_of_a_kind
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Full house: {}",
            score
                .lower_section
                .full_house
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Small straight: {}",
            score
                .lower_section
                .small_straight
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Large straight: {}",
            score
                .lower_section
                .large_straight
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Yahtzee: {}",
            score
                .lower_section
                .yahtzee
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Chance: {}",
            score
                .lower_section
                .chance
                .map_or("-".to_string(), |v| v.to_string())
        )),
        list_item(format!(
            "Total lower section: {}",
            score.lower_section.total_score(),
        )),
        list_item(format!("TOTAL SCORE: {}", score.total_score(),)),
    ]);

    List::new(items).block(scores).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    )
}

fn list_item<'a>(content: String) -> ListItem<'a> {
    ListItem::new(Spans::from(vec![Span::styled(content, Style::default())]))
}
