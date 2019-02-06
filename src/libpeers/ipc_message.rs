use actix::prelude::*;
use serde_derive::{ Serialize, Deserialize };


#[ derive( Debug, Serialize, Deserialize )]
//
pub struct IpcMessage
{
	pub inner: String
}

impl Message for IpcMessage { type Result = (); }
