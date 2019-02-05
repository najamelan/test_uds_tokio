// https://jsdw.me/posts/rust-asyncawait-preview/


// enable the await! macro, async support, and the new std::Futures api.
//
#![feature(await_macro, async_await, futures_api)]

// only needed to manually implement a std future:
//
#![feature(arbitrary_self_types)]


// 1. Spawn peerB and later also peerC
// 2. Give peerB our socket address
// 3. listen for a connection
// 4. receive "request+number"
// 5. respond "response+number"


// use tokio::prelude::*;
// use futures::prelude::*;
// use tokio_uds::{ UnixListener, UnixStream };
use futures::future::Future;
use std::process::Command;
use std::str;
// use failure::Error;
use libpeers::*;
use std::path::PathBuf;
use actix::prelude::*;

fn main()
{
	let sys = System::new( "peers" );

	const SOCK_ADDR: &str = "/home/user/peerAB.sock";

	Command::new( "target/debug/peerb" )

		.arg( "--server" )
		.arg( SOCK_ADDR  )
		.spawn()
		.expect( "PeerA: failed to execute process" )
	;

	let server    = IpcServer::new().start();
	let processor = Processor{}.start();

	let bind = server.send( IpcBind{ socket: SOCK_ADDR.to_string(), processor } );

	Arbiter::spawn( bind.map(|_|()).map_err(|_|()) );

	sys.run();
}






// struct Processor
// {

// }


// impl Processor
// {

// }


// struct DataBase
// {

// }


// impl DataBase
// {

// }
