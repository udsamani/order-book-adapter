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

** If not specified, by default the host would be `127.0.0.1` and port `3000` **


Please note these are also needed by `oba-client` to identify where to connect to the server, same defaults apply there as well.

## How to interact with the tool ?

The following are **mandatory** steps that need to be followed, if you want to interact with the tool.

1. Spin up OBA Server. This can be done in multiple ways

   - `cd $HOME/order-book-adapater && cargo run --bin oba-server`
   - `cd $HOME/order-book-adapter && cargo build && target/debug/oba-server`

   This would sping up the OBA Server. It would be listening on `OBA_HOST` and `OBA_PORT` if defined. If not defined, by default it will use `127.0.0.1` and `3000`.


### Available Commands with `oba-cli`


#### Subscribe






