Raider
======

[![Build Status](https://travis-ci.org/valeriansaliou/raider.svg?branch=master)](https://travis-ci.org/valeriansaliou/raider)

**Affiliates dashboard. Used by affiliates to generate tracking codes and review their balance.**

Raider is easy to integrate in your existing system. You can also customize the dashboard look & feel with templates and styles. It can be used as a self-service affiliates system, for your affiliate users to manage their account, create tracking URLs, review their balance and request for payouts.

![Raider](https://valeriansaliou.github.io/raider/images/raider.png)

## Who uses it?

<table>
<tr>
<td align="center"><a href="https://crisp.chat/"><img src="https://valeriansaliou.github.io/raider/images/crisp-icon.png" height="64" /></a></td>
</tr>
<tr>
<td align="center">Crisp</td>
</tr>
</table>

_👋 You use Raider and you want to be listed there? [Contact me](https://valeriansaliou.name/)._

## Features

* **Self-service affiliates dashboard**
* **Users can generate affiliates tracking codes**
* **Users can see their affiliates statistics** (eg. how much money they made)
* **Users can request for payouts** (you then receive a notification email)
* **Your backend reports referred customer payments to Raider**

## How does it work?

Raider provides a self-service affiliates dashboard on which users can sign up, login, and manage their account (eg. create tracking codes, request for payouts, etc.). Your backend can report referred customer payments to Raider, so that the affiliates can cash out their commission and request for a payout at any point.

**Raider provides two services:**

* **Self-service dashboard**: Used by your affiliates users
* **Payment reporting API**: Called by your backend once a payment is made (ie. to credit due commission money to an affiliate)

## How to use it?

### Installation

**Install from releases:**

The best way to install Raider is to pull the latest release from the [Raider releases](https://github.com/valeriansaliou/raider/releases) page.

Make sure to pick the correct server architecture (eg. Intel 32 bits).

**Install from Cargo:**

If you prefer managing `raider` via Rust's Cargo, install it directly via `cargo install`:

```bash
cargo install raider-server
```

Ensure that your `$PATH` is properly configured to source the Crates binaries, and then run Raider using the `raider` command.

**Install from sources:**

The last option is to pull the source code from Git and compile Raider via `cargo`:

```bash
cargo build --release
```

You can find the built binaries in the `./target/release` directory.

_Install the `libssl-dev` (ie. OpenSSL headers) before you compile Raider. SSL dependencies are required for email notifications._

### Configuration

Use the sample [config.cfg](https://github.com/valeriansaliou/raider/blob/master/config.cfg) configuration file and adjust it to your own environment.

**⚠️ Important: Make sure to change the default `server.secret_key` configuration value with a secret key you generated. Failing to do so will make your Raider instance insecure.**

**Available configuration options are commented below, with allowed values:**

**[server]**

* `log_level` (type: _string_, allowed: `debug`, `info`, `warn`, `error`, default: `warn`) — Verbosity of logging, set it to `error` in production
* `inet` (type: _string_, allowed: IPv4 / IPv6 + port, default: `[::1]:8080`) — Host and TCP port the Raider service should listen on
* `workers` (type: _integer_, allowed: any number, default: `4`) — Number of workers for the Raider service to run on
* `secret_key` (type: _string_, allowed: 192-bit base64 encoded secret key, default: no default) — Secret key for cookie encryption (see [Rocket docs](https://api.rocket.rs/rocket/struct.Config.html#method.set_secret_key) for details)

### Run Raider

Raider can be run as such:

`./raider -c /path/to/config.cfg`

## :fire: Report A Vulnerability

If you find a vulnerability in Raider, you are more than welcome to report it directly to [@valeriansaliou](https://github.com/valeriansaliou) by sending an encrypted email to [valerian@valeriansaliou.name](mailto:valerian@valeriansaliou.name). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Raider server.

**:warning: You must encrypt your email using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc).**

**:gift: Based on the severity of the vulnerability, I may offer a $100 (US) bounty to whomever reported it.**
