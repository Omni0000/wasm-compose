use std::sync::mpsc::{ Sender, Receiver, channel };
use std::sync::{ Arc, Mutex };

/// Unique identifier for UI objects
pub type UiHandle = u32;

/// Unique identifier for Window objects
pub type WindowHandle = u32;

/// Commands that can be sent from WASM threads to the GUI thread
#[derive( Debug )]
pub enum GuiCommand {
    /// Create a new UI instance
    CreateUi {
        response: ResponseSender<Result<UiHandle, String>>,
    },
    /// Destroy a UI instance
    DestroyUi {
        handle: UiHandle,
    },
    /// Create a new window
    CreateWindow {
        ui_handle: UiHandle,
        title: String,
        width: i32,
        height: i32,
        has_menubar: bool,
        response: ResponseSender<Result<WindowHandle, String>>,
    },
    /// Show a window
    ShowWindow {
        handle: WindowHandle,
    },
    /// Destroy a window
    DestroyWindow {
        handle: WindowHandle,
    },
    /// Shutdown the GUI thread
    Shutdown,
}

/// A thread-safe sender for responses from the GUI thread back to WASM threads
#[derive( Debug, Clone )]
pub struct ResponseSender<T> {
    sender: Arc<Mutex<Option<T>>>,
}

impl<T> ResponseSender<T> {
    pub fn new() -> (Self, ResponseReceiver<T>) {
        let shared = Arc::new(Mutex::new(None));
        let sender = ResponseSender {
            sender: shared.clone(),
        };
        let receiver = ResponseReceiver { receiver: shared };
        (sender, receiver)
    }

    pub fn send(self, value: T) {
        if let Ok(mut guard) = self.sender.lock() {
            *guard = Some(value);
        }
    }
}

/// A receiver for responses from the GUI thread
pub struct ResponseReceiver<T> {
    receiver: Arc<Mutex<Option<T>>>,
}

impl<T> ResponseReceiver<T> {
    /// Wait for a response from the GUI thread
    pub fn recv(self) -> Result<T, String> {
        // Poll until we get a response
        // In a real implementation, we'd want to use a condvar or similar
        loop {
            if let Ok(mut guard) = self.receiver.lock() {
                if let Some(value) = guard.take() {
                    return Ok(value);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}

/// Create a command channel for GUI communication
pub fn create_gui_channel() -> (Sender<GuiCommand>, Receiver<GuiCommand>) {
    channel()
}
