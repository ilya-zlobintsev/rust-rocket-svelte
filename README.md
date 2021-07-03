# rust-rocket-svelte
This example application demonstrates how to use Svelte with a Rust web app using Rocket.

Requirements: rust and npm.

You can run the app with just `cargo run`: there is a build script that calls npm to generate the JS app.

When built in debug mode (default), rollup (the JS bundler) will watch for changes and rebuild the app in real time. In release mode, the JS is bundled compile-time and additionally minified.

This is also not particularly Rocket-specific, this should work with any web framework that supports serving static files.

I recommend using `#` instead of `/` for routes in svelte, that way you can create additional routes in Rust without any conflicts.
