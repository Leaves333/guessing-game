use std::cmp::max;

use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    text::{Line, Span, Text},
    widgets::{
        Axis, Bar, BarChart, BarGroup, Block, Chart, Dataset, GraphType, List, ListItem, Paragraph,
    },
    Frame,
};

use crate::{app::App, input::Focus};

// contains functions used to render app state to the terminal
impl App {
    pub fn draw(&self, frame: &mut Frame) {
        let outer_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, bottom] = outer_layout.areas(frame.area());

        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [left, messages_area] = horizontal.areas(bottom);

        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [bars_area, plot_area] = vertical.areas(left);

        self.render_help(frame, help_area);
        self.render_input(frame, input_area);
        self.render_message(frame, messages_area);
        self.render_deviation_plot(frame, plot_area);
        self.render_bar_chart(frame, bars_area);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let (msg, style) = match self.focus {
            Focus::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to guess some numbers.".into(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            Focus::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop guessing, ".into(),
                    "Enter".bold(),
                    " to submit your guess.".into(),
                ],
                Style::default(),
            ),
        };

        let lines = vec![
            Line::from("numbers guessing game !!!"),
            Line::from("guess a number between 1 and 100"),
            Line::from(msg),
        ];
        let text = Text::from(lines).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, area);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.input.as_str())
            .style(match self.focus {
                Focus::Normal => Style::default(),
                Focus::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, area);

        match self.focus {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            Focus::Normal => {}

            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            Focus::Editing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                area.x + self.character_index as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
            )),
        }
    }

    fn render_message(&self, frame: &mut Frame, area: Rect) {
        let list_items = self.messages.iter().map(|s| {
            let content = Line::from(Span::raw(s));
            ListItem::new(content)
        });
        let messages = List::new(list_items).block(Block::bordered().title("Messages"));
        frame.render_widget(messages, area);
    }

    fn render_deviation_plot(&self, frame: &mut Frame, area: Rect) {
        let data = self
            .deviations
            .iter()
            .enumerate()
            .map(|(i, x)| (i as f64, *x as f64))
            .collect::<Vec<(f64, f64)>>();

        let datasets = vec![Dataset::default()
            .name("Deviation")
            .marker(Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::new().yellow())
            .data(&data)];

        let mid_bound_str = &(max(1, data.len() / 2)).to_string();
        let upper_bound_str = &(max(2, data.len()).to_string());
        let chart = Chart::new(datasets)
            .block(Block::bordered().title(Line::from("Bottom Chart").cyan().bold().centered()))
            .x_axis(
                Axis::default()
                    .title("Guess #")
                    .bounds([0., max(2, data.len()) as f64])
                    .style(Style::default().fg(Color::Gray))
                    .labels(["0", mid_bound_str, upper_bound_str]),
            )
            .y_axis(
                Axis::default()
                    .title("Deviation")
                    .bounds([-100., 100.])
                    .style(Style::default().fg(Color::Gray))
                    .labels(["-100", "0", "100"]),
            )
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

        frame.render_widget(chart, area);
    }

    fn render_bar_chart(&self, frame: &mut Frame, area: Rect) {
        let bars: Vec<Bar> = vec![
            Bar::default()
                .value(self.guesses_low.into())
                .label(Line::from("too low"))
                .text_value(format!("{}", self.guesses_low))
                .style(Style::new().fg(Color::Magenta))
                .value_style(Style::new().fg(Color::Black).bg(Color::Magenta)),
            Bar::default()
                .value(self.guesses_high.into())
                .label(Line::from("too high"))
                .text_value(format!("{}", self.guesses_high))
                .style(Style::new().fg(Color::Magenta))
                .value_style(Style::new().fg(Color::Black).bg(Color::Magenta)),
            Bar::default()
                .value(self.guesses_right.into())
                .label(Line::from("just right"))
                .text_value(format!("{}", self.guesses_right))
                .style(Style::new().fg(Color::Magenta))
                .value_style(Style::new().fg(Color::Black).bg(Color::Magenta)),
        ];

        let barchart = BarChart::default()
            .block(Block::bordered().title(Line::from("Your Guesses").cyan().bold().centered()))
            .data(BarGroup::default().bars(&bars))
            .bar_width((area.width - 6) / 3)
            .bar_gap(3)
            .direction(Direction::Vertical);
        frame.render_widget(barchart, area);
    }
}
