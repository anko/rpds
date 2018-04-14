/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

#[macro_use]
extern crate criterion;

mod utils;

use criterion::{black_box, Criterion};
use std::collections::LinkedList;
use utils::limit;

fn std_linked_list_push_front(c: &mut Criterion) {
    let limit = limit(10_000);

    c.bench_function("std linked list push front", move |b| {
        b.iter(|| {
            let mut linked_list: LinkedList<usize> = LinkedList::new();

            for i in 0..limit {
                linked_list.push_front(i);
            }

            linked_list
        })
    });
}

fn std_linked_list_push_back(c: &mut Criterion) {
    let limit = limit(10_000);

    c.bench_function("std linked list push back", move |b| {
        b.iter(|| {
            let mut linked_list: LinkedList<usize> = LinkedList::new();

            for i in 0..limit {
                linked_list.push_back(i);
            }

            linked_list
        })
    });
}

fn std_linked_list_iterate(c: &mut Criterion) {
    let limit = limit(10_000);
    let mut linked_list: LinkedList<usize> = LinkedList::new();

    for i in 0..limit {
        linked_list.push_back(i);
    }

    c.bench_function("std linked list iterate", move |b| {
        b.iter(|| {
            for i in &linked_list {
                black_box(i);
            }
        })
    });
}

criterion_group!(
    benches,
    std_linked_list_push_front,
    std_linked_list_push_back,
    std_linked_list_iterate
);
criterion_main!(benches);
