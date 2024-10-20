# System Architecture

In this document we describe...

* which components exists
* what they are doing
* how they are used
* and which use cases they fulfill

## Components

First things first: It's important to understand which component is responsible for which task and how they
are connected.

### Reactive Graph Client API `reactive_graph_client`

The **Client API** (`reactive_graph_client`) is a rust crate which can be used to communicate with a runtime via it's
GraphQL API. Instead of writing GraphQL queries one just have to call a (rust) method of the **Client API**.

### GraphQL Server `reactive_graph_server`

The **GraphQL Server** is part of the standalone executable. It is integrated in a HTTP webserver and accepts
requests with GraphQL queries and mutations. The GraphQL Server calls the GraphQL Query Service of the runtime.

### GraphQL Query Service `reactive-graph-graphql-api`

The **GraphQL Query Service** is part of the runtime. It can be used to execute GraphQL queries and mutations. The
GraphQL Query Service uses the GraphQL API.

### GraphQL API `reactive-graph-graphql-web`

The **GraphQL API** is part of the runtime. The GraphQL API encapsulates calls to the APIs of the Type System API, the
Instance System API and the System API in GraphQL resolvers. It converts internal data structures to and from the types
of the GraphQL schema.

### Type System API `reactive-graph-type-system-api`

The **Type System API** is part of the runtime. It manages the type system, allows to create, read, update and delete
types of the **reactive graph**. You can manage components, entity types, relation types and flow types.

### Instance System API `reactive-graph-instance-system-api`

The **Instance System API** is part of the runtime. It manages the reactive instances, allows to create, read, update
and delete reactive instances within the **Reactive Graph**. You can manage entity instances, relation instances
(including flow connectors) and flow instances.

### Runtime Service API `reactive-graph-runtime-service-api`

The **System API** is part of the runtime. It manages commands, plugins, the instance information and remote
instances.

### Command Line Interface API `reactive-graph`

The **Command Line Interface API (CLI)** is part of the standalone executable.

It is possible to use the standalone executable as

* **Server** (GraphQL Server + Runtime)
* **Client** (CLI)

The CLI uses the **Client API** to communicate via **GraphQL** with a runtime. You can specify the hostname and port of
the runtime to connect to.

## Use Cases

The components can be used for different use cases:

* Command Line Interface within the standalone executable
* Embed the runtime `reactive-graph-runtime-api` in your application
* Communicate from one runtime to another
* Let your application communicate with a (remote) runtime
* Integration tests

### Command Line Interface `reactive-graph`

The executable can act as CLI. The CLI can connect to any **Reactive Graph GraphQL Server** (local or remote). You
can specify to hostname and port of the GraphQL server to connect to.

```mermaid
graph TD
subgraph Command Line Interface[ ]
    USR([User])
    
    subgraph Reactive Graph Client
      direction LR
      CLI[Command Line Interface]
      subgraph `reactive_graph_client`
        CA(Client API)
      end
      subgraph `reactive_graph_client`[reactive_graph_runtime]
        UNUSED(Unused if in CLI mode)
      end
    end

    subgraph Reactive Graph Server
      GS[GraphQL Server]
      subgraph `reactive_graph_server`[reactive_graph_runtime]
        GQS[GraphQL Query Service]
        GA[GraphQL API]
        TSA[Type System API]
        ISA[Instance System API]
        SYSA[System API]
      end
    end
    
    USR -->|command| CLI
    CLI -->|call| CA
    CA -->|query| GS
    GS -->|query| GQS
    GQS --->|query| GA
    GA --->|call| TSA
    GA --->|call| ISA
    GA --->|call| SYSA
end
```

### Embedded runtime in an application `reactive-graph-runtime-service-api`

Any rust application can embed the Reactive Graph runtime (`reactive-graph-runtime-service-api`).

The following diagram shows that the application can:

* Call the **Type System API**, **Instance System API** and **System API** directly via the runtime.
* Execute GraphQL queries or mutations via the **GraphQL Query Service**.

```mermaid
graph TD
subgraph Embedded Runtime[ ]

  subgraph Application
    direction TB
  
    SRV1[Application's Service 1]
    SRV2[Application's Service 2]
  
    subgraph "reactive_graph_runtime"
      GQS[GraphQL Query Service]
      GA[GraphQL API]
      TSA[Type System API]
      ISA[Instance System API]
      SYSA[System API]
    end
  end

  GQS -->|query| GA
  GA --->|call| TSA
  GA --->|call| ISA
  GA --->|call| SYSA

  SRV1 -->|call| TSA
  SRV1 -->|call| ISA
  SRV1 -->|query| GQS
  SRV2 -->|call| TSA
  SRV2 -->|call| ISA
  SRV2 -->|query| GQS

end
```

### Communicate from the runtime with another runtime

In order to communicate from one runtime to another runtime you can use the **Reactive Graph Client**
(`reactive_graph_client`).

In this example two runtimes are calling methods of the own runtime and execute queries on the other runtime via the
**Reactive Graph Client**.

```mermaid
graph TD
subgraph RuntimeToRuntime[ ]
  direction LR

  subgraph "Runtime 1"
    direction TB
  
    SRV1_1[Service 1]
    SRV1_2[Service 2]
  
    subgraph "reactive_graph_runtime_1"[reactive_graph_runtime]
      GQS1[GraphQL Query Service]
      GA1[GraphQL API]
      TSA1[Type System API]
      ISA1[Instance System API]
      SYSA1[System API]
    end
    
    subgraph "reactive_graph_client_1"[reactive_graph_client]
      CA1(Client API)
    end

  end

  subgraph "Runtime 2"
    direction TB
    
    SRV2_1[Service 1]
    SRV2_2[Service 2]
  
    subgraph "reactive_graph_runtime_2"[reactive_graph_runtime]
      GQS2[GraphQL Query Service]
      GA2[GraphQL API]
      TSA2[Type System API]
      ISA2[Instance System API]
      SYSA2[System API]
    end
    
    subgraph "reactive_graph_client_2"[reactive_graph_client]
      CA2(Client API)
    end
  end

  GQS1 -->|query| GA1
  GA1 --->|call| TSA1
  GA1 --->|call| ISA1
  GA1 --->|call| SYSA1
  CA1 -->|query| GQS2
  SRV1_1 -->|call| TSA1
  SRV1_1 -->|query| CA1
  SRV1_2 -->|call| ISA1
  SRV1_2 -->|query| CA1

  GQS2 -->|query| GA2
  GA2 --->|call| TSA2
  GA2 --->|call| ISA2
  GA2 --->|call| SYSA2
  CA2 -->|query| GQS1
  SRV2_1 -->|call| TSA2
  SRV2_1 -->|query| CA2
  SRV2_2 -->|call| ISA2
  SRV2_2 -->|query| CA2

end
```

The same applies to two applications which have embedded runtimes.

### Let your application communicate with a (remote) runtime

In order to communicate from your application to a (remote) runtime you can use the **Reactive Graph Client**
(`reactive_graph_client`).

This example shows the application which uses the **Reactive Graph Client** to execute queries on the (remote) runtime:

```mermaid
graph TD
subgraph ApplicationToRuntime[ ]
  direction LR

  subgraph "Application"
    direction TB
  
    SRV1[Service 1]
    SRV2[Service 2]
  
    subgraph "reactive_graph_client_1"[reactive_graph_client]
      CA(Client API)
    end

  end

  subgraph Reactive Graph Server
    direction TB
    GS[GraphQL Server]
    subgraph reactive_graph_runtime
      GQS[GraphQL Query Service]
      GA[GraphQL API]
      TSA[Type System API]
      ISA[Instance System API]
      SYSA[System API]
    end
  end

  SRV1 -->|call| CA
  SRV2 -->|call| CA
  CA -->|query| GS
  GS -->|query| GQS
  GQS -->|query| GA
  GA --->|call| TSA
  GA --->|call| ISA
  GA --->|call| SYSA
end
```

## Integration Tests

An integration test verifies the correctness of:

* The **Client API**, because the **Integration Test** calls methods of the **Client API**.
* The **GraphQL API**, because the **Client API** queries the **GraphQL API**.
* The **GraphQL API**, because the **Integration Test** queries the **GraphQL API** of a started runtime directly.
* The APIs of the **runtime**, because the **GraphQL API** calls methods of the **runtime**.
* The APIs of the **runtime**, because the **Integration Test** calls methods of a started **runtime** directly.

```mermaid
graph TD
subgraph Integration Tests[ ]

  IT[Integration Test]
  CA(Client API)
  style IT fill:#333,stroke:#633,stroke-width:4px,margin-top:10px

  subgraph "reactive_graph_runtime"
    GQS[GraphQL Query Service]
    GA[GraphQL API]
    TSA[Type System API]
    ISA[Instance System API]
    SYSA[System API]
  end

  subgraph "reactive_graph_client"
    CA -->|query| GQS
  end

  IT -->|call| CA
  IT --->|query| GQS
  IT -->|call| TSA
  IT -->|call| ISA

  GQS -->|query| GA
  GA --->|call| TSA
  GA --->|call| ISA
  GA --->|call| SYSA

  IT -->|call| SYSA

end
```
