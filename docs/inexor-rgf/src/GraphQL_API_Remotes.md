# <center>GraphQL API: Remotes</center>

{{ graphql_playground(config="/examples/graphql/system/remotes/tabs.json") }}

## Functionality

- List Remotes
- Add Remote
- Remove Remote
- Update Remote Self-Information
  - Updates the `lastSeen` property
- Fetch Remotes from another Remote
  - Queries the remotes of another remote and adds them to the own list of remotes (except duplicate and offline remotes)
