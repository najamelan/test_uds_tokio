// Opens a unix domain socket and sends any incoming messages to the processor sent in via the Bind message
//
use crate::msg::Buffer;
use crate::msg::IpcBind;
use crate::EkkeResult;
use tokio_uds::UnixStream;
use tokio_uds::UnixListener;
use futures::future::Future;
use tokio_async_await::{ io::AsyncReadExt, stream::StreamExt };

use ::futures_util::{future::FutureExt, try_future::TryFutureExt};

use actix::prelude::*;

pub struct IpcServer {}


impl Actor for IpcServer { type Context = Context<Self>; }


impl IpcServer
{
	// TODO: Bubble up errors
	//
	async fn handle_stream( mut stream: UnixStream, processor: Recipient<Buffer> )
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
			let f = processor.send( Buffer{ inner: out } );

			Arbiter::spawn
			(
				f.map(|_|()).map_err(|_|())
			);
		}
	}
}



impl Handler<IpcBind> for IpcServer
{
	type Result = EkkeResult<()>;

	fn handle( &mut self, msg: IpcBind, _ctx: &mut Context<Self> ) -> Self::Result
	{
		std::fs::remove_file( &msg.socket )?;

		let listener = UnixListener::bind( &msg.socket ).expect( "PeerA: Could not bind to socket" );


		Arbiter::spawn( async move
		{
			let mut incoming = listener.incoming();

			while let Some( stream ) = await!( incoming.next() )
			{
				println!( "PeerA: New stream!" );

				tokio::spawn_async( Self::handle_stream( stream.expect( "PeerA: Invalid Stream" ), msg.clone().processor ) );
			}

			Ok(())

		}.boxed().compat());

		Ok(())
	}
}


