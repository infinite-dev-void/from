//
//
//
//
//
//
// This benchmark was run on an HP Victus with
// core-i5 12450h CPU and 16G 3200MT/s RAM.
//
//
//

use criterion::{criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};

use from::{from, FromJson};
use serde_json;
use std::hint::black_box;

#[derive(Serialize, Deserialize)]
#[from(json)]
struct Person {
    id: u64,
    name: String,
    age: u8,
    phones: Vec<String>,
    length: f32,
    married: bool,
}

fn normal_parse_from(json: &[u8]) -> Person {
    Person::from_json(json).unwrap()
}

fn normal_parse_serde(json: &[u8]) -> Person {
    serde_json::from_slice(json).unwrap()
}

fn noraml_input_benchmark(c: &mut Criterion) {
    let normal = black_box(
        r#"{
            "id": 78548954,
            "name": "person_name",
            "age": 25,
            "phones": ["7895432798332", "057984542357", "8840157956457"],
            "length": 184.5,
            "married": false
        }"#
        .as_bytes(),
    );

    c.bench_function("normal_parse_from", |b| {
        b.iter(|| normal_parse_from(normal))
    });

    c.bench_function("normal_parse_serde", |b| {
        b.iter(|| normal_parse_serde(normal))
    });
}

#[derive(Serialize, Deserialize)]
#[from(json)]
struct Ints {
    int8: i8,
    int16: i16,
    int32: i32,
    int64: i64,
    int128: i128,
    intsize: isize,
    uint8: u8,
    uint16: u16,
    uint32: u32,
    uint64: u64,
    uint128: u128,
    uintsize: usize,
}

fn ints_parse_from(json: &[u8]) -> Ints {
    Ints::from_json(json).unwrap()
}

fn ints_parse_serde(json: &[u8]) -> Ints {
    serde_json::from_slice(json).unwrap()
}

fn ints_input_benchmark(c: &mut Criterion) {
    let json = format!(
        r#"{{
            "int8": {},
            "int16": {},
            "int32": {},
            "int64": {},
            "int128": {},
            "intsize": {},
            "uint8": {},
            "uint16": {},
            "uint32": {},
            "uint64": {},
            "uint128": {},
            "uintsize": {}
        }}"#,
        i8::MAX,
        i16::MAX,
        i32::MAX,
        i64::MAX,
        i128::MAX,
        isize::MAX,
        u8::MAX,
        u16::MAX,
        u32::MAX,
        u64::MAX,
        u128::MAX,
        usize::MAX,
    );

    let ints = black_box(json.as_bytes());

    c.bench_function("ints_parse_from", |b| b.iter(|| ints_parse_from(ints)));

    c.bench_function("ints_parse_serde", |b| b.iter(|| ints_parse_serde(ints)));
}

#[derive(Serialize, Deserialize)]
#[from(json)]
struct Floats {
    float1: f32,
    float2: f64,
    float3: f32,
    float4: f64,
    float5: f32,
    float6: f64,
    float7: f32,
    float8: f64,
}

fn floats_parse_from(json: &[u8]) -> Floats {
    Floats::from_json(json).unwrap()
}

fn floats_parse_serde(json: &[u8]) -> Floats {
    serde_json::from_slice(json).unwrap()
}

fn floats_input_benchmark(c: &mut Criterion) {
    // i tried to use this but `serde_json` panicked because of `.unwrap()`
    /* let json = format!(
        r#"{{
            "float32": {},
            "float64": {}
        }}"#,
        f32::MAX,
        f64::MAX,
    ); */

    let floats = black_box(
        r#"{
            "float1": 0.7148438,
            "float2": 0.021333566466941734,
            "float3": 715.4744051,
            "float4": 309.05400620536216627,
            "float5": 212.43404543,
            "float6": 741.24126502187543786,
            "float7": 821.68977946,
            "float8": 783.641738116968562
        }"#
        .as_bytes(),
    );

    c.bench_function("floats_parse_from", |b| {
        b.iter(|| floats_parse_from(floats))
    });

    c.bench_function("floats_parse_serde", |b| {
        b.iter(|| floats_parse_serde(floats))
    });
}

criterion_group!(
    benches,
    noraml_input_benchmark,
    ints_input_benchmark,
    floats_input_benchmark
);

criterion_main!(benches);
