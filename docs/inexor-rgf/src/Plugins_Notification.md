# Plugin: Notification

Create desktop notifications.

## Entity Types

| Name                | Property | Data Type | Socket Type |
|---------------------|----------|-----------|-------------|
| DesktopNotification | show     | bool      | input       |
|                     | app_name | string    | input       |
|                     | summary  | string    | input       |
|                     | body     | string    | input       |
|                     | icon     | string    | input       |
|                     | timeout  | number    | input       |

## Platform Compatibility

| Platform | Compatibility | Notes                                                  |
|----------|---------------|--------------------------------------------------------|
| Linux    | ✓             | https://github.com/hoodie/notify-rust#linuxbsd-support |
| MacOS    | ✓             | https://github.com/hoodie/notify-rust#macos-support    |
| Windows  | ✓             | https://github.com/hoodie/notify-rust#windows-support  |

## Repository

| Name                           | Repository                                                                                                                   |
|--------------------------------|------------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-notification | [https://github.com/inexorgame/inexor-rgf-plugin-notification](https://github.com/inexorgame/inexor-rgf-plugin-notification) |

## Usage

### GraphQL

#### Create Desktop Notification

```graphql
mutation {
  instances {
    entities {
      create(
        type: "desktop_notification",
        id: "dfe30808-9242-4af8-aced-556ffe617038",
        properties: [
          {
            name: "show",
            value: false
          },
          {
            name: "body",
            value: "Test"
          }
        ]
      ) {
        id
        type {
          name
        }
        properties(
          names: [
            "show",
            "app_name",
            "summary",
            "body",
            "icon",
            "timeout"
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

#### Show Desktop Notification

The property `show=true` triggers the notification.

```graphql
mutation {
  instances {
    entities {
      update(
        id: "dfe30808-9242-4af8-aced-556ffe617038",
        properties: [
          {
            name: "show",
            value: true
          },
          {
            name: "summary",
            value: "Important Message"
          },
          {
            name: "body",
            value: "Lorem Ipsum"
          },
          {
            name: "icon",
            value: "computer"
          },
          {
            name: "timeout",
            value: 1000
          }
        ]
      ) {
        id
        type {
          name
        }
        properties(
          names: [
            "show",
            "app_name",
            "summary",
            "body",
            "icon",
            "timeout"
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
