# DDG Follow-ups

The CLI now follows the web implementation's sender modes and validation behavior.
The follow-ups below are implemented.

## Completed

- [x] Replace the hardcoded default sender with `DDG_DEFAULT_SENDER`.
  Missing, blank, and invalid values produce configuration errors.
- [x] Require custom and configured senders to be valid `@duck.com` addresses.
  Recipients may use any email provider.
- [x] Normalize generated aliases to full sender addresses before conversion.
  Both a local part and an already complete address are accepted; malformed or
  non-Duck API addresses are rejected. Standalone generation does not duplicate
  the domain suffix.
- [x] Require `DDG_BEARER` only when generating an alias.
- [x] Consolidate the Rust email validators and match the Worker's validation
  behavior, including trimming, hyphenated local parts, long domains, complete
  input matching, and case-insensitive Duck domains.
- [x] Cover configured and missing defaults, invalid custom senders, generated
  conversion output, and local conversions without a bearer token with tests.
- [x] Document the environment variables and sender modes in the README and CLI help.

## Implementation decisions

- Use the existing Worker configuration name, `DDG_DEFAULT_SENDER`, rather than
  introducing a config file or a new secret source.
- Share validation between CLI parsing and runtime conversion in
  `lifestuff-types::ddg::convert`. The TypeScript Worker remains a separate
  implementation; Rust regression cases cover the corresponding behavior.
- Require exactly one of `--use-default`, `--generate`, and `--sender`.
