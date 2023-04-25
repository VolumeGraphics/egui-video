use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Clone)]
/// Just `Arc<Mutex<T>>` with a local cache.
pub struct Cache<T: Copy> {
    cached_value: T,
    pub(crate) override_value: Option<T>,
    raw_value: Arc<Mutex<T>>,
}

impl<T: Copy> Cache<T> {
    /// Set the value. Blocks the thread until it can aquire the mutex.
    pub fn set(&mut self, value: T) {
        self.cached_value = value;
        *self.raw_value.lock() = value
    }
    /// Get the value.
    /// Priority: Override value -> Try update attempt -> Cached value
    pub fn get(&mut self) -> T {
        self.override_value.unwrap_or(self.get_true())
    }
    /// Get the "true" value, ignoring override.
    /// Priority: Try update attempt -> Cached value
    pub fn get_true(&mut self) -> T {
        self.try_update_cache().unwrap_or(self.cached_value)
    }
    /// Updates the cache. Blocks the thread until it can aquire the mutex.
    pub fn update_cache(&mut self) {
        self.cached_value = *self.raw_value.lock();
    }
    /// Get the updated value. Blocks the thread until it can aquire the mutex.
    pub fn get_updated(&mut self) -> T {
        self.update_cache();
        self.cached_value
    }
    /// Attempt to update the cache by trying to lock the mutex. Returns the updated value as an [`Option`] if it succeeeds.
    pub fn try_update_cache(&mut self) -> Option<T> {
        if let Some(new_value) = self.raw_value.try_lock() {
            self.cached_value = *new_value;
            Some(self.cached_value)
        } else {
            None
        }
    }
    /// Make a new cache.
    pub fn new(value: T) -> Self {
        Self {
            override_value: None,
            cached_value: value,
            raw_value: Arc::new(Mutex::new(value)),
        }
    }
}