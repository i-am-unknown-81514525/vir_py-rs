use std::io::Write;
use std::sync::{Arc};
use async_lock::RwLock;
use cfg_if::cfg_if;
use tokio::io;
#[cfg(feature = "tokio-io")]
use tokio::io::AsyncWriteExt;
use crate::stream::write::{OutputByteStream, OutputByteStreamInner};
use virtual_exec_extern::*;
use virtual_exec_type::base::Native;
use virtual_exec_type::vm_type::Error;

fn print_sync(vec: Vec<u8>) -> bool {
    let mut stdout = std::io::stdout();
    match stdout.write_all(&vec) {
        Ok(_) => true,
        Err(e) => false
    }
}

#[cfg(feature = "tokio-io")]
async fn print_async(vec: Vec<u8>) -> bool {
    let mut stdout = io::stdout();
    match stdout.write_all(&vec).await {
        Ok(_) => true,
        Err(e) => false
    }
}

cfg_if!(
    if #[cfg(all(feature = "tokio-io", feature = "async"))] {
        #[fn_extern_wrap]
        fn get_output_stream() -> Result<Native<OutputByteStream>, Error> {
            Ok(Native::from(Arc::new(RwLock::new(OutputByteStreamInner::new_async(print_sync, print_async)))))
        }
    } else {
        #[fn_extern_wrap]
        fn get_output_stream() -> Result<Native<OutputByteStream>, Error> {
            Ok(Native::from(Arc::new(RwLock::new(OutputByteStreamInner::new_sync(print_sync)))))
        }
    }
);

extern_link!(GetOutputStream, get_output_stream, 0);