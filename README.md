# Setup

* Install [rust](https://www.rust-lang.org/tools/install)
* Install [probe-rs](https://github.com/probe-rs/probe-rs?tab=readme-ov-file#installation)
* Follow probe-rs' [probe setup instructions for
  st-link](https://probe.rs/docs/getting-started/probe-setup/#st-link)

# Running

* Plug the board into computer with USB cable.
* Check which type of board you're using. It should either be an STM32L452 or an STM32G071.
* Change into the subdirectory for the type of board.
* `cargo run`

If it works, you should see output like the following:

```
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.10s
     Running `probe-rs run --chip STM32G071RBTx target/thumbv6m-none-eabi/debug/embedded-workshop`
      Erasing ✔ [00:00:00] [#########################################] 14.00 KiB/14.00 KiB @ 43.19 KiB/s (eta 0s )
  Programming ✔ [00:00:00] [#########################################] 14.00 KiB/14.00 KiB @ 25.31 KiB/s (eta 0s )    Finished in 0.899s
INFO  Press the blue button to turn the LED on and off
└─ embedded_workshop::____embassy_main_task::{async_fn#0} @ src/main.rs:19  
```

# Experimenting further

Have a look at [embassy.dev](https://embassy.dev/) for further inspiration.

Some things you might try to do:

* Blink the LED from a background task independent of the button presses.
* Blink the LED several times each time the button is pressed.

Some things to keep in mind:

* The pin for the button and the pin for the LED are specific to the particular model of board that
  you're using. You can see which pins they are by looking at the existing code in the directory for
  your board. If you're copying code from the Embassy documentation, you'll likely need to adjust it
  a little for the board you're using.
* Whether pressing a button sets the pin high or low depends on the board model. Some set it high,
  some set it low. This also affects whether you should set the input to `Pull::Up` or `Pull::Down`.

# License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT)
at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
Wild by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

Some bits are copied and adapted from https://embassy.dev/.
