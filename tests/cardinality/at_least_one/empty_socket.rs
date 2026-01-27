use wasm_compose::{ Engine, Linker, PluginTree, InterfaceId, PluginId, LoadError, InterfaceCardinality };

bind_fixtures!( "cardinality", "at_least_one", "empty_socket" );
use fixtures::{ InterfaceDir, PluginDir, FixtureError };

#[test]
fn cardinality_test_at_least_one_empty_socket() {

    let engine = Engine::default();
    let linker = Linker::new( &engine );

    let plugins = vec![
        PluginDir::new( PluginId::new( "startup".into() )).unwrap(),
    ];

    let ( tree, warnings ) = PluginTree::<InterfaceDir, _>::new::<FixtureError>( plugins, InterfaceId::new( 0 ));
    assert_no_warnings!( warnings );

    // Should fail: AtLeastOne requires at least 1 plugin
    match tree.load( &engine, &linker ) {
        Err(( LoadError::FailedCardinalityRequirements( InterfaceCardinality::AtLeastOne, 0 ), _ )) => {},
        Err(( err, _ )) => panic!( "Expected FailedCardinalityRequirements(AtLeastOne, 0), got: {}", err ),
        Ok( _ ) => panic!( "Expected failure" ),
    };

}
