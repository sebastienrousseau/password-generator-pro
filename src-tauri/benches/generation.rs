// Copyright © 2022-2026 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Performance benchmarks for the generation path.
//!
//! The interesting cost here is not the password construction — it is
//! **bcrypt**, which is deliberately slow and is the dominant term in
//! `generate_password`. `HASH_COST` is the knob that decides how slow;
//! benchmarking it is the only way to notice if that constant is
//! changed without appreciating what it costs, and to know the budget
//! for the tray's "Copy Password" action, which runs this synchronously
//! while the user waits.
//!
//! The other three are effectively free and are measured so a
//! regression in them would stand out rather than hide behind bcrypt.

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
// criterion::black_box is deprecated in 0.8 in favour of the std one.
use password_generator_pro::core::action::action_for;
use password_generator_pro::core::ids::{MENU_ITEM_IDS, TRAY_ITEM_IDS};
use password_generator_pro::util::qrcode::QRCode;
use password_generator_pro::util::uuid::UUID;
use std::hint::black_box;

/// bcrypt at the cost factor the application ships.
///
/// Each iteration is a full hash, so this benchmark is intentionally
/// slow; the sample size is reduced to keep the suite usable.
fn bench_bcrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("bcrypt");
    group.sample_size(10);
    group.bench_function("hash at HASH_COST", |b| {
        b.iter(|| {
            bcrypt::hash(black_box("correct horse battery staple"), black_box(8))
                .expect("hash should succeed")
        });
    });
    group.finish();
}

/// UUID v4 generation — the tray's "Copy UUID" action.
fn bench_uuid(c: &mut Criterion) {
    c.bench_function("uuid v4", |b| b.iter(|| black_box(UUID::uuid())));
}

/// QR-code rendering — the tray's "Save QR Code" action.
fn bench_qrcode(c: &mut Criterion) {
    let mut group = c.benchmark_group("qrcode");
    for content in ["short", "a-medium-length-password-with-separators"] {
        group.throughput(Throughput::Bytes(content.len() as u64));
        group.bench_function(format!("svg for {} bytes", content.len()), |b| {
            b.iter(|| black_box(QRCode::qrcode(black_box(content))));
        });
    }
    group.finish();
}

/// Menu dispatch — runs on every click, so it must stay trivial.
///
/// A regression here would mean someone replaced the match with
/// something allocating.
fn bench_dispatch(c: &mut Criterion) {
    let ids: Vec<&str> = TRAY_ITEM_IDS
        .iter()
        .chain(MENU_ITEM_IDS.iter())
        .copied()
        .collect();

    c.bench_function("action_for over every menu id", |b| {
        b.iter(|| {
            for id in &ids {
                black_box(action_for(black_box(id)));
            }
        });
    });

    c.bench_function("action_for miss", |b| {
        b.iter(|| black_box(action_for(black_box("not-a-menu-id"))));
    });
}

criterion_group!(
    benches,
    bench_bcrypt,
    bench_uuid,
    bench_qrcode,
    bench_dispatch
);
criterion_main!(benches);
