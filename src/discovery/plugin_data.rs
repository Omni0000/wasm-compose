use wasmtime::Engine ;
use wasmtime::component::Component ;

use crate::{ InterfaceId, PluginId };



/// Trait for accessing plugin metadata from a user-defined source (filesystem, database, etc.).
pub trait PluginData: Sized {

    type Error: std::error::Error ;
    type SocketIter<'a>: IntoIterator<Item = &'a InterfaceId> where Self: 'a ;

    /// Returns this plugin's unique identifier.
    ///
    /// # Errors
    /// Implementations may fail if the underlying data source is unavailable
    /// (e.g., IO errors, parse errors, missing manifest fields).
    fn get_id( &self ) -> Result<&PluginId, Self::Error> ;

    /// Returns the interface ID that this plugin implements (its "plug").
    ///
    /// # Errors
    /// Implementations may fail if the underlying data source is unavailable
    /// (e.g., IO errors, parse errors, missing manifest fields).
    fn get_plug( &self ) -> Result<&InterfaceId, Self::Error> ;

    /// Returns the interface IDs that this plugin depends on (its "sockets").
    ///
    /// # Errors
    /// Implementations may fail if the underlying data source is unavailable
    /// (e.g., IO errors, parse errors, missing manifest fields).
    fn get_sockets( &self ) -> Result<Self::SocketIter<'_>, Self::Error> ;

    /// Compiles this plugin's WASM binary into a wasmtime Component.
    ///
    /// # Errors
    /// Implementations may fail due to IO errors when reading the WASM file,
    /// or wasmtime compilation errors if the binary is invalid.
    fn component( &self, engine: &Engine ) -> Result<Component, Self::Error> ;

}
