# atereko

atereko is After Recording.
To synthesize a audio to the original video.

## Usage

```bash
$ cargo run <input video> <input audio> <input seconds> <output name>
```

### ex

```bash
$ git clone https://github.com/poccariswet/atereko
$ cd atereko
$ cargo run sample_audio/girl_shout.mp4 sample_audio/shout.mp3 9 output
```

## reference
- [https://ffmpeg.org/](https://ffmpeg.org/)
- [https://ffmpeg.org/ffmpeg.html](https://ffmpeg.org/ffmpeg.html)
