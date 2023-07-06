#!/bin/sh

git remote add rust_byc_helper https://github.com/barterYard/rust_byc_helper.git
git subtree add --prefix rust_byc_helper rust_byc_helper main --squash
