[![Crate](https://img.shields.io/crates/v/rusty_link.svg)](https://crates.io/crates/rusty_link)
[![API](https://docs.rs/rusty_link/badge.svg)](https://docs.rs/rusty_link)

# rusty_link

`rusty_link` is a Rust wrapper of [abl_link](https://github.com/Ableton/link/tree/master/extensions/abl_link),
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

## Examples

To run the examples, clone this repository and change into its directory. Then fetch the Ableton Link source by initializing the git submodules with:

```
git submodule update --init --recursive
```

[**link_hut_silent**](https://github.com/anzbert/rusty_link/blob/master/examples/link_hut_silent/main.rs): A Rust port [from C](https://github.com/Ableton/link/blob/master/extensions/abl_link/examples/link_hut/main.c) of the simple 'LinkHut' example without sound by Ableton. To run it:

```
cargo run --release --example link_hut_silent
```

[**link_hut**](https://github.com/anzbert/rusty_link/tree/master/examples/link_hut): A Rust port [from C++](https://github.com/Ableton/link/tree/master/examples) of the more complex 'LinkHut' example **with** sound by Ableton. Run it like this:

```
cargo run --release --example link_hut
```

See the [cpal documentation](https://github.com/RustAudio/cpal) for ASIO and Jack support, if required.

## Requirements

Requires a recent version of CMake (3.14 or newer) to be installed and available in your terminal. Test with `cmake --version`.

Linux _may_ require a few more system libraries to be installed for C compilation, depending on your distro, like `build-essential`, `libclang-dev` or `libasound2-dev` and `pkg-config` for examples, etc...

## Thread and Realtime Safety

['abl_link.h'](https://github.com/Ableton/link/blob/master/extensions/abl_link/include/abl_link.h) has doc comments about thread and realtime safety on some of its functions. Those comments have been copied to the functions of this library. A short explainer on what they mean:

- [Thread Safety](https://en.wikipedia.org/wiki/Thread_safety): Thread-safe code only manipulates shared data structures in a manner that ensures that all threads behave properly and fulfill their design specifications without unintended interaction.

- Realtime Safety: These functions can be called in a Realtime environment without blocking the thread. For example, the audio thread/callback.

## Implementation

- `rusty_link` currently wraps around all functions available in ['abl_link.h'](https://github.com/Ableton/link/blob/master/extensions/abl_link/include/abl_link.h) and makes them publicly available as methods on either the `AblLink` or the `SessionState` struct, except for the destructors, which are implemented on the Drop trait.struct.
- An instance of AblLink can be thought of as an Object with internal mutability. Thread safety is guaranteed in all functions, except for the capture/commit of Session States, with internal Mutexes on the C++ side. Check the function doc comments and official Link documentation for more.
- Includes a Rust port of the C++ [HostTimeFilter](https://github.com/Ableton/link/blob/master/include/ableton/link/HostTimeFilter.hpp), which can be used in the audio callback to align the host clock with the sample clock.
- Delete functions have been added to delete previously set `num_peers`, `start_stop` and `tempo` callbacks.

## Testing

Ableton designed a [Test Plan](https://github.com/Ableton/link/blob/master/TEST-PLAN.md) to test if your implementation of Ableton Link in your project meets all the expected requirements.

## Tested Platforms

`rusty_link` itsself works on all major platforms. I only had trouble with the example with sound on Linux. Could be my fault for not using `cpal` properly?! Any help with that is highly appreciated. 😘
Anyway, this shouldnt stop anyone from using this library in their project. Have fun!

|                            | MacOS M1 | Win 11 WASAPI | Ubuntu 22 on Pi4   |
| -------------------------- | -------- | ------------- | ------------------ |
| Building `rusty_link`      | &check;  | &check;       | &check;            |
| Example: `link_hut_silent` | &check;  | &check;       | &check;            |
| Example: `link_hut`        | &check;  | &check;       | cpal/ALSA issues?! |

## Feedback

I am not a professional Developer, just doing this as a hobby, so any help with updates and corrections of my work are welcome.

## License

Ableton Link is dual licensed under GPLv2+ and a proprietary [license](https://github.com/Ableton/link/blob/master/LICENSE.md).

This means that `rusty_link` has to be under the GPLv2+ as well.

If you would like to incorporate Link into a proprietary software application, please contact Ableton at <link-devs@ableton.com>.

## Credits

Thanks to Magnus Herold for [his implementation](https://crates.io/crates/ableton-link).
I made this library to learn about FFI in Rust and I started it as a fork of his.

Some code for splitting closures has been borrowed from [ffi_helpers](https://crates.io/crates/ffi_helpers) with altered functionality. Thanks to Michael F Bryan for his work.
[Pull request](https://github.com/Michael-F-Bryan/ffi_helpers/pull/8) to ffi_helpers pending...

## Links

For anyone interested, I also started making a multi-platform Ableton Link wrapper for Flutter, called [f_link](https://pub.dev/packages/f_link).
