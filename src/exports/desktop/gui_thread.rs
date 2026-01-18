use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use libui::prelude::{ UI, Window, WindowType };

use super::gui_commands::{ GuiCommand, UiHandle, WindowHandle };

/// State for the GUI thread that owns all actual UI objects
pub struct GuiThreadState {
    next_ui_handle: UiHandle,
    next_window_handle: WindowHandle,
    ui_registry: HashMap<UiHandle, UI>,
    window_registry: HashMap<WindowHandle, Window>,
}

impl GuiThreadState {
    fn new() -> Self {
        Self {
            next_ui_handle: 0,
            next_window_handle: 0,
            ui_registry: HashMap::new(),
            window_registry: HashMap::new(),
        }
    }

    fn allocate_ui_handle(&mut self) -> UiHandle {
        let handle = self.next_ui_handle;
        self.next_ui_handle += 1;
        handle
    }

    fn allocate_window_handle(&mut self) -> WindowHandle {
        let handle = self.next_window_handle;
        self.next_window_handle += 1;
        handle
    }

    fn create_ui(&mut self) -> Result<UiHandle, String> {
        let ui = UI::init().map_err(|e| format!("Failed to initialize UI: {:?}", e))?;
        let handle = self.allocate_ui_handle();
        self.ui_registry.insert(handle, ui);
        Ok(handle)
    }

    fn destroy_ui(&mut self, handle: UiHandle) {
        self.ui_registry.remove(&handle);
    }

    fn create_window(
        &mut self,
        ui_handle: UiHandle,
        title: String,
        width: i32,
        height: i32,
        has_menubar: bool,
    ) -> Result<WindowHandle, String> {
        let ui = self.ui_registry.get(&ui_handle)
            .ok_or_else(|| format!("UI handle {} not found", ui_handle))?;

        let window_type = if has_menubar {
            WindowType::HasMenubar
        } else {
            WindowType::NoMenubar
        };

        let window = Window::new(ui, &title, width, height, window_type);
        let handle = self.allocate_window_handle();
        self.window_registry.insert(handle, window);
        Ok(handle)
    }

    fn show_window(&mut self, handle: WindowHandle) -> Result<(), String> {
        let window = self.window_registry.get_mut(&handle)
            .ok_or_else(|| format!("Window handle {} not found", handle))?;
        window.show();
        Ok(())
    }

    fn destroy_window(&mut self, handle: WindowHandle) {
        self.window_registry.remove(&handle);
    }

    fn process_command(&mut self, command: GuiCommand) {
        match command {
            GuiCommand::CreateUi { response } => {
                let result = self.create_ui();
                response.send(result);
            }
            GuiCommand::DestroyUi { handle } => {
                self.destroy_ui(handle);
            }
            GuiCommand::CreateWindow {
                ui_handle,
                title,
                width,
                height,
                has_menubar,
                response,
            } => {
                let result = self.create_window(ui_handle, title, width, height, has_menubar);
                response.send(result);
            }
            GuiCommand::ShowWindow { handle } => {
                if let Err(e) = self.show_window(handle) {
                    eprintln!("Error showing window: {}", e);
                }
            }
            GuiCommand::DestroyWindow { handle } => {
                self.destroy_window(handle);
            }
            GuiCommand::Shutdown => {
                // Handled by the event loop
            }
        }
    }
}

/// Main GUI thread event loop
/// This function runs on the GUI thread and processes commands from WASM threads
pub fn gui_thread_main(receiver: Receiver<GuiCommand>) {
    let mut state = GuiThreadState::new();

    loop {
        // Process all pending commands
        while let Ok(command) = receiver.try_recv() {
            match command {
                GuiCommand::Shutdown => {
                    // Clean shutdown
                    return;
                }
                cmd => state.process_command(cmd),
            }
        }

        // TODO: Run the libui event loop here
        // For now, just sleep to avoid busy-waiting
        // In a real implementation, this would integrate with libui's event loop
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }
}
