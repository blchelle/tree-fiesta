use criterion::{criterion_group, criterion_main, Criterion};

mod avl;

fn create_tree(size: i32) {
	let mut tree = avl::AVL::new();

	for i in 0..size {
		tree.insert(i);
	}

	for i in 0..size / 10 {
		tree.search(i);
	}
}

fn criterion_benchmark(c: &mut Criterion) {
	let tree_sizes = vec![10000, 40000, 70000, 100000, 130000];
	for size in tree_sizes {
		c.bench_function("your function: ", |b| b.iter(|| create_tree(size)));
	}
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
