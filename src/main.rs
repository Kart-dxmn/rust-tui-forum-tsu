use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::*,
};
use std::{
    io,
    time::{Duration, Instant},
};

// Состояния презентации
#[derive(PartialEq)]
enum PresentationState {
    Loading,
    ForumInvitation,
}

struct App {
    state: PresentationState,
    progress: f64,
    paused: bool,
    scroll_offset: usize, // Добавляем offset для прокрутки
    text: Vec<String>,    // Текст для блока "Почему я?"
}

impl App {
    fn new() -> App {
        // Создаем много строк для прокрутки
        let mut text = Vec::new();
        text.push("1. Почему я хочу поступить в ТГУ?".to_string());
        text.push("• Я точно знаю, что хочу поступать в ТГУ на ИПМКН или Высшую IT-школу, другие места даже не рассматриваю. Множество моих знакомых, от лечащего врача до двоюродных братьев и сестёр,  учились именно здесь. Я наслышал об уровне образования в Томске, а особенно в ТГУ. Конечно я бы мог пойти учиться в Питер или МСК, но решил, что мне будет тяжко и не нужен пафос. В ТГУ же тот же уровень образования, или местами даже лучше, а ещё это рядом с родным городом, дешевле и проще, именно простота, которую я очень ценю как феномен во всем мире заставила решить идти в ТГУ".to_string());
        text.push("2. Почему меня стоит пригласить на форум?".to_string());
        text.push("• Несмотря на то, что я не являюсь выдающимся школьником, я изучил много технологий в IT. Сама это презентация пример для вас, я расчитываю, что сама презентация станет для вас причиной позвать меня, посмотреть на мои навыки и предложить задачи. С обязательствами ученика Гимназии №1, я не знал как себя проявить, считал все олимпиады либо слишком сложными, либо не интересными. В 9 классе после сдачи ОГЭ я понял, что моя личность — мои навыки, и потому время в 10 и 11 классе были потрачены на прокачку скиллов. То есть портфолио у меня нет, но есть я сам и мои возможности, которыми мне стоит воспользоваться чтобы показать себя. Именно поэтому вы видите этот TUI.".to_string());
        text.push("3. Ожидания от участия".to_string());
        text.push("• Я не планирую получить что-то от участия. Я хочу приехать, даже не пользуясь предложением от университета о бесплатном проживании, чтобы посмотреть город, увидеться со знакомыми, посмотреть ВУЗ и попасть в атмосферу ТГУ, проникнуться ей. И ещё хочу показать себя ВУЗу и другим ребятам, может быть даже найти надежных друзей для совместных проектов для open-source".to_string());
        text.push("4. О моих достижения и планах на учебу".to_string());
        text.push("• Как и говорил выше, особых достижений в учебе у меня нет. На данный момент я пишу свой проект: TUI с дистанционным сбором данных и управлением несколькими (не-)интернет устройствами и горжусь уже проделанной работой, также у меня полностью настроена ОС Archlinux. Я активно слежу за цифровыми технологиями, особенно сферой системного программирования и администрирования, и также активно изучаю интересные мне инструменты. У меня всего один план: мне необходимо завершить этап жизни <<Школа>> сдачей ЕГЭ на 80+ по предмету и начать новый - <<ВУЗ>>, где я хочу получить институтскую базу, увеличить свой стэк, как разработчик, найти своё общество, свою первую работу; к чему на данный момент готовлюсь  как морально так и реально".to_string());
        App {
            state: PresentationState::Loading,
            progress: 0.0,
            paused: false,
            scroll_offset: 0,
            text,
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        match self.state {
            PresentationState::Loading => {
                self.progress += 0.01;
                if self.progress >= 1.0 {
                    self.state = PresentationState::ForumInvitation;
                    self.progress = 0.0;
                    self.scroll_offset = 0;
                }
            }
            PresentationState::ForumInvitation => {}
        }
    }

    // прокрутка вверх
    fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    // Метод для прокрутки вниз
    fn scroll_down(&mut self, max_offset: usize) {
        if self.scroll_offset < max_offset {
            self.scroll_offset += 1;
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('p') | KeyCode::Char(' ') => {
                        app.paused = !app.paused;
                    }
                    KeyCode::Up => {
                        if app.state == PresentationState::ForumInvitation {
                            app.scroll_up();
                        }
                    }
                    KeyCode::Down => {
                        if app.state == PresentationState::ForumInvitation {
                            // Вычисляем максимальный offset
                            let inner_height =
                                terminal.size().map(|s| s.height).unwrap_or(20) as usize;
                            let visible_lines = (inner_height - 20) as usize; // Примерное кол-во видимых строк
                            let max_offset = app.text.len().saturating_sub(visible_lines);
                            app.scroll_down(max_offset);
                        }
                    }
                    KeyCode::PageUp => {
                        if app.state == PresentationState::ForumInvitation {
                            app.scroll_offset = app.scroll_offset.saturating_sub(5);
                        }
                    }
                    KeyCode::PageDown => {
                        if app.state == PresentationState::ForumInvitation {
                            let inner_height =
                                terminal.size().map(|s| s.height).unwrap_or(20) as usize;
                            let visible_lines = (inner_height - 20) as usize;
                            let max_offset = app.text.len().saturating_sub(visible_lines);
                            app.scroll_offset = (app.scroll_offset + 5).min(max_offset);
                        }
                    }
                    KeyCode::Home => {
                        if app.state == PresentationState::ForumInvitation {
                            app.scroll_offset = 0;
                        }
                    }
                    KeyCode::End => {
                        if app.state == PresentationState::ForumInvitation {
                            let inner_height =
                                terminal.size().map(|s| s.height).unwrap_or(20) as usize;
                            let visible_lines = (inner_height - 20) as usize;
                            app.scroll_offset = app.text.len().saturating_sub(visible_lines);
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate && !app.paused {
            app.update();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(size);

    match app.state {
        PresentationState::Loading => {
            let main_block = Block::default()
                .borders(Borders::ALL)
                .title(" TUI Презентация ");
            f.render_widget(main_block, chunks[0]);

            let area = chunks[0];
            let inner_area = Rect {
                x: area.x + 2,
                y: area.y + 1,
                width: area.width.saturating_sub(4),
                height: area.height.saturating_sub(2),
            };

            // TSU ASCII Art
            let tsu_art = Paragraph::new(vec![
                Line::from("████████╗███████╗██╗   ██╗"),
                Line::from("╚══██╔══╝██╔════╝██║   ██║"),
                Line::from("   ██║   ███████╗██║   ██║"),
                Line::from("   ██║   ╚════██║██║   ██║"),
                Line::from("   ██║   ███████║╚██████╔╝"),
                Line::from("   ╚═╝   ╚══════╝ ╚═════╝ "),
            ])
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            );
            f.render_widget(tsu_art, inner_area);

            // Проценты загрузки
            let percent = (app.progress * 100.0) as u8;
            let percent_text = format!("Загрузка: {}%", percent);

            let percent_y = inner_area.y + 8;
            if percent_y < inner_area.y + inner_area.height {
                let percent_area = Rect {
                    x: inner_area.x,
                    y: percent_y,
                    width: inner_area.width,
                    height: 1,
                };
                let percent_widget = Paragraph::new(percent_text)
                    .alignment(Alignment::Center)
                    .style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    );
                f.render_widget(percent_widget, percent_area);
            }

            // Прогресс бар
            let gauge_y = inner_area.y + 10;
            if gauge_y + 3 < inner_area.y + inner_area.height {
                let gauge_area = Rect {
                    x: inner_area.x,
                    y: gauge_y,
                    width: inner_area.width,
                    height: 3,
                };
                let gauge = Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title("Прогресс"))
                    .gauge_style(Style::default().fg(Color::Green))
                    .label("Взлом почты...")
                    .percent(percent.into());

                f.render_widget(gauge, gauge_area);
            }
        }
        PresentationState::ForumInvitation => {
            let block = Block::default().borders(Borders::ALL).title("ПИСЬМО");
            f.render_widget(block.clone(), chunks[0]);

            let inner = block.inner(chunks[0]);

            // Создаем разметку для трех частей
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(8), // Верхняя часть с крупным заголовком
                        Constraint::Min(8),    // Средняя часть с блоком "Почему я?"
                        Constraint::Length(6), // Нижняя часть с отправителем/получателем
                    ]
                    .as_ref(),
                )
                .split(inner);

            // === ВЕРХНЯЯ ЧАСТЬ - КРУПНЫЙ ЗАГОЛОВОК ===
            let title_text = vec![
                Line::from("╔════════════════════════════════════════╗"),
                Line::from("║                                        ║"),
                Line::from("║          Сопроводительное              ║"),
                Line::from("║               Письмо                   ║"),
                Line::from("║                                        ║"),
                Line::from("╚════════════════════════════════════════╝"),
            ];

            let title = Paragraph::new(title_text)
                .alignment(Alignment::Center)
                .style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );
            f.render_widget(title, layout[0]);

            // === СРЕДНЯЯ ЧАСТЬ - БЛОК "ПОЧЕМУ Я?" ===
            let why_block = Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    " Почему я?(Моя визитка) [строка {} из {}] ",
                    app.scroll_offset + 1,
                    app.text.len()
                ))
                .border_style(Style::default().fg(Color::Cyan));

            let why_inner = why_block.inner(layout[1]);
            f.render_widget(why_block, layout[1]);

            // Создаем текст с учетом прокрутки
            let visible_lines: Vec<Line> = app
                .text
                .iter()
                .skip(app.scroll_offset)
                .take(why_inner.height as usize)
                .enumerate()
                .map(|(i, s)| {
                    let real_index = app.scroll_offset + i;

                    if real_index % 2 == 0 {
                        // Четные индексы (0, 2, 4...) - заголовки жирные
                        Line::from(vec![Span::styled(
                            s,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )])
                    } else {
                        // Нечетные индексы - обычный текст
                        Line::from(vec![Span::styled(s, Style::default().fg(Color::White))])
                    }
                })
                .collect();

            let why_paragraph = Paragraph::new(visible_lines)
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(Color::White));
            f.render_widget(why_paragraph, why_inner);

            // === НИЖНЯЯ ЧАСТЬ - ОТПРАВИТЕЛЬ/ПОЛУЧАТЕЛЬ ===
            let footer_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(layout[2]);

            // Левая часть - подсказки по управлению
            let left_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4),
                    Constraint::Length(4),
                ].as_ref())
                .split(footer_layout[0]);
            let project_info = vec![
                Line::from(""),
                Line::from("Проект: github.com/Kart-dxmn/rust-tui-forum-tsu "),
                ];
            let project_paragraph = Paragraph::new(project_info)
                .alignment(Alignment::Right)
                .style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD));
            f.render_widget(project_paragraph, left_layout[1]);   
            let help_text = vec![
                Line::from("Управление:"),
                Line::from("↑/↓ - прокрутка"),
                Line::from("PgUp/PgDn - страница"),
                Line::from("Home/End - начало/конец"),
            ];

            
            let help_paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::Gray));
            f.render_widget(help_paragraph, left_layout[0]);

            // Правая часть - Кому/От кого
            let right_text = vec![
                Line::from(""),
                Line::from("Кому: НИ ТГУ"),
                Line::from(""),
                Line::from("От кого: Стенин Сергей, Гимназия №1"),
                Line::from(""),
                Line::from(""),
            ];

            let right_paragraph = Paragraph::new(right_text)
                .alignment(Alignment::Right)
                .style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                );
            f.render_widget(right_paragraph, footer_layout[1]);
        }
    }

    draw_controls(f, chunks[1], app.paused);
}

fn draw_controls(f: &mut Frame, area: Rect, paused: bool) {
    let pause_text = if paused { "▶" } else { "⏸" };

    let controls_text = vec![Line::from(vec![
        Span::styled(
            " Управление: ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled("p", Style::default().fg(Color::Cyan)),
        Span::raw(" — "),
        Span::styled(
            format!("{} пауза", pause_text),
            Style::default().fg(if paused { Color::Green } else { Color::Yellow }),
        ),
        Span::raw("  "),
        Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
        Span::raw(" — прокрутка  "),
        Span::styled(
            "q",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" — выход"),
    ])];

    let controls_block = Block::default()
        .borders(Borders::ALL)
        .title(" Команды ")
        .border_style(Style::default().fg(Color::Gray));

    let controls = Paragraph::new(controls_text)
        .block(controls_block)
        .alignment(Alignment::Center);

    f.render_widget(controls, area);
}
