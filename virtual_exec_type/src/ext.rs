use alloc::sync::Arc;
use async_lock::{Mutex, MutexGuard, MutexGuardArc, RwLock, RwLockReadGuard, RwLockReadGuardArc, RwLockWriteGuard, RwLockWriteGuardArc};

pub trait SafeReadArcExt<T> {
    fn read_arc_safe(&self) -> RwLockReadGuardArc<T>;
}

pub trait SafeLockArcExt<T> {
    fn lock_arc_safe(&self) -> MutexGuardArc<T>;
}

pub trait SafeWriteArcExt<T> {
    fn write_arc_safe(&self) -> RwLockWriteGuardArc<T>;
}

pub trait SafeReadExt<T> {
    fn read_safe(&self) -> RwLockReadGuard<T>;
}

pub trait SafeLockExt<T> {
    fn lock_safe(&self) -> MutexGuard<T>;
}

pub trait SafeWriteExt<T> {
    fn write_safe(&self) -> RwLockWriteGuard<T>;
}

impl<T> SafeReadArcExt<T> for Arc<RwLock<T>> {
    #[cfg(all(feature = "std", not(target_family = "wasm")))]
    #[inline]
    fn read_arc_safe(&self) -> RwLockReadGuardArc<T> {
        self.read_arc_blocking()
    }

    #[cfg(any(not(feature = "std"), target_family = "wasm"))]
    #[inline]
    fn read_arc_safe(&self) -> RwLockReadGuardArc<T> {
        self.try_read_arc().expect("Deadlock")
    }
}

impl<T> SafeLockArcExt<T> for Arc<Mutex<T>> {
    #[cfg(all(feature = "std", not(target_family = "wasm")))]
    #[inline]
    fn lock_arc_safe(&self) -> MutexGuardArc<T> {
        self.lock_arc_blocking()
    }

    #[cfg(any(not(feature = "std"), target_family = "wasm"))]
    #[inline]
    fn lock_arc_safe(&self) -> MutexGuardArc<T> {
        self.try_lock_arc().expect("Deadlock")
    }
}

impl<T> SafeWriteArcExt<T> for Arc<RwLock<T>> {
    #[cfg(all(feature = "std", not(target_family = "wasm")))]
    #[inline]
    fn write_arc_safe(&self) -> RwLockWriteGuardArc<T> {
        self.write_arc_blocking()
    }

    #[cfg(any(not(feature = "std"), target_family = "wasm"))]
    #[inline]
    fn write_arc_safe(&self) -> RwLockWriteGuardArc<T> {
        self.try_write_arc().expect("Deadlock")
    }
}



impl<T> SafeReadExt<T> for RwLock<T> {
    #[cfg(all(feature = "std", not(target_family = "wasm")))]
    #[inline]
    fn read_safe(&self) -> RwLockReadGuard<'_, T> {
        self.read_blocking()
    }

    #[cfg(any(not(feature = "std"), target_family = "wasm"))]
    #[inline]
    fn read_safe(&self) -> RwLockReadGuard<'_, T> {
        self.try_read().expect("Deadlock")
    }
}

impl<T> SafeLockExt<T> for Mutex<T> {
    #[cfg(all(feature = "std", not(target_family = "wasm")))]
    #[inline]
    fn lock_safe(&self) -> MutexGuard<'_, T> {
        self.lock_blocking()
    }

    #[cfg(any(not(feature = "std"), target_family = "wasm"))]
    #[inline]
    fn lock_safe(&self) -> MutexGuard<'_, T> {
        self.try_lock().expect("Deadlock")
    }
}

impl<T> SafeWriteExt<T> for RwLock<T> {
    #[cfg(all(feature = "std", not(target_family = "wasm")))]
    #[inline]
    fn write_safe(&self) -> RwLockWriteGuard<'_, T> {
        self.write_blocking()
    }

    #[cfg(any(not(feature = "std"), target_family = "wasm"))]
    #[inline]
    fn write_safe(&self) -> RwLockWriteGuard<'_, T> {
        self.try_write().expect("Deadlock")
    }
}