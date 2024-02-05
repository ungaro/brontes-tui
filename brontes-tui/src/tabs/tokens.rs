use std::borrow::BorrowMut;

use lazy_static::lazy_static;

use itertools::Itertools;
use tui_nodes::*;

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


pub struct TokensTab {
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

impl TokensTab {
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

impl Widget for TokensTab {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let area = area.inner(&Margin {
            vertical: 1,
            horizontal: 4,
        });

        let chunks = Layout::default()
            .constraints([
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ])
            .split(area);



        draw_nodes(&mut self, chunks[0], buf);
     
    }
}


fn draw_nodes(widget: &mut TokensTab, area: Rect, buf: &mut Buffer) {
    //let space = area.size();
    // TODO
    /*
	let mut graph = NodeGraph::new(
		vec![
			NodeLayout::new((40, 10)).with_title("a"),
			NodeLayout::new((40, 10)).with_title("b"),
			NodeLayout::new((40, 10)).with_title("c"),
		],
		vec![
			Connection::new(0,0,1,0),
			Connection::new(0,0,2,0),
			Connection::new(1,0,2,1),
		],
		area.width as usize,
		area.height as usize,
	);
	graph.calculate();
	for ea_zone in graph.split(area) {
		let mut minigraph = NodeGraph::new(
			vec![
				NodeLayout::new((2,3)),
				NodeLayout::new((2,3)),
				NodeLayout::new((2,4)),
			],
			vec![
				Connection::new(0,0,1,0),
				Connection::new(0,0,2,0),
				Connection::new(1,0,2,1),
			],
			ea_zone.width as usize,
			ea_zone.height as usize,
		);
		minigraph.calculate();
        minigraph.render(ea_zone, buf);
		//f.render_stateful_widget(minigraph, ea_zone, &mut ());
	}
    graph,render(area, buf);
	//f.render_stateful_widget(graph, space, &mut ());
    */
}
