#!/bin/bash

near deploy --wasmFile res/no_std_test.wasm --initFunction "new" --initArgs '' --accountId $1.testnet
