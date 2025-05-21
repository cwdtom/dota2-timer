extern crate native_windows_gui as nwg;
mod config;
mod notice;

use nwg::WindowFlags;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use winapi::shared::windef;
use winapi::um::winuser;

// app name
const APP_NAME: &str = "Dota2 Timer";
// init time
const INIT_TIME: &str = "-01:30";
// start text
const START_TEXT: &str = "START";
// pause text
const PAUSE_TEXT: &str = "PAUSE";
// resume text
const RESUME_TEXT: &str = "RESUME";
// clear text
const CLEAR_TEXT: &str = "CLEAR";
// font name
const FONT: &str = "Segoe UI";
// icon resource name
const ICON_NAME: &str = "MAINICON";
// config dir name
const CONFIG_DIR: &str = "config";
// negative mark
const NEGATIVE: &str = "-";
// split mark
const SPLIT: &str = ":";

fn main() {
    main_window();
}

/// main window execute
fn main_window() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family(FONT).expect("Failed to set default font");

    let nodes = Rc::new(RefCell::new(vec![]));
    let mut window = Default::default();
    let mut timer_text = Default::default();
    let mut start_button = Default::default();
    let mut clear_button = Default::default();
    let mut timer = Default::default();
    let mut button_font = Default::default();
    let mut timer_font = Default::default();
    let mut icon = Default::default();
    let mut embed = Default::default();
    let mut combo_box = Default::default();
    let layout = Default::default();

    nwg::EmbedResource::builder().build(&mut embed).unwrap();

    nwg::Icon::builder()
        .source_embed(Some(&embed))
        .source_embed_str(Some(ICON_NAME))
        .strict(true)
        .build(&mut icon)
        .unwrap();

    // main window
    nwg::Window::builder()
        .size((150, 115))
        .position((300, 300))
        .title(APP_NAME)
        .topmost(true)
        .flags(WindowFlags::WINDOW | WindowFlags::VISIBLE)
        .ex_flags(winuser::WS_EX_LAYERED)
        .icon(Some(&icon))
        .build(&mut window)
        .unwrap();

    nwg::Font::builder()
        .size(26)
        .family(FONT)
        .weight(800)
        .build(&mut timer_font)
        .unwrap();

    nwg::TextInput::builder()
        .text(INIT_TIME)
        .align(nwg::HTextAlign::Center)
        .font(Some(&timer_font))
        .parent(&window)
        .build(&mut timer_text)
        .unwrap();

    nwg::Font::builder()
        .size(18)
        .family(FONT)
        .weight(0)
        .build(&mut button_font)
        .unwrap();

    // combo box
    nwg::ComboBox::builder()
        .parent(&window)
        .font(Some(&button_font))
        .collection(config::get_text_list(CONFIG_DIR))
        .selected_index(Some(0))
        .build(&mut combo_box)
        .unwrap();

    nwg::Button::builder()
        .text(START_TEXT)
        .parent(&window)
        .font(Some(&button_font))
        .build(&mut start_button)
        .unwrap();

    nwg::Button::builder()
        .text(CLEAR_TEXT)
        .parent(&window)
        .font(Some(&button_font))
        .build(&mut clear_button)
        .unwrap();
    // init not visible
    clear_button.set_visible(false);

    // main timer
    nwg::AnimationTimer::builder()
        .parent(&window)
        .interval(Duration::from_secs(1))
        .build(&mut timer)
        .unwrap();

    // set transparency
    unsafe {
        // Set 60% transparency (153 out of 255)
        let w_hwnd = window.handle.hwnd().unwrap() as windef::HWND;
        winuser::SetLayeredWindowAttributes(w_hwnd, 0, 153, winuser::LWA_ALPHA);
    }

    // element layout
    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child(0, 0, &timer_text)
        .child_item(nwg::GridLayoutItem::new(&combo_box, 0, 1, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&start_button, 0, 2, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&clear_button, 0, 2, 1, 1))
        .build(&layout)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    // handler event
    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => {
                if &handle == &events_window as &nwg::Window {
                    nwg::stop_thread_dispatch();
                }
            }
            E::OnButtonClick => {
                if &handle == &start_button {
                    click_start_button(&start_button, &timer, &timer_text, &clear_button);
                    // get selected config
                    let selected = combo_box.selection_string();
                    let config = config::get_notice_config_list(selected);
                    let mut nodes_ref = nodes.borrow_mut();
                    *nodes_ref = notice::gen_notice_node(config);
                }
                if &handle == &clear_button {
                    click_clear_button(&timer, &timer_text, &start_button, &clear_button);
                }
            }
            E::OnMousePress(mouse_event) => {
                // click combo box change selection
                if &handle == &combo_box && mouse_event == nwg::MousePressEvent::MousePressLeftDown
                {
                    let list = combo_box.collection();
                    let index = combo_box.selection().unwrap_or(0) + 1;
                    if index > list.len() - 1 {
                        combo_box.set_selection(Some(0));
                    } else {
                        combo_box.set_selection(Some(index));
                    }
                }
            }
            E::OnTimerTick => {
                if &handle == &timer {
                    let mut timestamp = to_timestamp(timer_text.text());

                    timestamp += 1;
                    timer_text.set_text(format(timestamp).as_str());

                    // every 2 seconds, hide button
                    if timestamp % 2 == 0 {
                        start_button.set_visible(false);
                        combo_box.set_visible(false);
                    }

                    // control visibility
                    control_nodes_visibility(&nodes.borrow(), timestamp);
                }
            }
            E::OnMouseMove => {
                // show button when mouse into window
                if &handle == &events_window as &nwg::Window {
                    start_button.set_visible(true);
                    combo_box.set_visible(true);
                }
            }
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}

/// click start button
fn click_start_button(
    start_button: &nwg::Button,
    timer: &nwg::AnimationTimer,
    timer_text: &nwg::TextInput,
    clear_button: &nwg::Button,
) {
    if start_button.text() == PAUSE_TEXT {
        timer.stop();
        timer_text.set_readonly(false);
        start_button.set_text(RESUME_TEXT);

        let (s_x, _) = timer_text.size();
        let (_, s_y) = start_button.size();
        start_button.set_size(s_x / 2 - 5, s_y);
        clear_button.set_size(s_x / 2 - 5, s_y);
        let (_, p_y) = clear_button.position();
        clear_button.set_position((s_x / 2 + 10) as i32, p_y);
        clear_button.set_visible(true);
    } else {
        timer.start();
        timer_text.set_readonly(true);
        start_button.set_text(PAUSE_TEXT);

        let (x, _) = timer_text.size();
        let (_, y) = start_button.size();
        start_button.set_size(x, y);
        clear_button.set_visible(false);
    }
}

/// click clear button
fn click_clear_button(
    timer: &nwg::AnimationTimer,
    timer_text: &nwg::TextInput,
    start_button: &nwg::Button,
    clear_button: &nwg::Button,
) {
    // stop timer
    timer.stop();
    // reset text
    timer_text.set_text(INIT_TIME);
    // set text input readonly
    timer_text.set_readonly(false);
    // set button text
    start_button.set_text(START_TEXT);

    let (x, _) = timer_text.size();
    let (_, y) = start_button.size();
    start_button.set_size(x, y);
    clear_button.set_visible(false);
}

/// control visibility
fn control_nodes_visibility(nodes: &Vec<notice::NoticeNode>, timestamp: i32) {
    let mut visible_count = 0;

    // todo delete
    println!("==================");

    for node in nodes {
        // just show 2 visible nodes
        if timestamp <= node.timestamp && visible_count < 2 && node.visible {
            visible_count += 1;
            // todo show text label
            println!("{}  {}", format(timestamp - node.timestamp), node.text);
        }

        // play notice sound
        if timestamp == node.timestamp && !node.visible {
            unsafe {
                winapi::um::winuser::MessageBeep(0x00000010);
            }
        }
    }
}

/// format timestamp
fn format(timestamp: i32) -> String {
    if timestamp < 0 {
        format!(
            "{}{:02}{}{:02}",
            NEGATIVE,
            (timestamp / 60).abs(),
            SPLIT,
            (timestamp % 60).abs()
        )
    } else {
        format!("{:02}{}{:02}", timestamp / 60, SPLIT, timestamp % 60)
    }
}

/// to timestamp
fn to_timestamp(time_text: String) -> i32 {
    let parts: Vec<i32> = time_text
        .split(SPLIT)
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if time_text.starts_with(NEGATIVE) {
        parts[0] * 60 - parts[1]
    } else {
        parts[0] * 60 + parts[1]
    }
}
