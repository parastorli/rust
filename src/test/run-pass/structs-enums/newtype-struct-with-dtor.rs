// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(unused_unsafe)]
#![allow(unused_variables)]
// pretty-expanded FIXME #23616

pub struct Fd(u32);

fn foo(a: u32) {}

impl Drop for Fd {
    fn drop(&mut self) {
        unsafe {
            let Fd(s) = *self;
            foo(s);
        }
    }
}

pub fn main() {
}
