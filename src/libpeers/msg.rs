use actix::prelude::*;
use crate::EkkeResult;

#[ derive( Debug )]
//
pub struct Buffer
{
	pub inner: Vec<u8>
}

impl Message for Buffer { type Result = (); }



#[ derive( Debug ) ]
//
pub struct Write
{
	pub message: Vec<u8>,
}

impl Message for Write { type Result = (); }


#[ derive( Clone ) ]
//
pub struct IpcBind
{
	pub socket   : String,
	pub processor: Recipient<Buffer>,
}

impl Message for IpcBind {	type Result = EkkeResult<()>; }
