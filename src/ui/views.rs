use super::{Drawable, banner};
use catalog_view::Catalog;
use ratatui::layout::Constraint;
use ratatui::layout::Flex;
use ratatui::layout::{Layout, Rect};
use start_view::Start;

pub mod catalog_view;
pub mod start_view;

#[derive(Debug)]
pub enum View {
    StartView(Start),
    CatalogView(Catalog),
}

impl Default for View {
    fn default() -> Self {
        Self::StartView(Start)
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
