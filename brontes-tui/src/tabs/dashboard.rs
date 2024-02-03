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

const RATATUI_LOGO: [&str; 32] = [
    "               ███              ",
    "             ██████             ",
    "            ███████             ",
    "           ████████             ",
    "          █████████             ",
    "         ██████████             ",
    "        ████████████            ",
    "        █████████████           ",
    "        █████████████     ██████",
    "         ███████████    ████████",
    "              █████ ███████████ ",
    "               ███ ██ee████████ ",
    "                █ █████████████ ",
    "            ████ █████████████  ",
    "           █████████████████    ",
    "           ████████████████     ",
    "           ████████████████     ",
    "            ███ ██████████      ",
    "          ██    █████████       ",
    "         █xx█   █████████       ",
    "        █xxxx█ ██████████       ",
    "       █xx█xxx█ █████████       ",
    "      █xx██xxxx█ ████████       ",
    "     █xxxxxxxxxx█ ██████████    ",
    "    █xxxxxxxxxxxx█ ██████████   ",
    "   █xxxxxxx██xxxxx█ █████████   ",
    "  █xxxxxxxxx██xxxxx█ ████  ███  ",
    " █xxxxxxxxxxxxxxxxxx█ ██   ███  ",
    "█xxxxxxxxxxxxxxxxxxxx█ █   ███  ",
    "█xxxxxxxxxxxxxxxxxxxxx█   ███   ",
    " █xxxxxxxxxxxxxxxxxxxxx█ ███    ",
    "  █xxxxxxxxxxxxxxxxxxxxx█ █     ",
];

pub struct DashboardTab {
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

impl DashboardTab {
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

impl Widget for DashboardTab {
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

        let sub_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        draw_charts(&self, sub_layout[0], buf);
        draw_leaderboard(&self, sub_layout[1], buf);
        //draw_events(&self, chunks[1], buf, 1);
        draw_livestream(&mut self, chunks[1], buf);

        draw_logs(&self, chunks[2], buf, 1);
    }
}

//fn draw_livestream(f: &mut Frame, app: &mut App) {

fn draw_livestream(widget: &mut DashboardTab, area: Rect, buf: &mut Buffer) {
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
        Row::new(cells).height(height as u16).bottom_margin(1)
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
    .block(Block::default().borders(Borders::ALL).title("Live Stream"))
    .highlight_style(selected_style)
    .highlight_symbol(">> ");
    //t.render(area, buf, selected_row);
    //f.render_stateful_widget(t, rects[0], &mut app.state);
    ratatui::widgets::StatefulWidget::render(t, area, buf, &mut widget.stream_table_state);
}

fn draw_logs(widget: &DashboardTab, area: Rect, buf: &mut Buffer, selected_row: usize) {
    let area_width = area.width;

    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";

    let mut long_line = s.repeat(usize::from(area_width) / s.len() + 4);
    long_line.push('\n');

    let text = vec![
            Line::from("2024-01-31T08:28:19.741687Z  INFO brontes: Collected dex prices for block: 18804995"),
            Line::from("2024-01-31T08:28:19.741702Z  INFO brontes: dex pricing finished".green()),
            Line::from("2024-01-31T08:28:19.836077Z DEBUG brontes_inspect::cex_dex: No CEX quote found for pair: symbol: IXS, symbol: WETH at exchange: Kucoin".red()),
            Line::from("2024-01-31T08:28:19.835551Z  INFO brontes: Collected dex prices for block: 18804997"),
            Line::from(long_line.on_green()),
            Line::from("This is a line".green().italic()),
            Line::from(vec![
                "Masked text: ".into(),
                Span::styled(
                    Masked::new("password", '*'),
                    Style::default().fg(Color::Red),
                ),
            ]),
        ];

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .block(create_block("LOGS"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((widget.log_scroll, 0));
    paragraph.render(area, buf);

}

fn draw_events(widget: &DashboardTab, area: Rect, buf: &mut Buffer, selected_row: usize) {
    let mut state = ListState::default().with_selected(Some(selected_row));

    let events: Vec<ListItem> = widget
        .events
        .iter()
        .rev()
        .map(|&(event, blocknumber, tokens, profit, cost)| {
            // Colorcode the level depending on its type
            let s = match event {
                "Atomic Backrun" => Style::default().fg(Color::Red),
                "ERROR" => Style::default().fg(Color::Magenta),
                "WARNING" => Style::default().fg(Color::Yellow),
                "INFO" => Style::default().fg(Color::Blue),
                _ => Style::default(),
            };

            // Add a example datetime and apply proper spacing between them
            let header = Line::from(vec![
                Span::styled(format!("{tokens:<9}"), s),
                " ".into(),
                "2020-01-01 10:00:00".italic(),
            ]);

            // The event gets its own line
            let log = Line::from(vec![event.into()]);

            // Here several things happen:
            // 1. Add a `---` spacing line above the final list entry
            // 2. Add the Level + datetime
            // 3. Add a spacer line
            // 4. Add the actual event
            ListItem::new(vec![
                Line::from("-".repeat(area.width as usize)),
                header,
                Line::from(""),
                log,
            ])
        })
        .collect();
    let events_list = List::new(events)
        .block(Block::default().borders(Borders::ALL).title("Live Stream"))
        .direction(ListDirection::BottomToTop);

    ratatui::widgets::StatefulWidget::render(events_list, area, buf, &mut state);
    //ratatui::widgets::Widget::render(events_list, area, buf, state)`

    //events_list.render(area,buf,state);
    //f.render_widget(events_list, chunks[1]);

    //events_list.render(area, buf);
}

fn draw_charts(widget: &DashboardTab, area: Rect, buf: &mut Buffer) {
    let barchart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Performance of MEV Types"),
        )
        .data(&widget.data)
        .bar_width(1)
        .bar_gap(0)
        .bar_set(symbols::bar::NINE_LEVELS)
        .value_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::ITALIC),
        )
        .direction(Direction::Horizontal)
        .label_style(Style::default().fg(Color::Yellow))
        .bar_style(Style::default().fg(Color::Green));
    barchart.render(area, buf);
}

fn draw_leaderboard(widget: &DashboardTab, area: Rect, buf: &mut Buffer) {
    let barchart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Leaderboard"))
        //.data(&widget.leaderboard.iter().map(|x| (x[0], x[1].parse().unwrap())).collect::<Vec<_>>())
        .data(&widget.leaderboard)
        .bar_width(1)
        .bar_gap(0)
        .bar_set(symbols::bar::NINE_LEVELS)
        .value_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::ITALIC),
        )
        .direction(Direction::Horizontal)
        .label_style(Style::default().fg(Color::Yellow))
        .bar_style(Style::default().fg(Color::Green));
    barchart.render(area, buf);
}

fn render_crate_description(area: Rect, buf: &mut Buffer) {
    let area = area.inner(
        &(Margin {
            vertical: 4,
            horizontal: 2,
        }),
    );
    Clear.render(area, buf); // clear out the color swatches
    Block::new().style(THEME.content).render(area, buf);
    let area = area.inner(
        &(Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );
    let text = "- cooking up terminal user interfaces -

    Ratatui is a Rust crate that provides widgets (e.g. Paragraph, Table) and draws them to the \
    screen efficiently every frame.";
    Paragraph::new(text)
        .style(THEME.description)
        .block(
            Block::new()
                .title(" Ratatui ")
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP)
                .border_style(THEME.description_title)
                .padding(Padding::new(0, 0, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .render(area, buf);
}

/// Use half block characters to render a logo based on the RATATUI_LOGO const.
///
/// The logo is rendered in three colors, one for the rat, one for the terminal, and one for the
/// rat's eye. The eye color alternates between two colors based on the selected row.
pub fn render_logo(selected_row: usize, area: Rect, buf: &mut Buffer) {
    let eye_color = if selected_row % 2 == 0 {
        THEME.logo.rat_eye
    } else {
        THEME.logo.rat_eye_alt
    };
    let area = area.inner(&Margin {
        vertical: 0,
        horizontal: 2,
    });
    for (y, (line1, line2)) in RATATUI_LOGO.iter().tuples().enumerate() {
        for (x, (ch1, ch2)) in line1.chars().zip(line2.chars()).enumerate() {
            let x = area.left() + x as u16;
            let y = area.top() + y as u16;
            let cell = buf.get_mut(x, y);
            let rat_color = THEME.logo.rat;
            let term_color = THEME.logo.term;
            match (ch1, ch2) {
                ('█', '█') => {
                    cell.set_char('█');
                    cell.fg = rat_color;
                }
                ('█', ' ') => {
                    cell.set_char('▀');
                    cell.fg = rat_color;
                }
                (' ', '█') => {
                    cell.set_char('▄');
                    cell.fg = rat_color;
                }
                ('█', 'x') => {
                    cell.set_char('▀');
                    cell.fg = rat_color;
                    cell.bg = term_color;
                }
                ('x', '█') => {
                    cell.set_char('▄');
                    cell.fg = rat_color;
                    cell.bg = term_color;
                }
                ('x', 'x') => {
                    cell.set_char(' ');
                    cell.fg = term_color;
                    cell.bg = term_color;
                }
                ('█', 'e') => {
                    cell.set_char('▀');
                    cell.fg = rat_color;
                    cell.bg = eye_color;
                }
                ('e', '█') => {
                    cell.set_char('▄');
                    cell.fg = rat_color;
                    cell.bg = eye_color;
                }
                (_, _) => {}
            };
        }
    }
}
