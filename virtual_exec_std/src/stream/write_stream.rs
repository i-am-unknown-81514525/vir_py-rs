use std::ops::Deref;
use std::sync::{Arc};
use async_lock::RwLock;
use cfg_if::cfg_if;
use tokio::io;
use tokio::io::AsyncWriteExt;
use crate::stream::{OutputByteStream, OutputByteStreamInner};
use virtual_exec_extern::*;
use virtual_exec_type::base::Native;
use virtual_exec_type::ext::SafeReadArcExt;
use virtual_exec_type::vm_type::{Boolean, Error};

#[fn_extern_wrap]
fn write_stream_sync(Native(stream): Native<OutputByteStream>, data: String) -> Result<Boolean, Error> {
    let data: Vec<u8> = data.into_bytes();
    Ok(stream.read_arc_safe().sync_fn.f.deref()(data))
}

#[fn_extern_wrap_async]
async fn write_stream_async(Native(stream): Native<OutputByteStream>, data: String) -> Result<Boolean, Error> {
    let data: Vec<u8> = data.into_bytes();
    let stream = stream.read().await;
    if let Some(f) = &stream.async_fn {
        Ok(f.f.deref()(data).await)
    } else {
        Ok(stream.sync_fn.f.deref()(data))
    }
}

extern_link!(WriteStream, write_stream_sync, write_stream_async, 2);