# Distributed Key-Value Store

This project implements a distributed key-value store in Rust, designed to handle data storage and retrieval across multiple nodes. The development is divided into phases, starting with a single-node implementation and progressively adding distributed features.

## Project Overview

The goal is to create a system similar to simplified versions of Redis or Etcd, focusing on:

- High availability
- Fault tolerance
- Eventual consistency

## Phase 1: Single-Node Implementation

The current phase focuses on building a robust single-node key-value store. This serves as the foundation for the distributed system and includes the following features:

- In-memory data storage using Rust's `HashMap`
- Basic CRUD operations (Create, Read, Update, Delete)
- Simple persistence mechanism using file I/O
- Error handling with custom error types
- Basic client interface for interacting with the store

### Key Features of Single-Node Implementation

1. **Core Key-Value Operations**: Set, get, and delete operations for managing key-value pairs.
2. **Persistence**: Data is saved to disk, allowing the store to recover its state after restarts.
3. **Error Handling**: Robust error handling using Rust's `Result` type and custom error definitions.
4. **Simple API**: A straightforward API for client interactions with the store.

### Future Phases

Upcoming phases will focus on:

1. **Cluster Management**: Implementing mechanisms to add and remove nodes dynamically.
2. **Data Partitioning**: Using consistent hashing or another partitioning scheme to distribute data across nodes.
3. **Replication**: Ensuring data is replicated across multiple nodes for fault tolerance.
4. **Consensus Algorithm**: Implementing a consensus algorithm like Raft or Paxos to maintain consistency across replicas.
5. **Failure Detection and Recovery**: Detecting node failures and redistributing data as necessary.
6. **Enhanced Client API**: Expanding the API to support distributed operations.

## Getting Started

To run the single-node key-value store:

1. Ensure you have Rust installed on your system.
2. Clone this repository:
