# Rust-template

<p align="center">~  ~</p>
<p align="center">
  <a href="#get-started-">Get started</a>
  ·
  <a href="https://crates.io/crates/rust-template" target="_blank">Crates.io</a>
</p>
<p align="center">Developed by <a href="https://veeso.dev/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.1.0 (26/06/2023)</p>

<p align="center">
  <a href="https://opensource.org/license/mit/"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso-dev/rust-template/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso-dev/rust-template.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/rust-template"
    ><img
      src="https://img.shields.io/crates/d/rust-template.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/rust-template"
    ><img
      src="https://img.shields.io/crates/v/rust-template.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso-dev/rust-template/actions"
    ><img
      src="https://github.com/veeso-dev/rust-template/workflows/build-test/badge.svg"
      alt="Linux CI"
  /></a>
</p>

---

- [Rust-template](#rust-template)
  - [About rust-template](#about-rust-template)
  - [Get started](#get-started)
    - [Run with docker](#run-with-docker)
  - [rust-template API](#rust-template-api)
    - [Check](#check)
  - [Support the developer](#support-the-developer)
  - [Contributing and issues](#contributing-and-issues)
  - [Changelog](#changelog)
  - [License](#license)

---

## About rust-template

rust-template is a Rust web service which comes integrated with ClamAV. The service provides an API endpoint to scan files with ClamAV.

---

## Get started

### Run with docker

The entire rust-template web service comes with a docker compose file to easily run the service on your machine.
Just run:

```sh
docker-compose build
docker-compose up -d
```

At this point rust-template will be served on the specified port in the docker-compose file. (Default: `3010`)

## rust-template API

### Check

Check web service status:

```txt
GET /check
```

Response:

```json
{
  "status": "ok"
}
```

---

## Support the developer

If you like rust-template and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Contributing and issues

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve pavao, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog

View rust-template's changelog [HERE](CHANGELOG.md)

---

## License

rust-template is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
