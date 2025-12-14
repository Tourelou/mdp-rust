#! /usr/bin/env bash

# Copiez ce script à la racine de votre projet pour créer
# un exécutable universal binary.

set -e

NAME=$(basename "$PWD")

# Vérifie si les targets nécessaires sont installées
for TARGET in x86_64-apple-darwin aarch64-apple-darwin; do
  if ! rustup target list --installed | grep -q "^$TARGET$"; then
    echo "⛔ Le target '$TARGET' n'est pas installé. Installez-le avec :"
    echo "   rustup target add $TARGET"
    exit 1
  fi
done

echo "🔧 Compilation pour x86_64..."
cargo build --profile release-optsize --target x86_64-apple-darwin

echo "🔧 Compilation pour aarch64..."
cargo build --profile release-optsize --target aarch64-apple-darwin

echo "🧬 Fusion des architectures..."
mkdir -p target/universal
lipo -create \
  target/x86_64-apple-darwin/release-optsize/$NAME \
  target/aarch64-apple-darwin/release-optsize/$NAME \
  -output target/universal/$NAME

echo "✅ Binaire universel dispo : target/universal/$NAME"
file target/universal/$NAME
