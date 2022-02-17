#[derive(Copy, Clone, Debug)]
pub(crate) enum MenuItem {
    NewGame,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::NewGame => 0,
        }
    }
}
