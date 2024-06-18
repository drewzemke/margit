use crossterm::event::{Event, KeyCode};
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListItem, StatefulWidget},
};

use super::state::{Mode, State};

#[derive(Debug, Default)]
pub struct CollList<'a> {
    marker: std::marker::PhantomData<State<'a>>,
}

impl<'a> StatefulWidget for CollList<'a> {
    type State = State<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let focused = state.mode == Mode::ChoosingCollection;
        let border_color = if focused { Color::Green } else { Color::White };

        let items: Vec<ListItem> = state
            .coll_names
            .iter()
            .map(|db_name| ListItem::new(db_name.clone()))
            .collect();

        let list = List::new(items)
            .block(
                Block::bordered()
                    .title("Collections")
                    .border_style(Style::default().fg(border_color)),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(Color::White),
            );

        StatefulWidget::render(list, area, buf, &mut state.coll_list_state);
    }
}

impl<'a> CollList<'a> {
    pub fn handle_event(event: &Event, state: &mut State) -> bool {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => Self::next(state),
                KeyCode::Char('k') | KeyCode::Up => Self::previous(state),
                KeyCode::Enter => {
                    state.exec_query();
                    state.exec_count();
                    state.mode = Mode::MainView;
                    false
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn next(state: &mut State) -> bool {
        let i = match state.coll_list_state.selected() {
            Some(i) => {
                if i >= state.coll_names.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.coll_list_state.select(Some(i));
        true
    }

    fn previous(state: &mut State) -> bool {
        let i = match state.coll_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    state.coll_names.len() - 1
                } else {
                    i - 1
                }
            }
            None => state.coll_names.len() - 1,
        };
        state.coll_list_state.select(Some(i));
        true
    }
}