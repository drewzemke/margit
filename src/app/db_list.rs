use super::state::{State, WidgetFocus};
use crossterm::event::{Event, KeyCode};
use mongodb::results::DatabaseSpecification;
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListItem, ListState, StatefulWidget},
};

#[derive(Debug, Default)]
pub struct DatabaseListState {
    pub items: Vec<DatabaseSpecification>,
    pub state: ListState,
}

#[derive(Debug, Default)]
pub struct DbList<'a> {
    marker: std::marker::PhantomData<State<'a>>,
}

impl<'a> StatefulWidget for DbList<'a> {
    type State = State<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let focused = state.focus == WidgetFocus::DatabaseList;
        let border_color = if focused { Color::Green } else { Color::White };

        let items: Vec<ListItem> = state
            .db_list
            .items
            .iter()
            .map(|db| ListItem::new(db.name.clone()))
            .collect();

        let list = List::new(items)
            .block(
                Block::bordered()
                    .title("Databases")
                    .border_style(Style::default().fg(border_color)),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(Color::White),
            );

        StatefulWidget::render(list, area, buf, &mut state.db_list.state);
    }
}

impl<'a> DbList<'a> {
    pub fn handle_event(event: &Event, state: &mut State) -> bool {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    Self::next(state);
                    state.exec_get_collections();
                    true
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    Self::previous(state);
                    state.exec_get_collections();
                    true
                }
                KeyCode::Enter => {
                    state.focus = WidgetFocus::CollectionList;
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn next(state: &mut State) -> bool {
        let i = match state.db_list.state.selected() {
            Some(i) => {
                if i >= state.db_list.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.db_list.state.select(Some(i));
        true
    }

    fn previous(state: &mut State) -> bool {
        let i = match state.db_list.state.selected() {
            Some(i) => {
                if i == 0 {
                    state.db_list.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => state.db_list.items.len() - 1,
        };
        state.db_list.state.select(Some(i));
        true
    }
}
