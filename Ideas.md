# Ideas

- print is configurable with multiple cli flags
  - e.g. should show file paths
  - each file should be a list
- use logging

# Next Steps

- use .todocoignore @done
- use multiple lists
- use (console)[https://docs.rs/console/0.7.5/console/] crate
- implement TaskPaper todo format / TaskPaper Export @done

# App Integrations

## Things

Unfortunately it isn't as simple as I thought to use x-callback-url Scheme with Rust. To get reqwest as far as it could parse the x-callback Scheme Uri one has to use it in the following form `things://x-callback-url/show?id=today`. With the normal `things:///show?id=today` form it is no valid Uri. The next problem was, that reqwest said it found `x-callback-url` no matching server- or nodename. A working draft was to use the macos `open` command for these calls, but then it is not possible to get the callback information, which I need to get the created Todo Ids in Things. A possible solution could be, to build a rust wrapper around a small swift library to call these x-callback-urls.
