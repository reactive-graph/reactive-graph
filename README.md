# inexor-rgf-shared-bidule

Inexor RGF - Shared Modules - Bidule

Fork of https://github.com/phaazon/bidule

# bidule, a Rust FRP crate

**bidule** is a very simple FRP library built over functional concepts. The basic, most core concept
is the `Stream<Sig>`, which is a *stream of typed signals*. A stream of signals will get a *signal*
as input and will broadcast it downwards. You can compose streams with each other with very simple
combinators, such as `map`, `filter`, `filter_map`, `zip`, `unzip`, `merge`, `fold`, `sink`, etc.

**bidule** is intended to be used directly as-is and can be the starting point of any higher
abstracted FRP-driven programming (e.g. video game, GUI, animation, etc.).

Feel free to have a look at the documentation for a better understanding on how everything composes
and work.
