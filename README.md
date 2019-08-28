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
./rust-bitcoin-address-generator --type p2pkh

./rust-bitcoin-address-generator --type p2wpkh

./rust-bitcoin-address-generator --type p2sh --script "op_codes byte separated by comma"

./rust-bitcoin-address-generator --type p2wsh --script "op_codes byte separated by comma"
```

