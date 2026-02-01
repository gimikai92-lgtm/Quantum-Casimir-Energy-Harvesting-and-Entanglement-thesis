# Security Policy

## Purpose

This document outlines security policies and responsible disclosure procedures for the Quantum-Casimir Energy Harvesting thesis repository.

---

## Scope

This security policy applies to:
- GitHub repository and code
- Experimental data and measurements
- Intellectual property and patents
- Author credentials and institutional information

---

## Reporting Security Issues

### Responsible Disclosure

If you discover a security vulnerability, **do NOT** create a public GitHub issue. Instead:

1. **Email:** [author.email@institution.edu]
   - Subject: "SECURITY: [Brief description]"
   - Include detailed description of the vulnerability
   - Do NOT include public exploit code initially

2. **What to Include:**
   - Type of vulnerability (e.g., data exposure, unauthorized access)
   - Location (file, directory, system)
   - Steps to reproduce
   - Potential impact
   - Suggested remediation
   - Your contact information (optional)

3. **Response Timeline:**
   - Initial response: 48 hours
   - Detailed assessment: 1 week
   - Remediation plan: 2 weeks
   - Public disclosure: 30 days after fix (if applicable)

---

## Data Security

### Personal Data Protection

This repository contains:
- ✓ Public research thesis
- ✓ Experimental methodology
- ✓ Published measurements
- ✗ Personal information (minimal - author name only)

**Data Protection Compliance:**
- GDPR compliant (minimal personal data)
- No sensitive personal information stored
- No authentication credentials in repository
- No institutional secrets disclosed

### Experimental Data Security

Confidential elements:
- ✗ Proprietary fabrication details (protected under trade secret)
- ✗ Unreleased patent information (in patent application)
- ✓ Published experimental results (available in thesis)
- ✓ General methodologies (available in appendices)

**Handling Confidential Information:**
- Do not disclose proprietary techniques
- Do not publicize unreleased patent details
- Do not share unreleased experimental findings
- Contact author before discussing sensitive research

---

## Intellectual Property Security

### Copyright Protection

This work is protected by:
- **Copyright:** © 2026 Rolando M Garcia (All rights reserved)
- **License:** CC BY-NC-ND 4.0 (no derivatives allowed)
- **Patents:** Patent-pending (see LICENSE)

**Your Obligations:**
- Do not remove copyright notices
- Do not claim authorship
- Do not modify or create derivatives
- Do not use commercially without permission

### Patent Protection

**Current Status:** Patent-pending

**Restrictions:**
- Do not implement patented technology without license
- Do not file competing patents based on this work
- Do not attempt to design-around patents
- Report patent infringement attempts

**To Report Patent Issues:**
- Contact: [licensing@institution.edu]
- Include: evidence of infringement, jurisdiction, details

---

## Repository Security

### Access Control

**Public Repository:**
- ✓ Anyone can view (read-only)
- ✓ Anyone can clone
- ✗ Credentials required for write access
- ✗ Branch protection enabled

**Contributor Access:**
- Granted by author only
- Requires Code of Conduct agreement
- Subject to license terms
- Can be revoked for violations

### Commit Integrity

**Git Security:**
- ✓ Commits are signed (GPG verification recommended)
- ✓ Git history is complete and immutable
- ✗ No sensitive data in commits
- ✓ Regular backups maintained

### API & Token Security

**Credentials:**
- ✗ No API keys in repository
- ✗ No authentication tokens stored
- ✗ No personal credentials
- ✓ Use `.gitignore` for secrets

**If You Accidentally Expose Credentials:**
1. Immediately notify: [author.email@institution.edu]
2. Rotate compromised credentials
3. Report via GitHub security advisories
4. Document the incident

---

## Vulnerability Categories

### High Severity

Examples of critical issues:
- Unauthorized access to confidential data
- Data breach or theft
- Malware or malicious code injection
- Patent violations
- Copyright infringement

**Response:** Immediate investigation and mitigation

### Medium Severity

Examples of important issues:
- Broken access controls
- Exposed non-sensitive metadata
- Misleading licensing information
- Outdated security practices

**Response:** Assessment within 1 week

### Low Severity

Examples of minor issues:
- Documentation errors
- Typos or formatting
- Outdated links
- Minor improvements

**Response:** Standard issue tracking

---

## Specific Threats

### Code Injection

**Risk:** Malicious code in Python scripts

**Mitigation:**
- Code review before merge
- Dependency scanning for vulnerabilities
- Input validation in simulations
- Regular security audits

### Data Tampering

**Risk:** Modification of experimental data

**Mitigation:**
- SHA-256 checksums (see `.thesis_signatures.txt`)
- Git commit hashing
- Immutable archive storage
- Regular verification

### Intellectual Property Theft

**Risk:** Unauthorized commercial use

**Mitigation:**
- Clear licensing terms (LICENSE file)
- Copyright notices on all content
- Patent applications filed
- Legal enforcement available

### Impersonation

**Risk:** Someone claiming to be author

**Mitigation:**
- Verify through GitHub verified commits
- Check institutional email
- Request verification with author
- Report false claims to GitHub

---

## Security Best Practices

### For Users

✓ **Do:**
- Verify checksums of downloaded files
- Review license terms before use
- Respect copyright and IP protections
- Report suspicious activity
- Validate code before running

✗ **Don't:**
- Use credentials in scripts or notebooks
- Store sensitive data in forks
- Share confidential information publicly
- Attempt unauthorized access
- Distribute modified versions without permission

### For Contributors

✓ **Do:**
- Follow Code of Conduct
- Review security guidelines
- Report security issues responsibly
- Use strong authentication
- Sign commits with GPG

✗ **Don't:**
- Commit credentials or secrets
- Disclose unreleased patents
- Share confidential methodology
- Use malicious code
- Violate license terms

---

## Incident Response

### Discovery & Report

1. **Identify** the security issue
2. **Document** specifics and impact
3. **Report** to [author.email@institution.edu]
4. **Do NOT** publicly disclose initially

### Investigation & Assessment

1. Confirm the vulnerability
2. Assess impact and severity
3. Identify affected systems
4. Determine root cause
5. Develop remediation plan

### Remediation & Fix

1. Implement security fix
2. Test solution thoroughly
3. Deploy correction
4. Verify fix effectiveness
5. Update affected systems

### Disclosure & Communication

1. Notify affected users
2. Provide mitigation guidance
3. Release security update
4. Publish advisory (if appropriate)
5. Document lessons learned

---

## Supported Versions

**Current Version:** 1.0 (Released: February 1, 2026)

| Version | Status | Security Updates |
|---------|--------|-----------------|
| 1.0+ | Current | Active |
| < 1.0 | Deprecated | Limited support |

Security updates are provided for current version. For older versions, upgrade recommended.

---

## Security Audit & Monitoring

### Regular Reviews

- **Quarterly:** Dependency security scanning
- **Bi-annually:** Code security audit
- **Annually:** Comprehensive security review

### Monitoring

- GitHub security alerts (enabled)
- Dependency updates (automated)
- Access logs (if applicable)
- Integrity monitoring (checksums)

---

## Third-Party Security

### Dependencies

For Python code dependencies, security maintained through:
- Regular pip updates
- Vulnerability scanning via Safety & Bandit
- Minimal external dependencies
- Version pinning in requirements.txt

### External Services

- **GitHub:** Subject to GitHub's security policies
- **Zenodo:** Permanent archive with security guarantees
- **Institutional Servers:** Institutional security policies apply

---

## Institutional Compliance

This repository complies with:
- **GDPR:** European data protection regulation
- **HIPAA:** If applicable (minimal personal data)
- **Export Control:** Research publication guidelines
- **IP Policies:** Institutional intellectual property procedures
- **Data Protection:** Institutional data security standards

---

## Contact & Escalation

### Security Contacts

- **Author & Maintainer:** Rolando M Garcia - [author.email@institution.edu]
- **Institutional Security:** [Institution Security Team]
- **GitHub Security:** [security@github.com]

### Escalation Path

1. Initial report to author
2. Escalation to institutional IT (if needed)
3. Legal consultation (for IP issues)
4. GitHub security team (if critical)

---

## Disclaimer

This security policy represents best efforts to maintain repository security. However:
- No security is absolute
- New vulnerabilities may be discovered
- Users assume some risk in using this work
- Author is not liable for security breaches
- Compliance with security practices is recommended

---

## Policy Updates

This security policy was established **February 1, 2026** and is subject to updates. Users will be notified of material changes via:
- GitHub repository notices
- This file updates
- Email notifications (if subscribed)

---

## Additional Resources

- [GitHub Security Advisories](https://github.com/advisories)
- [OWASP Security Guidelines](https://owasp.org)
- [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)
- [Git Security Best Practices](https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work)

---

**Policy Version:** 1.0  
**Last Updated:** February 1, 2026  
**Effective Date:** February 1, 2026
