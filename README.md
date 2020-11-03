# Unipeer

Unipeer is non-custodial p2p fiat on/off ramp built using Ethereum Escrow contracts
and open banking APIs.

We are leveraging India's UPI (Unified Payments Interface) to connect traditional
payments to the Ethereum blockchain via Chainlink oracles.

## Roadmap

- [x] Integrate a UPI API to a chainlink node as an External Adapter.
- [ ] Add a mechanism to pay for initial buy tx.
- [ ] Add pricing information between ETH/INR or USD/INR. (UMA or chainlink)
- [ ] On/Off chain order matching mechanism that matches buyers to sellers.
- [ ] Move to a zk or optimistic rollup.

## Constants

| Network | Oracle                                       | Job Id                             |
| ------- | -------------------------------------------- | ---------------------------------- |
| Kovan   | `0x98cbfb4f664e6b35a32930c90e43f03b5eab50da` | `3dd25a102fe74157b1eae12b430336f4` |

## Install

```
yarn install
```

## Build

We use hardhat.org

```
yarn hardhat compile
```

## Debug

To get details about the transactions being executed, run the buidlerevm
separately in another terminal.

```
yarn hardhat node
```

Then run your tests with the localhost network

```
yarn hardhat --network localhost test
```

## Test

```
yarn hardhat test
```

## Deploy

```
PRODUCTION=1 yarn hardhat run scripts/deploy.kovan.ts --network kovan
```
