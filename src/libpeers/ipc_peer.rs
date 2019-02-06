use crate::IpcMessage;
use std::rc::Rc;
use std::cell::RefCell;

use tokio::codec::Decoder;
use tokio_serde_cbor::{ Codec, SdMode };

use actix::prelude::*;
use tokio_uds::UnixStream;

use tokio::codec::Framed;
use tokio::prelude::stream::{SplitSink, SplitStream};
use tokio::prelude::*;
use futures_util::{future::FutureExt, try_future::TryFutureExt};


#[ derive( Debug ) ]
//
pub struct IpcPeer
{
	sink: Rc<RefCell< SplitSink<Framed<UnixStream, Codec<IpcMessage, IpcMessage>>> >>
}

impl Actor for IpcPeer { type Context = Context<Self>; }



impl IpcPeer
{
	pub fn new( connection: UnixStream, processor: Recipient<IpcMessage> ) -> Self
	{
		let codec: Codec<IpcMessage, IpcMessage>  = Codec::new().sd( SdMode::Always );

		let (sink, stream) = codec.framed( connection ).split();

		tokio::spawn_async( async move
		{
			await!( Self::listen( stream, processor ) );
		} );


		Self
		{
			sink: Rc::new( RefCell::new( sink ))
		}
	}


	/// Will listen to a connection and send all incoming messages to the processor.
	///
	#[ inline ]
	//
	async fn listen( mut stream: SplitStream<Framed<UnixStream, Codec<IpcMessage, IpcMessage>>>, processor: Recipient<IpcMessage> )
	{
		loop
		{
			let option: Option< Result< IpcMessage, _ > > = await!( stream.next() );

			let frame = match option
			{
				Some( result ) =>
				{
					match result
					{
						Ok ( frame ) => frame,
						Err( error ) =>
						{
							eprintln!( "Error extracting IpcMessage from stream" );
							eprintln!( "{:#?}", error );
							continue;
						}
					}
				},

				None         => return     // Disconnected
			};

			let f = processor.send( frame );

			Arbiter::spawn
			(
				f.map(|_|()).map_err(|e| eprintln!( "IpcPeer::listen -> mailbox error: {}", e ))
			);
		}
	}
}



impl Handler< IpcMessage > for IpcPeer
{
	type Result = ();

	fn handle( &mut self, msg: IpcMessage, _ctx: &mut Context<Self> ) -> Self::Result
	{
		let sink = self.sink.clone();

		Arbiter::spawn( async move
		{
			let mut stay_alive = sink.borrow_mut();

			match await!( stay_alive.send_async( msg ) )
			{
				Ok (_) => { println! ( "PeerB: successfully wrote to stream"       ); },
				Err(e) => { eprintln!( "PeerB: failed to write to stream: {:?}", e ); }
			}

			Ok(())

		}.boxed().compat());
	}

}
