# sesamo

<p align="center">Developed by <a href="https://veeso.dev/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.1.0 (29/09/2023)</p>

<p align="center">
  <a href="https://opensource.org/license/mit/"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso-dev/sesamo/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso-dev/sesamo.svg"
      alt="Repo stars"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso-dev/sesamo/actions"
    ><img
      src="https://github.com/veeso-dev/sesamo/workflows/build-test/badge.svg"
      alt="Linux CI"
  /></a>
</p>

---

- [sesamo](#sesamo)
  - [About sesamo](#about-sesamo)
  - [Get started](#get-started)
    - [Setup env](#setup-env)
    - [Setup AWS credentials](#setup-aws-credentials)
    - [Run with Cargo make](#run-with-cargo-make)
  - [sesamo API](#sesamo-api)
    - [Check](#check)
    - [Send](#send)
  - [Changelog](#changelog)
  - [License](#license)

---

## About sesamo

sesamo is a Rust web service which exposes an endpoint to send custom email to custom recipients.

---

## Get started

### Setup env

```sh
cp .env.test .env
vim .env
```

```env
EMAIL_SENDER=EMAIL_ADDRESS_CONFIGURED_TO_AWS_SES
LISTEN_URL=127.0.0.1:3001
PIDFILE=/var/run/sesamo.pid
```

### Setup AWS credentials

AWS credentials must be configured with aws-cli with `aws configure`.
Credentials are then automatically loaded from defined profile

### Run with Cargo make

```sh
cargo make -p production run
```

## sesamo API

### Check

Check web service status:

```txt
GET /check
```

Response: Empty (200)

### Send

Send email to recipients with provided subject and body

```txt
POST /send
```

Body:

```json
{
  "recipients": ["test@test.com", "foo@bar.com"],
  "subject": "email subject",
  "body": "BODY BASE64 ENCODED"
}
```

Response: Empty (200)

---

## Changelog

View sesamo's changelog [HERE](CHANGELOG.md)

---

## License

sesamo is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
