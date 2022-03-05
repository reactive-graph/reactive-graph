# Plugin: System Command

This plugin enables system commands to be executed. This is useful in home automation applications, e.g. if you want to
control a headless Raspberry Pi Zero.

It can be configured in which directory the system command is executed.

A system command is executed by triggering (i.e. changing) the spawn property. Please see the GraphQL example.

When a program runs, it writes to stdout and stderr. These outputs are available in the corresponding properties after
the program has ended.

```admonish warning "A word about safety"
Only activate the plugin if you know what you are doing. Do not run the Reactive Graph Flow as root user, because the system command will be executed as the same user! The GraphQL endpoint should also be specially protected.
```

## Entity Types

| Name          | Property    | Data Type | Socket Type |
|---------------|-------------|-----------|-------------|
| SystemCommand | name        | string    | none        |
|               | current_dir | string    | none        |
|               | command     | string    | none        |
|               | spawn       | array     | input       |
|               | stdin       | string    | input       |
|               | stdout      | string    | output      |
|               | stderr      | string    | output      |

## Usage

### GraphQL: Create System Command "List Current Directory"

The system command is only generated but not yet executed.

```graphql
mutation {
  instances {
    entities {
      create(
        type: "system_command",
        id: "57cd91ba-b437-4ba9-b274-b5e1ad4abbe5",
        properties: [
          {
            name: "name",
            value: "List files in the current directory"
          },
          {
            name: "current_dir",
            value: "."
          },
          {
            name: "command",
            value: "ls"
          },
          {
            name: "spawn",
            value: []
          },
          {
            name: "stdin",
            value: ""
          },
          {
            name: "stdout",
            value: ""
          },
          {
            name: "stderr",
            value: ""
          }
        ]
      ) {
        id,
        type {name},
        properties(
          names: [
            "name",
            "current_dir",
            "command",
            "spawn",
            "stdin",
            "stdout",
            "stderr"
          ]
        ) {
          name
          value
        }
      }
    }
  }
}
```

### GraphQL: Execute System Command "List Current Directory"

A system command is executed when the property `spawn` is changed. This property expects an array with arguments. If the
command is to be executed without arguments, an empty array can be passed.

```graphql
mutation {
  instances {
    entities {
      update(
        id: "57cd91ba-b437-4ba9-b274-b5e1ad4abbe5",
        properties: [
          {
            name: "spawn",
            value: [
              "-l",
              "-a"
            ]
          }
        ]
      ) {
        id,
        type {name},
        properties(
          names: [
            "name",
            "current_dir",
            "command",
            "spawn",
            "stdin",
            "stdout",
            "stderr"
          ]
        ) {
          name
          value
        }
      }
    }
  }
}
```

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Repository

| Name                             | Repository                                                                                                                       |
|----------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-system-command | [https://github.com/aschaeffer/inexor-rgf-plugin-system-command](https://github.com/aschaeffer/inexor-rgf-plugin-system-command) |
