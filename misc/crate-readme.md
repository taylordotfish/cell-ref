cell-ref
========

This crate provides a [`Cell`] type (like the standard library’s
[`Cell`][std-cell]) with methods for safely mutating and inspecting the
inner value by reference ([`with`] and [`with_mut`]).

For [`Copy`] types, this is implemented with [`get`][std-get] and
[`set`][std-set], but [through an extension trait][cell-ext], this crate
provides those same operations for types that are [`Default`] but not
[`Copy`]. A [`get`] method is also available for types that are both
[`Default`] and [`Clone`].

This crate depends only on [`core`], so it can be used inside `no_std`
environments.

Example
-------

```rust
use cell_ref::{Cell, CellExt};

let c1 = Cell::new(2_u8);
c1.with_mut(|x| *x += 3);
assert!(c1.get() == 5);

let c2 = Cell::new(vec![1, 2, 3]);
c2.with_mut(|v| v.push(4)); // Works even though `Vec` isn't `Copy`
assert_eq!(c2.with(Vec::len), 4);
let v = c2.get(); // Clones the vector
```

[std-cell]: https://doc.rust-lang.org/stable/core/cell/struct.Cell.html
[cell-ext]: https://docs.rs/cell-ref/latest/cell_ref/struct.Cell.html#method.CellExt
[`with`]: https://docs.rs/cell-ref/latest/cell_ref/struct.Cell.html#method.with
[`with_mut`]: https://docs.rs/cell-ref/latest/cell_ref/struct.Cell.html#method.with_mut
[std-get]: https://doc.rust-lang.org/stable/core/cell/struct.Cell.html#method.get
[std-set]: https://doc.rust-lang.org/stable/core/cell/struct.Cell.html#method.set
[`get`]: https://docs.rs/cell-ref/latest/cell_ref/struct.Cell.html#method.get
[`Cell`]: https://docs.rs/cell-ref/latest/cell_ref/struct.Cell.html
[`Copy`]: https://doc.rust-lang.org/stable/core/marker/trait.Copy.html
[`Default`]: https://doc.rust-lang.org/stable/core/default/trait.Default.html
[`Clone`]: https://doc.rust-lang.org/stable/core/clone/trait.Clone.html
[`core`]: https://doc.rust-lang.org/stable/core/
