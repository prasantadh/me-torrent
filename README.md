# BitTorrent Implementation

This repo provides demo code for implementing bit torrent client
to be used as a template/sample for computer networks class
as taught in Summer, 2024 at Western Regional Campus (WRC), Pokhara.

## References

The code here is written with reference from

- [CodeCrafters Project](https://app.codecrafters.io/courses/bittorrent/overview)
  - This provided a step-by-step outline for steps necessary to download a file
  using bittorrent. The project also provided a sample torrent file and the associated
  tracker+peers necessary for the demo.
  - The repo does **not** aim to pass the tests for codecrafters but simply uses
  the service for guidelines on necessary steps.
- [Bittorrent Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html#peer-messages)
  - This being the official documentation, helps verify the information on codecrafters.
  However, as seen on online commentary, the specification is somewhat underspecified
  with information needed spread out in different parts.
- [Implementing (part of) a Bittorrent client](https://www.youtube.com/watch?v=jf_ddGnum_4)
  - Big fan of the creator. The stream itself is a delight to watch. However, the
  implementation provided there is much more complex than the template here. The
  aim of this template is to provide starting point to students.
  - The video links to [the github implementation](https://github.com/codecrafters-io/build-your-own-bittorrent) that was downloaded and run which helped inspect the packet formats.
- [crate:serde-bencode parse torrent example](https://github.com/toby/serde-bencode/blob/master/examples/parse_torrent.rs)
  - This example is used almost verbatim to parse the torrent file.

## Usage

Install rust (recommended source: [https://rustup.rs](https://rustup.rs))

```bash
git clone git@github.com:prasantadh/me-torrent.git 
cd me-torrent
cargo run < sample.torrent
```

## Future Work

- This implementation only provides demo up to downloading a piece.
Downloading multiple pieces is left for the future.
- Assumes that each peer has all the pieces (guaranteed by codecrafters).
Selecting peer is left for the future.
- All operations are synchronous. Asynchronous operations are left for the future.
- codecrafters only provided server for a single file torrent.
supporting multifile torrent is left for the future.
