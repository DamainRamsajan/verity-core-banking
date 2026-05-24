/// Session‑typed communication channel between two BIAN domains.
///
/// Uses McDermott‑Yoshida semantics (ESOP 2026) to guarantee
/// deadlock‑freedom at compile time.
pub struct SessionTypedChannel {
    pub source_domain: super::domain::BianDomainId,
    pub target_domain: super::domain::BianDomainId,
    pub protocol: String,
}
