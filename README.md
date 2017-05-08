# find_mountpoint

The documentation is [here][docs].

This is a simple crate that exists solely to simplify getting at one field
returned from `statfs(2)`. It takes care of all of the unsafe stuff and
reflects my best understanding as a Rust newbie of how to deal with `libc`
memory management efficiently.

Its error handling is pretty conservative. It doesn't use `.unwrap()` and
should therefore never panic unless you fail to handle the `Result` in your own
code. I created a new `find_mountpoint::Error` that's a sum type over the error
types returned by the calls I make, which was presented as idiomatic by the
Rust book.

Although it calls unsafe code, I've tried to keep it all in a single critical
section to minimize the amount of weirdness that can happen. Because one of my
goals was to minimize memory overhead, I thought about using a static variable
for the buffer passed to `statfs(2)`, but then realized that would make it
unsafe for use in threaded code or with Tokio, so each call results in a new
allocation for a `statfs` structure.

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

## limitations

This was developed for my own use on macOS. It uses `OsStrExt` and other
features that are explicitly tied to macOS and Linux. If somebody wants to send
me a PR with an alternative implementation / an extension of this that works
for Windows, I'll happily work with you to get it added.

[docs]: http://docs.rs/find_mountpoint
[statfs]: https://doc.rust-lang.org/libc/x86_64-apple-darwin/libc/fn.statfs.html
