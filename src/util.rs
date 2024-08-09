use std::{hash::Hasher, sync::Arc};

pub struct Comparable<T: ?Sized> {
    pub(crate) hash: u64,
    pub(crate) data: T,
}

impl<T: ?Sized> core::ops::Deref for Comparable<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: ?Sized> core::cmp::PartialEq for Comparable<T> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl<T: ?Sized> core::cmp::Eq for Comparable<T> {}

impl<T: ?Sized> Comparable<T> {
    pub fn hash<F: Sized>(value: &F) -> u64 {
        let len = core::mem::size_of::<F>();
        let b = unsafe { core::slice::from_raw_parts(value as *const F as *const u8, len) };

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(b);
        hasher.finish()
    }
    pub fn new_func<F, R>(clousure: F) -> Arc<Comparable<dyn Fn() -> R + Send + Sync>>
    where
        F: Fn() -> R + Sync + Send + 'static,
    {
        let hash = Self::hash(&clousure);

        Arc::new(Comparable {
            hash,
            data: clousure,
        })
    }
}

#[test]
fn test() {
    let mut last = 0;

    for i in 0..10 {
        let r = 9;

        let f = move || {
            r;
        };
        let h = Comparable::<()>::hash(&f);

        println!("{}", last == h);

        last = h;
    }
}
