# Security Policy

Thank you for helping keep **Rust-Mnemonic-Generator** secure! If you discover a security vulnerability, we ask that you follow the steps outlined below to report it. We take security very seriously and will work with you to resolve the issue as quickly as possible.

## Reporting a Vulnerability

If you believe you've discovered a security vulnerability in **Rust-Mnemonic-Generator**, please **do not** open an issue in the GitHub repository. Instead, follow the instructions below to report the vulnerability privately.

### Steps for Reporting:

1. **Email Us**:  
   Send a detailed email to our security team at:  
   **thesagemaster4ever@gmail.com**  
   Include the following information in your report:
   - A description of the vulnerability.
   - Steps to reproduce the issue (if possible).
   - Any potential exploits or risk levels associated with the vulnerability.
   - Your contact information (optional, but helpful for follow-up).

2. **Do not disclose the vulnerability publicly**:  
   We ask that you refrain from disclosing the vulnerability publicly until we have had a chance to investigate and address it. This allows us to patch the vulnerability without giving malicious actors a chance to exploit it.

3. **Confidentiality**:  
   We will keep your report confidential and will work with you to ensure that all security vulnerabilities are addressed in a timely manner.

## Supported Versions

We provide security updates for the following versions of **Rust-Mnemonic-Generator**:

- **Stable version** (current active release)
- **Beta version** (latest beta release)
- **Nightly version** (latest nightly release)

We recommend upgrading to the latest stable version once a security issue has been resolved, but we will continue to provide fixes for the beta and nightly versions as well.

## Security Fix Process

Once a security vulnerability has been reported and verified, we will take the following steps:

1. **Confirm the vulnerability**: We will review and verify the issue to understand its impact.
2. **Patch the vulnerability**: We will prioritize fixing the vulnerability in the codebase and ensure that the patch is effective.
3. **Test the fix**: We will thoroughly test the fix to ensure it resolves the issue without introducing new bugs.
4. **Release a security update**: A new release will be made to address the vulnerability. We will include a note in the release notes that details the security fix, along with any necessary instructions for users to update their installations.
5. **Inform the reporter**: We will notify the person who reported the vulnerability once the issue has been resolved.
6. **Publish a public disclosure**: After the vulnerability has been patched, we will publish a public security advisory to inform users about the issue and provide guidance on updating their installations.

## Security Best Practices

To help protect your systems and data while using **Rust-Mnemonic-Generator**, we recommend following these best practices:

- Always keep your dependencies up to date by regularly running:
  ```bash
  cargo update
  ```
- Use security-focused tooling such as [cargo audit](https://github.com/RustSec/cargo-audit) to check for known vulnerabilities in dependencies.
- Consider integrating automated security checks into your CI/CD pipeline.

## Credits

We would like to thank all security researchers and contributors for helping us improve the security of **Rust-Mnemonic-Generator**. Your work makes this project safer for everyone!
