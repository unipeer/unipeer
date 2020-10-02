# Unipeer

## TODO

* Fix solhint plugin
* Fix typechain plugin

## Constants

| Network | Oracle | Job Id |
|---------|--------|--------|
| Kovan | `0x98cbfb4f664e6b35a32930c90e43f03b5eab50da` | `10cb58b1b1cc43268d0928f62cec31bb` |

## Install

```
npm install
```

## Build

We use buidler.dev

```
npx buidler compile
```

## Generate Types for contracts

```
npx buidler compile && npm run generate-types
```

## Debug

To get details about the transactions being executed, run the buidlerevm
separately in another terminal.

```
npx buidler node
```

Then run your tests with the localhost network

```
npx buidler --network localhost test
```

## Test

```
npx buidler test
```
