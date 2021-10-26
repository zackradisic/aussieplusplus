# Go Playground WASM

This is an experiment in trying to make a version of play.golang.org that runs completely in the browser by 
compiling [goscript](https://github.com/oxfeeefeee/goscript) (by [oxfeeefeee](https://github.com/oxfeeefeee)) to WASM.

Most of Go's language features are supported, notably channels/goroutines/select.

## Building and running
The wasm binary is pre-compiled and available in the `public/` directory so you should be 
good to go to if you just want to run the Next.js server. You will need to install node and yarn of course.
```
# Run in development mode
yarn dev

# Build and serve optimized production build
yarn build
yarn start
```

If you want to play around with modifying the wasm build, `cd` into `goscript/wasm` and
you can mess around with the code. When you want to build the wasm binary run:
```
# Build wasm
cargo build --release --target=wasm32-wasi

# Copy into Next.js's public folder
cp target/wasm32-wasi/release/wasm.wasm ../public
```


### Disclaimer
goscript only makes guarantees that the syntax will be identical to Go's, there are implementation details that will cause discrepancies from running actual Go code with the actual Go compiler.

