#!/bin/bash


VERSION=$1

# replace version in Cargo.toml
sed -i "s/version = \"[0-9\.]*\"/version = \"$VERSION\"/gi" ./Cargo.toml

# commit Cargo.toml with the new version
git add ./Cargo.toml
git commit -m "Update to version $VERSION"

# create tag
git tag $VERSION
git push --tags

# publish to crates.io
cargo publish
