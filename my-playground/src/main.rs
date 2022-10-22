enum RSEnum {
    Foo2(Option<i32>),
    Foo(i32),
    Bar(String),
    Baz(Vec<String>),
}

fn main() {
    let foo = RSEnum::Foo(5);

    if let RSEnum::Foo(value) = foo {}

    match foo {
        RSEnum::Foo2(Some(value)) => {}
        RSEnum::Foo2(None) => {}
        RSEnum::Foo(value) => {}
        _ => {}
    }
}
