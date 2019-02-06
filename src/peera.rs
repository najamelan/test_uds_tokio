#![ feature( await_macro, async_await, futures_api ) ]



// 1. Spawn peerB and later also peerC
// 2. Give peerB our socket address
// 3. listen for a connection
// 4. receive "request+number"
// 5. respond "response+number"

use futures_util::{future::FutureExt, try_future::TryFutureExt};



use tokio_async_await::await;

use std::process::Command;
use std::str;

use libpeers::*;
use actix::prelude::*;

fn main()
{
	let sys = System::new( "peers" );

	let program = async
	{
		println!( "PeerA: Starting peer A" );

		const SOCK_ADDR: &str = "/home/user/peerAB.sock";

		Command::new( "target/debug/peerb" )

			.arg( "--server" )
			.arg( SOCK_ADDR  )
			.spawn()
			.expect( "PeerA: failed to execute process" )
		;



		let ipc_peer = await!( peer( SOCK_ADDR, Processor{}.start().recipient() ) );


		println!( "PeerA: Starting IpcPeer" );

		ipc_peer.start();

		Ok(())

	};

	Arbiter::spawn( program.boxed().compat() );

	sys.run();
}

