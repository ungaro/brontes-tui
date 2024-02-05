use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::{layout, RgbSwatch, THEME};

#[derive(Debug)]
pub struct MevDetailTab {
    selected_row: usize,
}

impl MevDetailTab {
    pub fn new(selected_row: usize) -> Self {
        Self {
            selected_row: selected_row ,
        }
    }
}

impl Widget for MevDetailTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RgbSwatch.render(area, buf);
        let area = area.inner(&Margin {
            vertical: 1,
            horizontal: 2,
        });
        Clear.render(area, buf);
        Block::new()
            .title("Ratatouille Recipe".bold().white())
            .title_alignment(Alignment::Center)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 2, 1))
            .render(area, buf);

      

        let area = area.inner(&Margin {
            horizontal: 2,
            vertical: 1,
        });
        let area = layout(area, Direction::Horizontal, vec![44, 0]);

        render_detail(area[0], buf);
    }
}

fn render_detail(area: Rect, buf: &mut Buffer) {
    unimplemented!();
}
