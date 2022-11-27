use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::VecDeque;
use std::marker::Unpin;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use unicode_width::UnicodeWidthStr;

const HEX_CHARS: &'static str = "ABCDEFabcdef0123456789";

fn hex_char_to_val(ch: char) -> u8 {
    match ch {
        '0'..='9' => (ch as u8) - ('0' as u8),
        'A'..='F' => (ch as u8) - ('A' as u8) + 10u8,
        'a'..='f' => (ch as u8) - ('a' as u8) + 10u8,
        _ => unreachable!(),
    }
}

fn hex_str_to_bytes(hex_str: &[char]) -> Option<Vec<u8>> {
    if hex_str.len() % 2 == 0 {
        Some(
            hex_str
                .iter()
                .step_by(2)
                .zip(hex_str.iter().skip(1).step_by(2))
                .map(|(a, b)| (hex_char_to_val(*a) << 4) | hex_char_to_val(*b))
                .collect::<Vec<u8>>(),
        )
    } else {
        None
    }
}

fn nibble_to_hex_char(nibble: u8) -> u8 {
    assert_eq!(nibble, nibble & 0xF);
    match nibble {
        0..=9 => ('0' as u8) + (nibble),
        0xa..=0xf => ('a' as u8) + (nibble - 10),
        _ => unreachable!(),
    }
}

fn byte_to_hex_str(byte: u8) -> [u8; 2] {
    [
        nibble_to_hex_char(byte >> 4),
        nibble_to_hex_char(byte & 0xF),
    ]
}

fn bytes_to_hex_str(data: &[u8]) -> String {
    String::from_utf8(
        data.iter()
            .map(|byte| byte_to_hex_str(*byte))
            .flatten()
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}

#[derive(Clone)]
enum Message {
    Sent { data: Vec<u8> },
    Received { data: Vec<u8> },
    Error { inner: String },
}

impl ToString for Message {
    fn to_string(&self) -> String {
        match &self {
            &Message::Sent { data } => format!("tx {}", bytes_to_hex_str(&data[..])),
            &Message::Received { data } => format!("rx {}", bytes_to_hex_str(&data[..])),
            &Message::Error { inner } => format!("err {}", inner),
        }
    }
}

struct ApplicationState {
    input: String,
    messages: Arc<Mutex<VecDeque<Message>>>,
    io_handle: tokio::sync::mpsc::Sender<Vec<u8>>,
}

impl ApplicationState {
    fn push_cmd(&mut self) {
        let input: Vec<_> = self.input.drain(..).collect();
        if !input.is_empty() {
            if input.len() % 2 == 0 {
                let byte_data = hex_str_to_bytes(&input[..]).unwrap();
                let mut msgs = self.messages.lock().unwrap();
                msgs.push_back(Message::Sent {
                    data: byte_data.clone(),
                });
                // If there's an error here the channel closed so just exit
                let _ = self.io_handle.blocking_send(byte_data);
            } else {
                let mut msgs = self.messages.lock().unwrap();
                msgs.push_back(Message::Error {
                    inner: format!("odd number of chars in hex string, found {}", input.len()),
                });
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const DEFAULT_TTY: &'static str = "/dev/ttyACM0";
    let mut args = std::env::args();
    let tty_path = args.nth(1).unwrap_or(DEFAULT_TTY.to_string());

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;

    let msg_queue = Arc::new(Mutex::new(VecDeque::new()));
    let (tx, rx) = tokio::sync::mpsc::channel(10);

    let mut app_state = ApplicationState {
        input: String::new(),
        messages: msg_queue.clone(),
        io_handle: tx,
    };

    // Run the app
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    std::thread::spawn(move || {
        rt.block_on(run_io_context(&tty_path, rx, msg_queue));
    });
    let res = run_ui_context(&mut terminal, &mut app_state);

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_ui_context<B: Backend>(
    terminal: &mut tui::Terminal<B>,
    app_state: &mut ApplicationState,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| ui(frame, app_state))?;

        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter => {
                    app_state.push_cmd();
                    app_state.input = String::new();
                }
                KeyCode::Char(ch) => {
                    if HEX_CHARS.contains(ch) {
                        // Only accept characters which are valid in hexadecimal
                        app_state.input.push(ch);
                    }
                }
                KeyCode::Backspace => {
                    app_state.input.pop();
                }
                _ => {}
            }
        }
    }

    Ok(())
}

async fn run_io_context(
    tty_path: &str,
    rcvr: tokio::sync::mpsc::Receiver<Vec<u8>>,
    messages: Arc<Mutex<VecDeque<Message>>>,
) {
    let port = tokio_serial::new(tty_path, 115200)
        .open_native_async()
        .unwrap();
    let (reader, writer) = tokio::io::split(port);

    tokio::select! {
        _ = io_receive(reader, messages.clone()) => {

        },
        _ = io_send(writer, rcvr, messages) => {

        }
    };
}

async fn io_receive<R: AsyncReadExt + Unpin>(
    mut reader: R,
    messages: Arc<Mutex<VecDeque<Message>>>,
) {
    // Read data from serial port, cobs decode, and drop new messages into the queue
    let mut rx_buffer = [0u8; 1024];
    let mut decoded_rx_buffer = [0u8; 1024];
    loop {
        if let Ok(bytes_read) = reader.read(&mut rx_buffer).await {
            if let Ok(bytes_decoded) =
                cobs::decode(&rx_buffer[..bytes_read], &mut decoded_rx_buffer)
            {
                let mut msg_queue = messages.lock().unwrap();
                msg_queue.push_back(Message::Received {
                    data: decoded_rx_buffer[..bytes_decoded].into(),
                });
            } else {
                let mut msg_queue = messages.lock().unwrap();
                msg_queue.push_back(Message::Error {
                    inner: String::from("cobs decode failed"),
                });
            }
        }
    }
}

async fn io_send<W: AsyncWriteExt + Unpin>(
    mut writer: W,
    mut rcvr: tokio::sync::mpsc::Receiver<Vec<u8>>,
    messages: Arc<Mutex<VecDeque<Message>>>,
) {
    let mut encoded_tx_buffer = [0u8; 1024];
    // When we get new messages on the channel, cobs encode, and send over the serial port
    while let Some(msg) = rcvr.recv().await {
        let bytes_encoded = cobs::encode(&msg[..], &mut encoded_tx_buffer);

        if let Err(err) = writer.write_all(&encoded_tx_buffer[..bytes_encoded]).await {
            let mut messages = messages.lock().unwrap();
            messages.push_back(Message::Error {
                inner: format!("{err:?}"),
            });
        }
    }
}

fn ui<B: Backend>(frame: &mut tui::Frame<B>, app_state: &mut ApplicationState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(frame.size());

    let input = Paragraph::new(app_state.input.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Send Cmd"));
    frame.render_widget(input, chunks[0]);
    frame.set_cursor(
        chunks[0].x + app_state.input.width() as u16 + 1,
        chunks[0].y + 1,
    );

    let messages = {
        let mut msg_queue = app_state.messages.lock().unwrap();
        while msg_queue.len() > 80 {
            msg_queue.pop_front();
        }
        msg_queue.clone()
    };
    let messages: Vec<ListItem> = messages
        .iter()
        .rev()
        .map(|msg| {
            let content = vec![Spans::from(Span::raw(msg.to_string()))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    frame.render_widget(messages, chunks[1]);
}
