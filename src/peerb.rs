use std::env::args;

use tokio::prelude::*;
use tokio::io;
use tokio_uds::UnixStream;

fn main()
{
	println!( "PeerB: Starting" );

	// for argument in args()
	// {
	// 	println!( "PeerB: Argument passed on cli: {}", argument );
	// }

	let sock_addr = args().nth( 2 ).expect( "No arguments passed in." );

	println!( "PeerB: socket addres set to: {:?}", sock_addr );

	let connection = UnixStream::connect( &sock_addr );


	let writing = connection.and_then( |socket|
	{
		println!( "PeerB: start writing future" );


		io::write_all( socket, "PeerB says hi.")

			.then( |result|
			{
				match result
				{
					Ok(_) => { println!( "PeerB: successfully wrote to stream" ); },
					Err(e) => { eprintln!("PeerB: failed to write to stream: {:?}", e ); }
				}

				Ok(())
			})
	})
		.map    ( |_| () )

		.map_err( |e| { eprintln!( "PeerB: Writing to stream failed: {:?}", e ); } );


	tokio::run( writing );
}
