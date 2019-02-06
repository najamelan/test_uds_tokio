use crate::IpcMessage;
use actix::prelude::*;


#[ derive( Debug, Clone )]
//
pub struct Processor {}

impl Actor for Processor { type Context = Context< Self >; }



impl Handler< IpcMessage > for Processor
{
	type Result = ();

	fn handle( &mut self, msg: IpcMessage, _ctx: &mut Context<Self> )
	{
		println!( "Received: {:?}", &msg.inner );
	}
}

