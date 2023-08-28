start docker
```
docker run -t -d ubuntu:22.04 bash
docker exec -it <container_id> bash
```

install dep
```
cd /root/

apt update && apt install -y vim gcc npm make cmake wget curl git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.bashrc

wget https://github.com/WebAssembly/binaryen/releases/download/version_112/binaryen-version_112-x86_64-linux.tar.gz
tar -zxvf binaryen-version_112-x86_64-linux.tar.gz
export PATH=/root/binaryen-version_112/bin:$PATH

wget https://github.com/rustwasm/wasm-pack/releases/download/v0.10.3/wasm-pack-v0.10.3-x86_64-unknown-linux-musl.tar.gz
tar -zxvf wasm-pack-v0.10.3-x86_64-unknown-linux-musl.tar.gz
mv wasm-pack-v0.10.3-x86_64-unknown-linux-musl/wasm-pack /usr/local/bin/

git clone https://github.com/DelphinusLab/zkWasm.git
git clone https://github.com/DelphinusLab/wasmi.git
git clone https://github.com/xgaozoyoe/zkWasm-Rust-Demo.git

pushd wasmi
git checkout fa91eb0
popd

pushd zkWasm
git submodule set-url third-party/wasmi https://github.com/DelphinusLab/wasmi.git
git submodule init
git submodule update
cargo build --release
cp target/release/delphinus-cli /usr/local/bin/
popd
```


```
cd /root/zkWasm-Rust-Demo
wasm-pack build --release
wasm-opt -O3 pkg/zkwasm_rlp_bg.wasm -o zkwasm_rlp_bg.wasm --signext-lowering
mv zkwasm_rlp_bg.wasm pkg/zkwasm_rlp_bg.wasm
delphinus-cli -k 19 --function zkmain --output ./output --wasm pkg/zkwasm_rlp_bg.wasm setup
delphinus-cli -k 19 --function zkmain --output ./output --wasm pkg/zkwasm_rlp_bg.wasm single-prove
delphinus-cli -k 19 --function zkmain --output ./output --wasm pkg/zkwasm_rlp_bg.wasm single-verify --proof output/zkwasm.0.transcript.data --instance .  
```
