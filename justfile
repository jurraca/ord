log := '0'

export RUST_LOG := log

ci: clippy forbid
  cargo fmt -- --check
  cargo test

forbid:
  ./bin/forbid

fmt:
  cargo fmt

clippy:
  cargo clippy --all --all-targets

bench:
  cargo criterion

watch +args='test':
  cargo watch --clear --exec '{{args}}'

install-dev-deps:
  cargo install cargo-criterion

deploy branch='master':
  ssh root@65.108.68.37 mkdir -p deploy
  rsync -avz deploy/checkout root@65.108.68.37:deploy/checkout
  ssh root@65.108.68.37 'cd deploy && ./checkout {{branch}}'

status:
  ssh root@65.108.68.37 systemctl status bitcoind
  ssh root@65.108.68.37 systemctl status ord

serve:
  python3 -m http.server --directory docs

generate-private-key:
  cargo run generate-private-key

generate-paper-wallets:
  cat private-keys.txt | cargo run generate-paper-wallets

print-paper-wallet path:
  wkhtmltopdf -L 25mm -R 25mm -T 50mm -B 25mm {{path}} wallet.pdf
  lp -o sides=two-sided-long-edge wallet.pdf

# publish current GitHub master branch
publish:
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  git clone git@github.com:casey/ord.git tmp/release
  VERSION=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  cd tmp/release
  git tag -a $VERSION -m "Release $VERSION"
  git push origin $VERSION
  cargo publish
  cd ../..
  rm -rf tmp/release
