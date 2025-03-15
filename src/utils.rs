use std::sync::atomic::AtomicU32;

static ID: AtomicU32 = AtomicU32::new(0);
pub fn get_id() -> u32 {
    ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}
