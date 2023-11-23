# Basic Reverb

A Rust implementation of the reverb presented at [Let's Write A Reverb](https://youtu.be/6ZK2Goiyotk?si=7DKV5o-jFFr3YSP8). It was made basically translating the [reverb example code](https://github.com/Signalsmith-Audio/reverb-example-code).  
It has differences tough:

- It has a fixed number of channels
- It does not include the examples of intermediary steps to come up with the final reverb.
- It uses a simpler delay (the original it is part of a separate library).
- It doesn't have the `configure` functions so the sample rate can't be changed once set.
