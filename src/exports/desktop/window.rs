use wasmtime::component::Resource;

use super::DesktopContext;
use super::omni::desktop::controls::{ Host, HostWindow, HostUi };
use super::gui_commands::{ GuiCommand, ResponseSender, UiHandle, WindowHandle };

/// Stored UI resource - contains only a handle, not the actual UI object
/// The actual libui::UI lives on the GUI thread
pub struct StoredUi {
    handle: UiHandle,
}

impl StoredUi {
    pub fn new(handle: UiHandle) -> Self {
        Self { handle }
    }

    pub fn handle(&self) -> UiHandle {
        self.handle
    }
}

/// Stored Window resource - contains only a handle, not the actual Window object
/// The actual libui::Window lives on the GUI thread
pub struct StoredWindow {
    handle: WindowHandle,
}

impl StoredWindow {
    pub fn new(handle: WindowHandle) -> Self {
        Self { handle }
    }

    pub fn handle(&self) -> WindowHandle {
        self.handle
    }
}

impl Host for DesktopContext {}

impl HostUi for DesktopContext {
    fn new(&mut self) -> wasmtime::Result<Resource<StoredUi>> {
        // Create response channel
        let (response_sender, response_receiver) = ResponseSender::new();

        // Send command to GUI thread
        self.gui_sender
            .send(GuiCommand::CreateUi { response: response_sender })
            .map_err(|e| wasmtime::Error::msg(format!("Failed to send GUI command: {}", e)))?;

        // Wait for response from GUI thread
        let ui_handle = response_receiver
            .recv()
            .map_err(|e| wasmtime::Error::msg(format!("Failed to receive GUI response: {}", e)))?
            .map_err(|e| wasmtime::Error::msg(format!("GUI thread error: {}", e)))?;

        // Store the handle in the resource table
        let stored_ui = StoredUi::new(ui_handle);
        Ok(self.resource_table.push(stored_ui)?)
    }

    fn drop(&mut self, handle: Resource<StoredUi>) -> wasmtime::Result<()> {
        // Get the UI handle from the resource table
        let stored_ui = self.resource_table.delete(handle)?;

        // Send destroy command to GUI thread (fire and forget, no response needed)
        self.gui_sender
            .send(GuiCommand::DestroyUi {
                handle: stored_ui.handle(),
            })
            .map_err(|e| wasmtime::Error::msg(format!("Failed to send GUI command: {}", e)))?;

        Ok(())
    }
}

impl HostWindow for DesktopContext {
    fn new(&mut self, ui: Resource<super::omni::desktop::controls::Ui>) -> wasmtime::Result<Resource<StoredWindow>> {
        // Get the UI handle from the resource table
        let ui_resource = self.resource_table.get(&ui)?;
        let ui_handle = ui_resource.handle();

        // Create response channel
        let (response_sender, response_receiver) = ResponseSender::new();

        // Send command to GUI thread to create a window
        self.gui_sender
            .send(GuiCommand::CreateWindow {
                ui_handle,
                title: "".to_string(),
                width: 800,
                height: 600,
                has_menubar: false,
                response: response_sender,
            })
            .map_err(|e| wasmtime::Error::msg(format!("Failed to send GUI command: {}", e)))?;

        // Wait for response from GUI thread
        let window_handle = response_receiver
            .recv()
            .map_err(|e| wasmtime::Error::msg(format!("Failed to receive GUI response: {}", e)))?
            .map_err(|e| wasmtime::Error::msg(format!("GUI thread error: {}", e)))?;

        // Store the handle in the resource table
        let stored_window = StoredWindow::new(window_handle);
        Ok(self.resource_table.push(stored_window)?)
    }

    fn drop(&mut self, handle: Resource<StoredWindow>) -> wasmtime::Result<()> {
        // Get the window handle from the resource table
        let stored_window = self.resource_table.delete(handle)?;

        // Send destroy command to GUI thread (fire and forget)
        self.gui_sender
            .send(GuiCommand::DestroyWindow {
                handle: stored_window.handle(),
            })
            .map_err(|e| wasmtime::Error::msg(format!("Failed to send GUI command: {}", e)))?;

        Ok(())
    }
}
