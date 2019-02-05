use crate::msg::Buffer;
use actix::prelude::*;
use std::str;


#[ derive( Debug, Clone )]
//
pub struct Processor {}

impl Actor for Processor { type Context = Context< Self >; }



impl Handler< Buffer > for Processor
{
	type Result = ();

	fn handle( &mut self, msg: Buffer, _ctx: &mut Context<Self> )
	{
		let message = str::from_utf8( &msg.inner ).expect( "Received invalid utf8" ).to_string();

		println!( "Received: {:?}", message );
	}
}

