enum RSEnum {
    Foo2(Option<i32>),
    Foo(i32),
    Bar(String),
    Baz(Vec<String>),
}

fn main() {
    let foo = Some(5);

    if let Some(value) = foo {}

    match foo {
        Some(value) => {}
        None => {}
    }

    foo.map(|x| {});

    foo.filter(|x| x < &10);
}
