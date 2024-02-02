use lazy_static::lazy_static;


use itertools::Itertools;

use ratatui::{
    prelude::*,
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
        ("B1", 9), ("B2", 12), ("B3", 5), ("B4", 8), ("B5", 2), ("B6", 4), ("B7", 5),
        ("B8", 9), ("B9", 14), ("B10", 15), ("B11", 1), ("B12", 0), ("B13", 4), ("B14", 6),
        ("B15", 4), ("B16", 6), ("B17", 4), ("B18", 7), ("B19", 13), ("B20", 8), ("B21", 11),
        ("B22", 9), ("B23", 3), ("B24", 5),
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

/*
    performance: vec![
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
    ],
    */
pub struct DashboardTab {
    selected_row: usize,
    data: Vec<(&'static str, u64)>,
    //pub barchart: Vec<(&'a str, u64)>,


}

impl DashboardTab {
    pub fn new(selected_row: usize) -> Self {
        Self { selected_row,
            data: vec![
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
            ], }
    }
}

impl Widget for DashboardTab {
    fn render(self, area: Rect, buf: &mut Buffer) {

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
    draw_charts(self, area, buf);

        /*
        RgbSwatch.render(area, buf);
        let area = layout(area, Direction::Horizontal, vec![34, 0]);
        render_crate_description(area[1], buf);
        render_logo(self.selected_row, area[0], buf);

        */
    }
}

fn draw_charts(widget: DashboardTab, area: Rect, buf: &mut Buffer) {

//fn draw_charts(f: &mut Frame, area: Rect) {
    /*
    let constraints = if app.show_chart {
        vec![Constraint::Percentage(50), Constraint::Percentage(50)]
    } else {
        vec![Constraint::Percentage(100)]
    };
*/

let barchart = BarChart::default()
.block(Block::default().borders(Borders::ALL).title("Bar chart"))
.data(&widget.data)
.bar_width(3)
.bar_gap(2)
.bar_set(
    symbols::bar::NINE_LEVELS)
.value_style(
    Style::default()
        .fg(Color::Black)
        .bg(Color::Green)
        .add_modifier(Modifier::ITALIC),
)
.label_style(Style::default().fg(Color::Yellow))
.bar_style(Style::default().fg(Color::Green));
barchart.render(area, buf);
//Canvas.render_widget(barchart, chunks[1]);


/*


    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);
        {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .direction(Direction::Horizontal)
                .split(chunks[0]);

            // Draw tasks
            let tasks: Vec<ListItem> = app
                .tasks
                .items
                .iter()
                .map(|i| ListItem::new(vec![text::Line::from(Span::raw(*i))]))
                .collect();
            let tasks = List::new(tasks)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");
            f.render_stateful_widget(tasks, chunks[0], &mut app.tasks.state);

            // Draw logs
            let info_style = Style::default().fg(Color::Blue);
            let warning_style = Style::default().fg(Color::Yellow);
            let error_style = Style::default().fg(Color::Magenta);
            let critical_style = Style::default().fg(Color::Red);
            let logs: Vec<ListItem> = app
                .logs
                .items
                .iter()
                .map(|&(evt, level)| {
                    let s = match level {
                        "ERROR" => error_style,
                        "CRITICAL" => critical_style,
                        "WARNING" => warning_style,
                        _ => info_style,
                    };
                    let content = vec![text::Line::from(vec![
                        Span::styled(format!("{level:<9}"), s),
                        Span::raw(evt),
                    ])];
                    ListItem::new(content)
                })
                .collect();
            let logs = List::new(logs).block(Block::default().borders(Borders::ALL).title("List"));
            f.render_stateful_widget(logs, chunks[1], &mut app.logs.state);
        }

        let barchart = BarChart::default()
            .block(Block::default().borders(Borders::ALL).title("Bar chart"))
            .data(&app.barchart)
            .bar_width(3)
            .bar_gap(2)
            .bar_set(if app.enhanced_graphics {
                symbols::bar::NINE_LEVELS
            } else {
                symbols::bar::THREE_LEVELS
            })
            .value_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::ITALIC),
            )
            .label_style(Style::default().fg(Color::Yellow))
            .bar_style(Style::default().fg(Color::Green));
        f.render_widget(barchart, chunks[1]);
    }
    if app.show_chart {
        let x_labels = vec![
            Span::styled(
                format!("{}", app.signals.window[0]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "{}",
                (app.signals.window[0] + app.signals.window[1]) / 2.0
            )),
            Span::styled(
                format!("{}", app.signals.window[1]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ];
        let datasets = vec![
            Dataset::default()
                .name("data2")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&app.signals.sin1.points),
            Dataset::default()
                .name("data3")
                .marker(if app.enhanced_graphics {
                    symbols::Marker::Braille
                } else {
                    symbols::Marker::Dot
                })
                .style(Style::default().fg(Color::Yellow))
                .data(&app.signals.sin2.points),
        ];
        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(Span::styled(
                        "Chart",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("X Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds(app.signals.window)
                    .labels(x_labels),
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([-20.0, 20.0])
                    .labels(vec![
                        Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw("0"),
                        Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                    ]),
            );
        f.render_widget(chart, chunks[1]);
    }


    */
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
