use criterion::{criterion_group, criterion_main, Criterion};
use pretty_assertions::Comparison;
use std::io::{Result, Write};

/// A writer that throws away all data passed to it.
struct NullWriter;

impl Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

pub fn diff_different(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff different");

    let values = ('L', 'R');
    group.bench_with_input("character", &values, |b, (left, right)| {
        b.iter(|| write!(NullWriter {}, "{}", Comparison::new(left, right)))
    });

    let values = (
        "mohmie9luchohw5eizeichugh0xoa4ro4naePeMuVie1aihi7pheshuotoosah2e",
        "ot4Fae8iete1ahYa2phei7ephai5iefeet7vaeng1itho6erahy5aichuS3Thee2",
    );
    group.bench_with_input("64 char string", &values, |b, (left, right)| {
        b.iter(|| write!(NullWriter {}, "{}", Comparison::new(left, right)))
    });

    group.finish();
}

pub fn diff_same(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff same");

    let values = ("", "");
    group.bench_with_input("empty string", &values, |b, (left, right)| {
        b.iter(|| write!(NullWriter {}, "{}", Comparison::new(left, right)))
    });

    let values = (
        "mohmie9luchohw5eizeichugh0xoa4ro4naePeMuVie1aihi7pheshuotoosah2e",
        "mohmie9luchohw5eizeichugh0xoa4ro4naePeMuVie1aihi7pheshuotoosah2e",
    );
    group.bench_with_input("64 char string", &values, |b, (left, right)| {
        b.iter(|| write!(NullWriter {}, "{}", Comparison::new(left, right)))
    });

    group.finish();
}

criterion_group!(pretty_diff, diff_different, diff_same);
criterion_main!(pretty_diff);
