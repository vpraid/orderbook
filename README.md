# Orderbook

Orderbook is a sample implementation of a market exchange system wirtten completely in Rust. The primary goal
is get as fast as possible while maintaining eventual consistency when matching orders. The second goal is
modularization: orderbook engine interacts with its clients over a Unix socket. Developers are free to create their
own clients for command-line access, REST web servers etc. A sample CLI is provided with the orderbook that reads
trading scenarions from a CSV file and outputs the results to the stdout.

## Architecture

At the center of the system is the orderbook engine daemon, communicating with the rest of the world through unix sockets, much in the same way this is done in Docker. This is done to make the engine easy to containerize, as well as maintaining clear separation of concerns in the system.
