use std::collections::HashMap ;
use std::sync::{ Arc, RwLock };
use wasmtime::Engine ;
use wasmtime::component::Linker ;

use crate::types::InterfaceId ;
use crate::discovery::{ InterfaceData, PluginData, discover_all };
use crate::loading::{ Socket, LoadError, load_plugin_tree, PluginInstance };
use crate::utils::{ PartialSuccess, PartialResult };



/// The root node of a loaded plugin tree.
///
/// Obtained from [`PluginTree::load`]. Provides access to dispatch functions
/// on the root socket's plugins.
pub struct PluginTreeHead<I: InterfaceData, P: PluginData + 'static> {
    /// Retained for future hot-loading support (adding/removing plugins at runtime).
    _interface: Arc<I>,
    pub(crate) socket: Arc<Socket<RwLock<PluginInstance<P>>>>,
}

/// An unloaded plugin dependency graph.
///
/// Built from a list of plugins by grouping them according to the interfaces
/// they implement (their "plug") and depend on (their "sockets").
///
/// # Type Parameters
/// - `I`: [`InterfaceData`] implementation for loading interface metadata
/// - `P`: [`PluginData`] implementation for loading plugin metadata
///
/// # Example
/// ```ignore
/// let plugins = vec![
///     MyPluginData::new( "auth-provider" ),
///     MyPluginData::new( "logger" ),
/// ];
/// let ( tree, discovery_errors ) = PluginTree::<MyInterfaceData, _>::new::<MyError>(
///     plugins,
///     InterfaceId::new( 0 ),
/// );
/// ```
pub struct PluginTree<I: InterfaceData, P: PluginData> {
    root_interface_id: InterfaceId,
    socket_map: HashMap<InterfaceId, ( I, Vec<P> )>,
}

impl<I: InterfaceData, P: PluginData> PluginTree<I, P> {

    /// Builds a plugin dependency graph from the given plugins.
    ///
    /// Plugins are grouped by the interface they implement. The `root_interface_id`
    /// specifies the entry point of the tree - the interface whose plugins will be
    /// directly accessible via [`PluginTreeHead::dispatch`] after loading.
    ///
    /// The error type `E` must be convertible from both `I::Error` and `P::Error`,
    /// allowing unified error handling across interface and plugin metadata access.
    pub fn new<E>(
        plugins: Vec<P>,
        root_interface_id: InterfaceId,
    ) -> PartialSuccess<Self, E>
    where
        E: From<I::Error> + From<P::Error>,
    {
        let ( socket_map, errors ) = discover_all::<I, P, E>( plugins, root_interface_id );
        ( Self { root_interface_id, socket_map }, errors )
    }

    /// Compiles and links all plugins in the tree, returning a loaded tree head.
    ///
    /// This recursively loads plugins starting from the root interface, compiling
    /// WASM components and linking their dependencies.
    ///
    /// # Errors
    /// Returns `LoadError` variants for:
    /// - Invalid or missing socket interfaces
    /// - Dependency cycles between plugins
    /// - Cardinality violations (too few/many plugins for an interface)
    /// - Corrupted interface or plugin manifests
    /// - WASM compilation or linking failures
    pub fn load(
        self,
        engine: &Engine,
        exports: &Linker<P>,
    ) -> PartialResult<PluginTreeHead<I, P>, LoadError<I, P>, LoadError<I, P>>
    where
        P: Send + Sync,
    {
        match load_plugin_tree( self.socket_map, engine, exports, self.root_interface_id ) {
            Ok((( interface, socket ), errors )) => Ok(( PluginTreeHead { _interface: interface, socket }, errors )),
            Err(( err, errors )) => Err(( err , errors )),
        }
    }

}
