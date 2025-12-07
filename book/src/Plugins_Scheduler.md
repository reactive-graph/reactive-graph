# Plugin: Scheduler

Timers and scheduled jobs

## Scheduled Jobs

A `scheduled job` triggers periodically using a cron expression.

### Cron Expression

Comma separated values such as `5,8,10` represent more than one time value. So for example, a schedule of
`0 2,14,26 * * * *` would execute on the 2nd, 14th, and 26th minute of every hour.

Ranges can be specified with a dash. A schedule of `0 0 * 5-10 * *` would execute once per hour but only
on day 5 through 10 of the month.

Day of the week can be specified as an abbreviation or the full name. A schedule of `0 0 6 * * Sun,Sat`
would execute at 6am on Sunday and Saturday.

Note that the year may be omitted.

#### Examples

| Cron Expression     | sec | min     | hour | day  of month | month | day of week | year | Description                                                 |
|---------------------|-----|---------|------|---------------|-------|-------------|------|-------------------------------------------------------------|
| `* * * * * * *`     | *   | *       | *    | *             | *     | *           | *    | Runs every second                                           |
| `0 2,14,26 * * * *` | 0   | 2,14,26 | *    | *             | *     | *           |      | Run once per hour but only on day 5 through 10 of the month |
| `0 0 * 5-10 * *`    | 0   | 0       | *    | 5-10          | *     | *           |      | Run at 6am on Sunday and Saturday                           |

## Timers

A `timer` triggers after a given duration. The duration can be specified either as a number in
milliseconds or as a string in [ISO 8601 Duration Format](https://en.wikipedia.org/wiki/ISO_8601#Durations).

### ISO 8601 Duration Format

> [!NOTE]
> What is the ISO 8601 Duration Format?
>
> Durations define the amount of intervening time in a time interval and are represented by the format
> `P[n]Y[n]M[n]DT[n]H[n]M[n]S` or `P[n]W`. In these representations, the `[n]` is replaced by the value
> for each of the date and time elements that follow the `[n]`. Leading zeros are not required, but the
> maximum number of digits for each element should be agreed to by the communicating parties. The
> capital letters `P`, `Y`, `M`, `W`, `D`, `T`, `H`, `M`, and `S` are designators for each of the date
> and time elements and are not replaced.
>
> [https://en.wikipedia.org/wiki/ISO_8601#Durations](https://en.wikipedia.org/wiki/ISO_8601#Durations)

#### Designators

| Designator | Description                                                                                     |
|------------|-------------------------------------------------------------------------------------------------|
| `P`        | `P` is the duration designator (for period) placed at the start of the duration representation. |
| `Y`        | `Y` is the year designator that follows the value for the number of years.                      |
| `M`        | `M` is the month designator that follows the value for the number of months.                    |
| `W`        | `W` is the week designator that follows the value for the number of weeks.                      |
| `D`        | `D` is the day designator that follows the value for the number of days.                        |
| `T`        | `T` is the time designator that precedes the time components of the representation.             |
| `H`        | `H` is the hour designator that follows the value for the number of hours.                      |
| `M`        | `M` is the minute designator that follows the value for the number of minutes.                  |
| `S`        | `S` is the second designator that follows the value for the number of seconds.                  |

#### Examples

| Duration           | Description                                                                                                  |
|--------------------|--------------------------------------------------------------------------------------------------------------|
| `PT10S`            | Represents a duration of ten seconds.                                                                        |
| `PT1M30S`          | Represents a duration of one minute and thirty seconds.                                                      |
| `PT1H`             | Represents a duration of one hour.                                                                           |
| `P1DT12H`          | Represents a duration of one day and twelve hours.                                                           |
| `P3Y6M4DT12H30M5S` | Represents a duration of three years, six months, four days, twelve hours, thirty minutes, and five seconds. |

## Entity Types

| Name          | Component | Property | Data Type              | Socket Type | Description                                            |
|---------------|-----------|----------|------------------------|-------------|--------------------------------------------------------|
|               |
| scheduled_job |           | schedule | string                 | input       | Cron Expression                                        |
|               | generator | trigger  | bool                   | output      |                                                        |
|               |
| timer         |           | duration | number<br>or<br>string | input       | Duration in milliseconds or as ISO8601 Duration Format |
|               | generator | trigger  | bool                   | output      |                                                        |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repositories

| Name                            | Repository                                                                                                                                               |
|---------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
| reactive-graph-plugin-scheduler | [https://github.com/reactive-graph/plugins-core/tree/main/plugins/scheduler](https://github.com/reactive-graph/plugins-core/tree/main/plugins/scheduler) |
| tokio-cron-scheduler            | [https://github.com/mvniekerk/tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler)                                                   |
| cron expression parser          | [https://github.com/zslayton/cron](https://github.com/zslayton/cron)                                                                                     |
| ISO8601 duration parser         | [https://github.com/PoiScript/iso8601-duration](https://github.com/PoiScript/iso8601-duration)                                                           |

## Usage

{{ graphql_playground(config="/examples/graphql/plugins/scheduler/tabs.json") }}

## Usage

### GraphQL: Create a new timer

```graphql
mutation {
  instances {
    entities {
      create(
        type: "timer",
        id: "46e2ecd0-3e91-4205-99c9-d9543923a73a",
        properties: [
          {
            name: "duration",
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
            "duration",
            "trigger"
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

### GraphQL: Update duration of an existing timer

```graphql
mutation {
  instances {
    entities {
      update(
        id: "46e2ecd0-3e91-4205-99c9-d9543923a73a",
        properties: [
          {
            name: "duration",
            value: 1500
          }
        ]
      ) {
        id
        type {
          name
        }
        properties(
          names: [
            "duration",
            "trigger"
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
