# Plugin: Taxonomy

This plugin provides a generic [taxonomy](https://en.wikipedia.org/wiki/Taxonomy) system with categories and tags.

## Use Cases

* Categorization of assets
* Categorization of servers
* Categorization of teams

## Categories and Tagging

[![Taxonomy](images/plugin_taxonomy.png)](images/plugin_taxonomy.png)

[//]: # (![Taxonomy]&#40;images/plugin_taxonomy.png&#41;)

## Components

| Name      | Properties | DataType | SocketType | Description                                                                   |
|-----------|------------|----------|------------|-------------------------------------------------------------------------------|
|           |
| weighted  | weight     | number   | none       | The weight of a relation between two entity instances (Range from 0.0 to 1.0) |

## Entity Types

| Name     | Components  | Properties  | DataType | SocketType | Description                     |
|----------|-------------|-------------|----------|------------|---------------------------------|
||
| category | named       | name        | string   | none       | The name of the category        |
|          | describable | description | string   | none       | The description of the category |
||
| tag      | named       | name        | string   | none       | The tag name                    |
||

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

| Name                       | Repository                                                                                                            |
|----------------------------|-----------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-taxonomy | [https://github.com/inexorgame/inexor-rgf-plugin-taxonomy](https://github.com/inexorgame/inexor-rgf-plugin-taxonomy)  |

## Usage

<graphql-playground
  id="plugin-taxonomy-example-create-category"
  title="Create category"
  href="/examples/plugin-taxonomy-example-create-category.graphql">
This example creates the root category.
</graphql-playground>

<graphql-playground
  id="plugin-taxonomy-example-create-subcategory"
  title="Create subcategory"
  href="/examples/plugin-taxonomy-example-create-subcategory.graphql">
This example creates another category (which will be the subcategory).
</graphql-playground>

<graphql-playground
  id="plugin-taxonomy-example-create-subcategory-relation"
  title="Make category a subcategory"
  href="/examples/plugin-taxonomy-example-create-subcategory-relation.graphql">
This example shows how to make the second category a subcategory of the root category by creating a relation.
</graphql-playground>
