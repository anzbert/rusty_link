# Rusty Link

Rusty Link is a Rust wrapper of [abl_link](https://github.com/Ableton/link/tree/master/extensions/abl_link), which
is a C 11 wrapper made by Ableton for their C++ codebase.
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

- Rusty Link currently wraps around all functions available in ['abl_link.h'](https://github.com/Ableton/link/blob/master/extensions/abl_link/include/abl_link.h) and makes them publicly available, except for the destructors, which are implemented on the Drop trait.
- The `abl_link_create()` functions for abl_link and session_state have been renamed to `new()` to make the API more Rust-intuitive.
- Functions have been implemented as methods on either the `AblLink` or the `SessionState` struct depending on which of the two the original C function uses as a primary parameter and what seemed to be the most intuitive.
- At this point, handling thread and realtime safety with Audio and App Session States is left up to the user, just like in the original library.
- Ableton's documentation should mostly still apply to this library, since implementations have been copied as they were.
- The function documentations have been copied from 'abl_link.h', except for the addition of the following safety warning for callbacks.

## Safety

The callbacks/closures are handled by the underlying Link C++ library and may be run at any time.
Data races and hidden mutations can occur if a closure captures and uses local variables at the same
time as another thread.

## Feedback

Pull requests and feedback in the discussions section is very welcome!

## License

Ableton Link is dual licensed under GPLv2+ and a proprietary [license](https://github.com/Ableton/link/blob/master/LICENSE.md).

This means that this wrapper is automatically under the GPLv2+ as well. A copy of the license is distributed with the source code.

If you would like to incorporate Link into a proprietary software application, please contact Ableton at <link-devs@ableton.com>.

## Credits

Thanks to Magnus Herold for [his implementation](https://github.com/magdaddy/ableton-link-rs).
I made this library to learn about FFI in Rust and I started it as a fork of his. His library is great and adds a number of additional mappings, such as a ones to Clock in Ableton's C++ code. This crate on the other hand is purely built on Ableton's own C Wrapper, and requires Host Time Filters and Clock helper functions to be implemented in pure Rust, if these are required by the user.

Some code for splitting closures has been borrowed from [ffi_helpers](https://crates.io/crates/ffi_helpers) with altered functionality. Thanks to Michael F Bryan for his work.
[Pull request](https://github.com/Michael-F-Bryan/ffi_helpers/pull/8) to ffi_helpers pending.

## Links

I also made an Ableton Link wrapper plugin for Flutter, called [f_link](https://pub.dev/packages/f_link), based on what I learned in this project.
