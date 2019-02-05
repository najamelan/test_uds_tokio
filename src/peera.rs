#![ feature( await_macro, async_await, futures_api ) ]



// 1. Spawn peerB and later also peerC
// 2. Give peerB our socket address
// 3. listen for a connection
// 4. receive "request+number"
// 5. respond "response+number"



use libpeers::msg::IpcBind;
use futures::future::Future;
use std::process::Command;
use std::str;

use libpeers::*;
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

	let server    = IpcServer{}.start();
	let processor = Processor{}.start().recipient();

	let bind = server.send( IpcBind{ socket: SOCK_ADDR.to_string(), processor } );

	Arbiter::spawn( bind.map(|_|()).map_err(|_|()) );

	sys.run();
}

