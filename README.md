# Remote Egui

This repo is an example of an egui app that can be run natively,as well as compiled to a wasm frontend that communicates with a host application over websockets.

![remote](https://github.com/matthewjberger/remote-egui/assets/7131091/68f7c67f-9dd0-4b3d-bfdc-a0358f9586e8)

## Local

Run the app natively:

```
cargo run --release
```

## Remote

Run the websocket server app (native, on host):

```
cd server
cargo run -r -- 127.0.0.1:12345
```

From the root of the repo, compile and serve the app to wasm. This includes a websocket client.

```
trunk serve
```

Then if you go to a web browser and visit the url trunk serves (make sure to append `#dev` to the url), you can enter `127.0.0.1:12345` and hit `Enter` to connect. You should see the server accept the connection. Any text sent will be echoed back by the server and show up as a received event in the wasm frontend.

Now you have a egui frontend running in wasm in the browser, connecting to a program on the host over a websocket connection :rocket:

## Details

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Local

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

Use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web Deploy
1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website
3. Upload the `dist` directory. Serve the `dist` directory as a static site.
