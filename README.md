# Order Book Adapter
A Rust Service that Subscribes to the L2 Order Book Updates for a user defined instrument and maintains an in-memory representation of the order book.


## Project Structure

The repository follows a `Cargo` workspace structure. There are 3 crates defined

- oba-cli - A CLI tool to interact with `oba-server`.
- oba-client - An HTTP Client to interact with `oba-server`
- oba-server - A server, which interacts with `bitstamp` to subscribe to order books for specified `instrument` with `certain` depth.

Both `oba-cli` and `oba-server` are binaries, whereas `oba-client` is a library.

In order, to spin up `oba-server`, a host and port need to be defined. Following two environment variables are needed
- `OBA_HOST`
- `OBA_PORT`

**If not specified, by default the host would be `127.0.0.1` and port `3000`**


Please note these are also needed by `oba-client` to identify where to connect to the server, same defaults apply there as well.

## How to interact with the tool ?

The following are **mandatory** steps that need to be followed, if you want to interact with the tool.

1. Spin up OBA Server. This can be done in multiple ways

   - `cd $HOME/order-book-adapater && cargo run --bin oba-server`
   - `cd $HOME/order-book-adapter && cargo build && target/debug/oba-server`

   This would sping up the OBA Server. It would be listening on `OBA_HOST` and `OBA_PORT` if defined. If not defined, by default it will use `127.0.0.1` and `3000`.


### Available Commands with `oba-cli`

All of the commands have two options on how they can be invoked.

1. `cd $HOME/order-book-adapter && cargo run --bin oba-cli {command} {flags}`
2. `cd $HOME/order-book-adapter && cargo build && target/debug/oba-cli {command} {flags}`

#### Subscribe

Run the following command with these required flags to subscribe.

`target/debug/oba-cli subscribe --instrument btcusd --depth 50`

#### Unsubscribe

Run the following command with these required flag(s) to unsubscribe.

`target/debug/oba-cli unsubscribe --instrument btcusd`


#### Get Order Book


Run the following command with these required flag(s) to get order book.

`target/debug/oba-cli get-order-book --instrument btcusd`


#### Best Bid

Run the following command with these required flag(s) to best bid.

`target/debug/oba-cli best-bid --instrument btcusd`

#### Best Ask 
Run the following command with these required flag(s) to best bid.

`target/debug/oba-cli best-ask --instrument btcusd`


# GIF Showing the Workflow

![img](docs/cli.gif)
