// 1. Spawn peerB and later also peerC
// 2. Give peerB our socket address
// 3. listen for a connection
// 4. receive "request+number"
// 5. respond "response+number"


use tokio::prelude::*;
use tokio_uds::UnixListener;
use std::process::Command;
use std::str;


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


	let server = listener.incoming().for_each( |socket| -> tokio::io::Result<()>
	{
		println!( "PeerA: Start server future" );

		let (reader, _writer) = socket.split();


		let receive = tokio::io::read_to_end( reader, vec![] ).and_then( |(_readhalf, buf)|

			{
				tokio::spawn( process( str::from_utf8( &buf ).expect( "Invalid utf encoding received" ) ) );

				Ok(())
			})

			.map_err( |error|
			{
				eprintln!( "PeerA: Error while reading from socket: {:?}", error );
			});


		tokio::spawn( receive );

		Ok(())
	})

	.map_err( |err|
	{
		// Handle error by printing to STDOUT.
		//
		eprintln!( "PeerA: accept error = {:?}", err );
	});

	println!( "PeerA: server running on {}", SOCK_ADDR );


	tokio::run( server );
}


fn process( message: &str ) -> future::FutureResult<(),()>
{
	println!( "PeerA: process called" );

	println!("PeerA: received: {:?}", message );

	future::ok(())
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
