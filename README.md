# `rtic-mut-not-mut`

This repositorie's entire point it to understand why in below code, `bar` has to be declared `let mut bar = ...` in `timer1` whereas `let bar = ...` is sufficient in `timer2`.

See the `minimal` binary for a minimal working example targeted at an `nRF52840`.

```rust
mod app {
    // ...

    #[task(binds = TIMER1, shared = [bar])]
    fn timer1(ctx: timer1::Context) {
        // Why do I need mut here?
        // let bar = ctx.shared.bar; // triggers a 'cannot borrow `bar` as mutable' below
        let bar = ctx.shared.bar;
        bar.lock(|_bar| {});

        // The same is true for
        // ctx.shared.bar.lock(|_bar| { });
    }

    #[task(binds = TIMER2, shared = [foo, bar])]
    fn timer2(ctx: timer2::Context) {
        #[allow(clippy::disallowed_names)]
        let foo = ctx.shared.foo;
        let bar = ctx.shared.bar;

        (bar, foo).lock(|_bar, _foo| {});
    }
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
