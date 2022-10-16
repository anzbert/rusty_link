[![Crate](https://img.shields.io/crates/v/rusty_link.svg)](https://crates.io/crates/rusty_link)
[![API](https://docs.rs/rusty_link/badge.svg)](https://docs.rs/rusty_link)

# rusty_link

rusty_link is a Rust wrapper of [abl_link](https://github.com/Ableton/link/tree/master/extensions/abl_link),
which is a C 11 extension made by Ableton for their C++ codebase.
This library attempts to be unopinionated and plain in
copying the functionality of abl_link, while providing Rust's safety guarantees.

[Ableton Link](http://ableton.github.io/link) is a technology that synchronizes musical beat, tempo,
phase, and start/stop commands across multiple applications running
on one or more devices. Applications on devices connected to a local
network discover each other automatically and form a musical session
in which each participant can perform independently: anyone can start
or stop while still staying in time. Anyone can change the tempo, the
others will follow. Anyone can join or leave without disrupting the session.

## Implementation

- rusty_link currently wraps around all functions available in ['abl_link.h'](https://github.com/Ableton/link/blob/master/extensions/abl_link/include/abl_link.h) and makes them publicly available, except for the destructors, which are implemented on the Drop trait.
- Function documentation has been copied almost 1:1 from 'abl_link.h' as it should still apply.
- The `create` functions for abl_link and session_state have been renamed to `new` to make the API more Rust-intuitive.
- Functions have been implemented as methods on either the `AblLink` or the `SessionState` struct depending on which of the two the original C function uses as a primary parameter and what seemed to be the most intuitive.
- Delete functions have been added to delete previously set `num_peers`, `start_stop` and `tempo` callbacks.

## Example

This crate includes a Rust port of the Ableton Link ['link_hut'](https://github.com/Ableton/link/blob/master/extensions/abl_link/examples/link_hut/main.c) C example. See the code [here](https://github.com/anzbert/rusty_link/blob/master/examples/link_hut_silent/main.rs).

To run the example, clone this repository and change into its directory. Then fetch the Ableton Link source by initializing the git submodules with:

```
git submodule update --init --recursive
```

Compile and run a release build with:

```
cargo run --release --example link_hut_silent
```

## Requirements

Requires a recent version of CMake (3.14 or newer) to be installed and available in your terminal. Test with `cmake --version`.

## Safety

### Thread and Realtime Safety

['abl_link.h'](https://github.com/Ableton/link/blob/master/extensions/abl_link/include/abl_link.h) has doc comments about thread and realtime safety on some of its functions. Those comments have been copied to the functions of this library. A short explainer on what they mean:

- Thread Safety: If marked as `Yes`, this function can be safely called from multiple threads.

- Realtime Safety: If marked as `Yes`, this function can be called in a Realtime environment without blocking the thread. For example, an audio thread / callback.

### Callback Handling

The callback functions / closures set with `set_num_peers_callback`, `set_tempo_callback` and set_start_stop_callback` are handled by the underlying Link C++ library and may be run at any time. Data races and hidden mutations can occur if a closure has captured local variables and uses them at the same time as other code.

## Testing

Ableton designed a [Test Plan](https://github.com/Ableton/link/blob/master/TEST-PLAN.md) to test if your implementation of Ableton Link in your project meets all the expected requirements.

## Feedback

Pull requests and feedback in the github [Discussions](https://github.com/anzbert/rusty_link/discussions) section is very welcome!

## License

Ableton Link is dual licensed under GPLv2+ and a proprietary [license](https://github.com/Ableton/link/blob/master/LICENSE.md).

This means that this wrapper is automatically under the GPLv2+ as well. See the included Licence file.

If you would like to incorporate Link into a proprietary software application, please contact Ableton at <link-devs@ableton.com>.

## Credits

Thanks to Magnus Herold for [his implementation](https://crates.io/crates/ableton-link).
I made this library to learn about FFI in Rust and I started it as a fork of his. His library is great and adds a number of additional mappings, such as the ones to Clock in Ableton's C++ code. This crate on the other hand is purely built on Ableton's own C Wrapper, and requires additional functions to be implemented in pure Rust, if these are required by the user.

Some code for splitting closures has been borrowed from [ffi_helpers](https://crates.io/crates/ffi_helpers) with altered functionality. Thanks to Michael F Bryan for his work.
[Pull request](https://github.com/Michael-F-Bryan/ffi_helpers/pull/8) to ffi_helpers pending.

## Links

I also made a multi-platform Ableton Link wrapper for Flutter, called [f_link](https://pub.dev/packages/f_link), based on what I learned in this project.
