#![ feature( await_macro, async_await, futures_api ) ]


use std::process::exit;
use futures_util::{future::FutureExt, try_future::TryFutureExt};
use std::env::args;

use tokio_uds::UnixStream;
use tokio_async_await::await;

use libpeers::*;

use std::path::PathBuf;

use actix::prelude::*;

fn main()
{
	System::run( move ||	{ Arbiter::spawn( async move
	{
		println!( "PeerB: Starting" );

		// for argument in args()
		// {
		// 	println!( "PeerB: Argument passed on cli: {}", argument );
		// }

		let sock_addr = args().nth( 2 ).expect( "No arguments passed in." );

		println!( "PeerB: socket addres set to: {:?}", sock_addr );

		let socket = match await!( UnixStream::connect( PathBuf::from( &sock_addr ) ) )
		{
			Ok ( cx  ) => { cx },
			Err( err ) => { eprintln!( "{}", err ); exit( 1 ); }
		};

		let ipc_peer = IpcPeer::new( socket, Processor{}.start().recipient() ).start();


		let write   = IpcMessage{ inner: "PeerB says hi!\n".into() };
		let write2  = IpcMessage{ inner: "PeerB says hi more!\n".into() };
		let write3  = IpcMessage{ inner: "PeerB says \nhi even more!!!".into() };

		await!( ipc_peer.send( write  ) ).expect( "PeerB: Write failed" );
		await!( ipc_peer.send( write2 ) ).expect( "PeerB: Write failed" );
		await!( ipc_peer.send( write3 ) ).expect( "PeerB: Write failed" );

		System::current().stop();
		println!( "PeerB: Shutting down" );

		Ok(())

	}.boxed().compat())});
}

