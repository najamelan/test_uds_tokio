
// enable the await! macro, async support, and the new std::Futures api.
//
#![feature(await_macro, async_await, futures_api)]

// only needed to manually implement a std future:
//
#![feature(arbitrary_self_types)]

use std::env::args;

use tokio::prelude::*;
use tokio::io;
use tokio_uds::UnixStream;
use tokio_async_await::compat::forward::IntoAwaitable;

fn main()
{
	println!( "PeerB: Starting" );

	// for argument in args()
	// {
	// 	println!( "PeerB: Argument passed on cli: {}", argument );
	// }

	let sock_addr = args().nth( 2 ).expect( "No arguments passed in." );

	println!( "PeerB: socket addres set to: {:?}", sock_addr );

	let writing = async
	{
		let mut socket = await! ( UnixStream::connect( sock_addr ).into_awaitable() ).expect( "PeerB: Failed to connect" );

		println!( "PeerB: start writing future" );

		match await!( socket.write_async( "PeerB says hi.".as_bytes() ) )
		// match await!( io::write_all( socket, "PeerB says hi." ).into_awaitable() )
		{
			Ok (_) => { println! ( "PeerB: successfully wrote to stream"       ); },
			Err(e) => { eprintln!( "PeerB: failed to write to stream: {:?}", e ); }
		}

		std::thread::sleep_ms( 5000 );

		match await!( socket.write_async( "PeerB says hi.".as_bytes() ) )
		// match await!( io::write_all( socket, "PeerB says hi." ).into_awaitable() )
		{
			Ok (_) => { println! ( "PeerB: successfully wrote to stream"       ); },
			Err(e) => { eprintln!( "PeerB: failed to write to stream: {:?}", e ); }
		}
	};

	tokio::run_async( writing );

	println!( "PeerB: Shutting down" );
}

