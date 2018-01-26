// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::time::Duration;
use log;
use native_tls::TlsConnector;
use lettre::smtp::{ClientSecurity, SmtpTransportBuilder, SmtpTransport, ConnectionReuseParameters};
use lettre::smtp::authentication::Credentials;
use lettre::smtp::client::net::ClientTlsParameters;
use lettre::EmailTransport;
use lettre_email::EmailBuilder;

use APP_CONF;

pub struct EmailNotifier;

impl EmailNotifier {
    pub fn dispatch(to: &str, subject: String, body: &str) -> Result<(), bool> {
        // Build up the message text
        let mut message = String::new();

        message.push_str(body);
        message.push_str("\n\n--\n\n");

        message.push_str(&format!(
            "You receive this email because you have initiated an action on your {} account at: {}",
            APP_CONF.branding.page_title,
            APP_CONF.branding.page_url.as_str()
        ));

        log::debug!("will send email notification with message: {}", &message);

        // Build up the email
        let email_message = EmailBuilder::new()
            .to(to)
            .from((
                APP_CONF.email.from.as_str(),
                APP_CONF.branding.page_title.as_str(),
            ))
            .subject(subject)
            .text(message)
            .build()
            .or(Err(true))?;

        // Deliver the message
        return acquire_transport(
            &APP_CONF.email.smtp_host,
            APP_CONF.email.smtp_port,
            APP_CONF.email.smtp_username.to_owned(),
            APP_CONF.email.smtp_password.to_owned(),
            APP_CONF.email.smtp_encrypt,
        ).map(|mut transport| transport.send(&email_message))
            .and(Ok(()))
            .or(Err(true));
    }
}

fn acquire_transport(
    smtp_host: &str,
    smtp_port: u16,
    smtp_username: Option<String>,
    smtp_password: Option<String>,
    smtp_encrypt: bool,
) -> Result<SmtpTransport, ()> {
    let mut security = ClientSecurity::None;

    if smtp_encrypt == true {
        if let Ok(connector_builder) = TlsConnector::builder() {
            if let Ok(connector) = connector_builder.build() {
                security = ClientSecurity::Required(ClientTlsParameters {
                    connector: connector,
                    domain: smtp_host.to_string(),
                });
            }
        }

        // Do not deliver email if TLS context cannot be acquired (prevents unencrypted emails \
        //   to be sent)
        if let ClientSecurity::None = security {
            log::error!("could not build smtp encrypted connector");

            return Err(());
        }
    }

    match SmtpTransportBuilder::new(format!("{}:{}", smtp_host, smtp_port), security) {
        Ok(transport) => {
            let mut transport_builder = transport
                .timeout(Some(Duration::from_secs(5)))
                .connection_reuse(ConnectionReuseParameters::NoReuse);

            match (smtp_username, smtp_password) {
                (Some(smtp_username_value), Some(smtp_password_value)) => {
                    transport_builder = transport_builder.credentials(Credentials::new(
                        smtp_username_value,
                        smtp_password_value,
                    ));
                }
                _ => {}
            }

            Ok(transport_builder.build())
        }
        Err(err) => {
            log::error!("could not acquire smtp transport: {}", err);

            Err(())
        }
    }
}
