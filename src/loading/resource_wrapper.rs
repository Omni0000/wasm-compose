use std::sync::Arc ;
use thiserror::Error ;
use wasmtime::component::{ Resource, ResourceAny, Val };
use wasmtime::StoreContextMut ;

use crate::plugin::{ PluginId, PluginData };
use super::PluginContext ;



pub(super) struct ResourceWrapper {
    pub plugin_id: PluginId,
    pub resource_handle: ResourceAny,
}

#[derive( Debug, Error )]
pub enum ResourceCreationError {
    #[error( "Resource Table Full" )] ResourceTableFull,
}
impl From<ResourceCreationError> for Val {
    fn from( error: ResourceCreationError ) -> Self { match error {
        ResourceCreationError::ResourceTableFull => Val::Variant( "resource-table-full".to_string(), None ),
    }}
}

#[derive( Debug, Error )]
pub enum ResourceReceiveError {
    #[error( "Invalid Handle" )] InvalidHandle,
}
impl From<ResourceReceiveError> for Val {
    fn from( error: ResourceReceiveError ) -> Self { match error {
        ResourceReceiveError::InvalidHandle => Val::Variant( "invalid-resource-handle".to_string(), None ),
    }}
}

impl ResourceWrapper {
    pub fn new( plugin_id: PluginId, resource_handle: ResourceAny ) -> Self {
        Self { plugin_id, resource_handle }
    }
    pub fn attach<P: PluginData>(
        self,
        store: &mut StoreContextMut<PluginContext<P>>,
    ) -> Result<ResourceAny, ResourceCreationError> {
        let resource = store.data_mut().add_resource( Arc::new( self )).map_err(|_| ResourceCreationError::ResourceTableFull )?;
        ResourceAny::try_from_resource( resource, store ).map_err(|_| unreachable!( "Resource already taken" ))
    }
    pub fn from_handle<P: PluginData>(
        handle: ResourceAny,
        store: &mut StoreContextMut<PluginContext<P>>,
    ) -> Result<Arc<Self>, ResourceReceiveError> {
        let resource = Resource::try_from_resource_any( handle, &mut *store ).map_err(|_| ResourceReceiveError::InvalidHandle )?;
        let wrapped = store.data().get_resource( &resource ).map_err(|_| ResourceReceiveError::InvalidHandle )?;
        Ok( Arc::clone( wrapped ))
    }
    pub fn drop<P: PluginData>( mut store: StoreContextMut<PluginContext<P>>, handle: u32 ) -> Result<(), wasmtime::Error> {
        let resource = Resource::<Arc<Self>>::new_own( handle );
        store.data_mut().delete_resource( resource ).map_err(|_| wasmtime::Error::new( ResourceReceiveError::InvalidHandle ))?;
        Ok(())
    }
}
