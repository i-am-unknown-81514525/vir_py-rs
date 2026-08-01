#![cfg(feature = "stream")]

use std::panic::Location;
use std::pin::Pin;
use std::sync::Arc;
use async_lock::RwLock;
use virtual_exec_type::base::VmAnyType;

macro_rules! func {
    ($name: ident) => {
        pub mod $name;
        #[allow(unused)]
        pub use $name::*;
    };
}

func!(get_output_stream);
func!(write_stream);
func!(read_stream);


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

pub mod write {
    use std::panic::Location;
    use std::pin::Pin;
    use std::sync::Arc;
    use async_lock::RwLock;
    use virtual_exec_type::base::VmAnyType;
    use crate::stream::Named;

    #[derive(Debug)]
    pub struct OutputByteStreamInner {
        pub sync_fn: Named<SyncWriter>,
        pub async_fn: Option<Named<AsyncWriter>>
    }



    pub type SyncWriter  = Arc<dyn Fn(Vec<u8>) -> bool + Send + Sync + 'static>;
    pub type AsyncWriter = Arc<dyn Fn(Vec<u8>) -> Pin<Box<dyn Future<Output = bool> + Send + 'static>> + Send + Sync + 'static>;

    impl OutputByteStreamInner {
        #[track_caller]
        pub fn new_sync<F>(f: F) -> Self
        where F: Fn(Vec<u8>) -> bool + Send + Sync + 'static
        {
            Self {
                sync_fn: Named {
                    f: Arc::new(f),
                    name: std::any::type_name::<F>(),
                    at: Location::caller()
                },
                async_fn: None
            }
        }

        #[track_caller]
        pub fn new_async<F, AF, Fut>(f: F, af: AF) -> Self
        where
            F: Fn(Vec<u8>) -> bool + Send + Sync + 'static,
            AF: Fn(Vec<u8>) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = bool> + Send + 'static
        {
            let at = Location::caller();
            let af = Arc::new(af);
            let async_writer: AsyncWriter = Arc::new(move |b| Box::pin(af(b)) as _);
            Self {
                sync_fn: Named {
                    f: Arc::new(f),
                    name: std::any::type_name::<F>(),
                    at
                },
                async_fn: Some(Named {
                    f: async_writer,
                    name: std::any::type_name::<AF>(),
                    at
                })
            }
        }
    }

    pub type OutputByteStream = Arc<RwLock<OutputByteStreamInner>>;


    impl VmAnyType for OutputByteStreamInner {
        fn get_size(&self) -> usize {
            256
        }
    }
}


pub mod read {
    use std::panic::Location;
    use std::pin::Pin;
    use std::sync::Arc;
    use async_lock::RwLock;
    use virtual_exec_type::base::VmAnyType;
    use crate::stream::Named;

    #[derive(Debug)]
    pub struct InputByteStreamInner {
        pub sync_fn: Named<SyncReader>,
        pub async_fn: Option<Named<AsyncReader>>
    }




    /// None should be returned when the stream have closed, Empty vector should be returned when no data is currently available
    pub type SyncReader  = Arc<dyn Fn() -> Option<Vec<u8>> + Send + Sync + 'static>;
    pub type AsyncReader = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Option<Vec<u8>>> + Send + 'static>> + Send + Sync + 'static>;






    impl InputByteStreamInner {
        #[track_caller]
        pub fn new_sync<F>(f: F) -> Self
        where F: Fn() -> Option<Vec<u8>> + Send + Sync + 'static
        {
            Self {
                sync_fn: Named {
                    f: Arc::new(f),
                    name: std::any::type_name::<F>(),
                    at: Location::caller()
                },
                async_fn: None
            }
        }

        #[track_caller]
        pub fn new_async<F, AF, Fut>(f: F, af: AF) -> Self
        where
            F: Fn() -> Option<Vec<u8>> + Send + Sync + 'static,
            AF: Fn() -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Option<Vec<u8>>> + Send + 'static
        {
            let at = Location::caller();
            let af = Arc::new(af);
            let async_writer: AsyncReader = Arc::new(move || Box::pin(af()) as _);
            Self {
                sync_fn: Named {
                    f: Arc::new(f),
                    name: std::any::type_name::<F>(),
                    at
                },
                async_fn: Some(Named {
                    f: async_writer,
                    name: std::any::type_name::<AF>(),
                    at
                })
            }
        }
    }

    pub type InputByteStream = Arc<RwLock<InputByteStreamInner>>;


    impl VmAnyType for InputByteStreamInner {
        fn get_size(&self) -> usize {
            256
        }
    }
}
