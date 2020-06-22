# Unipeer

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
npx buidler compile && npm generate-types
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
