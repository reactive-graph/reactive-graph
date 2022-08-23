# Plugin: Taxonomy

This plugin provides a generic taxonomy system with categories and tags.

## Components

| Name      | Properties | DataType | SocketType | Description                                                |
|-----------|------------|----------|------------|------------------------------------------------------------|
|           |
| weighted  | weight     | number   | none       | The weight of a relation between two entity instances      |

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
| categorized_as  |             |            | *                  | category           |
| has_subcategory |             |            | category           | category           |
| tagged_with     |             | weighted   | *                  | tag                |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Repositories

| Name                       | Repository                                                                                                            |
|----------------------------|-----------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-taxonomy | [https://github.com/inexorgame/inexor-rgf-plugin-taxonomy](https://github.com/inexorgame/inexor-rgf-plugin-taxonomy)  |
