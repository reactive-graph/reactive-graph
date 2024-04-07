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
    git clone https://github.com/reactive-graph/plugin-mqtt.git
    cd plugin-mqtt
    cargo build (or cargo build --release)
    ```
2. Edit `config/plugins.toml` and add a section for the plugin. The name must match the
   crate name of the plugin. Specify the path to the dynamically linked library. The path
   can be either absolute or relative to the working directory of the application.

    ```toml
    [[plugin]]
    name = "plugin-mqtt"
    active = true
    path = "../plugin-mqtt/target/debug/libinexor_rgf_plugin_mqtt.so"
    ```

## Roadmap

### Plugin Context

-[x] A plugin can create entity/relation instances by itself

### Scheduler and Timer

| Name                        | Repository                                                    |
|-----------------------------|---------------------------------------------------------------|
| inexor-rgf-plugin-scheduler | https://github.com/reactive-graph/inexor-rgf-plugin-scheduler |

This plugin provides a scheduler which starts tasks on a regular basis.

#### Use cases

* Update the server list every X minutes
* Reindex the texture directory every day at 04:00 am
* Start a game server at 08:00 h and shutdown it at 20:00 h because of the opening hours of the internet

#### Entity Types

| Name            | Properties                               | Behaviours                                                     | Description |
|-----------------|------------------------------------------|----------------------------------------------------------------|-------------|
| Cron Expression | expression (string)<br>activation (bool) | A cron based scheduler propagates the activation output (true) |             |

### Audio

| Name                    | Repository                                                |
|-------------------------|-----------------------------------------------------------|
| inexor-rgf-plugin-audio | https://github.com/reactive-graph/inexor-rgf-plugin-audio |

#### Rust Crate / Rust Reference

* https://github.com/RustAudio/cpal
* https://github.com/RustAudio/rodio (MP3, WAV, Vorbis, Flac, MP4, AAC)

#### Entity Types

* Sound Source
* Sound Playback
* TODO

### Color

| Name                       | Repository                                                   |
|----------------------------|--------------------------------------------------------------|
| inexor-rgf-plugin-graphics | https://github.com/reactive-graph/inexor-rgf-plugin-graphics |

#### Components

* color_rgb
* color_rgba
* color_cmyk
* image (BASE64)

#### Entity Types

* TO_RGB
* TO_RGBA
* TO_CMYK

### Texture

| Name                      | Repository                                                  |
|---------------------------|-------------------------------------------------------------|
| inexor-rgf-plugin-texture | https://github.com/reactive-graph/inexor-rgf-plugin-texture |

* GLTF Buffer (`data:application/gltf-buffer;base64,`)
* https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_013_SimpleTexture.md

### Texture Processing

| Name                                 | Repository                                                             |
|--------------------------------------|------------------------------------------------------------------------|
| inexor-rgf-plugin-texture-processing | https://github.com/reactive-graph/inexor-rgf-plugin-texture-processing |

* Color Filter
* Blend Filter
* Noise Generation

### Noise Generation

| Name                               | Repository                                                           |
|------------------------------------|----------------------------------------------------------------------|
| inexor-rgf-plugin-noise-generation | https://github.com/reactive-graph/inexor-rgf-plugin-noise-generation |

#### Use Cases

* Texture Processing / Procedural Texture Generation
* Terrain Generation
* Particle System

#### Rust Crate / Rust Reference

* https://docs.rs/noise/0.7.0/noise/
* https://github.com/Razaekel/noise-rs

### WebAssembly / Scripting

| Name                   | Repository                                               |
|------------------------|----------------------------------------------------------|
| inexor-rgf-plugin-wasm | https://github.com/reactive-graph/inexor-rgf-plugin-wasm |

This plugin provides the possibility to run scripts.

Deno is a simple, modern and secure runtime for JavaScript and TypeScript that uses V8 and is built in Rust.

#### Rust Crate / Rust Reference

* https://deno.land/
* https://github.com/denoland/rusty_v8
* https://crates.io/crates/deno_core
* https://docs.rs/deno_core/0.99.0/deno_core/
* https://github.com/inexorgame-obsolete/entity-system-inactive/issues/129

#### Entity Types

| Name          | Property   | Data Type | Socket Type |
|---------------|------------|-----------|-------------|
|               |
| Script        | filename   | string    | none        |
|               | script     | string    | output      |
|               |
| ExecuteScript | script     | string    | input       |
|               | input      | object    | input       |
|               | result     | object    | output      |
|               | activation | bool      | none        |

#### Relation Types

| Source Entity Type | Relation Type Name | Target Entity Type |
|--------------------|--------------------|--------------------|
| Script             | -- LoadsScript --> | ExecuteScript      |

#### Entity Behaviour

| Name          | Description                                                           |
|---------------|-----------------------------------------------------------------------|
| Script        | Load the script from `filename`                                       |
| ExecuteScript | Executes the script if either `activation` or `input` gets triggered. |

### Physics

| Name                       | Repository                                                  |
|----------------------------|-------------------------------------------------------------|
| inexor-rgf-plugins-physics | https://github.com/reactive-graph/inexor-rgf-plugin-physics |

#### Components

| Name     | Properties     | Behaviours             |
|----------|----------------|------------------------|
| position | x<br>y<br>z    | -                      |
| velocity | vx<br>vy<br>vz | VelocityTransformation |

### Game-Server inexor-rgf-plugins-game-server

#### Entity Types

| Name        | Properties                                                                                              | Behaviours |
|-------------|---------------------------------------------------------------------------------------------------------|------------|
| game_server | hostname<br>port<br>name<br>owner<br>description<br>game_mode_name<br>map_name<br>number_of_players<br> | -          |
