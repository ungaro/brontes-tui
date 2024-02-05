#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use std::borrow::BorrowMut;

use lazy_static::lazy_static;

use itertools::Itertools;

use ratatui::{
    prelude::*,
    text::Line,
    widgets::{canvas::*, *},
};

use crate::{layout, RgbSwatch, THEME};



const LOGS: [(&str, &str); 26] = [
    ("Event1", "INFO"),
    ("Event2", "INFO"),
    ("Event3", "CRITICAL"),
    ("Event4", "ERROR"),
    ("Event5", "INFO"),
    ("Event6", "INFO"),
    ("Event7", "WARNING"),
    ("Event8", "INFO"),
    ("Event9", "INFO"),
    ("Event10", "INFO"),
    ("Event11", "CRITICAL"),
    ("Event12", "INFO"),
    ("Event13", "INFO"),
    ("Event14", "INFO"),
    ("Event15", "INFO"),
    ("Event16", "INFO"),
    ("Event17", "ERROR"),
    ("Event18", "ERROR"),
    ("Event19", "INFO"),
    ("Event20", "INFO"),
    ("Event21", "WARNING"),
    ("Event22", "INFO"),
    ("Event23", "INFO"),
    ("Event24", "WARNING"),
    ("Event25", "INFO"),
    ("Event26", "INFO"),
];

lazy_static! {
    static ref BARCHART_DATA: Vec<(&'static str, u64)> = vec![
        ("B1", 9),
        ("B2", 12),
        ("B3", 5),
        ("B4", 8),
        ("B5", 2),
        ("B6", 4),
        ("B7", 5),
        ("B8", 9),
        ("B9", 14),
        ("B10", 15),
        ("B11", 1),
        ("B12", 0),
        ("B13", 4),
        ("B14", 6),
        ("B15", 4),
        ("B16", 6),
        ("B17", 4),
        ("B18", 7),
        ("B19", 13),
        ("B20", 8),
        ("B21", 11),
        ("B22", 9),
        ("B23", 3),
        ("B24", 5),
    ];
}


pub struct ProtocolsTab {
    selected_row: usize,
    data: Vec<(&'static str, u64)>,
    log_scroll: u16,
    items: Vec<Vec<&'static str>>,
    stream_table_state: TableState,
    //leaderboard: Vec<Vec<&'static str>>,
    leaderboard: Vec<(&'static str, u64)>,

    //    events: Vec<(&'static str, &'static str,)>,
    events: Vec<(
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    )>,
    //pub barchart: Vec<(&'a str, u64)>,
}

impl ProtocolsTab {
    pub fn new(selected_row: usize) -> Self {
        Self {
            selected_row,
            log_scroll: 0,
            data: vec![
                ("Sandwich", 9),
                ("Jit Sandwich", 8),
                ("Cex-Dex", 12),
                ("Jit", 5),
                ("Atomic Backrun", 2),
                ("Liquidation", 4),
            ],
            stream_table_state: TableState::default(),
            items: vec![
                vec![
                    "19150733",
                    "4 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
                vec![
                    "19150733",
                    "10 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
                vec![
                    "19150733",
                    "24 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
                vec![
                    "19150733",
                    "32 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
                vec![
                    "19150733",
                    "33 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
                vec![
                    "19150733",
                    "44 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
                vec![
                    "19150733",
                    "54 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
                vec![
                    "19150733",
                    "64 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
                vec![
                    "19150733",
                    "78 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
                vec![
                    "19150733",
                    "80 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
                vec![
                    "19150733",
                    "90 mins ago",
                    "Sandwich",
                    "WETH/UNCL",
                    "0x2823784...2344",
                    "0x2823784...2344",
                    "$10",
                    "$20",
                ],
            ],
            leaderboard: vec![
                ("jaredfromsubway.eth", 1_200_000),
                ("0x23892382394..212", 1_100_000),
                ("0x13897682394..243", 1_000_000),
                ("0x33899882394..223", 900_000),
                ("0x43894082394..265", 800_000),
                ("0x53894082394..283", 700_000),
                ("0x83894082394..293", 600_000),
                // Repeat as necessary
            ],

            events: vec![
                ("Atomic Backrun", "#123456789", "ETHIB/WETH", "$4", "$20"),
                ("Sandwich", "#123456789", "ETHIB/WETH", "$4", "$20"),
                ("Jit Sandwich", "#123456789", "ETHIB/WETH", "$4", "$20"),
                ("Jit", "#123456789", "ETHIB/WETH", "$4", "$20"),
                ("Cex-Dex", "#123456789", "ETHIB/WETH", "$4", "$20"),
                ("Liquidation", "#123456789", "ETHIB/WETH", "$4", "$20"),
            ],
        }
    }
    pub fn next(&mut self) {
        let i = match self.stream_table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.stream_table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.stream_table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.stream_table_state.select(Some(i));
    }

    fn on_tick(&mut self) {
        self.log_scroll += 1;
        self.log_scroll %= 10;
    }
}

impl Widget for ProtocolsTab {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let area = area.inner(&Margin {
            vertical: 1,
            horizontal: 4,
        });

        let chunks = Layout::default()
            .constraints([
                Constraint::Length(9),
                Constraint::Length(9),
                Constraint::Length(9),
                Constraint::Length(9),
                Constraint::Length(9),
            ])
            .split(area);

        let sub_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

            let sub_layout2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

            let sub_layout3 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

            let sub_layout4 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[3]);
        let sub_layout5 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[4]);


        //draw_protocol(&self, sub_layout[0], buf);
        draw_protocol(&mut self, sub_layout[0], buf,"ALL");
        draw_protocol(&mut self, sub_layout[1], buf,"AaveV2Pool");
        draw_protocol(&mut self, sub_layout2[0], buf,"AaveV3Pool");
        draw_protocol(&mut self, sub_layout2[1], buf,"Curve");
        draw_protocol(&mut self, sub_layout3[0], buf,"MakerPSM");
        draw_protocol(&mut self, sub_layout3[1], buf,"PancakeSwapV3");
        draw_protocol(&mut self, sub_layout4[0], buf,"SushiSwapV2");
        draw_protocol(&mut self, sub_layout4[1], buf,"SushiSwapV3");
        draw_protocol(&mut self, sub_layout5[0], buf,"UniSwapV2");
        draw_protocol(&mut self, sub_layout5[1], buf,"UniSwapV3");
    }
}

//fn draw_livestream(f: &mut Frame, app: &mut App) {

fn draw_protocol(widget: &mut ProtocolsTab, area: Rect, buf: &mut Buffer, protocol: &str) {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::DarkGray);
    let header_cells = [
        "Block#", "Time", "MEV Type", "Tokens", "From", "Contract", "Profit", "Cost",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = widget.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(0)
    });
    let t = Table::new(
        rows,
        [
            Constraint::Max(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(protocol))
    .highlight_style(selected_style)
    .highlight_symbol(">> ");
    //t.render(area, buf, selected_row);
    //f.render_stateful_widget(t, rects[0], &mut app.state);
    ratatui::widgets::StatefulWidget::render(t, area, buf, &mut widget.stream_table_state);
}
