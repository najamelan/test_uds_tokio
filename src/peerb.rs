
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
use libpeers::*;
use std::path::PathBuf;
use std::sync::Arc;
use actix::prelude::*;

fn main()
{
	println!( "PeerB: Starting" );

	// for argument in args()
	// {
	// 	println!( "PeerB: Argument passed on cli: {}", argument );
	// }

	let sock_addr = args().nth( 2 ).expect( "No arguments passed in." );

	println!( "PeerB: socket addres set to: {:?}", sock_addr );

	let ipc_client = IpcClient{ connection: None }.start();
	let connect = Connect { address: PathBuf::from( &sock_addr ) };

	let connection = ipc_client.send( connect );

	let writer     = Write{ }

	let write =  async move
	{
		if let Err( err ) = await!( connection.into_awaitable() ) { eprintln!( "{}", err ) }
		if let Err( err ) = await!( channel.write( Vec::from( "ha".as_bytes() ) ) ) { eprintln!( "{}", err ) }
	};

	tokio::run_async( write );

	println!( "PeerB: Shutting down" );
}

