# GCODER

g-code processing system to make old 3d printers work wirelessly




## Admin privilages

GCoder sometimes needs to re-bind a serial device driver. This simulates breifly
unplugging the printer, which is needed in some cases when the printer becomes
unresponsive. For GCoder to work correctly, elevate it to admin priviliges by
pasting these snippets into a terminal:

```
cargo build
sudo chown root:root target/debug/gcoder
sudo chmod u+s target/debug/gcoder


cargo build --release 
sudo chown root:root target/release/gcoder
sudo chmod u+s target/release/gcoder

```

