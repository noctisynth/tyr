# Changelog

## \[0.1.2]

### New Features

- [`87e516e`](https://github.com/noctisynth/tyr/commit/87e516e0990bd612c40acfe050fd118532cfdb8b) ([#8](https://github.com/noctisynth/tyr/pull/8) by [@fu050409](https://github.com/noctisynth/tyr/../../fu050409)) Add support for ARP payload.
- [`415a5c4`](https://github.com/noctisynth/tyr/commit/415a5c421fcb2285a36720726f0ad39252e5bc5d) ([#6](https://github.com/noctisynth/tyr/pull/6) by [@fu050409](https://github.com/noctisynth/tyr/../../fu050409)) Add `get_num_threads` to calculate the number of threads to use by rated power.
- [`415a5c4`](https://github.com/noctisynth/tyr/commit/415a5c421fcb2285a36720726f0ad39252e5bc5d) ([#6](https://github.com/noctisynth/tyr/pull/6) by [@fu050409](https://github.com/noctisynth/tyr/../../fu050409)) Optimize SYN payload packet sequence.

## \[0.1.1]

### New Features

- [`3d37fc6`](https://github.com/noctisynth/tyr/commit/3d37fc6787e4621b0c401fbf7f360dce87fd2fc6) ([#4](https://github.com/noctisynth/tyr/pull/4) by [@fu050409](https://github.com/noctisynth/tyr/../../fu050409)) Support for get default interface.

### Refactors

- [`3d37fc6`](https://github.com/noctisynth/tyr/commit/3d37fc6787e4621b0c401fbf7f360dce87fd2fc6) ([#4](https://github.com/noctisynth/tyr/pull/4) by [@fu050409](https://github.com/noctisynth/tyr/../../fu050409)) Refactor constant values to `constant.rs`.
- [`3d37fc6`](https://github.com/noctisynth/tyr/commit/3d37fc6787e4621b0c401fbf7f360dce87fd2fc6) ([#4](https://github.com/noctisynth/tyr/pull/4) by [@fu050409](https://github.com/noctisynth/tyr/../../fu050409)) Refactor `Error` struct using `#[error]` attribute.
- [`293428e`](https://github.com/noctisynth/tyr/commit/293428eeb735023f853c0d775b2f44d6bd5360c3) ([#1](https://github.com/noctisynth/tyr/pull/1) by [@fu050409](https://github.com/noctisynth/tyr/../../fu050409)) Refactor payload to `payload` directory and use `mod.rs` to export all modules.
- [`b49f1df`](https://github.com/noctisynth/tyr/commit/b49f1df5d718b76a5323f771455b605b7432c672) ([#3](https://github.com/noctisynth/tyr/pull/3) by [@fu050409](https://github.com/noctisynth/tyr/../../fu050409)) Move methods inside interface and lib to `util.rs`.
