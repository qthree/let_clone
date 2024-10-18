/// # Examples
///
/// ```
/// tokio::spawn({
///     let_clone!(self: this, cx.{io, disk, health_check: check});
///     async move {
///         this.do_something(io, disk, check)
///     }
/// })
/// ```
#[macro_export]
macro_rules! let_clone {
  ($self:ident . { $($cloneable:ident $(: $rename:ident)?),+$(,)? }, $($tail:tt)+) => {
    $crate::let_clone!($($self.$cloneable $(: $rename)?),+);
    $crate::let_clone!($($tail)+);
  };
  ($self:ident . { $($cloneable:ident $(: $rename:ident)?),+$(,)? }) => {
    $crate::let_clone!($($self.$cloneable $(: $rename)?),+);
  };

  ($($cloneable:ident).+ $(: $rename:ident)?, $($tail:tt)+) => {
    $crate::let_clone!(@inner $($cloneable).+;;$($rename)?);
    $crate::let_clone!($($tail)+);
  };
  ($($cloneable:ident).+ $(: $rename:ident)? $(,)?) => {
    $crate::let_clone!(@inner $($cloneable).+;;$($rename)?);
  };

  (@inner $root:ident$(.$nested:ident)+; $($tail:ident).*; $($rename:ident)?) => {
    $crate::let_clone!(@inner $($nested).+; $($tail.)*$root; $($rename)?);
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
            boo: String,
        }
        impl Foo {
            fn foobar(self, _baz: String) {}

            fn run(&self, bar: &Bar) {
                std::thread::spawn({
                    let_clone!(self: this, bar.baz, bar.{baz: baz2, boo}, bar.boo: boo2);
                    let _ = (baz2, boo, boo2);
                    move || this.foobar(baz)
                });
            }
        }
        Foo.run(&Bar {
            baz: "baz".into(),
            boo: "boo".into(),
        });
    }
}
