use wasmtime::component::Linker;
use wasmtime::Engine;
use std::thread;

use crate::utils::PartialSuccess;
use super::PluginContext;

pub mod desktop;

use desktop::{ Desktop, DesktopContext, create_gui_channel, gui_thread_main };

pub struct ExportsContext {
    desktop_context: DesktopContext,
    #[allow(dead_code)]
    gui_thread_handle: Option<thread::JoinHandle<()>>,
}

impl ExportsContext {
    pub fn new() -> Self {
        // Create channel for GUI communication
        let (sender, receiver) = create_gui_channel();

        // Spawn the GUI thread
        let gui_thread_handle = thread::spawn(move || {
            gui_thread_main(receiver);
        });

        Self {
            desktop_context: DesktopContext::new(sender),
            gui_thread_handle: Some(gui_thread_handle),
        }
    }
}

impl Default for ExportsContext {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! add_exports {
    ( $engine:expr ; $( $( #[$attr:meta] )* $world:ident => $closure:expr ),* $(,)? ) => {
        {
            let mut linker = wasmtime::component::Linker::new( $engine );
            let mut errors = Vec::new();
            $(
                $( #[$attr] )* 
                if let Err(e) = $world::add_to_linker::<_, wasmtime::component::HasSelf<_>>( &mut linker, $closure ) {
                    errors.push(e);
                }
            )*
            ( linker, errors )
        }
    };
}

pub fn default_linker( engine: &Engine ) -> PartialSuccess<Linker<PluginContext>, wasmtime::Error> {

    add_exports!( engine ;
        #[cfg( feature = "desktop" )] Desktop => | state: &mut PluginContext | &mut state.exports_context.desktop_context,
    )

}
