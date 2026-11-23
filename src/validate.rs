use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct ResponseSuccess {
    // The email address you are validating.
    pub address: String,
    // [valid, invalid, catch-all, unknown, spamtrap, abuse, do_not_mail]
    pub status: ResponseSuccessStatus,
    // [antispam_system, greylisted, mail_server_temporary_error, forcible_disconnect, mail_server_did_not_respond, timeout_exceeded, failed_smtp_connection, mailbox_quota_exceeded, exception_occurred, possible_trap, role_based, global_suppression, mailbox_not_found, no_dns_entries, failed_syntax_check, possible_typo, unroutable_ip_address, leading_period_removed, does_not_accept_mail, alias_address, role_based_catch_all, disposable, toxic]
    pub sub_status: String,
    // [true/false] If the email comes from a free provider.
    pub free_email: bool,
    // Suggestive Fix for an email typo
    pub did_you_mean: Option<String>,
    // The portion of the email address before the "@" symbol or null.
    pub account: Option<String>,
    // The portion of the email address after the "@" symbol or null.
    pub domain: Option<String>,
    // Age of the email domain in days or [null].
    pub domain_age_days: Option<String>,
    //The SMTP Provider of the email or [null] [BETA].
    pub smtp_provider: Option<String>,
    // The preferred MX record of the domain
    pub mx_record: Option<String>,
    // [true/false] Does the domain have an MX record. [they return "bool" not bool, which makes it a string...]
    pub mx_found: Option<String>,
    // The first name of the owner of the email when available or [null].
    pub firstname: Option<String>,
    // The last name of the owner of the email when available or [null].
    pub lastname: Option<String>,
    // The gender of the owner of the email when available or [null].
    pub gender: Option<String>,
    // The country of the IP passed in or [null]
    pub country: Option<String>,
    // The region/state of the IP passed in or [null]
    pub region: Option<String>,
    // The city of the IP passed in or [null]
    pub city: Option<String>,
    // The zipcode of the IP passed in or [null]
    pub zipcode: Option<String>,
    // The UTC time the email was validated.
    pub processed_at: String,
}

#[derive(Deserialize, Debug)]
pub enum ResponseSuccessStatus {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "catch-all")]
    CatchAll,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "spamtrap")]
    Spamtrap,
    #[serde(rename = "abuse")]
    Abuse,
    #[serde(rename = "do_not_mail")]
    DoNotMail,
}

impl ResponseSuccessStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ResponseSuccessStatus::Valid => "valid",
            ResponseSuccessStatus::Invalid => "invalid",
            ResponseSuccessStatus::CatchAll => "catch_all",
            ResponseSuccessStatus::Unknown => "unknown",
            ResponseSuccessStatus::Spamtrap => "spamtrap",
            ResponseSuccessStatus::Abuse => "abuse",
            ResponseSuccessStatus::DoNotMail => "do_not_mail",
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(self, ResponseSuccessStatus::Valid)
    }

    pub fn is_invalid(&self) -> bool {
        matches!(self, ResponseSuccessStatus::Invalid)
    }

    pub fn is_catch_all(&self) -> bool {
        matches!(self, ResponseSuccessStatus::CatchAll)
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, ResponseSuccessStatus::Unknown)
    }

    pub fn is_spamtrap(&self) -> bool {
        matches!(self, ResponseSuccessStatus::Spamtrap)
    }

    pub fn is_abuse(&self) -> bool {
        matches!(self, ResponseSuccessStatus::Abuse)
    }

    pub fn is_do_not_mail(&self) -> bool {
        matches!(self, ResponseSuccessStatus::DoNotMail)
    }
}

impl From<String> for ResponseSuccessStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "valid" => ResponseSuccessStatus::Valid,
            "invalid" => ResponseSuccessStatus::Invalid,
            "catch_all" => ResponseSuccessStatus::CatchAll,
            "unknown" => ResponseSuccessStatus::Unknown,
            "spamtrap" => ResponseSuccessStatus::Spamtrap,
            "abuse" => ResponseSuccessStatus::Abuse,
            "do_not_mail" => ResponseSuccessStatus::DoNotMail,
            _ => ResponseSuccessStatus::Unknown,
        }
    }
}
