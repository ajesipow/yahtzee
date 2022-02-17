use crate::dice::Dice;
use std::collections::{HashMap, HashSet};

pub(crate) struct Score {
    pub upper_section: ScoreUpperSection,
    pub lower_section: ScoreLowerSection,
}

pub(crate) struct ScoreUpperSection {
    pub aces: Option<u32>,
    pub twos: Option<u32>,
    pub threes: Option<u32>,
    pub fours: Option<u32>,
    pub fives: Option<u32>,
    pub sixes: Option<u32>,
    pub bonus: Option<u32>,
}

pub(crate) struct ScoreLowerSection {
    pub three_of_a_kind: Option<u32>,
    pub four_of_a_kind: Option<u32>,
    pub full_house: Option<u32>,
    pub small_straight: Option<u32>,
    pub large_straight: Option<u32>,
    pub yahtzee: Option<u32>,
    pub chance: Option<u32>,
}

impl ScoreUpperSection {
    pub fn new() -> Self {
        Self {
            aces: None,
            twos: None,
            threes: None,
            fours: None,
            fives: None,
            sixes: None,
            bonus: None,
        }
    }

    pub fn set_aces(&mut self, dice_roll: &Dice) -> () {
        let value = dice_roll.0.as_slice().iter().filter(|die| **die == 1).sum();
        self.aces = self.aces.or(Some(value));
    }

    pub fn set_twos(&mut self, dice_roll: &Dice) -> () {
        let value = dice_roll.0.as_slice().iter().filter(|die| **die == 2).sum();
        self.twos = self.twos.or(Some(value));
    }

    pub fn set_threes(&mut self, dice_roll: &Dice) -> () {
        let value = dice_roll.0.as_slice().iter().filter(|die| **die == 3).sum();
        self.threes = self.threes.or(Some(value));
        self.check_and_set_bonus();
    }

    pub fn set_fours(&mut self, dice_roll: &Dice) -> () {
        let value = dice_roll.0.as_slice().iter().filter(|die| **die == 4).sum();
        self.fours = self.fours.or(Some(value));
        self.check_and_set_bonus();
    }

    pub fn set_fives(&mut self, dice_roll: &Dice) -> () {
        let value = dice_roll.0.as_slice().iter().filter(|die| **die == 5).sum();
        self.fives = self.fives.or(Some(value));
        self.check_and_set_bonus();
    }

    pub fn set_sixes(&mut self, dice_roll: &Dice) -> () {
        let value = dice_roll.0.as_slice().iter().filter(|die| **die == 6).sum();
        self.sixes = self.sixes.or(Some(value));
        self.check_and_set_bonus();
    }

    pub fn score_without_bonus(&self) -> u32 {
        vec![
            self.aces,
            self.twos,
            self.threes,
            self.fours,
            self.fives,
            self.sixes,
        ]
        .iter()
        .map(|value| value.unwrap_or_default())
        .sum()
    }

    pub fn total_score(&self) -> u32 {
        self.score_without_bonus() + self.bonus.unwrap_or_default()
    }

    fn check_and_set_bonus(&mut self) {
        let score_without_bonus = self.score_without_bonus();
        if score_without_bonus >= 63 {
            self.bonus = Some(35)
        };
    }
}

impl ScoreLowerSection {
    pub fn new() -> Self {
        Self {
            three_of_a_kind: None,
            four_of_a_kind: None,
            full_house: None,
            small_straight: None,
            large_straight: None,
            yahtzee: None,
            chance: None,
        }
    }
    pub fn total_score(&self) -> u32 {
        vec![
            self.three_of_a_kind,
            self.four_of_a_kind,
            self.full_house,
            self.small_straight,
            self.large_straight,
            self.yahtzee,
            self.chance,
        ]
        .iter()
        .map(|value| value.unwrap_or_default())
        .sum()
    }

    pub fn set_three_of_a_kind(&mut self, dice_roll: &Dice) -> () {
        let frequencies = get_dice_frequencies(dice_roll.0.as_slice());
        if frequencies.values().any(|count| *count >= 3) {
            let value = dice_roll.0.as_slice().iter().sum();
            self.three_of_a_kind = self.three_of_a_kind.or(Some(value));
        }
    }

    // TODO set them all to 0 if wrong
    pub fn set_four_of_a_kind(&mut self, dice_roll: &Dice) -> () {
        let frequencies = get_dice_frequencies(dice_roll.0.as_slice());
        if frequencies.values().any(|count| *count >= 4) {
            let value = dice_roll.0.as_slice().iter().sum();
            self.four_of_a_kind = self.four_of_a_kind.or(Some(value));
        }
    }

    pub fn set_full_house(&mut self, dice_roll: &Dice) -> () {
        let frequencies = get_dice_frequencies(dice_roll.0.as_slice());
        if frequencies.values().any(|count| *count == 3)
            && frequencies.values().any(|count| *count == 2)
        {
            self.full_house = Some(25);
        }
    }

    pub fn set_small_straight(&mut self, dice_roll: &Dice) -> () {
        if has_at_least_n_consecutive_numbers(dice_roll.0.clone(), 4) {
            self.small_straight = Some(30);
        }
    }

    pub fn set_large_straight(&mut self, dice_roll: &Dice) -> () {
        if has_at_least_n_consecutive_numbers(dice_roll.0.clone(), 5) {
            self.large_straight = Some(40);
        }
    }

    pub fn set_yahtzee(&mut self, dice_roll: &Dice) -> () {
        if dice_roll.0.iter().collect::<HashSet<_>>().len() == 1 {
            self.yahtzee = self.yahtzee.map_or_else(|| Some(50), |v| Some(v + 50));
        }
    }

    pub fn set_chance(&mut self, dice_roll: &Dice) -> () {
        let value = dice_roll.0.as_slice().iter().sum();
        self.chance = self.chance.or(Some(value));
    }
}

fn get_dice_frequencies(dice_vec: &[u32]) -> HashMap<u32, i32> {
    dice_vec.iter().fold(HashMap::new(), |mut map, value| {
        *map.entry(*value).or_insert(0) += 1;
        map
    })
}

fn has_at_least_n_consecutive_numbers(mut rolls: Vec<u32>, n: u32) -> bool {
    rolls.sort();
    let n_consecutive_increases = rolls.windows(2).fold(1, |mut consecutive_incr, window| {
        if let (Some(a), Some(b)) = (window.first(), window.last()) {
            if *a + 1 == *b {
                // if two consecutive elements
                consecutive_incr += 1;
            } else if *a == *b {
                ()
            } else if consecutive_incr < n {
                // Break the streak if we haven't reached our minimum yet
                consecutive_incr = 1
            }
        }
        consecutive_incr
    });
    n <= n_consecutive_increases
}

impl Score {
    pub fn new() -> Self {
        Self {
            upper_section: ScoreUpperSection::new(),
            lower_section: ScoreLowerSection::new(),
        }
    }

    pub fn total_score(&self) -> u32 {
        self.upper_section.total_score() + self.lower_section.total_score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2, 2, 1, 1, 1], 7)]
    #[case(vec![2, 1, 1, 1, 1], 6)]
    #[case(vec![1, 1, 1, 1, 1], 5)]
    #[case(vec![2, 2, 2, 5, 6], 17)]
    #[case(vec![3, 2, 3, 5, 3], 16)]
    #[case(vec![3, 4, 3, 4, 4], 18)]
    #[case(vec![3, 4, 5, 5, 5], 22)]
    #[case(vec![6, 6, 5, 1, 6], 24)]
    fn test_three_of_a_kind_accepts_valid_cases(
        #[case] dice_input: Vec<u32>,
        #[case] expected_score: u32,
    ) {
        let mut score = ScoreLowerSection::new();
        score.set_three_of_a_kind(&Dice(dice_input));
        assert_eq!(score.three_of_a_kind, Some(expected_score));
        assert_eq!(score.total_score(), expected_score);
    }

    #[rstest]
    #[case(vec![2, 2, 1, 3, 1])]
    #[case(vec![2, 2, 4, 4, 1])]
    #[case(vec![2, 5, 4, 4, 1])]
    #[case(vec![2, 5, 4, 4, 5])]
    #[case(vec![6, 5, 6, 4, 5])]
    fn test_three_of_a_kind_fails_invalid_cases(#[case] dice_input: Vec<u32>) {
        let mut score = ScoreLowerSection::new();
        score.set_three_of_a_kind(&Dice(dice_input));
        assert_eq!(score.three_of_a_kind, None);
        assert_eq!(score.total_score(), 0);
    }

    #[rstest]
    #[case(vec![2, 2, 2, 2, 1], 9)]
    #[case(vec![2, 1, 1, 1, 1], 6)]
    #[case(vec![3, 1, 3, 3, 3], 13)]
    #[case(vec![4, 4, 3, 4, 4], 19)]
    #[case(vec![5, 5, 5, 5, 4], 24)]
    #[case(vec![6, 5, 6, 6, 6], 29)]
    fn test_four_of_a_kind_accepts_valid_cases(
        #[case] dice_input: Vec<u32>,
        #[case] expected_score: u32,
    ) {
        let mut score = ScoreLowerSection::new();
        score.set_four_of_a_kind(&Dice(dice_input));
        assert_eq!(score.four_of_a_kind, Some(expected_score));
        assert_eq!(score.total_score(), expected_score);
    }

    #[rstest]
    #[case(vec![2, 2, 2, 3, 1])]
    #[case(vec![2, 2, 4, 4, 2])]
    #[case(vec![2, 5, 4, 4, 4])]
    #[case(vec![2, 5, 4, 4, 5])]
    #[case(vec![6, 5, 6, 4, 5])]
    fn test_four_of_a_kind_fails_invalid_cases(#[case] dice_input: Vec<u32>) {
        let mut score = ScoreLowerSection::new();
        score.set_four_of_a_kind(&Dice(dice_input));
        assert_eq!(score.four_of_a_kind, None);
        assert_eq!(score.total_score(), 0);
    }

    #[rstest]
    #[case(vec![1, 2, 3, 4, 1])]
    #[case(vec![2, 3, 4, 5, 6])]
    #[case(vec![1, 2, 3, 4, 5])]
    #[case(vec![3, 4, 5, 6, 1])]
    #[case(vec![2, 3, 5, 4, 5])]
    #[case(vec![6, 2, 1, 4, 3])]
    #[case(vec![6, 5, 4, 1, 3])]
    #[case(vec![2, 2, 3, 4, 5])]
    #[case(vec![2, 3, 3, 4, 5])]
    fn test_small_straight_accepts_valid_cases(#[case] dice_input: Vec<u32>) {
        let mut score = ScoreLowerSection::new();
        score.set_small_straight(&Dice(dice_input));
        assert_eq!(score.small_straight, Some(30));
        assert_eq!(score.total_score(), 30);
    }

    #[rstest]
    #[case(vec![1, 2, 3, 5, 6])]
    #[case(vec![2, 3, 2, 4, 6])]
    #[case(vec![1, 1, 1, 1, 1])]
    #[case(vec![5, 5, 6, 1, 1])]
    #[case(vec![6, 5, 4, 2, 1])]
    fn test_small_straight_fails_invalid_cases(#[case] dice_input: Vec<u32>) {
        let mut score = ScoreLowerSection::new();
        score.set_small_straight(&Dice(dice_input));
        assert_eq!(score.small_straight, None);
        assert_eq!(score.total_score(), 0);
    }

    #[rstest]
    #[case(vec![1, 2, 3, 4, 5])]
    #[case(vec![2, 3, 4, 5, 6])]
    fn test_large_straight_accepts_valid_cases(#[case] dice_input: Vec<u32>) {
        let mut score = ScoreLowerSection::new();
        score.set_large_straight(&Dice(dice_input));
        assert_eq!(score.large_straight, Some(40));
        assert_eq!(score.total_score(), 40);
    }

    #[rstest]
    #[case(vec![1, 2, 3, 5, 6])]
    #[case(vec![2, 3, 2, 4, 6])]
    #[case(vec![1, 1, 1, 1, 1])]
    #[case(vec![5, 5, 6, 1, 1])]
    #[case(vec![6, 5, 4, 2, 1])]
    fn test_large_straight_fails_invalid_cases(#[case] dice_input: Vec<u32>) {
        let mut score = ScoreLowerSection::new();
        score.set_large_straight(&Dice(dice_input));
        assert_eq!(score.large_straight, None);
        assert_eq!(score.total_score(), 0);
    }

    #[rstest]
    #[case(vec![2, 2, 1, 1, 1])]
    #[case(vec![2, 2, 3, 3, 3])]
    #[case(vec![3, 5, 5, 5, 3])]
    #[case(vec![6, 4, 6, 4, 6])]
    #[case(vec![1, 1, 1, 5, 5])]
    #[case(vec![6, 2, 2, 6, 2])]
    fn test_full_house_accepts_valid_cases(#[case] dice_input: Vec<u32>) {
        let mut score = ScoreLowerSection::new();
        score.set_full_house(&Dice(dice_input));
        assert_eq!(score.full_house, Some(25));
        assert_eq!(score.total_score(), 25);
    }

    #[rstest]
    #[case(vec![1, 1, 1, 1, 1])]
    #[case(vec![1, 2, 3, 4, 5])]
    #[case(vec![3, 3, 3, 4, 5])]
    #[case(vec![3, 3, 4, 4, 5])]
    #[case(vec![6, 3, 4, 4, 5])]
    fn test_full_house_fails_invalid_cases(#[case] dice_input: Vec<u32>) {
        let mut score = ScoreLowerSection::new();
        score.set_full_house(&Dice(dice_input));
        assert_eq!(score.full_house, None);
        assert_eq!(score.total_score(), 0);
    }

    #[rstest]
    #[case(vec![1, 1, 1, 1, 1])]
    #[case(vec![2, 2, 2, 2, 2])]
    #[case(vec![3, 3, 3, 3, 3])]
    #[case(vec![4, 4, 4, 4, 4])]
    #[case(vec![5, 5, 5, 5, 5])]
    #[case(vec![6, 6, 6, 6, 6])]
    fn test_yahtzee_accepts_valid_cases(#[case] dice_input: Vec<u32>) {
        let mut score = ScoreLowerSection::new();
        score.set_yahtzee(&Dice(dice_input));
        assert_eq!(score.yahtzee, Some(50));
        assert_eq!(score.total_score(), 50);
    }

    #[rstest]
    #[case(vec![1, 2, 3, 5, 6])]
    #[case(vec![2, 3, 2, 4, 6])]
    #[case(vec![5, 5, 6, 1, 1])]
    #[case(vec![6, 5, 4, 2, 1])]
    #[case(vec![1, 1, 1, 1, 2])]
    fn test_yahtzee_fails_invalid_cases(#[case] dice_input: Vec<u32>) {
        let mut score = ScoreLowerSection::new();
        score.set_yahtzee(&Dice(dice_input));
        assert_eq!(score.yahtzee, None);
        assert_eq!(score.total_score(), 0);
    }

    #[test]
    fn test_yahtzee_multiple_works() {
        let mut score = ScoreLowerSection::new();
        let dice_input = Dice(vec![1, 1, 1, 1, 1]);
        score.set_yahtzee(&dice_input);
        assert_eq!(score.yahtzee, Some(50));
        assert_eq!(score.total_score(), 50);

        score.set_yahtzee(&dice_input);
        assert_eq!(score.yahtzee, Some(100));
        assert_eq!(score.total_score(), 100);
    }

    #[rstest]
    #[case(vec![None, None, None, None, None, None, None], 0, 0)]
    #[case(vec![Some(4), Some(8), Some(12), Some(16), Some(20), Some(24), Some(35)], 84, 119)]
    #[case(vec![Some(3), Some(2), Some(9), Some(8), Some(10), Some(12), None], 44, 44)]
    fn test_score_empty_upper_section(
        #[case] dice_input: Vec<Option<u32>>,
        #[case] expected_output_without_bonus: u32,
        #[case] expected_output_total: u32,
    ) {
        let score_upper = ScoreUpperSection {
            aces: dice_input[0],
            twos: dice_input[1],
            threes: dice_input[2],
            fours: dice_input[3],
            fives: dice_input[4],
            sixes: dice_input[5],
            bonus: dice_input[6],
        };
        assert_eq!(
            score_upper.score_without_bonus(),
            expected_output_without_bonus
        );
        assert_eq!(score_upper.total_score(), expected_output_total);
    }

    #[rstest]
    #[case(vec![None, None, None, None, None, None, None], 0)]
    #[case(vec![Some(17), Some(24), Some(25), Some(30), Some(40), Some(50), Some(14)], 200)]
    fn test_score_empty_lower_section(
        #[case] dice_input: Vec<Option<u32>>,
        #[case] expected_output: u32,
    ) {
        let score_lower = ScoreLowerSection {
            three_of_a_kind: dice_input[0],
            four_of_a_kind: dice_input[1],
            full_house: dice_input[2],
            small_straight: dice_input[3],
            large_straight: dice_input[4],
            yahtzee: dice_input[5],
            chance: dice_input[6],
        };
        assert_eq!(score_lower.total_score(), expected_output);
    }

    #[test]
    fn test_score_full_score() {
        let score = Score {
            upper_section: ScoreUpperSection {
                aces: Some(4),
                twos: Some(8),
                threes: Some(12),
                fours: Some(16),
                fives: Some(20),
                sixes: Some(24),
                bonus: Some(35),
            },
            lower_section: ScoreLowerSection {
                three_of_a_kind: Some(17),
                four_of_a_kind: Some(24),
                full_house: Some(25),
                small_straight: Some(30),
                large_straight: Some(40),
                yahtzee: Some(50),
                chance: Some(14),
            },
        };
        assert_eq!(score.total_score(), 319);
    }
}
