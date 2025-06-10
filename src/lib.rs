/*
 * Copyright 2022 taylor.fish <contact@taylor.fish>
 *
 * This file is part of cell-ref.
 *
 * cell-ref is licensed under the Apache License, Version 2.0
 * (the "License"); you may not use cell-ref except in compliance
 * with the License. You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![no_std]
#![forbid(unsafe_code)]

//! This crate provides a [`Cell`] type (like the standard library’s
//! [`Cell`][std-cell]) with methods for safely mutating and inspecting the
//! inner value by reference ([`with`] and [`with_mut`]).
//!
//! For [`Copy`] types, this is implemented with [`get`][std-get] and
//! [`set`][std-set], but [through an extension trait][cell-ext], this crate
//! provides those same operations for types that are [`Default`] but not
//! [`Copy`]. A [`get`] method is also available for types that are both
//! [`Default`] and [`Clone`].
//!
//! This crate depends only on [`core`], so it can be used inside `no_std`
//! environments.
//!
//! Example
//! -------
//!
//! ```rust
//! use cell_ref::{Cell, CellExt};
//!
//! let c1 = Cell::new(2_u8);
//! c1.with_mut(|x| *x += 3);
//! assert!(c1.get() == 5);
//!
//! let c2 = Cell::new(vec![1, 2, 3]);
//! c2.with_mut(|v| v.push(4)); // Works even though `Vec` isn't `Copy`
//! assert_eq!(c2.with(Vec::len), 4);
//! let v = c2.get(); // Clones the vector
//! ```
//!
//! [std-cell]: StdCell
//! [cell-ext]: CellExt
//! [`with`]: Cell::with
//! [`with_mut`]: Cell::with_mut
//! [std-get]: StdCell::get
//! [std-set]: StdCell::set
//! [`get`]: Cell::get

use core::cell::Cell as StdCell;
use core::cmp::Ordering;
use core::fmt;
use core::ops::{Deref, DerefMut};

/// A `Cell` type with methods for by-reference mutation and inspection.
#[derive(Default)]
pub struct Cell<T>(StdCell<T>);

impl<T> Cell<T> {
    /// Creates a new [`Cell`] with the given value.
    pub fn new(value: T) -> Self {
        Self(StdCell::new(value))
    }
}

impl<T> Deref for Cell<T> {
    type Target = StdCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Cell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Cell<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> From<StdCell<T>> for Cell<T> {
    fn from(cell: StdCell<T>) -> Self {
        Self(cell)
    }
}

impl<T> From<Cell<T>> for StdCell<T> {
    fn from(cell: Cell<T>) -> Self {
        cell.0
    }
}

impl<T: Copy> Cell<T> {
    /// Gets the value held by the cell.
    pub fn get(&self) -> T {
        self.0.get()
    }

    /// Calls `f` with a reference to the contents of the cell.
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(&self.get())
    }

    /// Calls `f` with a mutable reference to the contents of the cell.
    pub fn with_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut value = self.get();
        let result = f(&mut value);
        self.set(value);
        result
    }
}

mod sealed {
    pub trait Sealed {}
}

/// Provides additional methods for non-[`Copy`] types.
pub trait CellExt<T>: sealed::Sealed {
    /// Gets the value held by the cell.
    fn get(&self) -> T
    where
        T: Clone + Default;

    /// Calls `f` with a reference to the contents of the cell.
    fn with<F, R>(&self, f: F) -> R
    where
        T: Default,
        F: FnOnce(&T) -> R;

    /// Calls `f` with a mutable reference to the contents of the cell.
    fn with_mut<F, R>(&self, f: F) -> R
    where
        T: Default,
        F: FnOnce(&mut T) -> R;
}

impl<T> sealed::Sealed for Cell<T> {}

impl<T> CellExt<T> for Cell<T> {
    fn get(&self) -> T
    where
        T: Clone + Default,
    {
        self.with(T::clone)
    }

    fn with<F, R>(&self, f: F) -> R
    where
        T: Default,
        F: FnOnce(&T) -> R,
    {
        let value = self.take();
        let result = f(&value);
        self.set(value);
        result
    }

    fn with_mut<F, R>(&self, f: F) -> R
    where
        T: Default,
        F: FnOnce(&mut T) -> R,
    {
        let mut value = self.take();
        let result = f(&mut value);
        self.set(value);
        result
    }
}

impl<T: Copy> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: fmt::Debug + Copy> fmt::Debug for Cell<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

impl<T: Ord + Copy> Ord for Cell<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(other)
    }
}

impl<T: PartialOrd + Copy> PartialOrd for Cell<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: PartialEq + Copy> PartialEq for Cell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other)
    }
}

impl<T: Eq + Copy> Eq for Cell<T> {}
