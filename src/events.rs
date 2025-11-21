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

    if app.command_mode {
        match key.code {
            KeyCode::Esc => {
                app.exit_command_mode();
            }
            KeyCode::Enter => {
                app.execute_command();
            }
            KeyCode::Backspace => {
                app.command_backspace();
            }
            KeyCode::Char(c) => {
                app.command_input_char(c);
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
        KeyCode::Tab => {
            app.next_tab();
        }
        KeyCode::BackTab => {
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
        KeyCode::Enter => {
            if app.current_tab == crate::app::Tab::Processes {
                app.toggle_category_expanded();
            }
        }
        KeyCode::Char('/') => {
            if app.current_tab == crate::app::Tab::Processes {
                app.enter_command_mode();
            }
        }
        KeyCode::Esc => {
            if app.current_tab == crate::app::Tab::Processes {
                if app.show_all_processes {
                    app.exit_command_mode();
                } else if app.category_expanded {
                    app.collapse_category();
                }
            }
        }
        KeyCode::Char('h') | KeyCode::Left => {
            if app.current_tab == crate::app::Tab::Processes {
                if !app.category_expanded && !app.show_all_processes {
                    app.move_category_left();
                }
            } else {
                app.previous_tab();
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if app.current_tab == crate::app::Tab::Processes {
                if !app.category_expanded && !app.show_all_processes {
                    app.move_category_right();
                }
            } else {
                app.next_tab();
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if app.current_tab == crate::app::Tab::Processes {
                if app.category_expanded || app.show_all_processes {
                    app.scroll_up();
                } else {
                    app.move_category_up();
                }
            } else {
                app.scroll_up();
            }
        }
        KeyCode::Char('j') | KeyCode::Down => {
            if app.current_tab == crate::app::Tab::Processes {
                if app.category_expanded || app.show_all_processes {
                    app.scroll_down();
                } else {
                    app.move_category_down();
                }
            } else {
                app.scroll_down();
            }
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
