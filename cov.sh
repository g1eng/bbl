#!/bin/sh
cargo tarpaulin -o Xml --target-dir=coverage
mv -v cobertura.xml ./coverage/
