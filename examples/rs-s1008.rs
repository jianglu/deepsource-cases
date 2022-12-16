// Missing regex anchor RS-S1008
// Security
// It is unsafe to match untrusted input against regular expressions without ^ anchor. URLs with missing anchors can prove fatal as malicious inputs bypass the system's security checks.

// BAD PRACTICE
// fn validate_url(s: &str) -> bool {
//     let re = Regex::new("https?://www\.deepsource\.com/").unwrap();
//     re.is_match(s)
// }

// // matches:
// //   "http://www.deepsource.com/" 
// //   "http://www.example.com/?x=http://www.deepsource.io/"
// Here, the check with the regular expression match (Regex::is_match) is easy to bypass. For example, the string http://deepsource.com/ can be embedded in the query string component: http://<any>/?x=http://deepsource.com/ where (other parts are configurable as well) could be any malicious site that attacker chooses.

// RECOMMENDED
// fn validate_url(s: &str) -> bool {
//     let re = Regex::new("^https?://www\.deepsource\.com/").unwrap();
//     re.is_match(s)
// }

// // matches:
// //   "http://www.deepsource.com/" 
// //
// // but not:
// //   "http://www.example.com/?x=http://www.deepsource.io/"
// REFERENCES
// CWE-20
// OWASP: Unvalidated Redirects and Forwards Cheat Sheet
// OWASP: SSRF


use regex::Regex;


fn main() {
    fn validate_url(s: &str) -> bool {
        let re = Regex::new("https?://www\.deepsource\.com/").unwrap();
        re.is_match(s)
    }
    
    // matches:
    //   "http://www.deepsource.com/" 
    //   "http://www.example.com/?x=http://www.deepsource.io/"
}