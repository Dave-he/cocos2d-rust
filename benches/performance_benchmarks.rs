use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cocos2d_rust::math::{Vec2, Vec3, Mat4};

fn vec2_operations_benchmark(c: &mut Criterion) {
    c.bench_function("vec2_add", |b| {
        let a = Vec2::new(3.0, 4.0);
        let v = Vec2::new(1.0, 2.0);
        b.iter(|| black_box(a + v));
    });
    
    c.bench_function("vec2_normalize", |b| {
        let mut v = Vec2::new(3.0, 4.0);
        b.iter(|| {
            v.normalize();
            black_box(&v);
        });
    });
    
    c.bench_function("vec2_distance", |b| {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(100.0, 100.0);
        b.iter(|| black_box(a.distance(&b)));
    });
    
    c.bench_function("vec2_dot_product", |b| {
        let a = Vec2::new(3.0, 4.0);
        let v = Vec2::new(5.0, 6.0);
        b.iter(|| black_box(a.dot(&v)));
    });
}

fn vec3_operations_benchmark(c: &mut Criterion) {
    c.bench_function("vec3_cross_product", |b| {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        b.iter(|| black_box(a.cross(&v)));
    });
    
    c.bench_function("vec3_normalize", |b| {
        let mut v = Vec3::new(3.0, 4.0, 5.0);
        b.iter(|| {
            v.normalize();
            black_box(&v);
        });
    });
}

fn mat4_operations_benchmark(c: &mut Criterion) {
    c.bench_function("mat4_multiply", |b| {
        let m1 = Mat4::create_translation(&Vec3::new(10.0, 20.0, 30.0));
        let m2 = Mat4::create_scale(&Vec3::new(2.0, 2.0, 2.0));
        b.iter(|| black_box(m1 * m2));
    });
    
    c.bench_function("mat4_invert", |b| {
        let m = Mat4::create_translation(&Vec3::new(5.0, 10.0, 15.0));
        b.iter(|| black_box(m.inverted()));
    });
    
    c.bench_function("mat4_transform_point", |b| {
        let m = Mat4::create_translation(&Vec3::new(10.0, 20.0, 30.0));
        let p = Vec3::new(1.0, 2.0, 3.0);
        b.iter(|| black_box(m.transform_point(&p)));
    });
}

fn node_hierarchy_benchmark(c: &mut Criterion) {
    c.bench_function("node_add_children", |b| {
        b.iter(|| {
            let mut parent = Node::new();
            for i in 0..100 {
                let child = Node::new();
                parent.add_child(child, 0, i);
            }
            black_box(parent);
        });
    });
    
    c.bench_function("node_find_child_by_tag", |b| {
        let mut parent = Node::new();
        for i in 0..1000 {
            let mut child = Node::new();
            child.set_tag(i);
            parent.add_child(child, 0, i);
        }
        
        b.iter(|| black_box(parent.get_child_by_tag(500)));
    });
}

criterion_group!(
    benches,
    vec2_operations_benchmark,
    vec3_operations_benchmark,
    mat4_operations_benchmark,
    node_hierarchy_benchmark
);

criterion_main!(benches);
