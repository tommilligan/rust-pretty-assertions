use criterion::{criterion_group, criterion_main, Criterion};
use pretty_assertions::{assert_eq, assert_ne};

pub fn assert_eq_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("assert_eq pass");

    let value = "";
    group.bench_with_input("empty string", value, |b, value| {
        b.iter(|| assert_eq!(value, value))
    });

    let value = "mohmie9luchohw5eizeichugh0xoa4ro4naePeMuVie1aihi7pheshuotoosah2e";
    group.bench_with_input("64 char string", value, |b, value| {
        b.iter(|| assert_eq!(value, value))
    });

    group.finish();
}

pub fn assert_ne_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("assert_ne pass");

    let values = ('L', 'R');
    group.bench_with_input("character", &values, |b, (left, right)| {
        b.iter(|| assert_ne!(left, right))
    });

    let values = (
        "mohmie9luchohw5eizeichugh0xoa4ro4naePeMuVie1aihi7pheshuotoosah2e",
        "ot4Fae8iete1ahYa2phei7ephai5iefeet7vaeng1itho6erahy5aichuS3Thee2",
    );
    group.bench_with_input("64 char string", &values, |b, (left, right)| {
        b.iter(|| assert_ne!(left, right))
    });

    group.finish();
}

criterion_group!(macros, assert_eq_pass, assert_ne_pass);
criterion_main!(macros);
