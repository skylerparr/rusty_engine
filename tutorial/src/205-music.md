# Music

One music file may be played at a time.  Music always loops repeatedly until explicitly stopped (or the program exits). As with other `Engine` fields, the audio manager is also available through the `Game` in your `main` function:

### Play

The `play_music` method starts playing looping music. The first parameter should be a `MusicPreset` enum variant or a music file path relative to `assets/audio`. All music from the asset pack have variants present in the `MusicPreset` enum for convenience.

The second parameter is the volume, which should be a value between `0.0` (silent) and `1.0` full volume.

```rust,ignored
// using a preset
game.audio_manager.play_music(MusicPreset::Classy8Bit, 1.0);

// using a filepath relative to `assets/audio`
game.audio_manager.play_music("music/Classy 8-Bit.oga", 1.0);
```

Any music already playing will be stopped to play a new music selection.

### Stop

The `stop_music` method stops any music that is playing. If no music is playing, it is a no-op.

```rust,ignored
engine.audio_manager.stop_music();
```

### Music playing status

The `music_playing` method will return a `bool` indicating whether or not music is currently playing.

```rust,ignored
if engine.audio_manager.music_playing() {
    // yep, you remembered to start the music
}
```
