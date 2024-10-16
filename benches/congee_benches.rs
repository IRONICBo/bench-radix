use congee::Art;
use criterion::{criterion_group, criterion_main, Criterion};

fn get_text() -> Vec<String> {
    use std::fs::File;
    use std::io::Read;
    const DATA: &str = "data/prompts.txt";
    let mut contents = String::new();
    File::open(DATA)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn make_art(words: &[String]) -> Art<&String, usize> {
    let mut art = Art::default();
    let guard = art.guard();
    for (i, w) in words.iter().enumerate() {
        art.insert(w.as_str(), w.len(), &guard);
    }
    art
}

fn art_insert(b: &mut Criterion) {
    let words = get_text();
    b.bench_function("art insert", |b| {
        b.iter(|| {
            let _ = make_art(&words);
        })
    });
}

fn art_get(b: &mut Criterion) {
    let words = get_text();
    let art = make_art(&words);
    b.bench_function("art get", |b| {
        b.iter(|| {
            words.iter().enumerate().map(|(i, _)| {
                art.get(&(i as u64))
            }).collect::<Vec<Option<&usize>>>()
        })
    });
}

fn art_insert_remove(b: &mut Criterion) {
    let words = get_text();
    b.bench_function("art remove", |b| {
        b.iter(|| {
            let mut art = make_art(&words);
            for (i, _) in words.iter().enumerate() {
                art.remove(&(i as u64));
            }
        });
    });
}

criterion_group!(benches, art_insert, art_get);
criterion_main!(benches);
