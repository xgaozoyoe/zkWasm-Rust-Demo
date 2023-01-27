wasm := "pkg/zkwasm_rlp_bg.wasm"
args := "-k 19 --function rlp --output ./output --wasm " + wasm
proof := "output/zkwasm.0.transcript.data"

cli := env_var_or_default('ZKWASM_CLI', 'zkwasm-cli-x86')

build:
  wasm-pack build --release

setup:
  rm -rf output
  mkdir -p output
  {{cli}} {{args}} setup

prove:
  {{cli}} {{args}} single-prove

verify:
  {{cli}} {{args}} single-verify --proof {{proof}}

test:
  just build
  just setup
  just prove
  just verify
