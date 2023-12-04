use std::{
    collections::{BTreeMap, HashMap},
    iter,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day3::{GroupCollect, GroupCollectWithHasher};
pub fn criterion_benchmark(c: &mut Criterion) {
    let input_1_100000 = (0..100_000).zip(0..100_000);
    let input_10_10000 = iter::repeat((0..10_000).zip(0..10_000)).take(10).flatten();
    let input_100_1000 = iter::repeat((0..1000).zip(0..1000)).take(100).flatten();
    let input_1000_100 = iter::repeat((0..100).zip(0..100)).take(1000).flatten();
    let input_10000_10 = iter::repeat((0..10).zip(0..10)).take(10000).flatten();
    c.bench_function("group_collect-vec-vec 1_100000", |b| {
        b.iter(|| black_box(input_1_100000.clone()).group_collect::<Vec<_>, Vec<_>>())
    });
    c.bench_function("group_collect-vec-vec 10_10000", |b| {
        b.iter(|| black_box(input_10_10000.clone()).group_collect::<Vec<_>, Vec<_>>())
    });
    c.bench_function("group_collect-vec-vec 100_1000", |b| {
        b.iter(|| black_box(input_100_1000.clone()).group_collect::<Vec<_>, Vec<_>>())
    });
    c.bench_function("group_collect-vec-vec 1000_100", |b| {
        b.iter(|| black_box(input_1000_100.clone()).group_collect::<Vec<_>, Vec<_>>())
    });
    c.bench_function("group_collect-vec-vec 10000_10", |b| {
        b.iter(|| black_box(input_10000_10.clone()).group_collect::<Vec<_>, Vec<_>>())
    });
    c.bench_function("group_collect-hashmap-vec 100_1000", |b| {
        b.iter(|| black_box(input_100_1000.clone()).group_collect::<HashMap<_, _>, Vec<_>>())
    });
    c.bench_function("group_collect-btreemap-vec 100_1000", |b| {
        b.iter(|| black_box(input_100_1000.clone()).group_collect::<BTreeMap<_, _>, Vec<_>>())
    });
    c.bench_function("group_collect-vec-vec-fnv 100_1000", |b| {
        let fnv = fnv::FnvBuildHasher::default();
        b.iter(|| {
            black_box(input_100_1000.clone())
                .group_collect_with_hasher::<Vec<_>, Vec<_>>(fnv.clone())
        })
    });
    c.bench_function("group_collect-vec-vec-fnv2 100_1000", |b| {
        b.iter(|| {
            GroupCollectWithHasher::<fnv::FnvBuildHasher>::group_collect_with_hasher::<Vec<_>, Vec<_>>(
                black_box(input_100_1000.clone()), Default::default()
            )
        })
    });
    c.bench_function("fold-hashmap 100_1000", |b| {
        b.iter(|| {
            black_box(input_100_1000.clone()).fold(
                HashMap::<_, Vec<_>>::new(),
                |mut acc, (k, v)| {
                    acc.entry(k).or_default().push(v);
                    acc
                },
            )
        })
    });
    c.bench_function("fold-hashmap-to-vec 100_1000", |b| {
        b.iter(|| {
            black_box(input_100_1000.clone())
                .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (k, v)| {
                    acc.entry(k).or_default().push(v);
                    acc
                })
                .into_iter()
                .collect::<Vec<_>>()
        })
    });
    c.bench_function("ordered-hash-map 100_1000", |b| {
        b.iter(|| {
            let mut map = ordered_hash_map::OrderedHashMap::<_, Vec<_>>::new();
            black_box(input_100_1000.clone()).for_each(|(k, v)| match map.get_mut(&k) {
                Some(vi) => {
                    vi.push(v);
                }
                None => {
                    map.insert(k, vec![v]);
                }
            });
            map
        })
    });
    c.bench_function("ordered-hash-map-to-vec 100_1000", |b| {
        b.iter(|| {
            let mut map = ordered_hash_map::OrderedHashMap::<_, Vec<_>>::new();
            black_box(input_100_1000.clone()).for_each(|(k, v)| match map.get_mut(&k) {
                Some(vi) => {
                    vi.push(v);
                }
                None => {
                    map.insert(k, vec![v]);
                }
            });
            map.into_iter().collect::<Vec<_>>()
        })
    });
    c.bench_function("foreach-hashmap 100_1000", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            black_box(input_100_1000.clone())
                .for_each(|(k, v)| map.entry(k).or_insert(Vec::new()).push(v));
            map
        })
    });
    c.bench_function("foreach-hashmap-fnv 100_1000", |b| {
        b.iter(|| {
            let mut map = HashMap::<_, _, fnv::FnvBuildHasher>::default();
            black_box(input_100_1000.clone())
                .for_each(|(k, v)| map.entry(k).or_insert(Vec::new()).push(v));
            map
        })
    });
    c.bench_function("foreach-hashmap-fnv-to-vec 100_1000", |b| {
        b.iter(|| {
            let mut map = HashMap::<_, _, fnv::FnvBuildHasher>::default();
            black_box(input_100_1000.clone())
                .for_each(|(k, v)| map.entry(k).or_insert(Vec::new()).push(v));
            map.into_iter().collect::<Vec<_>>()
        })
    });
    c.bench_function("itertools::group_into_map 100_1000", |b| {
        use itertools::Itertools;
        b.iter(|| black_box(input_100_1000.clone()).into_group_map())
    });
    c.bench_function("itertools::group_into_map-to-vec 100_1000", |b| {
        use itertools::Itertools;
        b.iter(|| {
            black_box(input_100_1000.clone())
                .into_group_map()
                .into_iter()
                .collect::<Vec<_>>()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
