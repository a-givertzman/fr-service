pub mod lock_timer;
use std::{net::TcpStream, sync::{mpsc::Receiver, Arc, Mutex, MutexGuard}, time::Duration};
use log::{info, trace};
use crate::{services::safe_lock::lock_timer::LockTimer, tcp::{steam_read::TcpStreamRead, tcp_read_alive::TcpReadAlive, tcp_stream_write::TcpStreamWrite, tcp_write_alive::TcpWriteAlive}};
use super::{multi_queue::subscriptions::Subscriptions, server::connections::TcpServerConnections, service::service::Service, services::Services};
///
/// Defines methods to wrap the lock method on the kind of Mutex / RwLock.. etc...
///  - for measure lock by timer to detect the dedlock 
///  - for debugging purposes
pub trait SafeLock<T> where T: ?Sized {
    fn slock(&self) -> MutexGuard<T>;
}
//
// 
impl SafeLock<dyn Service> for Arc<Mutex<dyn Service>> {
    fn slock(&self) -> MutexGuard<'_, (dyn Service + 'static)> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        info!("SafeLock.slock | Lock service: '{:?}'...", self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Lock service: '{:?}' - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}
//
// 
impl SafeLock<dyn Service + Send> for Arc<Mutex<dyn Service + Send>> {
    fn slock(&self) -> MutexGuard<'_, (dyn Service + Send + 'static)> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        info!("SafeLock.slock | Lock service: '{:?}'...", self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Lock service: '{:?}' - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}
//
//
impl SafeLock<Services> for Arc<Mutex<Services>> {
    fn slock(&self) -> MutexGuard<'_, Services> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Lock {:?}...", self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Lock {:?} - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}
//
//
impl SafeLock<Subscriptions> for Arc<Mutex<Subscriptions>> {
    fn slock(&self) -> MutexGuard<'_, Subscriptions> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        lock_timer.run().unwrap();
        trace!("SafeLock.slock | Lock {:?}...", self_id);
        let mutax_guard = self.lock().unwrap();
        trace!("SafeLock.slock | Lock {:?} - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}
//
//
impl SafeLock<TcpServerConnections> for Arc<Mutex<TcpServerConnections>> {
    fn slock(&self) -> MutexGuard<'_, TcpServerConnections> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Lock {:?}...", self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Lock {:?} - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}
//
//
impl SafeLock<TcpReadAlive> for Arc<Mutex<TcpReadAlive>> {
    fn slock(&self) -> MutexGuard<'_, TcpReadAlive> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Lock {:?}...", self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Lock {:?} - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}
//
//
impl SafeLock<TcpWriteAlive> for Arc<Mutex<TcpWriteAlive>> {
    fn slock(&self) -> MutexGuard<'_, TcpWriteAlive> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Lock {:?}...", self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Lock {:?} - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}
//
//
impl SafeLock<TcpStreamWrite> for Arc<Mutex<TcpStreamWrite>> {
    fn slock(&self) -> MutexGuard<'_, TcpStreamWrite> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Lock {:?}...", self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Lock {:?} - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}
//
// 
impl SafeLock<dyn TcpStreamRead> for Arc<Mutex<dyn TcpStreamRead>> {
    fn slock(&self) -> MutexGuard<'_, (dyn TcpStreamRead + 'static)> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Lock: '{:?}'...", self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Lock: '{:?}' - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}
//
// 
impl SafeLock<Receiver<bool>> for Arc<Mutex<Receiver<bool>>> {
    fn slock(&self) -> MutexGuard<'_, Receiver<bool>> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Lock: '{:?}'...", self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Lock: '{:?}' - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}
//
// 
impl SafeLock<Vec<TcpStream>> for Arc<Mutex<Vec<TcpStream>>> {
    fn slock(&self) -> MutexGuard<'_, Vec<TcpStream>> {
        let self_id = format!("{:?}/SafeLock", self);
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(100));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Lock: '{:?}'...", self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Lock: '{:?}' - ok", self_id);
        lock_timer.exit();
        mutax_guard
    }
}