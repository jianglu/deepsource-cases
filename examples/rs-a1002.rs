// Audit required: Sensitive cookie without secure attribute RS-A1002
// Security
// Cookies set without the secure flag can cause the user agent to send those cookies in plaintext over an HTTP session with the same server. This can lead to man-in-the-middle attacks.

// In past it has led to the following vulnerabilities:

// CVE-2009-2272
// CVE-2008-3663
// CVE-2008-0128
// Generally, the production sites redirect any requests that are sent over HTTP to the same URL but on HTTPS. In this case, make sure that these HTTP requests that are immediately redirected to HTTPS do not carry any cookie that contains sensitive information. The secure flag limits cookies to HTTPS traffic only so, the browser will never send secure cookies with requests that are not encrypted.

// BAD PRACTICE
// use cookie::Cookie;

// let mut c = Cookie::new("data", "sensitive value")
// c.set_secure(false);
// RECOMMENDED
// use cookie::Cookie;

// let mut c = Cookie::new("data", "sensitive value")
// c.set_secure(true);
// REFERENCES
// Cookie::set_secure
// CookieBuilder::secure
// OWASP Top 10:2021 > A02 - Cryptographic Failures
// CWE-314: Missing Encryption of Sensitive Data
// CWE-315: Cleartext Storage of Sensitive Information in a Cookie
// CWE-614: Sensitive Cookie in HTTPS Session Without 'Secure' Attribute
// EXCEPTIONS
// While this issue mostly makes sense if you're setting a sensitive cookie, DeepSource will flag all the cookies encountered without the secure flag. This is to ensure that every cookie is audited carefully.

fn main() {
    use cookie::Cookie;

    let mut c = Cookie::new("data", "sensitive value")
    c.set_secure(false);
}