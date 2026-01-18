use wasmtime::component::ResourceTable;
use std::sync::mpsc::Sender;

mod gui_commands;
mod gui_thread;
mod window;

pub use gui_commands::{ GuiCommand, create_gui_channel };
pub use gui_thread::gui_thread_main;

wasmtime::component::bindgen!({
    path: "wit/exports/desktop/root.wit",
    world: "desktop",
    imports: { default: trappable },
    with: {
        "omni:desktop/controls.ui": window::StoredUi,
        "omni:desktop/controls.window": window::StoredWindow,
    },
});

pub(super) struct DesktopContext {
    resource_table: ResourceTable,
    gui_sender: Sender<GuiCommand>,
}

impl DesktopContext {
    pub fn new(gui_sender: Sender<GuiCommand>) -> Self {
        Self {
            resource_table: ResourceTable::new(),
            gui_sender,
        }
    }
}
