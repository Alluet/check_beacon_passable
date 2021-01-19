# check_beacon_passable

This script allows you to generate a predicate that checks whether a beacon can emit a beam.

Note: This repository has not been updated since 1.15, so the block tags are missing some new blocks.
The script does not need to be updated, only the tag.

## Usage

Build from source and execute using [`cargo`](https://rust-lang.org).

```
$ cargo run
```

The required block tags are in `tags`, and upon running the script the predicate will be in `predicates`.

## License

`check_beacon_passable` is made freely available under the terms of the [BSD 0-Clause License](LICENSE).
Third-party contributions shall be licensed under the same terms unless explicitly stated otherwise.
