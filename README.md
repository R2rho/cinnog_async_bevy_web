- `cargo make watch` starts the Bevy app with watch mode and hot-reload enabled. Use this if you're trying it out!

- `cargo make build` builds the project in release. The output will be in the `dist` directory and the command will not serve it, but quit instead.
- `cargo make fmt` formats with `rustfmt` and `leptosfmt`.
- `cargo make e2e` runs the end-to-end tests from the `end2end` directory using [Playwright].