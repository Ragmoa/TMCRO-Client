# Main Documentation and Notes for later


## Goals & Architecture

The goal of this project is to provide a way to play The Lgened of Zelda: The Minish Cap randomizer seeds together.

The solution uses 3 different software pieces to achieve this:

#### Bridge

The bridge is a small lua script that will handle low levels operations: Reading and Writing into the game's RAM. It will be provided for bizhawk at first and for mgba when lua support for it becomes avilable.

#### Client

The client will handle the communications between the bridge and the server. It recieves low-level instructions from the bridges and high level ones from the server.

#### Server

The server handles the communications between players. It uses high-level instructions to manage the game session, according to predetermined rules (some of them could be chosen when creating the session)

## Bridge conception

#### Technology

The bridge is a lua script that uses webSocket to communicate with the client.

#### Reading in RAM

The bridge will recieve instructions from the client as for which addresses should be watched. Here's an exemple:

```
WATCH()
```
