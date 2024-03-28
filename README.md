# raiko

## Usage

### Building

- Install the `cargo risczero` tool and the `risc0` toolchain:

```console
$ cargo install cargo-risczero
$ cargo risczero install
```
- Install the `cargo prove` tool and the `succinct` toolchain:

```console
$ curl -L https://sp1.succinct.xyz | bash
$ sp1up
$ cargo prove --version
```

- For SGX, install gramine: https://github.com/gramineproject/gramine. If you're running ubuntu 22.04 (or a compatible distro) you can just download and install this deb file: https://packages.gramineproject.io/pool/main/g/gramine/gramine_1.6.2_amd64.deb

- Clone the repository and build with `cargo`:

```console
$ cargo build
```

### Running

Run the host in a terminal that will listen to requests:

Just for development with the native prover:
```
cargo run
```

Then in another terminal you can do requests like this:

```
./prove_block.sh testnet native 10
```

Look into `prove_block.sh` for the available options or run the script without inputs and it will tell you.

## Provers

Provers can be enabled using features. To compile with all of them (using standard options):

```
cargo run --release --features "risc0 succinct"
```

### risc zero
#### Testing
```
RISC0_DEV_MODE=1 cargo run --release --features risc0
```

#### Bonsai
```
# edit run_bonsai.sh and run
run_bonsai.sh
# then
prove_block.sh testnet risc0-bonsai 10
```

#### CPU
```
cargo run --release --features risc0
```

#### GPU

```
cargo run -F cuda --release --features risc0
```
OR
```
cargo run -F metal --release --features risc0
```

CUDA needs to be installed when using `cuda`: https://docs.nvidia.com/cuda/cuda-installation-guide-linux/index.html

### succinct's SP1:
```
cargo run --release --features succinct
```

### SGX:
```
cargo run --release --features sgx
```

If your CPU doesn't support SGX, you can still run the SGX code through gramine like it would on an SGX machine:

```
SGX_DIRECT=1 cargo run --release --features sgx
```