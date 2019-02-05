// https://jsdw.me/posts/rust-asyncawait-preview/


// enable the await! macro, async support, and the new std::Futures api.
//
#![feature(await_macro, async_await, futures_api)]

// only needed to manually implement a std future:
//
#![feature(arbitrary_self_types)]

#![feature(never_type)]


use std::sync::Arc;
use tokio_async_await::io::AsyncReadExt;
use tokio_async_await::stream::StreamExt;
use failure::{ Error, Fail };
use std::path::PathBuf;
use tokio_uds::{ UnixListener, UnixStream, ConnectFuture };
use tokio::io;
use tokio_async_await::compat::forward::IntoAwaitable;
use tokio_async_await::io::AsyncWriteExt;
use std::str;
use actix::prelude::*;
use std::future::Future as StdFuture;
use tokio::prelude::*;

type EkkeResult<T> = Result< T, Error >;


#[ derive( Debug, Fail ) ]
//
enum EkkeError
{
	#[ fail( display = "Cannot use socket before connecting" ) ]
	//
	UseSocketBeforeConnect
}


pub struct IpcServer
{
	connection: Option< UnixStream >
}


impl Actor for IpcServer { type Context = Context<Self>; }


impl IpcServer
{
	pub fn new() -> Self
	{
		Self
		{
			connection: None,
		}
	}


	// TODO: Bubble up errors
	//
	async fn handle_stream( mut stream: UnixStream, processor: Addr< Processor > )
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


#[ derive( Debug ) ]
//
pub struct IpcClient
{
	pub connection: UnixStream,
}


impl IpcClient
{
	pub fn new( sock_addr: PathBuf ) -> Self
	{

	}
	// pub fn connect( &mut self ) -> EkkeResult<()>
	// {
	// 	self.connection = Some( await!( UnixStream::connect( &self.address ).into_awaitable() )? );
	// 	Ok(())
	// }

	// pub fn write( &mut self, message: Vec<u8> ) -> EkkeResult<()>
	// {
	// 	if let Some( ref mut connection ) = &mut self.connection
	// 	{
	// 		await!( connection.write_async( &message ) )?;

	// 		Ok(())
	// 	}

	// 	else { Err( EkkeError::UseSocketBeforeConnect.into() ) }
	// }
}


impl Actor for IpcClient { type Context = Context<Self>; }


#[ derive( Debug, Clone ) ]
//
pub struct Connect
{
	pub address: PathBuf,
}

impl Message for Connect
{
	type Result = Result< (), std::io::Error >;
}

impl Handler< Connect > for IpcClient
{
	type Result = Box< Future< Item = (), Error = std::io::Error >>;

	fn handle( &mut self, msg: Connect, _ctx: &mut Context<Self> ) -> Self::Result
	{
		// self.connection = Some( ? );
		//
		let connection = async
		{
			match await!( UnixStream::connect( &msg.address ).into_awaitable() )
			{
				Ok ( cx  ) => { self.connection = cx },
				Err( err ) => { return Err( err )            }
			}

			Ok(())
		};

		Box::new( backward( connection ) )
	}

}




#[ derive( Debug, Clone ) ]
//
pub struct Write
{
	pub message: Vec<u8>,
}

impl Message for Write
{
	type Result = Result< UnixStream, std::io::Error >;
}


impl Handler< Write > for IpcClient
{
	type Result = ResponseFuture< UnixStream, std::io::Error >;

	fn handle( &mut self, msg: Write, _ctx: &mut Context<Self> ) -> Self::Result
	{
		// self.connection = Some( ? );
		//

		Box::new( UnixStream::connect( &msg.address ) )
	}

}

// Actor IpcChannel has the following messages.
//
// connect (will create the socket)
// listen  (takes processor address)
// write   (takes bytes array)
// close

#[ derive( Debug, Clone ) ]
//
pub struct IpcBind
{
	pub socket   : String         ,
	pub processor: Addr<Processor>,
}

impl Message for IpcBind
{
	type Result = EkkeResult<()>;
}

impl Handler<IpcBind> for IpcServer
{
	type Result = EkkeResult<()>;

	fn handle( &mut self, msg: IpcBind, _ctx: &mut Context<Self> ) -> Self::Result
	{
		std::fs::remove_file( &msg.socket )?;

		let listener = UnixListener::bind( &msg.socket ).expect( "PeerA: Could not bind to socket" );


		tokio::run_async
		(
			async move
			{
				let mut incoming = listener.incoming();

				while let Some( stream ) = await!( incoming.next() )
				{
					println!( "PeerA: New stream!" );

					tokio::spawn_async( Self::handle_stream( stream.expect( "PeerA: Invalid Stream" ), msg.clone().processor ) );
				}
			}
		);

		Ok(())
	}
}

#[ derive( Debug, Clone )]
//
pub struct Processor
{

}

impl Actor for Processor { type Context = Context< Self >; }



#[ derive( Debug )]
struct Buffer
{
	inner: Vec<u8>
}



impl Message for Buffer
{
	type Result = ();
}


impl Handler< Buffer > for Processor
{
	type Result = ();

	fn handle( &mut self, msg: Buffer, _ctx: &mut Context<Self> )
	{
		let message = str::from_utf8( &msg.inner ).expect( "Received invalid utf8" ).to_string();

		println!("Received: {:?}", message );
	}
}





































// Converting new style Futures to old style Futures

// To make use of new style Futures alongside the various combinators and such exposed on old style Futures, you'll need to convert them. Although not explicitly exposed, you can make use of the machinery in the tokio-async-await crate to make quick work of it (This is correct as of tokio-async-await 0.1.4 but may change at any time):
//
// The only caveat here is that the new style future needs to output a Result, so that we can map to the Item and Error associated types needed for old style futures. With this in hand, we can use a new Future (made by an async thing) as if it was an old one:

// // Map our hello_world() future to return a Result<&str,()> rather
// // than just &'str, so that we can convert it to an old style one:
// let hello_world_result = async {
//     let s = await!(hello_world());
//     Ok::<_,()>(s)
// };

// // use the above function to convert back:
// let hello_world_old = backward(hello_world_result);

// // We can then run it like any old style future, allowing to to use any
// // of the machinery currently available for 0.1 style futures:
// tokio::run(
//     hello_world_old.map(|val| println!("Running as 0.1 future: {}", val))
// );

// The main use case for this that I can see is making use of combinators like select and join from the land of old Futures, rather than having to reimplement them to work alongside new Futures.

use std::future::Future as NewFuture;
use futures::Future as OldFuture;
use tokio_async_await::compat::backward::Compat as Backward;

// converts from a new style Future to an old style one:
pub fn backward< I,E >(f: impl NewFuture<Output=std::result::Result<I,E>>) -> impl OldFuture<Item=I, Error=E>
{
	compat::backward::Compat::new(f)
}


// Converting old style Futures into new style Futures

// The easiest way to convert an old style Future into a new one is simply by using the await! macro that Tokio gives us (rather than std::await), since it will convert old style Futures for us. We've already seen this above when using the tokio::timer::Delay future in a new style async block.

// If we want, we can use the same approach to write ourselves a function to manually convert them for us:

// use std::future::Future as NewFuture;
// use futures::Future as OldFuture;



// We can test that it works by using the std::await macro on a converted Future instead of the Tokio version:

// tokio::run_async(async {
//     // Create some old style Future:
//     let old_future = futures::future::ok::<_,()>("Awaiting a manually converted 0.1 future!");
//     // Convert to a new style one:
//     let new_future = forward(old_future);
//     // `await` the result and print it:
//     println!("{}", std::await!(new_future).unwrap());
// });

// I'm not really sure if this is super useful, but it's nice to know that you can easily go back and forth between new and old style Futures.

// converts from an old style Future to a new style one:
pub fn forward<I,E>(f: impl OldFuture<Item=I, Error=E> + Unpin) -> impl NewFuture<Output=std::result::Result<I,E>>
{
    use tokio_async_await::compat::forward::IntoAwaitable;
    f.into_awaitable()
}
