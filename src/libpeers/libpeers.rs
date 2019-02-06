#![feature(await_macro, async_await, futures_api, arbitrary_self_types, never_type)]

use tokio_async_await::stream::StreamExt;
use tokio_uds::{ UnixStream, UnixListener };
use failure::*;
use actix::prelude::*;

use std::str;


mod ipc_peer;
mod ipc_message;
mod processor;

pub mod msg;

pub use ipc_peer::IpcPeer;
pub use ipc_message::IpcMessage;
pub use processor::Processor;

pub type EkkeResult<T> = Result< T, Error >;


#[ derive( Debug, Fail ) ]
//
pub enum EkkeError
{
	#[ fail( display = "Cannot use socket before connecting" ) ]
	//
	UseSocketBeforeConnect,

	#[ fail( display = "Nobody connected to the socket" ) ]
	//
	NoConnectionsReceived
}







pub async fn peer( sock_addr: &str, processor: Recipient<IpcMessage> ) -> IpcPeer
{
	let connection = await!( bind( sock_addr ) ).expect( "failed to bind socket address");

	IpcPeer::new( connection, processor )
}


// We only want one program to connect, so we stop listening after the first stream comes in
//
async fn bind( sock_addr: &str ) -> Result< UnixStream, failure::Error >
{
	std::fs::remove_file( &sock_addr ).context( format!( "Cannot unlink socket address: {:?}", sock_addr ) )?;

	let listener = UnixListener::bind( sock_addr ).expect( "PeerA: Could not bind to socket" );
	let mut connection = listener.incoming();

	while let Some( income ) = await!( connection.next() )
	{
		match income
		{
			Ok ( stream ) => return Ok( stream ),
			Err( _ ) => { eprintln!( "PeerA: Got Invalid Stream" ); continue }
		};
	};

	Err( EkkeError::NoConnectionsReceived.into() )
}
