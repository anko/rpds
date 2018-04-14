/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

#[macro_use]
extern crate criterion;

mod utils;

use criterion::{black_box, Criterion};
use std::collections::HashMap;
use utils::limit;

fn std_hash_map_insert(c: &mut Criterion) {
    let limit = limit(10_000);

    c.bench_function("std hash map insert", move |b| {
        b.iter(|| {
            let mut map: HashMap<usize, isize> = HashMap::new();

            for i in 0..limit {
                map.insert(i, -(i as isize));
            }

            map
        })
    });
}

// TODO implement rust_btreemap_remove in the same style as the test of `RedBlackTreeMap::remove()`
// once we can do per-iteration initialization.

fn std_hash_map_get(c: &mut Criterion) {
    let limit = limit(10_000);
    let mut map: HashMap<usize, isize> = HashMap::new();

    for i in 0..limit {
        map.insert(i, -(i as isize));
    }

    c.bench_function("std hash map get", move |b| {
        b.iter(|| {
            for i in 0..limit {
                black_box(map.get(&i));
            }
        })
    });
}

fn std_hash_map_iterate(c: &mut Criterion) {
    let limit = limit(10_000);
    let mut map: HashMap<usize, isize> = HashMap::new();

    for i in 0..limit {
        map.insert(i, -(i as isize));
    }

    c.bench_function("std hash map iterate", move |b| {
        b.iter(|| {
            for kv in &map {
                black_box(kv);
            }
        })
    });
}

criterion_group!(
    benches,
    std_hash_map_insert,
    std_hash_map_get,
    std_hash_map_iterate
);
criterion_main!(benches);
