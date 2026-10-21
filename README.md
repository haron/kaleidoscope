# webcam-kaleidoscope
## What is it?
Is it a party activity, a moving wallpaper or a simple screensaver... We're not sure.
### What's going on?
A web camera is observing something. 
It could be a table with small objects, your face or fingers, or anything else you want to show.
The application transforms this image in real-time like a true kaleidoscope and displays the results on the screen. 
Depending on the shaders used, it can rotate it quickly or slowly, change colors or make it shake.
Keyboard controls are used to manage the shaders.
One can use a standard keyboard and mouse or some mini-keyboard or numpad. 
We used this one: https://aliexpress.ru/item/1005006109748866.html
For others, it may be necessary to develop their own control module (see `src/controls`).
## What do you need?
You will need a system with v4l, a web camera, and a screen.
## How to configure?
In the `config` folder, create a `local.json` file. Then, you can modify the parameters in this file to change settings.
For example, if you want to run the application in fullscreen mode and use our mini keyboard, add the following code to the file:
```
{"fullscreen": true, "keyboard": "mini"}
```
## How to run?
```bash
cargo run
```
## How to deploy?
Copy `.env.example` to `.env` and set `HOST`. `make provision` once turns the box into a kiosk (boots into the app, restarts it on failure), then `make deploy`.
`make pad` flashes the 3 key + knob macropad (`macropad.yaml`): keys switch mesh, vertex shader, fragment shader; the knob zooms.
After `autopilot.idle` seconds without input, the app presses random keys/knob every `autopilot.period` seconds until touched. Effect switches crossfade over `crossfade` seconds (0 disables).
## Standard controls
### Keyboard controls

| Key            | Action                 |
|----------------|------------------------|
| `r`            | Start/stop rotation    |
| `m`            | Change mesh            |
| `←`, `→`       | Change fragment shader |
| `↑`, `↓`       | Change vertex shader   |
| `PgUp`, `PgDn` | Change camera height   |
| `ESC`          | Exit                   |
| `9`            | Slow down rotation     |
| `0`            | Speed up rotation      |
| `7`            | Speed up cycle         |
| `8`            | Slow down cycle        |

### Mouse wheel
Change camera height

## Mini keyboad controls
### Wheels

| Action        | Wheel1                | Wheel2             | Wheel3 (big)            |
|---------------|-----------------------|--------------------|-------------------------|
| anticlockwise | Slow down shaders     | Slow down rotation | Camera down             |
| press         | Default shaders speed | Stop rotation      | Default camera position |
| clockwise     | Speed up shaders      | Speed up rotation  | Camera up               |

### Keys

| Col1       | Col2               | Col3                 |
|------------|--------------------|----------------------|
| Next mesh  | Next vertex shader | Next fragment shader |
| default.vs | inner_rotation.vs  |                      |
| default.fs | test.fs            | rgb_corners.fs       |
| corners.fs | time.fs            | rainbow.fs           |

## Local configuration
To redefine config params put `local.json` file to `config` folder.
