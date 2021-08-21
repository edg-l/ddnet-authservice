use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport,
    Tokio1Executor, Transport,
};
use std::convert::TryFrom;

use crate::settings::SmtpConfig;

pub(crate) struct Mailer {
    pub transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl TryFrom<SmtpConfig> for Mailer {
    type Error = lettre::transport::smtp::Error;

    fn try_from(value: SmtpConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            transport: AsyncSmtpTransport::<Tokio1Executor>::relay(&value.host)?
                .credentials(Credentials::new(value.username, value.password))
                .port(value.port)
                .build(),
        })
    }
}
