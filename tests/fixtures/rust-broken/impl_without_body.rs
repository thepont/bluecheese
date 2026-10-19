//! Not smelly, but not valid either: the `impl` has no braces, so it has a
//! target and no body. Parsing it must not panic, and its type must simply come
//! back with no methods.

pub struct Foo;

impl Foo
