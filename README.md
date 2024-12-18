To build + test
```bash
cargo build --release --target x86_64-unknown-uefi
uefi-run -b OVMF.4m.fd target/x86_64-unknown-uefi/rele ase/krnl_uefi.efi
```