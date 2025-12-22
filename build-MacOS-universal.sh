#! /usr/bin/env bash

# Copiez ce script à la racine de votre projet pour créer
# un exécutable universal binary.

set -e

NAME=$(basename "$PWD")
PROFILE="release"

function build_this() {
  echo "Profile = $1"
  echo "🔧 Compilation pour x86_64..."
  cargo build --profile $1 --target x86_64-apple-darwin

  echo "🔧 Compilation pour aarch64..."
  cargo build --profile $1 --target aarch64-apple-darwin

  echo "🧬 Fusion des architectures..."
  mkdir -p target/universal/$1
  lipo -create \
    target/x86_64-apple-darwin/$1/$NAME \
    target/aarch64-apple-darwin/$1/$NAME \
    -output target/universal/$1/$NAME

  echo "✅ Binaire universel dispo : target/universal/$1/$NAME"
  file target/universal/$1/$NAME
}

# Vérifie si les targets nécessaires sont installées
for TARGET in x86_64-apple-darwin aarch64-apple-darwin; do
  if ! rustup target list --installed | grep -q "^$TARGET$"; then
    echo "⛔ Le target '$TARGET' n'est pas installé. Installez-le avec :"
    echo "   rustup target add $TARGET"
    exit 1
  fi
done

if [[ $# -gt 0 ]]; then
  for arg in "$@"
  do
    build_this $arg
  done
else
  build_this $PROFILE
fi
