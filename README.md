# Orderbook

Orderbook is a sample implementation of a market exchange system wirtten completely in Rust. The system consists of a
standalone market server that connects to the outside world through Unix socket. This povides are secure and performant
interface while allowing other developers to write the kind of clients they wish for. A sample CLI client interacting
through the socket is also provided. It is designed according to the technical assessment requirements, so it is not
a full-fledged admistrative CLI.

## Market engine considerations

In real life scenario system architecture will be heavily dependend on the actual trading patterns, but since this is
an assessment task, I had to rely on my own understanding and reasearch. The following are some of the constraints are kept in mind:
 - Adding orders and cancelations are much more frequent than trades. This means in a highly concurrent environment
   there are more writes than reads. This rules out eventual consistency synchronization primitives since they expect
   reads to be dominating.
 - The orders are much more frequent around the top of the book. When designing an (ideally) lock-free data structure
   to handle orders, delays when inserting in the middle of order queue are not as important as what happens at the
   inner ends of the bid and ask queues.

## Market engine architecture

As I was pressed on time, I had to forego lock-free data strucures. Although there are several good ones in the Rust
ecosystem, they are either not stable, or don't provide a good API for my purposes. Therefore I decided to stick to
the usual mutex-based concurrency, however slow it might be for the purposes of HFT.

On the top of the hierarchy is a 'Market' data strcutre. It provides and interface to add and cancel limit orders, and
it does a primitive logging as required by the problem description. 'Market' maintains a hash map of 'Order Books',
each referenced by the symbol of the instrument traded there.

Order book maintains a collection of price levels. I chose BTreeMap to store levels since it provides a good balance
between runtime complexity of frequently used operations and it keeps everything sorted. It still requirs O(log(n))
time to access the top of the book, so it makes sense to maintain a separate (weak) reference to that level. This
optimization is not implemented yet. Accessing all other levels (mostly for the purposes of cancllations) requires
logarithmic time as well.

Inside each price level is a dequeue of orders. It has a good enough performance and, considering that price levels
themselves don't have many orders (especially at TOB), CPU caching mitigates any problems caused by deletions inside
the dequeue when cancelling an order. It is also good for the FIFO matching strategy I employed.

## Threads and processes

The server is built around green threads provded by the Tokio runtime. As the problem is mostly IO-bound, it makes
sense to use lightweigt tasks instead of threads to minimize time spent on context switching. The server is being
executed inside one such task, while Tokio spawn another reader tasks for each new connection to the Unix socket.

This approah also simplifies deployment. It is enough to designate the server as a systemd service. On linux hosts,
it is possible to expose the socket from inside the container to the outisde world and run CLI on the host.

The client is a completely separate process. Its sole goal is to read csv and send the encoded commands over the
server through the socket. The commands themselves are encoded as JSON objects for convenience of parsing on the
server side.

## What still needs to be done

 - Implement either a lock-free data structure or (at least) use mutex sharding for different order books.
 - Optimizations to access the TOB price levels in constant time.
 - Proper client and REST API.
