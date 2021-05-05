## Usage

Below are instructions on how to use XP-Compiler.
Prerequisites: installed Rust & cargo.

### To add the required dependencies

Run in the terminal:

```terminal
git submodule update --init --recursive
```

### To test on the Ropsten testnet:

Run in the terminal the following command. Replace the text in <...> with the corresponding values.

```terminal
./broadcast_test.sh "<Ethereum node address>" "<Ethereum address private key>"
```

The Ethereum address must have some ETH.
The Node must be on the [Ropsten test net](https://faucet.ropsten.be/).
The node can run locally with https://geth.ethereum.org/

### Example of an Expected Bytecode of the Solidity Smart Contract:

```terminal
608060405234801561001057600080fd5b50610116806100206000396000f3fe60806040526004361060205760003560e01c8063b46300ec14602b57600080fd5b36602657005b600080fd5b60316033565b005b60405173106ca83003090c63b03d3fe3a9ee3b5e36c155cd90600090819083906020908381818185875af1925050503d8060008114608c576040519150601f19603f3d011682016040523d82523d6000602084013e6091565b606091505b50915091508160db5760405162461bcd60e51b815260206004820152601260248201527108cc2d2d8cac840e8de40e6cadcc8408ae8d60731b604482015260640160405180910390fd5b50505056fea2646970667358fe122056a2fafefefe6ccb86e5df84c7d7f788e02b0387fefe
```

### Example of an expected output:
```terminal
Deployed contract address: 0x87cA931C7669344C7f9c5a185991c65BdB3A1B9B
sent wei to contract. hash: 0xde0001b2717a3f7afcc84202d12ace85bcef8f633214b27541ca9bbfbb8e74b8
Executed contract! TxHash: 0x9e5419ab57a2ae7607a9c2f985fd79ae099e195eb78459052d447202cb87d7e7
```