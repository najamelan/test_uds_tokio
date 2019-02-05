use crate::msg::Write;
use actix::prelude::*;
use tokio_uds::UnixStream;
use tokio_async_await::io::AsyncWriteExt;
use futures_util::{future::FutureExt, try_future::TryFutureExt};
use std::rc::Rc;
use std::cell::RefCell;

#[ derive( Debug ) ]
//
pub struct IpcClient
{
	pub socket: Rc<RefCell<UnixStream>>,
}

impl Actor for IpcClient { type Context = Context<Self>; }


impl Handler< Write > for IpcClient
{
	type Result = ();

	fn handle( &mut self, write: Write, _ctx: &mut Context<Self> ) -> Self::Result
	{
		let socket = self.socket.clone();

		Arbiter::spawn( async move
		{
			let mut stay_alive = socket.borrow_mut();

			match await!( stay_alive.write_async( &write.message ) )
			{
				Ok (_) => { println! ( "PeerB: successfully wrote to stream"       ); },
				Err(e) => { eprintln!( "PeerB: failed to write to stream: {:?}", e ); }
			}

			Ok(())
		}.boxed().compat());
	}

}
