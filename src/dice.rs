use rand::distributions::Uniform;
use rand::Rng;
use std::collections::HashSet;

fn throw_n_dice(die_range: Uniform<u32>, n_dice: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (&mut rng).sample_iter(die_range).take(n_dice).collect()
}

#[derive(Debug)]
pub(crate) struct Dice(pub(crate) Vec<u32>);

pub(crate) enum DiceStateError {
    MaxRollsReached,
    WrongDiceIds,
}

impl Dice {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn roll_dice(n_dice: usize) -> Self {
        let die_range = Uniform::new_inclusive(1, 6);
        Self(throw_n_dice(die_range, n_dice))
    }
}

pub(crate) struct DiceState {
    pub dice: Dice,
    number_of_dice: usize,
    pub(crate) max_number_of_rolls: usize,
    pub(crate) number_of_rolls: usize,
}

impl DiceState {
    pub fn new() -> Self {
        Self {
            dice: Dice::new(),
            number_of_dice: 5,
            number_of_rolls: 0,
            max_number_of_rolls: 3,
        }
    }

    pub fn reset(&mut self) {
        self.dice = Dice::new();
        self.number_of_rolls = 0;
    }

    pub fn reached_max_rolls(&self) -> bool {
        self.number_of_rolls >= self.max_number_of_rolls
    }

    pub fn roll_all_dice(&mut self) {
        if !self.reached_max_rolls() {
            self.number_of_rolls += 1;
            self.dice = Dice::roll_dice(self.number_of_dice);
        }
    }

    pub fn reroll_selected_dice(
        &mut self,
        dice_ids_to_reroll: Vec<usize>,
    ) -> Result<(), DiceStateError> {
        let allowed_dice_ids: HashSet<usize> = (0..self.number_of_dice).into_iter().collect();
        let is_selection_subset = dice_ids_to_reroll
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .is_subset(&allowed_dice_ids);

        if !is_selection_subset {
            return Err(DiceStateError::WrongDiceIds);
        }

        if self.reached_max_rolls() {
            return Err(DiceStateError::MaxRollsReached);
        }

        let new_dice = Dice::roll_dice(dice_ids_to_reroll.len());
        self.number_of_rolls += 1;

        // Replace the selected dice with new ones, retaining the original order
        for (index, new_value) in dice_ids_to_reroll.iter().zip(new_dice.0) {
            // Put new value at end
            self.dice.0.push(new_value);
            // Remove element at index and replace with last one
            self.dice.0.swap_remove(*index);
        }
        Ok(())
    }
}
