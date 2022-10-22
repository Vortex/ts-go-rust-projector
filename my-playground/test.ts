type Foo = {
    bar?: String
}

function doSomething(foo: Foo): boolean {
    if (foo.bar) {
        return true;
    } else {
        // ...c
        return false;
    }
}