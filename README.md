# Piper TTS Rust direct binding

## Tips
- Convert RAW to WAV for piper, use `ffmpeg -f f32le -ar 22050 -ac 1 -i audio.raw audio.wav` (soon this convertation will be implemented)
- `cargo run --example simple` - generates .raw file (raw pcm chunks 22050 Hz, 1 mono)

[Let's listen](https://github.com/user-attachments/files/24545995/audio.wav)
