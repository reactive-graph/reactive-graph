# Generators and Actions

## What is a generator?

A [generator](./Plugins_Logical.md) produces boolean `true`s and **sends** them via the `output` property `trigger`.

There are many types of generators:

- A periodic timer
- A scheduler
- A changed file on the filesystem (fs_notify)
- A random bool generator
- An user which has pressed a button on the user interface
- A physical event from an input device like a keystroke

## What is a action?

A [action](./Plugins_Logical.md) is something which should be executed or processed _only and only if_ the `input`
property `trigger` **receives** a boolean `true`.

## Combine triggers and actions within a single entity using components

An entity instance can have multiple components.

For example, you can combine `file` with `fs_notify` and `load_binary`.

By doing so, the `trigger` property is both:

- A `generator`: The component `fs_notify` fires `trigger` when the file has been modified on the filesystem
- A `action`: The component `load_binary` gets triggered immediately and loads the content of the file into a property

This example shows the great possibilities of the component system and how to use the `generator`-`action`-system. By
combining components into one single entity instance, the behaviour of the entity instance can get more complex. At
the same time a `flow` is more compact.

Another example is to combine a `user-interface-button` as `generator` with an `update-the-asset-repository` as
an action.
