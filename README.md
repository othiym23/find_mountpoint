# find_mountpoint

The documentation is [here][docs].

This started as a simple crate to simplify getting at one field returned from
`statfs(2)` on macOS. It takes care of all of the unsafe stuff and reflects my
best understanding as a Rust newbie of how to deal with `libc` memory
management efficiently. It has since grown a bit and should work on all
platforms supported by Rust.

Its error handling is pretty conservative. It doesn't use `.unwrap()` and
should therefore never panic unless you fail to handle the `Result` in your own
code. I created a new `find_mountpoint::Error` that's a sum type over the error
types returned by the calls I make, which was presented as idiomatic by the
Rust book.

Although the macOS version makes unsafe calls, I've tried to keep it all in a
single critical section to minimize the amount of weirdness that can happen.
Because one of my goals was to minimize memory overhead, I thought about using
a static variable for the buffer passed to `statfs(2)`, but then realized that
would make it unsafe for use in threaded code or with Tokio, so each call
results in a new allocation for a `statfs` structure.

Since developing the macOS version, I've written a (slower) version that
doesn't rely on libc and should work on all other variants of UNIX.  There's
also an even simpler version for Windows, which passes its (pretty simple)
tests on AppVeyor, so maybe it will work for you, friendly Windows developer,
as well. Sample code for the API I'm using on Windows is [thin on the
ground][lol].

That said, It took me less than half an hour to get the project up and tests
passing on AppVeyor, thanks to [this configuration][appveyor-rust] and
`rustup`. It's pretty impressive how quickly everything came together.  Thanks,
Rust!  Thust!

This is my first published Rust crate, so I'm sure even this short chunk of
code has problems that I would love help in fixing. There aren't a lot of
examples out there of [`libc::statfs`][statfs] in use, and pretty much
everybody does some variant of what I do, but bugs and PRs are welcome.

## justification

Dealing with `libc` calls is fiddly (`&str` → `Path` → `OsStr` → `CStr` and
back again, just for strings!). nix doesn't expose the field I want in its
`Statfs` structure. None of the filesystem crates I could find exposed this
function. If somebody wants to assimilate this into their library, or work with
me to get this function merged into their crate (including nix), that would be
fantastic!

Until then, I needed this for my own nefarious purposes, so here it is.

[appveyor-rust]: https://github.com/starkat99/appveyor-rust/
[docs]: https://docs.rs/find_mountpoint
[lol]: https://github.com/search?utf8=%E2%9C%93&q=PrefixComponent+extension%3Ars&type=Code&ref=advsearch&l=&l=
[statfs]: https://doc.rust-lang.org/libc/x86_64-apple-darwin/libc/fn.statfs.html
