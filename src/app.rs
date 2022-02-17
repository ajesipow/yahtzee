use crate::dice::DiceState;
use crate::Score;
use std::num::ParseIntError;

#[derive(Copy, Clone, Debug)]
pub(crate) enum InputMode {
    Normal,
    Selecting,
}

pub(crate) enum Event<I> {
    Input(I),
    Tick,
}

pub(crate) struct AppState {
    pub selection_input: String,
    pub input_mode: InputMode,
    pub dice_state: DiceState,
    pub score: Score,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            selection_input: String::new(),
            input_mode: InputMode::Normal,
            dice_state: DiceState::new(),
            score: Score::new(),
        }
    }

    pub fn new_game(&mut self) {
        *self = Self::new();
    }

    pub fn reset(&mut self) {
        self.dice_state.reset();
        self.selection_input = String::new();
        self.input_mode = InputMode::Normal;
    }

    pub fn roll_all_dice(&mut self) {
        self.dice_state.roll_all_dice()
    }

    pub fn select_dice_to_reroll(&mut self) {
        if !self.dice_state.dice.0.is_empty() {
            self.input_mode = InputMode::Selecting;
        }
    }

    pub fn cancel_selection_mode(&mut self) {
        self.selection_input = String::new();
        self.input_mode = InputMode::Normal;
    }

    pub fn reroll_selected_dice(&mut self) {
        if let Ok(one_indexed) = parse_selection_input_to_dice_indices(
            self.selection_input.drain(..).collect::<String>().as_str(),
        ) {
            // TODO protect against overflow (0-1)
            let dice_ids_to_reroll = one_indexed.into_iter().map(|v| v - 1).collect();
            if self
                .dice_state
                .reroll_selected_dice(dice_ids_to_reroll)
                .is_err()
            {
                self.cancel_selection_mode()
            }
        }
    }

    pub fn set_aces(&mut self) {
        self.score.upper_section.set_aces(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_twos(&mut self) {
        self.score.upper_section.set_twos(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_threes(&mut self) {
        self.score.upper_section.set_threes(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_fours(&mut self) {
        self.score.upper_section.set_fours(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_fives(&mut self) {
        self.score.upper_section.set_fives(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_sixes(&mut self) {
        self.score.upper_section.set_sixes(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_three_of_a_kind(&mut self) {
        self.score
            .lower_section
            .set_three_of_a_kind(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_four_of_a_kind(&mut self) {
        self.score
            .lower_section
            .set_four_of_a_kind(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_full_house(&mut self) {
        self.score
            .lower_section
            .set_full_house(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_small_straight(&mut self) {
        self.score
            .lower_section
            .set_small_straight(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_large_straight(&mut self) {
        self.score
            .lower_section
            .set_large_straight(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_yahtzee(&mut self) {
        self.score.lower_section.set_yahtzee(&self.dice_state.dice);
        self.reset();
    }

    pub fn set_chance(&mut self) {
        self.score.lower_section.set_chance(&self.dice_state.dice);
        self.reset();
    }
}

fn parse_selection_input_to_dice_indices(
    selection_input: &str,
) -> Result<Vec<usize>, ParseIntError> {
    let index_strings: Vec<&str> = selection_input.split(",").collect();
    index_strings
        .into_iter()
        .map(|s| s.parse::<usize>())
        .collect()
}
