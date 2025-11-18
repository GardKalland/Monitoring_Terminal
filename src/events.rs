use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub fn handle_key_event(key: KeyEvent, app: &mut crate::app::App) {
    if app.show_help {
        match key.code {
            KeyCode::Char('?') | KeyCode::Esc | KeyCode::Char('q') => {
                app.toggle_help();
            }
            _ => {}
        }
        return;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.quit();
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
        }
        KeyCode::Char('?') => {
            app.toggle_help();
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app.next_tab();
        }
        KeyCode::Char('h') | KeyCode::Left => {
            app.previous_tab();
        }
        KeyCode::Char('1') => {
            app.current_tab = crate::app::Tab::Overview;
        }
        KeyCode::Char('2') => {
            app.current_tab = crate::app::Tab::Processes;
        }
        KeyCode::Char('3') => {
            app.current_tab = crate::app::Tab::SystemInfo;
        }
        KeyCode::Char('4') => {
            app.current_tab = crate::app::Tab::Vpn;
        }
        KeyCode::Char('s') => {
            app.cycle_process_sort();
        }
        KeyCode::Char('o') => {
            app.toggle_sort_order();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.scroll_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.scroll_down();
        }
        _ => {}
    }
}

pub fn poll_events(timeout: Duration) -> anyhow::Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}
