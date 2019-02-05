#![ feature( await_macro, async_await, futures_api ) ]


use std::process::exit;
use futures_util::{future::FutureExt, try_future::TryFutureExt};
use std::env::args;

use tokio_uds::UnixStream;
use tokio_async_await::await;

use libpeers::*;
use libpeers::msg::Write;

use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

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


		let ipc_client = IpcClient{ socket: Rc::new( RefCell::new( socket ) ) }.start();

		let write   = Write{ message: Vec::from( "PeerB says hi!".as_bytes() ) };
		let write2  = Write{ message: Vec::from( "PeerB says hi more!".as_bytes() ) };
		let write3  = Write{ message: Vec::from( "PeerB says hi even more!!!".as_bytes() ) };

		await!( ipc_client.send( write  ) ).expect( "PeerB: Write failed" );
		await!( ipc_client.send( write2 ) ).expect( "PeerB: Write failed" );
		await!( ipc_client.send( write3 ) ).expect( "PeerB: Write failed" );

		System::current().stop();
		println!( "PeerB: Shutting down" );

		Ok(())

	}.boxed().compat())});
}

