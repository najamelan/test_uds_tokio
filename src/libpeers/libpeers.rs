#![feature(await_macro, async_await, futures_api, arbitrary_self_types, never_type)]

use failure::{ Error, Fail };

use std::str;


mod ipc_client;
mod ipc_server;
mod processor;

pub mod msg;

pub use ipc_client::IpcClient;
pub use ipc_server::IpcServer;
pub use processor::Processor;

pub type EkkeResult<T> = Result< T, Error >;


#[ derive( Debug, Fail ) ]
//
pub enum EkkeError
{
	#[ fail( display = "Cannot use socket before connecting" ) ]
	//
	UseSocketBeforeConnect
}
