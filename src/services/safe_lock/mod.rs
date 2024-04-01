pub mod lock_timer;
use std::{net::TcpStream, sync::{mpsc::Receiver, Arc, Mutex, MutexGuard}, thread, time::Duration};
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
const DEDLOCK_TIMEOUT: u64 = 1500;
///
/// 
impl SafeLock<dyn Service + Send> for Arc<Mutex<dyn Service + Send>> {
    fn slock(&self) -> MutexGuard<'_, (dyn Service + Send + 'static)> {
        let self_id = format!("{:?}/dyn Service/SafeLock", self);
        let thread_name = thread::current().name().unwrap().to_owned();
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(DEDLOCK_TIMEOUT));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock service: '{:?}'...", thread_name, self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock service: '{:?}' - ok", thread_name, self_id);
        lock_timer.exit();
        mutax_guard
    }
}
///
///
impl SafeLock<Services> for Arc<Mutex<Services>> {
    fn slock(&self) -> MutexGuard<'_, Services> {
        let self_id = format!("{:?}/Services/SafeLock", self);
        let thread_name = thread::current().name().unwrap().to_owned();
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(DEDLOCK_TIMEOUT));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock {:?}...", thread_name, self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock {:?} - ok", thread_name, self_id);
        lock_timer.exit();
        mutax_guard
    }
}
///
///
impl SafeLock<Subscriptions> for Arc<Mutex<Subscriptions>> {
    fn slock(&self) -> MutexGuard<'_, Subscriptions> {
        let self_id = format!("{:?}/Subscriptions/SafeLock", self);
        let thread_name = thread::current().name().unwrap().to_owned();
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(DEDLOCK_TIMEOUT));
        lock_timer.run().unwrap();
        trace!("SafeLock.slock | Tread '{}' -> Lock {:?}...", thread_name, self_id);
        let mutax_guard = self.lock().unwrap();
        trace!("SafeLock.slock | Tread '{}' -> Lock {:?} - ok", thread_name, self_id);
        lock_timer.exit();
        mutax_guard
    }
}
///
///
impl SafeLock<TcpServerConnections> for Arc<Mutex<TcpServerConnections>> {
    fn slock(&self) -> MutexGuard<'_, TcpServerConnections> {
        let self_id = format!("{:?}/TcpServerConnections/SafeLock", self);
        let thread_name = thread::current().name().unwrap().to_owned();
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(DEDLOCK_TIMEOUT));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock {:?}...", thread_name, self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock {:?} - ok", thread_name, self_id);
        lock_timer.exit();
        mutax_guard
    }
}

///
///
impl SafeLock<TcpReadAlive> for Arc<Mutex<TcpReadAlive>> {
    fn slock(&self) -> MutexGuard<'_, TcpReadAlive> {
        let self_id = format!("{:?}/TcpReadAlive/SafeLock", self);
        let thread_name = thread::current().name().unwrap().to_owned();
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(DEDLOCK_TIMEOUT));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock {:?}...", thread_name, self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock {:?} - ok", thread_name, self_id);
        lock_timer.exit();
        mutax_guard
    }
}
///
///
impl SafeLock<TcpWriteAlive> for Arc<Mutex<TcpWriteAlive>> {
    fn slock(&self) -> MutexGuard<'_, TcpWriteAlive> {
        let self_id = format!("{:?}/TcpWriteAlive/SafeLock", self);
        let thread_name = thread::current().name().unwrap().to_owned();
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(DEDLOCK_TIMEOUT));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock {:?}...", thread_name, self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock {:?} - ok", thread_name, self_id);
        lock_timer.exit();
        mutax_guard
    }
}
///
///
impl SafeLock<TcpStreamWrite> for Arc<Mutex<TcpStreamWrite>> {
    fn slock(&self) -> MutexGuard<'_, TcpStreamWrite> {
        let self_id = format!("{:?}/TcpStreamWrite/SafeLock", self);
        let thread_name = thread::current().name().unwrap().to_owned();
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(DEDLOCK_TIMEOUT));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock {:?}...", thread_name, self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock {:?} - ok", thread_name, self_id);
        lock_timer.exit();
        mutax_guard
    }
}
///
/// 
impl SafeLock<dyn TcpStreamRead> for Arc<Mutex<dyn TcpStreamRead>> {
    fn slock(&self) -> MutexGuard<'_, (dyn TcpStreamRead + 'static)> {
        let self_id = format!("{:?}/dyn TcpStreamRead/SafeLock", self);
        let thread_name = thread::current().name().unwrap().to_owned();
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(DEDLOCK_TIMEOUT));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock: '{:?}'...", thread_name, self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock: '{:?}' - ok", thread_name, self_id);
        lock_timer.exit();
        mutax_guard
    }
}
///
/// 
impl SafeLock<Receiver<bool>> for Arc<Mutex<Receiver<bool>>> {
    fn slock(&self) -> MutexGuard<'_, Receiver<bool>> {
        let self_id = format!("{:?}/Receiver<bool>/SafeLock", self);
        let thread_name = thread::current().name().unwrap().to_owned();
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(DEDLOCK_TIMEOUT));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock: '{:?}'...", thread_name, self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock: '{:?}' - ok", thread_name, self_id);
        lock_timer.exit();
        mutax_guard
    }
}
///
/// 
impl SafeLock<Vec<TcpStream>> for Arc<Mutex<Vec<TcpStream>>> {
    fn slock(&self) -> MutexGuard<'_, Vec<TcpStream>> {
        let self_id = format!("{:?}/TcpStream/SafeLock", self);
        let thread_name = thread::current().name().unwrap().to_owned();
        let lock_timer = LockTimer::new(&self_id, Duration::from_millis(DEDLOCK_TIMEOUT));
        lock_timer.run().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock: '{:?}'...", thread_name, self_id);
        let mutax_guard = self.lock().unwrap();
        info!("SafeLock.slock | Tread '{}' -> Lock: '{:?}' - ok", thread_name, self_id);
        lock_timer.exit();
        mutax_guard
    }
}