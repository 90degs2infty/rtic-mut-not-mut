#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use test_app as _; // global logger + panicking-behavior + memory layout

#[rtic::app(
    device = nrf52840_hal::pac,
    dispatchers = [SWI0_EGU0]
)]
mod app {

    // Shared resources go here
    #[shared]
    struct Shared {
        foo: u8,
        bar: u8,
    }

    // Local resources go here
    #[local]
    struct Local {}

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local) {
        (Shared { foo: 42, bar: 43 }, Local {})
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        #[allow(clippy::empty_loop)]
        loop {}
    }

    #[task(binds = TIMER1, shared = [bar])]
    fn timer1(ctx: timer1::Context) {
        // Why do I need mut here?
        // let bar = ctx.shared.bar; // triggers a 'cannot borrow `bar` as mutable' below
        let mut bar = ctx.shared.bar;
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
