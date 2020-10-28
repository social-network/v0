[![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](LICENSE) [![GitLab Status](https://gitlab.parity.io/parity/substrate/badges/master/pipeline.svg)](https://gitlab.parity.io/parity/substrate/pipelines) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](docs/CONTRIBUTING.adoc)

<p align="center">
  <img src="/docs/media/cover.png">
</p>

Social network substrate is a next-generation governance, financial, and social system built for the new internet (also known as Web 3.0) 🌍🌎🌏. For more info visit substrate.social.network.

## Running a node

Simply go to [substrate.dev](https://substrate.dev) and follow the [installation](https://substrate.dev/docs/en/knowledgebase/getting-started/) instructions.

## Contributions & Code of Conduct

Please follow the contributions guidelines as outlined in [`docs/CONTRIBUTING.adoc`](docs/CONTRIBUTING.adoc). In all communications and contributions, this project follows the [Contributor Covenant Code of Conduct](docs/CODE_OF_CONDUCT.adoc).

## Security

The security policy and procedures can be found in [`docs/SECURITY.md`](docs/SECURITY.md).

## License

- Substrate Primitives (`sp-*`), Frame (`frame-*`) and the pallets (`pallets-*`), binaries (`/bin`) and all other utilities are licensed under [Apache 2.0](LICENSE-APACHE2).
- Substrate Client (`/client/*` / `sc-*`) is licensed under [GPL v3.0 with a classpath linking exception](LICENSE-GPL3).

## Development

### Type definitions

```
{
  "AttributeTransaction": {
    "signature": "Signature",
    "name": "Vec<u8>",
    "value": "Vec<u8>",
    "validity": "u32",
    "signer": "AccountId",
    "identity": "AccountId"
  },
  "Attribute": {
    "name": "Vec<u8>",
    "value": "Vec<u8>",
    "validity": "BlockNumber",
    "creation": "Moment",
    "nonce": "u64"
  },
  "TokenId": "u64",
  "SwapId": "u64",
  "TokenBalance": "u64",
  "MissionTokenId": "u32"
}
```
