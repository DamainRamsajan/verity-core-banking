# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Do not open a public issue.**  
Please email security@verity.io with a detailed report.

We will respond within 72 hours with a plan of action.

### Scope
- Verity Core Banking Platform (all crates)
- Verity Agent OS (VAOS)
- Cloudflare Workers & Supabase Edge Functions
- Mission Control Dashboard

### Out of Scope
- Demo/test deployments with `TEE_MODE=simulation`
- Third-party dependencies (please report to upstream)

## Security Model
Verity is built on **capability-based security** with hardware‑rooted trust.
For architectural details, see [VERITY_ARC42.md](./VERITY_ARC42.md).

### Bounty Program
We offer bounties for critical vulnerabilities affecting the sovereign core.
See [HackerOne](https://hackerone.com/verity) for details.
