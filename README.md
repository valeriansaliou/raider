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

### Database

Raider requires a MySQL to be running on your host (it is unfortunately not compatible with PostgreSQL and others, _at the moment_).

The Raider SQL schema should be imported in the Raider database you created, which you can find at [raider.sql](https://github.com/valeriansaliou/raider/blob/master/doc/fixtures/raider.sql).

### Configuration

Use the sample [config.cfg](https://github.com/valeriansaliou/raider/blob/master/config.cfg) configuration file and adjust it to your own environment.

---

**⚠️ Important: Make sure to change the default `server.secret_key` configuration value with a secret key you generated. Also, generate a random arbitrary length string for `database.password_salt`. Failing to change any of those values will make your Raider instance insecure.**

---

**Available configuration options are commented below, with allowed values:**

**[server]**

* `log_level` (type: _string_, allowed: `debug`, `info`, `warn`, `error`, default: `warn`) — Verbosity of logging, set it to `error` in production
* `inet` (type: _string_, allowed: IPv4 / IPv6 + port, default: `[::1]:8080`) — Host and TCP port the Raider service should listen on
* `workers` (type: _integer_, allowed: any number, default: `4`) — Number of workers for the Raider service to run on
* `secret_key` (type: _string_, allowed: 192-bit base64 encoded secret key, default: no default) — Secret key for cookie encryption (see [Rocket docs](https://api.rocket.rs/rocket/struct.Config.html#method.set_secret_key) for details)

**[database]**

* `url` (type: _string_, allowed: MySQL URL, no default) — URL of the MySQL database to connect to
* `pool_size` (type: _integer_, allowed: any number, default: `4`) — Number of connections to maintain to MySQL
* `idle_timeout` (type: _integer_, allowed: seconds, default: `300`) — Idle timeout in seconds to MySQL
* `connection_timeout` (type: _integer_, allowed: seconds, default: `10`) — Connection timeout in seconds to MySQL
* `password_salt` (type: _string_, allowed: any string, no default) — Password salt (preferably strong and long; do not change this after accounts got created as it will make them unusable)

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

* `currency` (type: _string_, allowed: currency code, default: `USD`) — Currency to be used for payouts (and balances in general)
* `administrator_email` (type: _string_, allowed: email address, no default) — Email address of the affiliates system administrator (payout request emails will be sent there)

### Run Raider

Raider can be run as such:

`./raider -c /path/to/config.cfg`

## :fire: Report A Vulnerability

If you find a vulnerability in Raider, you are more than welcome to report it directly to [@valeriansaliou](https://github.com/valeriansaliou) by sending an encrypted email to [valerian@valeriansaliou.name](mailto:valerian@valeriansaliou.name). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Raider server.

**:warning: You must encrypt your email using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc).**

**:gift: Based on the severity of the vulnerability, I may offer a $100 (US) bounty to whomever reported it.**
