# Rusty Link

Rusty Link is a Rust wrapper of [abl_link](https://github.com/Ableton/link/tree/master/extensions/abl_link), which is a C 11 wrapper made by Ableton for their original C++ code.
This library attempts to be as unoppionionated and plain as possible in
copying the functionality of abl_link, while hiding any unsafe behaviour and
providing Rust's safety guarantees.

[Ableton Link](http://ableton.github.io/link) is a technology that synchronizes musical beat, tempo,
phase, and start/stop commands across multiple applications running
on one or more devices. Applications on devices connected to a local
network discover each other automatically and form a musical session
in which each participant can perform independently: anyone can start
or stop while still staying in time. Anyone can change the tempo, the
others will follow. Anyone can join or leave without disrupting the session.

## Implementation

Rusty Link currently wraps around all functions available in ['abl_link.h'](https://github.com/Ableton/link/blob/master/extensions/abl_link/include/abl_link.h) except for the callbacks, since I havn't figured out a safe of implementing the context void\* pointer yet. Any help is appreciated.

## Credits

Thanks to Magnus Herold for [his implementation](https://github.com/magdaddy/ableton-link-rs).
This library started as a fork of his, but is now purely built on Ableton's C Wrapper, instead
of his custom C wrapper.
