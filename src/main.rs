mod nvidia_smi_interface;

use std::io;
use std::time::{Duration, Instant};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use nvidia_smi_interface::GpuInfo;
use tui::style::{Color, Modifier, Style};
use tui::text::Text;
use tui::widgets::{Block, BorderType, Borders, Gauge, Paragraph, Sparkline, Wrap};
use tui::{Frame, Terminal};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Direction, Layout};


struct TempState{
    data: Vec<u64>,
    peak : u64,
    elapsed : Instant,
}

fn show_ui<B: Backend>(f: &mut Frame<B>, info:  &GpuInfo, temp_state: &mut TempState){
    let frame_vbox = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
        ].as_ref())
        .split(f.size());

    let top_hbox = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ].as_ref())
        .split(frame_vbox[0]);

    let info_vbox = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ].as_ref())
        .split(top_hbox[0]);

    let info_background = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL);

    f.render_widget(info_background, top_hbox[0]);

    let gpu_name = text(&info.name,Color::Green,Color::Black,Alignment::Center);

    let driver_text : &str = &format!("Driver: {}", info.driver);
    let driver = text(driver_text, Color::White, Color::Black, Alignment::Left);

    let bios_text : &str = &format!("Bios: {}", info.bios);
    let bios = text(bios_text,Color::White, Color::Black, Alignment::Left);

    f.render_widget(gpu_name, info_vbox[0]);
    f.render_widget(driver, info_vbox[1]);
    f.render_widget(bios, info_vbox[2]);


    let gauge_vbox = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(40)
        ].as_ref())
        .split(top_hbox[1]);




    if let Some(data) = nvidia_smi_interface::get_clock_data(){
        let graphics_gauge = gauge("Graphics","MHz".to_string(),  data.graphics_current, data.graphics_max,Color::Green);
        let memory_gauge = gauge("Memory","MHz".to_string(), data.memory_current, data.memory_max,Color::Yellow);
        let fan_gauge = gauge("Fan Speed", "%".to_string(), data.fan_speed, 100, Color::Blue);


        f.render_widget(graphics_gauge, gauge_vbox[0]);
        f.render_widget(memory_gauge, gauge_vbox[1]);
        f.render_widget(fan_gauge, gauge_vbox[2]);

        let temp_block = Block::default()
            .style(Style::default().bg(Color::Black))
            .title("Temperature")
            .borders(Borders::ALL);

        let temp_hbox = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Percentage(80),
                Constraint::Percentage(20)
            ].as_ref())
            .split(gauge_vbox[3]);

        let current_temp = data.temperature as u64;
        if current_temp > temp_state.peak{
            temp_state.peak  = current_temp;
        }
        if temp_state.elapsed.elapsed() > Duration::from_millis(1500){

            temp_state.data.push(current_temp);

            while temp_state.data.len() as u16 > temp_hbox[0].width{
                temp_state.data.remove(0);
            }
            temp_state.elapsed = Instant::now();
        }

        let temp_status_vbox = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ].as_ref()).split(temp_hbox[1]);

        let temp_string = format!("Current: {}℃", current_temp);
        let ctemp = text(&temp_string,Color::White,Color::Black, Alignment::Center);
        let peak_string = format!("Peak:    {}℃", &temp_state.peak);
        let ptemp = text(&peak_string, Color::White, Color::Black, Alignment::Center);


        let degree_marker_vbox = Layout::default()
            .margin(0)
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(49),
                Constraint::Percentage(51),
            ].as_ref())
            .split(temp_hbox[0]);

        let max_temp = text("-100℃",Color::Red, Color::Black, Alignment::Left);
        let mid_temp = text("-50℃",Color::Red, Color::Black, Alignment::Left);

        let sparkline = sparkline(&temp_state.data);
        f.render_widget(temp_block, gauge_vbox[3]);
        f.render_widget(sparkline, temp_hbox[0]);
        f.render_widget(ctemp, temp_status_vbox[0]);
        f.render_widget(ptemp, temp_status_vbox[1]);

        f.render_widget(max_temp, degree_marker_vbox[0]);
        f.render_widget(mid_temp, degree_marker_vbox[1]);
        // f.render_widget(min_temp, degree_marker_vbox[2]);

    }

}

fn sparkline(data: &[u64])->Sparkline{
    Sparkline::default()
        .block(Block::default().borders(Borders::RIGHT))
        .data(data)
        .max(100)
        .style(Style::default().fg(Color::Red))
}

fn gauge(name : &str, suffix : String, current : i32, max : i32,color : Color)->Gauge{
    let percentage : f32 = (current as f32) / (max as f32);
    let label : String = format!("{} {}",current,suffix);
    Gauge::default()
        .block(Block::default().title(name).borders(Borders::BOTTOM | Borders::TOP | Borders::RIGHT).border_type(BorderType::Plain))
        .gauge_style(Style::default().fg(color).bg(Color::Black).add_modifier(Modifier::BOLD))
        .percent((percentage * 100.0) as u16)
        .label(label)

}

fn text(text: &str,fg: Color, bg: Color,align: Alignment)->Paragraph{
    Paragraph::new(Text::from(text))
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(fg).bg(bg))
        .alignment(align)
        .wrap(Wrap{trim:true})
}


fn main() -> Result<(),io::Error>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let info = nvidia_smi_interface::get_gpu_info().unwrap();
    let mut temp_state = TempState{
        data : vec![],
        peak : 0,
        elapsed : Instant::now()
    };

    loop{
        terminal.draw(|f| show_ui(f,&info,&mut temp_state))?;

        if crossterm::event::poll(Duration::from_millis(250))?{
            if let Event::Key(key) = event::read()?{
                if let KeyCode::Char('q') = key.code{
                    break;
                }
            }
        }

    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;


    Ok(())
}
