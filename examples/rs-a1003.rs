// Cookies set without the HttpOnly flag can be read by a client-side script, leading to cookie theft from Cross-Site Scripting (XSS) attacks.

// In past it has led to vulnerabilities such as: - CVE-2014-8958 - CVE-2008-5770

// Cross-Site Scripting (XSS) attacks target the theft of cookies set by the application. Setting the HttpOnly attribute to true mitigates the possibility of XSS attacks.

// BAD PRACTICE
// use cookie::Cookie;

// let mut c = Cookie::new("data", "sensitive value")
// c.set_http_only(false);
// RECOMMENDED
// use cookie::Cookie;

// let mut c = Cookie::new("data", "sensitive value")
// c.set_http_only(true);
// REFERENCES
// Cookie::set_http_only
// CookieBuilder::http_only
// OWASP Top 10:2021 > A03 - Injection
// CWE-79: Improper Neutralization of Input During Web Page Generation ('Cross-site Scripting')
// CWE-1004: Sensitive Cookie Without 'HttpOnly' Flag
fn main(){
    use cookie::Cookie;

    let mut c = Cookie::new("data", "sensitive value");
    c.set_http_only(false);
}