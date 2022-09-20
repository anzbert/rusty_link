# Rusty Link

Rusty Link is a Rust wrapper of [abl_link](https://github.com/Ableton/link/tree/master/extensions/abl_link), which
is a C 11 wrapper made by Ableton for their C++ codebase.
This library attempts to be as unopinionated and plain as possible in
copying the functionality of abl_link, while hiding unsafe behaviour and
providing Rust's guarantees.

[Ableton Link](http://ableton.github.io/link) is a technology that synchronizes musical beat, tempo,
phase, and start/stop commands across multiple applications running
on one or more devices. Applications on devices connected to a local
network discover each other automatically and form a musical session
in which each participant can perform independently: anyone can start
or stop while still staying in time. Anyone can change the tempo, the
others will follow. Anyone can join or leave without disrupting the session.

## Implementation

Rusty Link currently wraps around all functions available in ['abl_link.h'](https://github.com/Ableton/link/blob/master/extensions/abl_link/include/abl_link.h) and makes them publicly available, except for the destructors, which are implemented on the Drop trait.

## Credits

Thanks to Magnus Herold for [his implementation](https://github.com/magdaddy/ableton-link-rs).
This library started as a fork of his, but is now purely built on Ableton's basic C Wrapper.
