[![Crate](https://img.shields.io/crates/v/rusty_link.svg)](https://crates.io/crates/rusty_link)
[![API](https://docs.rs/rusty_link/badge.svg)](https://docs.rs/rusty_link)

# rusty_link

rusty_link is a Rust wrapper of [abl_link](https://github.com/Ableton/link/tree/master/extensions/abl_link),
which is a C 11 extension for Ableton Link, provided by Ableton.
This library attempts to be mostly unopinionated and plain in
copying the functionality of abl_link, while providing some of Rust's safety guarantees.

[Ableton Link](http://ableton.github.io/link) is a technology that synchronizes musical beat, tempo,
phase, and start/stop commands across multiple applications running
on one or more devices. Applications on devices connected to a local
network discover each other automatically and form a musical session
in which each participant can perform independently: anyone can start
or stop while still staying in time. Anyone can change the tempo, the
others will follow. Anyone can join or leave without disrupting the session.

## Example

To run the examples, clone this repository and change into its directory. Then fetch the Ableton Link source by initializing the git submodules with:

```
git submodule update --init --recursive
```

This crate includes a Rust port of the simple ['LinkHut' C example](https://github.com/Ableton/link/blob/master/extensions/abl_link/examples/link_hut/main.c) . To run it:

```
cargo run --release --example link_hut_silent
```

There is also a Rust port of the more complex ['LinkHut' C++ example](https://github.com/Ableton/link/tree/master/examples), which has sound. Run it like this:

```
cargo run --release --example link_hut
```

## Requirements

Requires a recent version of CMake (3.14 or newer) to be installed and available in your terminal. Test with `cmake --version`.

## Safety

### Thread and Realtime Safety

['abl_link.h'](https://github.com/Ableton/link/blob/master/extensions/abl_link/include/abl_link.h) has doc comments about thread and realtime safety on some of its functions. Those comments have been copied to the functions of this library. A short explainer on what they mean:

- [Thread Safety](https://en.wikipedia.org/wiki/Thread_safety): Thread-safe code only manipulates shared data structures in a manner that ensures that all threads behave properly and fulfill their design specifications without unintended interaction.

- Realtime Safety: These functions can be called in a Realtime environment without blocking the thread. For example, an audio thread / callback.

### Callback Handling

The callback functions / closures set with `set_num_peers_callback`, `set_tempo_callback` and set_start_stop_callback` are handled by the underlying Link C++ library and may be run at any time. Data races and hidden mutations can occur if a closure has captured local variables and uses them at the same time as other code.

## Testing

Ableton designed a [Test Plan](https://github.com/Ableton/link/blob/master/TEST-PLAN.md) to test if your implementation of Ableton Link in your project meets all the expected requirements.

## Implementation

- An instance of AblLink can be thought of as an Object with internal mutability. Kind of like a RefCell. Thread safety is guaranteed with internal Mutexes on the C++ side. Check the function and Link documentation for more.
- rusty_link currently wraps around all functions available in ['abl_link.h'](https://github.com/Ableton/link/blob/master/extensions/abl_link/include/abl_link.h) and makes them publicly available, except for the destructors, which are implemented on the Drop trait.
- Function documentation has been copied almost 1:1 from 'abl_link.h' as it should still apply.
- The `create` functions for abl_link and session_state have been renamed to `new` to make the API more Rust-intuitive.
- Functions have been implemented as methods on either the `AblLink` or the `SessionState` struct.
- Delete functions have been added to delete previously set `num_peers`, `start_stop` and `tempo` callbacks.

## Known Issues

- The Example with sound has been implemented with [cpal](https://crates.io/crates/cpal) for cross-platform audio support. Compatibility depends on cpal and my possibly dodgy use of it. Help is appreciated. Especially with sample and host timings, since that stuff is tricky :)

## Feedback

Pull requests and feedback in the github [Discussions](https://github.com/anzbert/rusty_link/discussions) section is very welcome!

## License

Ableton Link is dual licensed under GPLv2+ and a proprietary [license](https://github.com/Ableton/link/blob/master/LICENSE.md).

This means that this wrapper is automatically under the GPLv2+ as well. See the included Licence file.

If you would like to incorporate Link into a proprietary software application, please contact Ableton at <link-devs@ableton.com>.

## Credits

Thanks to Magnus Herold for [his implementation](https://crates.io/crates/ableton-link).
I made this library to learn about FFI in Rust and I started it as a fork of his.

Some code for splitting closures has been borrowed from [ffi_helpers](https://crates.io/crates/ffi_helpers) with altered functionality. Thanks to Michael F Bryan for his work.
[Pull request](https://github.com/Michael-F-Bryan/ffi_helpers/pull/8) to ffi_helpers pending.

## Links

For anyone interested, I also started making a multi-platform Ableton Link wrapper for Flutter, called [f_link](https://pub.dev/packages/f_link).
