/// # Examples
///
/// ```
/// tokio::spawn({
///     let_clone!(self: this, cx.io, cx.disk, cx.health_check);
///     async move {
///         this.do_something(io, disk, health_check)
///     }
/// })
/// ```
#[macro_export]
macro_rules! let_clone {
  ($($($cloneable:ident).+ $(: $rename:ident)?),+$(,)?) => {
    $(
      let_clone!(@inner $($cloneable).+;;$($rename)?);
    )+
  };
  (@inner $root:ident$(.$nested:ident)+; $($tail:ident).*; $($rename:ident)?) => {
    let_clone!(@inner $($nested).+; $($tail.)*$root; $($rename:ident)?);
  };
  (@inner $cloneable:ident; $($nested:ident).*; $rename:ident) => {
    let $rename = $($nested.)*$cloneable.clone();
  };
  (@inner $cloneable:ident; $($nested:ident).*; ) => {
    let $cloneable = $($nested.)*$cloneable.clone();
  };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        #[derive(Clone)]
        struct Foo;
        struct Bar {
            baz: String,
        }
        impl Foo {
            fn foobar(self, _baz: String) {}

            fn run(&self, bar: &Bar) {
                std::thread::spawn({
                    let_clone!(self: this, bar.baz);
                    move || this.foobar(baz)
                });
            }
        }
        Foo.run(&Bar { baz: "baz".into() });
    }
}
