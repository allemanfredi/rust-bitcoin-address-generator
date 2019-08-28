# rust-bitcoin-address-generator

Simple Bitcoin address generator written in Rust.

### Supported format: P2PKH, P2SH, P2WPKH, P2WSH

## How to install it

```
git clone https://github.com/allemanfredi/rust-bitcoin-address-generator.git
```

```
cd rust-bitcoin-address-generator/target/debug
```

```
./rust-bitcoin-address-generator [params]
```

## How to use it
```
usage: ./rust-bitcoin-address-generator --type <type> --script <script> [--mainnet | --testnet]

where <type> can be [p2pkh,p2wpkh,p2sh,p2wsh] and <script> is an array of op_codes byte

ex: ./rust-bitcoin-address-generator --type p2sh --script 00,14 --mainnet
```

