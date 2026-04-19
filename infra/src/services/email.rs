// use crate::settings::{EnvironmentSettings, SmtpSettings};
// use lettre::message::header::ContentType;
// use lettre::transport::smtp::authentication::Credentials;
// use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

// #[derive(Debug, derive_more::Display, derive_more::Error)]
// pub enum EmailError {
//     #[display("SMTP transport error: {_0}")]
//     Smtp(#[error(not(source))] String),
//     #[display("Email address parse error: {_0}")]
//     Address(#[error(not(source))] String),
//     #[display("Email message build error: {_0}")]
//     MessageBuild(#[error(not(source))] String),
// }

// #[derive(Clone)]
// pub struct EmailService {
//     transport: AsyncSmtpTransport<Tokio1Executor>,
//     sender: String,
// }

// impl EmailService {
//     pub fn new(settings: &SmtpSettings) -> Self {
//         let creds = Credentials::new(settings.user.clone(), settings.pass.clone());

//         let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&settings.host)
//             .expect("Failed to create SMTP transport")
//             .port(settings.port)
//             .credentials(creds)
//             .build();

//         Self {
//             transport,
//             sender: settings.emailer.clone(),
//         }
//     }

//     pub async fn send(
//         &self,
//         to: String,
//         subject: String,
//         body: String,
//         settings: EnvironmentSettings,
//     ) -> Result<(), EmailError> {
//         let from = lettre::message::Mailbox::new(
//             Some(settings.name.to_string()),
//             self.sender
//                 .parse::<lettre::Address>()
//                 .map_err(|e: lettre::address::AddressError| EmailError::Address(e.to_string()))?,
//         );
//         let to = to
//             .parse()
//             .map_err(|e: lettre::address::AddressError| EmailError::Address(e.to_string()))?;

//         let email = Message::builder()
//             .from(from)
//             .to(to)
//             .subject(subject)
//             .header(ContentType::TEXT_HTML)
//             .body(body)
//             .map_err(|e| EmailError::MessageBuild(e.to_string()))?;

//         self.transport
//             .send(email)
//             .await
//             .map_err(|e| EmailError::Smtp(e.to_string()))?;

//         Ok(())
//     }
// }
