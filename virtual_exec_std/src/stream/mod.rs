use std::panic::Location;
use std::pin::Pin;
use std::sync::Arc;
use futures::future::BoxFuture;
use virtual_exec_type::base::VmAnyType;

pub struct Named<W> {
    f: W,
    name: &'static str,
    at: &'static Location<'static>,
}

impl<W> std::fmt::Debug for Named<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}:{}", self.name, self.at.file(), self.at.line())
    }
}

pub type SyncWriter  = Arc<dyn Fn(Vec<u8>) -> bool + Send + Sync + 'static>;
pub type AsyncWriter = Arc<dyn Fn(Vec<u8>) -> Pin<Box<dyn Future<Output = bool> + Send + 'static>> + Send + Sync + 'static>;

#[derive(Debug)]
pub struct OutputByteStreamInner {
    pub sync_fn: Named<SyncWriter>,
    pub async_fn: Option<Named<AsyncWriter>>
}

pub type OutputByteStream = Arc<async_lock::RwLock<OutputByteStreamInner>>;


impl VmAnyType for OutputByteStreamInner {
    fn get_size(&self) -> usize {
        256
    }
}

