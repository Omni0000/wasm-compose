use std::collections::HashMap ;
use std::sync::{ RwLock, RwLockReadGuard, PoisonError };
use wasmtime::component::Val ;

use crate::PluginId ;
use super::PluginData ;
use super::PluginInstance ;



/// Container for plugin instances matching an interface's cardinality.
///
/// The variant used reflects the interface's cardinality constraint:
/// - `AtMostOne` - Zero or one plugin allowed
/// - `ExactlyOne` - Exactly one plugin required
/// - `AtLeastOne` - One or more plugins required
/// - `Any` - Zero or more plugins allowed
#[derive( Debug )]
pub enum Socket<T> {
    AtMostOne( Option<T> ),
    ExactlyOne( T ),
    AtLeastOne( HashMap<PluginId, T> ),
    Any( HashMap<PluginId, T> ),
}

impl<T> Socket<T> {
    /// Transforms each plugin value by reference, preserving cardinality.
    pub fn map<N>( &self, mut map: impl FnMut( &T ) -> N ) -> Socket<N> {
        match self {
            Self::AtMostOne( Option::None ) => Socket::AtMostOne( Option::None ),
            Self::AtMostOne( Some( t )) => Socket::AtMostOne( Some( map( t ))),
            Self::ExactlyOne( t ) => Socket::ExactlyOne( map( t )),
            Self::AtLeastOne( vec ) => Socket::AtLeastOne( vec.iter().map(|( id, item ): ( &PluginId, _ )| ( id.clone(), map( item ) )).collect() ),
            Self::Any( vec ) => Socket::Any( vec.iter().map(|( id, item ): ( &PluginId, _ )| ( id.clone(), map( item ) )).collect() ),
        }
    }
    /// Transforms each plugin value by ownership, preserving cardinality.
    pub fn map_mut<N>( self, mut map: impl FnMut(T) -> N ) -> Socket<N> {
        match self {
            Self::AtMostOne( Option::None ) => Socket::AtMostOne( Option::None ),
            Self::AtMostOne( Some( t )) => Socket::AtMostOne( Some( map( t ))),
            Self::ExactlyOne( t ) => Socket::ExactlyOne( map( t )),
            Self::AtLeastOne( vec ) => Socket::AtLeastOne( vec.into_iter().map(|( id, item )| ( id, map( item ) )).collect() ),
            Self::Any( vec ) => Socket::Any( vec.into_iter().map(|( id, item )| ( id, map( item ))).collect() ),
        }
    }
}
impl<T: PluginData> Socket<RwLock<PluginInstance<T>>> {
    /// Looks up a plugin instance by ID within this socket.
    ///
    /// Returns `None` if no plugin with the given ID exists in this socket.
    ///
    /// # Errors
    /// Returns `PoisonError` if a plugin's `RwLock` was poisoned by a panic in another thread.
    #[allow( clippy::type_complexity )]
    pub(crate) fn get( &self, id: &PluginId ) -> Result<Option<&RwLock<PluginInstance<T>>>,PoisonError<RwLockReadGuard<'_, PluginInstance<T>>>> {
        Ok( match self {
            Self::AtMostOne( Option::None ) => None,
            Self::AtMostOne( Some( plugin )) | Self::ExactlyOne( plugin ) => {
                if &plugin.read()?.id == id { Some( plugin ) } else { None }
            },
            Self::AtLeastOne( plugins ) | Self::Any( plugins ) => plugins.get( id ),
        })
    }
}

impl From<Socket<Val>> for Val {
    fn from( socket: Socket<Val> ) -> Self {
        match socket {
            Socket::AtMostOne( Option::None ) => Val::Option( Option::None ),
            Socket::AtMostOne( Some( val )) => Val::Option( Some( Box::new( val ))),
            Socket::ExactlyOne( val ) => val,
            Socket::AtLeastOne( items )
            | Socket::Any( items ) => Val::List( items.into_values().collect() ),
        }
    }
}
