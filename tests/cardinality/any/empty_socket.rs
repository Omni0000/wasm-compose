use wasm_compose::{ Engine, Linker, PluginTree, InterfaceId, PluginId };

bind_fixtures!( "cardinality", "any", "empty_socket" );
use fixtures::{ InterfaceDir, PluginDir, FixtureError };

#[test]
fn cardinality_test_any_empty_socket() {

    let engine = Engine::default();
    let linker = Linker::new( &engine );

    let plugins = vec![
        PluginDir::new( PluginId::new( "startup".into() )).unwrap(),
    ];

    let ( tree, warnings ) = PluginTree::<InterfaceDir, _>::new::<FixtureError>( plugins, InterfaceId::new( 0 ));
    assert_no_warnings!( warnings );

    // Should succeed: Any allows 0 plugins
    let ( _tree, warnings ) = tree.load( &engine, &linker ).unwrap();
    assert_no_warnings!( warnings );

}
