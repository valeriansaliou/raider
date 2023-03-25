Raider
======

[![Test and Build](https://github.com/valeriansaliou/raider/workflows/Test%20and%20Build/badge.svg?branch=master)](https://github.com/valeriansaliou/raider/actions?query=workflow%3A%22Test+and+Build%22) [![Build and Release](https://github.com/valeriansaliou/raider/workflows/Build%20and%20Release/badge.svg)](https://github.com/valeriansaliou/raider/actions?query=workflow%3A%22Build+and+Release%22) [![dependency status](https://deps.rs/repo/github/valeriansaliou/raider/status.svg)](https://deps.rs/repo/github/valeriansaliou/raider) [![Buy Me A Coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-donate-yellow.svg)](https://www.buymeacoffee.com/valeriansaliou)

**Affiliates dashboard. Used by affiliates to generate tracking codes and review their balance.**

Raider is easy to integrate in your existing system. You can also customize the dashboard look & feel with templates and styles. It can be used as a self-service affiliates system, for your affiliate users to manage their account, create tracking URLs, review their balance and request for payouts.

_Tested at Rust version: `rustc 1.51.0-nightly (c8915eebe 2021-01-07)`_

**🇭🇺 Crafted in Budapest, Hungary.**

![Raider](https://valeriansaliou.github.io/raider/images/raider.png)

## Who uses it?

<table>
<tr>
<td align="center"><a href="https://crisp.chat/"><img src="https://valeriansaliou.github.io/raider/images/crisp-icon.png" width="64" /></a></td>
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

👉 _Each release binary comes with an `.asc` signature file, which can be verified using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc)._

**Install from Cargo:**

If you prefer managing `raider` via Rust's Cargo, install it directly via `cargo install`:

```bash
cargo install raider-server
```

Ensure that your `$PATH` is properly configured to source the Crates binaries, and then run Raider using the `raider` command.

**Install from source:**

The last option is to pull the source code from Git and compile Raider via `cargo`:

```bash
cargo build --release
```

You can find the built binaries in the `./target/release` directory.

_Install the `libssl-dev` (ie. OpenSSL headers) and `libmysqlclient-dev` (ie. MySQL client headers) before you compile Raider. SSL dependencies are required for email notifications, and MySQL dependencies are required to connect to your database._

**Install from Docker Hub:**

You might find it convenient to run Raider via Docker. You can find the pre-built Raider image on Docker Hub as [valeriansaliou/raider](https://hub.docker.com/r/valeriansaliou/raider/).

First, pull the `valeriansaliou/raider` image:

```bash
docker pull valeriansaliou/raider:v1.2.3
```

Then, seed it a configuration file and run it (replace `/path/to/your/raider/config.cfg` with the path to your configuration file):

```bash
docker run -p 8080:8080 -v /path/to/your/raider/config.cfg:/etc/raider.cfg valeriansaliou/raider:v1.2.3
```

In the configuration file, ensure that:

* `server.inet` is set to `0.0.0.0:8080` (this lets Raider be reached from outside the container)
* `assets.path` is set to `./res/assets/` (this refers to an internal path in the container, as the assets are contained there)

Raider will be reachable from `http://localhost:8080`.

### Database

Raider requires a MySQL to be running on your host (it is unfortunately not compatible with PostgreSQL and others, _at the moment_).

The Raider SQL schema should be imported in the Raider database you created, which you can find at [raider.sql](https://github.com/valeriansaliou/raider/blob/master/doc/fixtures/raider.sql).

### Configuration

Use the sample [config.cfg](https://github.com/valeriansaliou/raider/blob/master/config.cfg) configuration file and adjust it to your own environment.

---

**⚠️ Important: Make sure to change the default `server.secret_key`, `server.track_token` and `server.management_token` configuration values with secret keys you generated. Also, generate a random arbitrary length string for `database.password_salt`. Failing to change any of those values will make your Raider instance insecure. You can easily create these tokens by running `openssl rand -base64 32`.**

---

**Available configuration options are commented below, with allowed values:**

**[server]**

* `log_level` (type: _string_, allowed: `debug`, `info`, `warn`, `error`, default: `error`) — Verbosity of logging, set it to `error` in production
* `inet` (type: _string_, allowed: IPv4 / IPv6 + port, default: `[::1]:8080`) — Host and TCP port the Raider service should listen on
* `workers` (type: _integer_, allowed: any number, default: `4`) — Number of workers for the Raider service to run on
* `track_token` (type: _string_, allowed: secret token, default: no default) — Track API secret token (ie. secret password)
* `management_token` (type: _string_, allowed: secret token, default: no default) — Management API secret token (ie. secret password)
* `secret_key` (type: _string_, allowed: 192-bit base64 encoded secret key, default: no default) — Secret key for cookie encryption (see [Rocket docs](https://api.rocket.rs/rocket/struct.Config.html#method.set_secret_key) for details)

**[database]**

* `url` (type: _string_, allowed: MySQL URL, no default) — URL of the MySQL database to connect to
* `pool_size` (type: _integer_, allowed: any number, default: `4`) — Number of connections to maintain to MySQL
* `idle_timeout` (type: _integer_, allowed: seconds, default: `300`) — Idle timeout in seconds to MySQL
* `connection_timeout` (type: _integer_, allowed: seconds, default: `10`) — Connection timeout in seconds to MySQL
* `password_salt` (type: _string_, allowed: any string, no default) — Password salt (preferably strong and long; do not change this after accounts got created as it will make them unusable)
* `account_create_allow` (type: _boolean_, allowed: `true`, `false`, default: `true`) — Whether to allow accounts to be created or not

**[exchange]**

**[exchange.fixer]**

* `endpoint` (type: _string_, allowed: any string, default: `https://api.apilayer.com/fixer`) — Fixer API endpoint (on APILayer)
* `api_key` (type: _string_, allowed: any string, no default) — APILayer API key (for Fixer)

**[email]**

* `from` (type: _string_, allowed: email address, no default) — Email address from which to send emails
* `smtp_host` (type: _string_, allowed: hostname, IPv4, IPv6, default: `localhost`) — SMTP host to connect to
* `smtp_port` (type: _integer_, allowed: TCP port, default: `587`) — SMTP TCP port to connect to
* `smtp_username` (type: _string_, allowed: any string, no default) — SMTP username to use for authentication (if any)
* `smtp_password` (type: _string_, allowed: any string, no default) — SMTP password to use for authentication (if any)
* `smtp_encrypt` (type: _boolean_, allowed: `true`, `false`, default: `true`) — Whether to encrypt SMTP connection with `STARTTLS` or not

**[assets]**

* `path` (type: _string_, allowed: UNIX path, default: `./res/assets/`) — Path to Raider assets directory

**[branding]**

* `page_title` (type: _string_, allowed: any string, default: `Affiliates`) — Affiliates system title
* `page_url` (type: _string_, allowed: URL, no default) — Affiliates system URL
* `help_url` (type: _string_, allowed: URL, no default) — Help URL to be used in dashboard (ie. knowledge base where users can search for help)
* `support_url` (type: _string_, allowed: URL, no default) — Support URL to be used in dashboard (ie. where users can contact you if something is wrong)
* `icon_color` (type: _string_, allowed: hexadecimal color code, no default) — Icon color (ie. your icon background color)
* `icon_url` (type: _string_, allowed: URL, no default) — Icon URL, the icon should be your squared logo, used as favicon (PNG format recommended)
* `logo_white_url` (type: _string_, allowed: URL, no default) — Logo URL, the logo should be your full-width logo, used as login, signup & account recover form logo (whiter logo, SVG format recommended)
* `logo_dark_url` (type: _string_, allowed: URL, no default) — Logo URL, the logo should be your full-width logo, used as dashboard header logo (darker logo, SVG format recommended)
* `custom_html` (type: _string_, allowed: HTML, default: empty) — Custom HTML to include in affiliates system `head` (optional)

**[tracker]**

* `track_url` (type: _string_, allowed: tracker URL, no default) — Tracker URL, to which tracker links will point to
* `track_parameter` (type: _string_, allowed: tracker query parameter, default: `t`) — Tracker query parameter used in URL (eg. `?t=xDJSas10`)
* `commission_default` (type: _float_, allowed: percentage from `0.00` to `1.00`, default: `0.20`) — Default commission percentage (for new accounts)

**[[tracker.banner]]**

* `banner_url` (type: _string_, allowed: image URL, no default) — URL to the banner image
* `size_width` (type: _integer_, allowed: image size in pixels, no default) — Width of the banner (in pixels)
* `size_height` (type: _integer_, allowed: image size in pixels, no default) — Height of the banner (in pixels)

**[payout]**

* `currency` (type: _string_, allowed: currency code, default: `EUR`) — Currency to be used for payouts (and balances in general)
* `amount_minimum` (type: _float_, allowed: any number, default: `100.00`) — Minimum amount for payout requests
* `administrator_email` (type: _string_, allowed: email address, no default) — Email address of the affiliates system administrator (payout request emails will be sent there)

### Run Raider

Raider can be run as such:

`./raider -c /path/to/config.cfg`

## How can I integrate Raider reporting in my code?

When a payment for which you have a `tracking_id` is made on your platform (ie. a payment for a customer that was referred by an affiliate); your backend needs to submit this payment to the Raider tracking API. The full payment amount needs to be submitted, as the commission percentage is applied by Raider itself.

### Raider reporting libraries

* **Python**: **[py-raider-reporter](https://pypi.org/project/py-raider-reporter/)**

👉 Cannot find the library for your programming language? Build your own and be referenced here! ([contact me](https://valeriansaliou.name/))

## How can I use Raider HTTP APIs?

### 1️⃣ Track API

#### Payment tracking

In case you need to manually report tracked payments to the Raider endpoint, use the following HTTP configuration (adjust it to yours):

**Endpoint URL:**

`HTTP POST https://affiliates.example.com/track/payment/<tracking_id>/`

Where:

* `tracking_id`: The tracking identifier associated to customer who paid

**Request headers:**

* Add an `Authorization` header with a `Basic` authentication where the password is your configured `server.track_token`.

**Request data:**

Adjust the request data to your payment context and send it as `HTTP POST`:

```json
{
  "amount": 95.00,
  "currency": "EUR",
  "trace": "Plan: Unlimited; Customer: valerian@crisp.chat; Website: crisp.chat"
}
```

Where:

* `amount`: The full amount of the payment (Raider process the commission amount itself, eg. with `20%` commission you send `100.00` and Raider processes it as `20.00`)
* `currency`: The payment currency code (if the currency is different than the default currency configured with `payout.currency`, a conversion is applied using current day market rates)
* `trace`: An optional trace value which is logged in the database (may be used for your own records; this is never visible to your affiliate users)

#### Signup tracking

In case you need to manually report tracked signups to the Raider endpoint, use the following HTTP configuration (adjust it to yours):

**Endpoint URL:**

`HTTP POST https://affiliates.example.com/track/signup/<tracking_id>/`

Where:

* `tracking_id`: The tracking identifier associated to customer who signed up

**Request headers:**

* Add an `Authorization` header with a `Basic` authentication where the password is your configured `server.track_token`.

### 2️⃣ Management API

#### Account creation

In case you need to create accounts in Raider database from a third-party system in your infrastructure (eg. if regular signups are disabled), you may us create new accounts via the Raider endpoint, use the following HTTP configuration (adjust it to yours):

**Endpoint URL:**

`HTTP POST https://affiliates.example.com/management/account/`

**Request headers:**

* Add an `Authorization` header with a `Basic` authentication where the password is your configured `server.management_token`.

**Request data:**

Adjust the request data to your payment context and send it as `HTTP POST`:

```json
{
  "email": "john.doe@gmail.com",
  "full_name": "John Doe",
  "address": "1 Market Street, San Francisco, CA",
  "country": "US"
}
```

Where:

* `email`: The email address for the new account (an auto-generated password will be sent to this email)
* `full_name`: An optional full name value to preconfigure in the created account
* `address`: An optional address value to preconfigure in the created account
* `country`: An optional country value to preconfigure in the created account

## :fire: Report A Vulnerability

If you find a vulnerability in Raider, you are more than welcome to report it directly to [@valeriansaliou](https://github.com/valeriansaliou) by sending an encrypted email to [valerian@valeriansaliou.name](mailto:valerian@valeriansaliou.name). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Raider server.

**:warning: You must encrypt your email using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc).**
