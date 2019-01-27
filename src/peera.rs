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


use tokio::prelude::*;
// use futures::prelude::*;
use tokio_uds::{ UnixListener, UnixStream };
use std::process::Command;
use std::str;
// use failure::Error;


fn main()
{
	const SOCK_ADDR: &str = "/home/user/peerAB.sock";

	Command::new( "target/debug/peerb" )

		.arg( "--server" )
		.arg( SOCK_ADDR  )
		.spawn()
		.expect( "PeerA: failed to execute process" )
	;


	std::fs::remove_file( SOCK_ADDR ).expect( "PeerA: Could remove socket file" );
	let listener = UnixListener::bind( &SOCK_ADDR ).expect( "PeerA: Could not bind to socket" );

	tokio::run_async
	(
		async
		{
			let mut incoming = listener.incoming();

			while let Some( stream ) = await!( incoming.next() )
			{
				println!("PeerA: New stream!");

				tokio::spawn_async( handle_stream( stream.expect( "PeerA: Invalid Stream" ) ) );
			}
		}
	);
}


// TODO: Bubble up errors
//
async fn handle_stream( mut stream: UnixStream )
{
	let mut out: Vec<u8> = Vec::new();
	let mut buf          = [0u8; 1024];


	let success: bool = loop
	{
		let read = await!( stream.read_async( &mut buf ) );

		if let Ok( amount ) = read
		{
			if amount == 0 { break true; }

			println!( "PeerA: Read {:?} bytes", amount );
			out.extend_from_slice( &buf[ 0..amount ] );

			continue;
		}

		else if let Err( err ) = read
		{
			eprintln!( "PeerA: Error during reading from stream: {:?}", err );

			break false;
		}
	};

	if success
	{
		tokio::spawn_async
		(
			process( str::from_utf8( &out ).expect( "PeerA: Got invalid utf from stream" ).to_string() )
		);
	}
}


async fn process( message: String )
{
	println!( "PeerA: process called" );

	println!( "PeerA: received: {:?}", &message );
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
