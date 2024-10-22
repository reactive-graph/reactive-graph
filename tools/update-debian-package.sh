#!/bin/bash
cargo +stable build --release
echo "Generate zsh completions"
target/release/reactive-graph shell-completions print zsh > crates/reactive-graph/debian/usr/share/zsh/functions/Completion/Base/_reactive-graph
target/release/reactive-graph-client shell-completions print zsh > crates/reactive-graph/debian/usr/share/zsh/functions/Completion/Base/_reactive-graph-client
target/release/reactive-graph-server shell-completions print zsh > crates/reactive-graph/debian/usr/share/zsh/functions/Completion/Base/_reactive-graph-server
target/release/reactive-graph-tooling shell-completions print zsh > crates/reactive-graph/debian/usr/share/zsh/functions/Completion/Base/_reactive-graph-tooling
echo "Generate bash completions"
target/release/reactive-graph shell-completions print bash > crates/reactive-graph/debian/usr/share/bash-completion/completions/reactive-graph
target/release/reactive-graph-client shell-completions print bash > crates/reactive-graph/debian/usr/share/bash-completion/completions/reactive-graph-client
target/release/reactive-graph-server shell-completions print bash > crates/reactive-graph/debian/usr/share/bash-completion/completions/reactive-graph-server
target/release/reactive-graph-tooling shell-completions print bash > crates/reactive-graph/debian/usr/share/bash-completion/completions/reactive-graph-tooling
echo "Generate man pages"
target/release/reactive-graph man-pages print > crates/reactive-graph/debian/usr/share/man/man1/reactive-graph.1
target/release/reactive-graph-client man-pages print > crates/reactive-graph/debian/usr/share/man/man1/reactive-graph-client.1
target/release/reactive-graph-server man-pages print > crates/reactive-graph/debian/usr/share/man/man1/reactive-graph-server.1
target/release/reactive-graph-tooling man-pages print > crates/reactive-graph/debian/usr/share/man/man1/reactive-graph-tooling.1
