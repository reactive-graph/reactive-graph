# Plugins

Inexor Reactive Graph Flow provides a plugin system. The philosophy is to have a small core which can be extended
by plugins.

The core application provides:

* Graph Database
* Management of types and relations
* Plugin System
* HTTP-Server with GraphQL-Resolvers

Plugins can provide:

* Components using a `ComponentProvider`
* Entity Types using an `EntityTypeProvider`
* Relation Types using a `RelationTypeProvider`
* Entity Behaviour using an `EntityBehaviourProvider`
* Relation Behaviour using a `RelationBehaviourProvider`
* Flows using a `FlowProvider`
* Web resource using a `WebResourcesProvider`

## Linkage

* https://doc.rust-lang.org/reference/linkage.html

## Compile and configure plugins

1. Checkout and build the plugin
    ```shell
    cd ..
    git clone https://github.com/aschaeffer/inexor-rgf-plugin-mqtt.git
    cd inexor-rgf-plugin-mqtt
    cargo build (or cargo build --release)
    ```
2. Edit `config/plugins.toml` and add a section for the plugin. The name must match the
   crate name of the plugin. Specify the path to the dynamically linked library. The path
   can be either absolute or relative to the working directory of the application.

    ```toml
    [[plugin]]
    name = "inexor-rgf-plugin-mqtt"
    active = true
    path = "../inexor-rgf-plugin-mqtt/target/debug/libinexor_rgf_plugin_mqtt.so"
    ```

## Roadmap

### Plugin Context

-[x] A plugin can create entity/relation instances by itself

### Numeric

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-numeric | https://github.com/aschaeffer/inexor-rgf-plugin-numeric |

- [x] Extend the numeric plugin, so that entity instances are created which contains
      numeric constants (pi, eulers number, ...)

#### Rust Crate / Rust Reference

* Math Constants: https://doc.rust-lang.org/std/f64/consts/index.html

### Config

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-config | https://github.com/aschaeffer/inexor-rgf-plugin-config |

Reads TOML config files from the location defined in property `filename`. The goal is
that flows which uses an entity of type `ConfigFile` can be configured easily. The
TOML file defined in the property `filename` gets read from disk and the data structure
gets stored in the property `value` as an object.

The property `value` is an output socket and can be used for anything you want in your
flow.

The config can be used to allow customization of flows implementing
* game modes
* maps

---
**Example:**
A flow which controls lights based on a scheduler the cron expressions can be defined
in a config file.
---

---
**Note:**
The UUIDs of the entity instances have to be stable for the name of a config file.
This allows to create flows which uses these config files by name.
---

#### Entity Types

| Name | Properties | Data Type | Socket Type |
| --- | --- | --- | --- |
| ConfigFile | filename | string | none |
| | value | object | output |

#### Entity Behaviours

| Name | Description |
| --- | --- |
| ConfigFileBehaviour | On every change of the property `filename` the configuration file gets loaded. Automatically updates the entity instance if the TOML file has changed on disk.|

### System Environment

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-system-environment | https://github.com/aschaeffer/inexor-rgf-plugin-system-environment |

The plugin creates entity instances for each environment variable. As environment variables doesn't change
at runtime this happens only at initialization.

---
**Note:**
The UUIDs of the entity instances have to be stable for the name of an environment variable.
This allows to create flows which uses these environment variables by name.
---

#### Entity Types

| Name | Properties | Data Type | Socket Type
| --- | --- | --- | --- |
| EnvVar | name | string | none |
| | value | string | output |

### Raw Keyboard

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-raw-keyboard | https://github.com/aschaeffer/inexor-rgf-plugin-raw-keyboard |

This plugin queries mouse and keyboard inputs on demand without a window.

#### Use cases

* Global Hotkeys
  * Running basic functions (server list, chat, ...) in background
  * No window have to be open or minimized
* Home Automation
  * Use a real keyboard (for example numpad keyboards) as interface to control flows
* Robotics
  * Control your robot with keys or a 3d mouse

#### Rust Crate / Rust Reference

https://crates.io/crates/device_query

#### Entity Types

| Name | Property | Data Type | Socket Type |
| --- | --- | --- | --- |
| RawKey | keycode | number | none |
| | keydown | bool | output |

#### Entity Behaviours

| Name | Description |
| --- | --- |
| RawKeyBehaviour | Filters incoming key events by the keycode defined in the property `keycode`. Sets the output property `keydown` to true or false |

### String Operations inexor-rgf-plugins-string

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-string | https://github.com/aschaeffer/inexor-rgf-plugin-string |

#### Rust Crate / Rust Reference

* https://doc.rust-lang.org/std/string/struct.String.html

#### Component

| Name | Property | Data Type | Socket Type |
| --- | --- | --- | --- |
| StringOperation | lhs | string | input |
|  | result | string | output |
| StringGate | lhs | string | input |
|  | rhs | string | input |
|  | result | string | output |

#### Entity Types / Behaviours

| Name | Component | Description |
| --- | --- | --- |
| Trim | StringOperation | Removes whitespace at the beginning and end of a string |
| Uppercase |
| Lowercase |
| ... |

### Generators and Random Numbers

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-generators | https://github.com/aschaeffer/inexor-rgf-plugin-generators |

Time-based generators and random number generators are very useful.


#### Use cases

* Entity types which needs a activation
* Random behaviour in game modes (for example team size)
* Random behaviour in maps (for example colors)
* Variance in texture manipulation
* ...

#### Rust Crate / Rust Reference

* https://docs.rs/rand/0.5.0/rand/prng/index.html

#### Entity Types

| Name | Properties | Behaviours | Description |
| --- | --- | --- | --- |
| Metronom | millis (number)<br>result (bool) | MetronomBehaviour | Every X millis the boolean result toggles |
| Counter | millis (number)<br>step (number)<br>count (number) | EpochTimestampBehaviour | Every X millis the output property `count` gets increased by the value of the input property `step`1 |
| EpochTimestamp | timestamp | EpochTimestampBehaviour | Every second the output property `timestamp` gets updated. |
| RandomNumber | min (number)<br>max (number)<br>activation (any)<br>random (number) | RandomNumberBehaviour | Each time the property `activation` changes a new random number gets generated and stored in the output property `random` |
| PseudoRandomNumber | seed (any)<br>random (number) | RandomNumberBehaviour | Each time the property `activation` changes a new random number gets generated and stored in the output property `random` |

### Scheduler and Timer

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-scheduler | https://github.com/aschaeffer/inexor-rgf-plugin-scheduler |

This plugin provides a scheduler which starts tasks on a regular basis.

#### Use cases

* Update the server list every X minutes
* Reindex the texture directory every day at 04:00 am
* Start a game server at 08:00 h and shutdown it at 20:00 h because of the opening hours of the internet

#### Entity Types

| Name | Properties | Behaviours | Description |
| --- | --- | --- | --- |
| Cron Expression | expression (string)<br>activation (bool) | A cron based scheduler propagates the activation output (true) |

### Math Expression

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-math-expression | https://github.com/aschaeffer/inexor-rgf-plugin-math-expression |

#### Entity Types

| Name | Properties | Behaviours | Description |
| --- | --- | --- | --- |
| Expression | expression (string)<br>variables (object)<br>result (number> | ExpressionBehaviour | A mathematical expression gets evaluated each time the input property `variables` changes. |

### JSON Objects and JSON Arrays

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-json | https://github.com/aschaeffer/inexor-rgf-plugin-json |

This plugin adds functionality to operate with complex data structures. Properties of entity instances or
relation instances can have different data types. It's possible to store even complex data using the data
types array and object. This is handy if you receive data from an MQTT endpoint or if you want to represent
more complex data. But it makes it also necessary to unpack or pack these data in order to operate with it.

#### Entity Types

| Name | Property | Data Type | Socket Type |
| --- | --- | --- | --- |
| |
| ArrayPush | array | array | input |
| | value | any | input |
| | result | array | output |
| |
| ArrayPop | array | array | input |
| | result | array | output |
| | value | any | input |
| |
| ObjectInsert | object | object | input |
| | property | string | input |
| | value | any | input |
| | result | object | output |
| |
| ObjectRemove | object | object | input |
| | property | string | input |
| | result | object | output |
| | value | any | output |

### Sound

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-sound | https://github.com/aschaeffer/inexor-rgf-plugin-sound |

#### Rust Crate / Rust Reference

* https://github.com/RustAudio/cpal
* https://github.com/RustAudio/rodio (MP3, WAV, Vorbis, Flac, MP4, AAC)

#### Entity Types

* Sound Source
* Sound Playback
* TODO

### Color

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-color | https://github.com/aschaeffer/inexor-rgf-plugin-color |

#### Components

* color_rgb
* color_rgba
* color_cmyk

#### Entity Types

* TO_RGB
* TO_RGBA
* TO_CMYK

### Texture

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-texture | https://github.com/aschaeffer/inexor-rgf-plugin-texture |

* GLTF Buffer (`data:application/gltf-buffer;base64,`)
* https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_013_SimpleTexture.md

### Texture Processing

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-texture-processing | https://github.com/aschaeffer/inexor-rgf-plugin-texture-processing |

* Color Filter
* Blend Filter
* Noise Generation

### Noise Generation

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-noise-generation | https://github.com/aschaeffer/inexor-rgf-plugin-noise-generation |

#### Use Cases

* Texture Processing / Procedural Texture Generation
* Terrain Generation
* Particle System

#### Rust Crate / Rust Reference

* https://docs.rs/noise/0.7.0/noise/
* https://github.com/Razaekel/noise-rs

### Scripting

| Name | Repository |
| --- | --- |
| inexor-rgf-plugin-scripting | https://github.com/aschaeffer/inexor-rgf-plugin-scripting |

This plugin provides the possibility to run scripts.

Deno is a simple, modern and secure runtime for JavaScript and TypeScript that uses V8 and is built in Rust.

#### Rust Crate / Rust Reference

* https://deno.land/
* https://github.com/denoland/rusty_v8
* https://crates.io/crates/deno_core
* https://docs.rs/deno_core/0.99.0/deno_core/
* https://github.com/inexorgame-obsolete/entity-system-inactive/issues/129

#### Entity Types

| Name | Property | Data Type | Socket Type |
| --- | --- | --- | --- |
| |
| Script | filename | string | none |
| | script | string | output |
| |
| ExecuteScript | script | string | input |
| | input | object | input |
| | result | object | output |
| | activation | bool | none |

#### Relation Types

| Source Entity Type | Relation Type Name | Target Entity Type |
| --- | --- | --- |
| Script | -- LoadsScript --> | ExecuteScript |

#### Entity Behaviour

| Name | Description |
| --- | --- |
| Script | Load the script from `filename` |
| ExecuteScript | Executes the script if either `activation` or `input` gets triggered. |

### Physics 

| Name | Repository |
| --- | --- |
| inexor-rgf-plugins-physics | https://github.com/aschaeffer/inexor-rgf-plugin-physics |

#### Components

| Name | Properties | Behaviours |
| --- | --- | --- |
| position | x<br>y<br>z | - |
| velocity | vx<br>vy<br>vz | VelocityTransformation |

### Game-Server inexor-rgf-plugins-game-server

#### Entity Types

| Name | Properties | Behaviours |
| --- | --- | --- |
| game_server | hostname<br>port<br>name<br>owner<br>description<br>game_mode_name<br>map_name<br>number_of_players<br> | - |
