use wasmtime::component::{ Resource, ResourceTable, ResourceTableError };

pub struct PluginContext<P> {
    data: P,
    resource_table: ResourceTable,
}

impl<P> PluginContext<P> {
    pub(super) fn new( data: P ) -> Self {
        Self {
            data,
            resource_table: ResourceTable::new(),
        }
    }
    pub(super) fn add_resource<R: Send>( &mut self, resource: R ) -> Result<Resource<R>, ResourceTableError> {
        self.resource_table.push( resource )
    }
    pub(super) fn get_resource<R: Send>( &self, handle: &Resource<R> ) -> Result<&R, ResourceTableError> {
        self.resource_table.get( handle )
    }
    pub(super) fn delete_resource<R: Send>( &mut self, handle: Resource<R> ) -> Result<R, ResourceTableError> {
        self.resource_table.delete( handle )
    }
}

impl<P: std::fmt::Debug> std::fmt::Debug for PluginContext<P> {
    fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        f.debug_struct( "PluginContext" )
            .field( "data", &self.data )
            .finish_non_exhaustive()
    }
}

impl<P> std::ops::Deref for PluginContext<P> {
    type Target = P ;
    fn deref( &self ) -> &Self::Target { &self.data }
}

impl<P> std::ops::DerefMut for PluginContext<P> {
    fn deref_mut( &mut self ) -> &mut Self::Target { &mut self.data }
}
