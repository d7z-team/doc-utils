use std::sync::atomic::{AtomicU64, Ordering};

static ID: AtomicU64 = AtomicU64::new(0);

fn new_id() -> String {
    let i = ID.fetch_add(1, Ordering::Release);
    format!("doc-id-{}", i)
}
