use num_traits::{CheckedAdd, One, Zero};

pub fn fibonacci<T: CheckedAdd + Clone + One + Zero>() -> impl Iterator<Item = T> {
    let (mut u, mut v) = (T::zero(), T::one());

    std::iter::from_fn(move || {
        let w = u.checked_add(&v)?;
        u = std::mem::replace(&mut v, w);
        Some(u.clone())
    })
}
