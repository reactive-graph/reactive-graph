# Plugin: Taxonomy

This plugin provides a generic [taxonomy](https://en.wikipedia.org/wiki/Taxonomy) system with categories and tags.

## Use Cases

* Categorization of assets
* Categorization of servers
* Categorization of teams

## Categories and Tagging

[![Taxonomy](images/plugin_taxonomy.png)](images/plugin_taxonomy.png)

## Components

| Name     | Properties | DataType | SocketType | Description                                                                   |
|----------|------------|----------|------------|-------------------------------------------------------------------------------|
|          |
| weighted | weight     | number   | none       | The weight of a relation between two entity instances (Range from 0.0 to 1.0) |

## Entity Types

| Name     | Components  | Properties  | DataType | SocketType | Description                     |
|----------|-------------|-------------|----------|------------|---------------------------------|
|          |
| category | named       | name        | string   | none       | The name of the category        |
|          | describable | description | string   | none       | The description of the category |
|          |
| tag      | named       | name        | string   | none       | The tag name                    |
|          |

## Relation Types

| Name            | Description | Components | Source Entity Type | Target Entity Type |
|-----------------|-------------|------------|--------------------|--------------------|
| categorized_as  |             | weighted   | *                  | category           |
| has_subcategory |             | weighted   | category           | category           |
| tagged_with     |             | weighted   | *                  | tag                |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repositories

| Name                       | Repository                                                                                                                                             |
|----------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-taxonomy | [https://github.com/reactive-graph/plugins-core/tree/main/plugins/taxonomy](https://github.com/reactive-graph/plugins-core/tree/main/plugins/taxonomy) |

## Usage

{{ graphql_playground(config="/examples/graphql/plugins/taxonomy/categories/tabs.json") }}

{{ graphql_playground(config="/examples/graphql/plugins/taxonomy/tags/tabs.json") }}
