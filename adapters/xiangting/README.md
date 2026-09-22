# xiangting Adapter

This crate adapts `xiangting` to the shared benchmark harness.

## Upstream and pinned version

- Source: <https://github.com/Apricot-S/xiangting>
- Version: 6.0.2

## License

MIT License. See the workspace `THIRD-PARTY-NOTICES.md`.

## Adapter configuration

- Four-player rules (`PlayerCount::Four`)
- The upstream public replacement-number API

## Calculation scope

The result is the minimum over the general form, Seven Pairs, and Thirteen Orphans.
The public upstream function cannot be restricted to the general form.

## Validation

Upstream input validation is enabled by the public API.

## Result meaning

The upstream replacement number is converted to the conventional shanten number by subtracting one.
Because special hand forms and validation are included, benchmark timings also include their overhead.
