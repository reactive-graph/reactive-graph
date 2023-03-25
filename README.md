<div align="center">
<a href="https://inexor.org/">
<img src="docs/inexor-rgf/src/images/rgf_logo_k2_v4_fg_sm.png" style="width: 50%; height: 50%; margin-left: 25%; margin-right: 25%;" />
</a>
</div>


<h1 align="center" style="text-align: center;">Inexor Reactive Graph Flow</h1>

<div align="center" style="text-align: center">

[<img src="https://img.shields.io/badge/Docs-mdBook-brightgreen">](https://inexorgame.github.io/inexor-rgf-application/)

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-application">](https://github.com/inexorgame/inexor-rgf-application/blob/main/LICENSE)

[<img src="https://img.shields.io/github/actions/workflow/status/inexorgame/inexor-rgf-application/rust.yml">](https://github.com/inexorgame/inexor-rgf-application/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-application">](https://app.codecov.io/gh/inexorgame/inexor-rgf-application)

[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

</div>


<h2 align="center" style="text-align: center;">What is this?</h2>

<div style="background-color: #333333; padding: 10px 50px; font-size: 12pt;">

➔ The `Reactive Graph Flow` is a **graph database**

➔ The `Reactive Graph Flow` is a **document store**

➔ The `Reactive Graph Flow` is a **flow control** runtime

➔ The `Reactive Graph Flow` is a **web server**

➔ The `Reactive Graph Flow` is **pluggable** and **extensible**

➔ The `Reactive Graph Flow` is **fast**, **secure** and **small**

</div>


<h2 align="center" style="text-align: center;">What is it for?</h2>

<div style="background-color: #333333; padding: 10px 50px; font-size: 12pt;">

➔ Game Entity Component System (ECS) - especially for [Inexor](https://inexor.org/)

➔ Smart Home

➔ Data Conversion Tools

➔ Flow control system for card size computers and embedded devices

➔ Desktop Automation

</div>


<h2 align="center" style="text-align: center;">Graph</h2>

<div style="background-color: #333333; padding: 10px 50px; font-size: 12pt;">

A **graph** organizes highly interconnected data. The state of an `Entity Component System` can be ideally represented
with the help of the graph. Inexor is the first game engine to introduce a graph as a basis.

The main benefits of a graph are:
 
* A universal data structure for everything
* Relations are first class citizens
* Benefit from types and instances which makes things intuitive
* Benefit from navigation which is fast and intuitive
* Benefit from the semantics of highly connected, intuitive data
* Properties can store not only certain primitive data but complete documents

</div>


<h2 align="center" style="text-align: center;">Reactive</h2>

<div style="background-color: #333333; padding: 10px 50px; font-size: 12pt;">

Now that we understand how data is stored, here's how data interacts. The approach is that the data itself is "alive".
To do this, Inexor adopts a concept from reactive programming.

In computing, reactive programming is a declarative programming paradigm concerned with data streams and the
propagation of change.

It is the ingenious combination of a graph with reactive programming. The property instances are not static
and only contain data. Rather, they are streams of data. If you change the value of a property instance, you
fill the data stream of this property instance. Data streams are linked together. For example, if the stream
of one property instance is linked to the stream of another property instance and you change the value of the
first property instance, the value of the second property instance will automatically change as well. Data is
thus propagated from one station to the next, triggering a cascade of propagations.

In addition, Inexor remembers the last value in each property instance. This is done by subscribing to your
own data stream and caching it. This allows subsequent querying of the value of a property instance.

Remember this basic concept:

* Every property is a stream not only data
* Property streams can be subscribed and published
* The streams of two properties can be connected and changes will be propagated (cascade)

</div>


<h2 align="center" style="text-align: center;">Behaviour driven design</h2>

<div style="background-color: #333333; padding: 10px 50px; font-size: 12pt;">

The data flow is therefore automatic. Building on this, Inexor applies the concept of behaviour-driven design.
The goal is to use these data streams to simulate behaviour.

Behaviors can be implemented on components, entities and relations. To do this, one or more incoming data streams
are combined, calculations are performed and written to one or more outgoing data streams.

For example, the entity type "AND gate" implements a behavior by subscribing to the two input properties, combining
them into a combination data stream and performing an AND operation on the incoming pairs of values. The result of
the AND operation is itself a data stream and this is linked to the output property.

This example shows how an entity type is wired internally. They are all data streams that are cleverly combined
with one another and thus depict behavior.

It is interesting that this behavior also works for relations. For example, connectors are also implemented
behaviors of streams. It is interesting that connectors connect the data stream from a property instance of the
outgoing entity instance with the data stream from a property instance of the incoming entity instance.

For example the AND-Gate accepts inputs at the properties lhs and rhs. Both streams are subscribed and zipped.
The zipped stream is calculated with a function - in this case the AND-Operator. This results in another
(invisible) stream which is connected with the property result. The entity type AND-Gate defines that the
properties lhs, rhs and result have to exist. Furthermore, the socket types are defined: lhs and rhs are
Input-Sockets and result is a Output-Socket. The behaviour is like the internal wiring of entity instances and
of relation instances. A behaviour can be added to entity instances and removed from entity/relation instances.

</div>


<h2 align="center" style="text-align: center;">Flow</h2>

<div style="background-color: #333333; padding: 10px 50px; font-size: 12pt;">

Control flows can be implemented based on the graph, the data streams and the behavior-driven design. It is
important here that the available modules that implement the behavior are linked with connectors.

For example, a flow can consist of a logic that links several AND gates with each other using connectors. Both
the AND gate and the connector are behaviors. But the arrangement of these behaviors within a flow makes them
powerful.

Entire game modes can be implemented with the help of flows. Or just parts of it that are used in multiple game
modes, such as a mechanism to pick up, drop, and score flags.

Flows are also useful for making maps more interactive. With the help of flows and behaviors, it can be ensured
that a door opens in a map when you press switch 1 and switch 2. Or you determine the color of your own team's
base based on the current score. Or you control particle emitters, depending on how many players are near the
emitter. The possibilities for this are endless and want to be used!

</div>


<h2 align="center" style="text-align: center;">Inexor</h2>

<div style="background-color: #333333; padding: 10px 50px; font-size: 12pt;">

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++20 and Rust.
* You can contribute anything you want: code, content, ideas..
* Inexor and all its content is 100% open source!

</div>


<h2 align="center" style="text-align: center;">Plugins</h2>

<div align="center" style="text-align: center">

[The Plugin System](http://localhost:3000/Plugin_System.html)

[List of Plugins](https://inexorgame.github.io/inexor-rgf-application/Plugins.html)

</div>


<h2 align="center" style="text-align: center;">Development</h2>

<div align="center" style="text-align: center">

[Build Instructions](https://inexorgame.github.io/inexor-rgf-application/Development_Build.html)

[Data Model](https://inexorgame.github.io/inexor-rgf-application/Model.html)

[GraphQL API](https://inexorgame.github.io/inexor-rgf-application/GraphQL_API.html)

[Dynamic Graph API](https://inexorgame.github.io/inexor-rgf-application/Dynamic_Graph_API.html)

</div>


<h2 align="center" style="text-align: center;">Configuration</h2>

<div align="center" style="text-align: center">

[Configuration](https://inexorgame.github.io/inexor-rgf-application/Configuration.html)

</div>


<h2 align="center" style="text-align: center;">GraphQL Type System</h2>

<img src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-application/main/docs/graphql/queries.png">

<img src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-application/main/docs/graphql/mutations.png">


<h2 align="center" style="text-align: center;">Flow Editor</h2>

<div align="center" style="text-align: center">

Coming soon.

</div>


<h2 align="center" style="text-align: center;">Sponsors</h2>

<div align="center" style="text-align: center">

<a href="https://www.jetbrains.com/?from=github.com/inexorgame">
<img src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-application/main/docs/images/icon_CLion.svg" style="width: 30%; height: 30%;">
</a>

_Special thanks to JetBrains for providing us with CLion licenses!_

</div>
