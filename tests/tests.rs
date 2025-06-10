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

use cell_ref::{Cell, CellExt};
use core::cell::Cell as StdCell;

#[derive(Default)]
struct DefaultType(u8);

#[derive(Default, Clone)]
struct DefaultCloneType(u8);

#[test]
fn copy_type() {
    for cell in [Cell::new(5), 5.into()] {
        cell.with_mut(|x| *x += 1);
        assert!(cell.get() == 6);
        cell.set(10);
        cell.with(|x| assert!(*x == 10));
    }
}

#[test]
fn default_type() {
    let inner = || DefaultType(8);
    for cell in [Cell::new(inner()), inner().into()] {
        cell.with_mut(|x| x.0 += 4);
        cell.with(|x| assert!(x.0 == 12));
        cell.set(DefaultType(20));
        cell.with(|x| assert!(x.0 == 20));
    }
}

#[test]
fn default_clone_type() {
    let inner = || DefaultCloneType(5);
    for cell in [Cell::new(inner()), inner().into()] {
        cell.with_mut(|x| x.0 *= 2);
        assert!(cell.get().0 == 10);
        cell.set(DefaultCloneType(15));
        cell.with(|x| assert!(x.0 == 15));
    }
}

#[test]
fn convert() {
    let c = Cell::<u8>::from(StdCell::new(1));
    assert!(c.get() == 1);
    let c = StdCell::<u8>::from(Cell::new(2));
    assert!(c.get() == 2);
}
