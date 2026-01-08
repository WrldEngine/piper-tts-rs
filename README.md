# Piper TTS Rust direct binding

## Tips
- Convert RAW to WAV for piper, use `ffmpeg -f f32le -ar 22050 -ac 1 -i audio.raw audio.wav` (soon this convertation will be implemented)
- `LD_LIBRARY_PATH=$PWD/libpiper/install:$PWD/libpiper/install/lib cargo run --example simple` need to set ld lib path right now, it will be resolved soon (no need to build)