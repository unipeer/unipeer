# Unipeer contracts 

[![CI](https://github.com/unipeer/unipeer/actions/workflows/contracts.yml/badge.svg)](https://github.com/unipeer/unipeer/actions/workflows/contracts.yml)

## Technical Overview

The contract supports meta-transactions via the [delegatable framework][1] to support
new users wanting to buy their first crypto/tokens.

### Payment methods

A payment method represents a fiat payment processor/platform with each
having their own policy.
Only a subset of tokens is allowed to be sold via a payment method.

Sellers opt-in to each payment method they wish to accept fiat payment in
and the payment address of the platform buyers should make the payment to.

Sellers can deposit one or more of the accepted tokens for a particular payment
method they want to sell.

## Deployments

### Gnosischain Chiado Testnet


* Unipeer: [0x304D014228Dfba6ad7706ABEE16967Af29E2Aee2](https://gnosis-chiado.blockscout.com/address/0x304d014228dfba6ad7706abee16967af29e2aee2)
* AutoAppealableArbitrator: [0x60cE8c27757399735969d736Ba3987586501e514](https://blockscout.chiadochain.net/address/0x60cE8c27757399735969d736Ba3987586501e514)
* DelegatableRelay: [0x98CbFB4F664e6b35a32930c90e43F03b5Eab50DA](https://blockscout.chiadochain.net/address/0x98CbFB4F664e6b35a32930c90e43F03b5Eab50DA)
* WXDAI: [0x18c8a7ec7897177E4529065a7E7B0878358B3BfF](https://blockscout.chiadochain.net/address/0x18c8a7ec7897177E4529065a7E7B0878358B3BfF)

### Goerli Testnet

* Unipeer: [0x604e56aD882C820B8286aC474f22Ac530e714521](https://goerli.etherscan.io/address/0x604e56aD882C820B8286aC474f22Ac530e714521)
* Unipeer (Centralized Arbitrator): [0xd56e8f3c7731f6e0d85a47de55926f0de8cc0368](https://goerli.etherscan.io/address/0xd56e8f3c7731f6e0d85a47de55926f0de8cc0368)

## Testing

Given the repository contains both Solidity and Rust code, there's 2 different
workflows.

### Solidity

```bash
forge test
```

[1]: https://github.com/delegatable/delegatable-sol
