# Basic Bitcoin Core Node handshake library

This library contains basic types and serialization/deserialization functionality to be able to do a handshake with a Bitcoin Core Node. Right now can only be used to get basic information from the node, like the user agent name.

## Testing

To test the library, know an address of a Bitcoin Core Node or have a instance running on your system. Here are some basic instructions to test this library locally:

1. Download and install the [Bitcoin Core Node](https://bitcoin.org/en/bitcoin-core/) on your system;
2. Configure and launch the local Bitcoin Core Node;
3. Clone this project to a directory on your system by using the following command: `git clone https://github.com/T-Hacker/p2p-handshake.git`
4. Change directory to the project: `cd p2p-handshake`
5. Test the library by doing:
    - Linux: `BTC_NODE_ADDRESS=127.0.0.1:8333 cargo test handshake -- --nocapture`
    - Windows (PS): `$env:BTC_NODE_ADDRESS="127.0.0.1:8333"; cargo test handshake -- --nocapture`
	- Windows (CMD): `set BTC_NODE_ADDRESS=127.0.0.1:8333 && cargo test handshake -- --nocapture`
6. If everything is working correctly, you should see in the terminal a message displaying the name of the user agent that is running on your system.

*NOTE: This guide assumes that you have Git and Cargo already installed on your system.*