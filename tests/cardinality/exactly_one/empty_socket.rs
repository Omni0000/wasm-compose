use wasm_compose::{ Engine, Linker, PluginTree, InterfaceId, PluginId, LoadError, InterfaceCardinality };

bind_fixtures!( "cardinality", "exactly_one", "empty_socket" );
use fixtures::{ InterfaceDir, PluginDir, FixtureError };

#[test]
fn cardinality_test_exactly_one_empty_socket() {

    let engine = Engine::default();
    let linker = Linker::new( &engine );

    let plugins = vec![
        PluginDir::new( PluginId::new( "startup".into() )).unwrap(),
    ];

    let ( tree, warnings ) = PluginTree::<InterfaceDir, _>::new::<FixtureError>( plugins, InterfaceId::new( 0 ));
    assert_no_warnings!( warnings );

    // Should fail: ExactlyOne requires exactly 1 plugin
    match tree.load( &engine, &linker ) {
        Err(( LoadError::FailedCardinalityRequirements( InterfaceCardinality::ExactlyOne, 0 ), _ )) => {},
        Err(( err, _ )) => panic!( "Expected FailedCardinalityRequirements(ExactlyOne, 0), got: {}", err ),
        Ok( _ ) => panic!( "Expected failure" ),
    };

}
