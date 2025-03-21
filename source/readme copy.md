BUILD:
RUSTFLAGS="-C link-arg=-Tlinker.ld" cargo build -Z build-std=core,compiler_builtins --release --target x86_64-hola_mundo_os.json

GENERATE ISO
cp target/x86_64-hola_mundo_os/release/os iso/boot/os.elf
docker build -t mi_compilador_iso . // first time only

docker run --rm -v "$(pwd)/iso":/iso mi_compilador_iso \
  -o /iso/hola_mundo_os.iso /iso


EXECUTE ISO
cp iso/hola_mundo_os.iso /tmp/
cd /tmp
qemu-system-x86_64 -drive file=hola_mundo_os.iso,media=cdrom,readonly=on

ALL TOGETHER
RUSTFLAGS="-C link-arg=-Tlinker.ld" cargo build -Z build-std=core,compiler_builtins --release --target x86_64-hola_mundo_os.json \
&& cp target/x86_64-hola_mundo_os/release/os iso/boot/os.elf \
&& rm -rf iso/hola_mundo_os.iso \
&& docker run --rm -v "$(pwd)/iso":/iso mi_compilador_iso \
  -o /iso/hola_mundo_os.iso /iso \
&& cp iso/hola_mundo_os.iso /tmp/ \
&& qemu-system-x86_64 -vga cirrus -drive file=/tmp/hola_mundo_os.iso,media=cdrom,readonly=on

