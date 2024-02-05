use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::{layout, RgbSwatch, THEME};

#[derive(Debug, Default, Clone)]
struct Protocol {
    name: &'static str,
    mevtypes: Vec<MevType>,
}

#[derive(Debug, Default, Clone, Copy)]
struct MevType{
    name: &'static str,
}

#[derive(Debug, Clone, Copy)]
enum InputMode {
    Normal,
    Editing,
}


/*

pub mod bundle;
pub use bundle::*;
pub mod sandwich;
pub use sandwich::*;
pub mod jit;
pub use jit::*;
pub mod backrun;
pub use backrun::*;
pub mod cex_dex;
pub use cex_dex::*;
pub mod liquidation;
pub use liquidation::*;
pub mod jit_sandwich;
pub use jit_sandwich::*;
pub mod block;
pub use block::*;
*/




/*

BRONTES_DB_PATH="/media/alp/blockchain/_node/brontes_data/"
DB_PATH="/media/alp/blockchain/_node/reth_data/db/"

PROMETHEUS_ENDPOINT_IP="http://127.0.0.1"
PROMETHEUS_ENDPOINT_PORT="9090"

BRONTES_DB_PATH="/media/alp/blockchain/_node/brontes_data/"
DB_PATH="/media/alp/blockchain/_node/reth_data/db/"
ROOT_FOLDER_PATH="/media/alp/blockchain/_node/brontes_db/Database/"

CLICKHOUSE_URL="192.168.1.171"
CLICKHOUSE_PORT="8123"
CLICKHOUSE_USER="admin"
CLICKHOUSE_PASS="YK6Z1rYZ"
CLICKHOUSE_DATABASE="brontes"

RETH_URL="http://127.0.0.1"
RETH_PORT="8545"
RETH_DB_PATH="/media/alp/blockchain/_node/reth_data/db/"
RETH_WS_URL="ws://127.0.0.1"
RETH_WS_PORT="8546"

ARKHAM_API_KEY=""

BIGQUERY_PROJECT_ID=""
BIGQUERY_SA_FILE=""
BIGQUERY_DC_FILE=""

TARDIS_API_KEY=""

CEX_BINANCE_S3_URL=""

ETHERSCAN_API_KEY="IXH6AE5PJPG28H3U7UFP64YV3FG19W3FVD"

*/




#[derive(Debug)]
pub struct SettingsTab {
    selected_row: usize,
    protocols: Vec<Protocol>,
    messages: Vec<String>,
     /// Current value of the input box
     input: String,
     /// Position of cursor in the editor area.
     cursor_position: usize,
     /// Current input mode
     input_mode: InputMode,

}

impl SettingsTab {
    pub fn new(selected_row: usize) -> Self {
        Self {
            selected_row: selected_row ,
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            cursor_position: 0,
            protocols: vec![Protocol{
                name: "AaveV2Pool",
                mevtypes: vec![
                    MevType { name: "Sandwich" },
                    MevType { name: "Jit Sandwich" },
                    MevType { name: "Cex-Dex" },
                    MevType { name: "Jit" },
                    MevType { name: "Atomic Backrun" },
                    MevType { name: "Liquidation" },
                ]},
                Protocol{name: "AaveV3Pool",
                mevtypes: vec![
                    MevType { name: "Sandwich" },
                    MevType { name: "Jit Sandwich" },
                    MevType { name: "Cex-Dex" },
                    MevType { name: "Jit" },
                    MevType { name: "Atomic Backrun" },
                    MevType { name: "Liquidation" },
                ]},
                Protocol{name: "Curve",
                mevtypes: vec![
                    MevType { name: "Sandwich" },
                    MevType { name: "Jit Sandwich" },
                    MevType { name: "Cex-Dex" },
                    MevType { name: "Jit" },
                    MevType { name: "Atomic Backrun" },
                    MevType { name: "Liquidation" },
                ]},
                Protocol{name: "MakerPSM",
                mevtypes: vec![
                    MevType { name: "Sandwich" },
                    MevType { name: "Jit Sandwich" },
                    MevType { name: "Cex-Dex" },
                    MevType { name: "Jit" },
                    MevType { name: "Atomic Backrun" },
                    MevType { name: "Liquidation" },
                ]},
                Protocol{name: "PancakeSwapV3",
                mevtypes: vec![
                    MevType { name: "Sandwich" },
                    MevType { name: "Jit Sandwich" },
                    MevType { name: "Cex-Dex" },
                    MevType { name: "Jit" },
                    MevType { name: "Atomic Backrun" },
                    MevType { name: "Liquidation" },
                ]},
                Protocol{name: "SushiSwapV2",
                mevtypes: vec![
                    MevType { name: "Sandwich" },
                    MevType { name: "Jit Sandwich" },
                    MevType { name: "Cex-Dex" },
                    MevType { name: "Jit" },
                    MevType { name: "Atomic Backrun" },
                    MevType { name: "Liquidation" },
                ]},
                Protocol{name: "SushiSwapV3",
                mevtypes: vec![
                    MevType { name: "Sandwich" },
                    MevType { name: "Jit Sandwich" },
                    MevType { name: "Cex-Dex" },
                    MevType { name: "Jit" },
                    MevType { name: "Atomic Backrun" },
                    MevType { name: "Liquidation" },
                ]},
                Protocol{name: "UniswapV2",
                mevtypes: vec![
                    MevType { name: "Sandwich" },
                    MevType { name: "Jit Sandwich" },
                    MevType { name: "Cex-Dex" },
                    MevType { name: "Jit" },
                    MevType { name: "Atomic Backrun" },
                    MevType { name: "Liquidation" },
                ]},
                Protocol{name: "UniswapV3",
                mevtypes: vec![
                    MevType { name: "Sandwich" },
                    MevType { name: "Jit Sandwich" },
                    MevType { name: "Cex-Dex" },
                    MevType { name: "Jit" },
                    MevType { name: "Atomic Backrun" },
                    MevType { name: "Liquidation" },
                ]},
            ]
        }
    }
}

impl Widget for SettingsTab {
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

        let scrollbar_area = Rect {
            y: area.y + 2,
            height: area.height - 3,
            ..area
        };

        let area = area.inner(&Margin {
            horizontal: 2,
            vertical: 1,
        });
        let area = layout(area, Direction::Horizontal, vec![44, 0]);

    }
}


fn render_variables(widget: &mut SettingsTab, area: Rect, buf: &mut Buffer) {

    let (msg, style) = match widget.input_mode {
        InputMode::Normal => (
            vec![
                "Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "e".bold(),
                " to start editing.".bold(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to stop editing, ".into(),
                "Enter".bold(),
                " to record the message".into(),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    help_message.render(area, buf);
    //f.render_widget(help_message, area);

    let input = Paragraph::new(widget.input.as_str())
        .style(match widget.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    //f.render_widget(input, chunks[1]);
    input.render(area, buf);
    match widget.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}
/*
        InputMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            f.set_cursor(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
               area.x + app.cursor_position as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
            )
        }
*/


    }

    let messages: Vec<ListItem> = widget
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(format!("{i}: {m}")));
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    //messages.render(area, buf, state)
    //f.render_widget(messages, chunks[2]);


}

