use std::net::IpAddr;
use clap::Parser;


/*
https://en.wikipedia.org/wiki/Private_network
https://en.wikipedia.org/wiki/Link-local_address
https://en.wikipedia.org/wiki/Google_Public_DNS
https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing
https://superuser.com/questions/708777/if-google-provides-public-dns-8-8-8-8-and-8-8-4-4-who-provides-4-2-2-2
*/


/// Returns true if the given string is a CIDR bare netmask prefix
fn is_bare_netmask(s: &str) -> bool {
   // Return true if the first character is a `/`
   s.starts_with('/')
}


/// Parse a bare netmask (`/24` -> 24-bit prefix, 8-bit host numbers)
fn parse_bare_netmask(s: &str) -> u8 {
    s[1..].parse::<u8>().unwrap()
}


/// Returns true if the given string is a CIDR specification
/// <https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>
fn is_cidr_spec(s: &str) -> bool {
    s.contains('/')
}


/// Parse CIDR specification
///
/// panics if the given string is not a CIDR specification
///
fn parse_cidr_spec(s: &str) -> (IpAddr, u8) {
    let parts = s.split('/').collect::<Vec<&str>>();
    let ip = parts[0].parse::<IpAddr>().unwrap();
    let mask_pre = parts[1].parse::<u8>().unwrap();
    (ip, mask_pre)
}


fn calculate_netmask(mask: u8) -> u32 {
    let mut netmask = 0;
    for _ in 0..mask {
        netmask = netmask << 1;
        netmask = netmask | 1;
    }
    netmask
}


fn convert_cidr_pfx_to_netmask(mask_pre: u8) -> String {
    let mut binary_mask = String::new();
    for i in 0..32 {
        if i < mask_pre {
            binary_mask.push('1');
        } else {
            binary_mask.push('0');
        }
    }

    let octet1_binary = &binary_mask[0..8];
    let octet2_binary = &binary_mask[8..16];
    let octet3_binary = &binary_mask[16..24];
    let octet4_binary = &binary_mask[24..32];

    let octet1 = u8::from_str_radix(octet1_binary, 2).expect("Invalid binary string");
    let octet2 = u8::from_str_radix(octet2_binary, 2).expect("Invalid binary string");
    let octet3 = u8::from_str_radix(octet3_binary, 2).expect("Invalid binary string");
    let octet4 = u8::from_str_radix(octet4_binary, 2).expect("Invalid binary string");

    format!("{}.{}.{}.{}", octet1, octet2, octet3, octet4)
}


fn interpret_ip(ip: IpAddr) {
    println!("Loopback:         {}", ip.is_loopback());
    println!("Unspecified:      {}", ip.is_unspecified());
    println!("Multicast:        {}", ip.is_multicast());
    let canonical_ip = ip.to_canonical();
    println!("Canonicalized:    {}", canonical_ip);
    println!("IPv4:             {}", ip.is_ipv4());
    println!("IPv6:             {}", ip.is_ipv6());
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            println!("Well-known:       {}", check_well_known_ip_addrs(octets));
            println!("Private:          {}", ipv4.is_private());
            println!("Link-local:       {}", ipv4.is_link_local());
            println!("Broadcast:        {}", ipv4.is_broadcast());
            println!("Documentation:    {}", ipv4.is_documentation());
            println!("IPv6 compatible:  {}", ipv4.to_ipv6_compatible());
            println!("IPv6 mapped:      {}", ipv4.to_ipv6_mapped());
            if octets[0] == 10 {
                println!("Private network, class A, 24-bit block, 8-bit mask");
            }
            else if (octets[0] == 172) && (octets[1] >= 16) && (octets[1] <= 31) {
                println!("Private network, class B, 20-bit block, 12-bit mask");
            }
            else if (octets[0] == 192) && (octets[1] == 168) {
                println!("Private network, class C, 16-bit block, 16-bit mask");
            }
            else if (octets[0] == 100) && (octets[1] >= 64) && (octets[1] <= 127) {
                println!("Carrier-grade NAT range, 22-bit block, 10-bit mask");
            }
            else if [octets[0], octets[1]] == [169, 254] {
                println!("Link-local unicast, 16-bit block, 16-bit mask");
            }
            else if octets[0] == 224 {
                println!("Link-local multicast, 24-bit block, 8-bit mask");
            }

            if octets[0] <= 127 {
                println!("CIDR Class A");
            }
            else if octets[0] >= 128 && octets[0] <= 191 {
                println!("CIDR Class B");
            }
            else if octets[0] >= 192 && octets[0] <= 223 {
                println!("CIDR Class C");
            }
            else if octets[0] >= 224 {
                println!("CIDR Class D/E");
            }
        }
        IpAddr::V6(ipv6) => {
            let segments = ipv6.segments();
            println!("{}", segments[0]);
        }
    }
}


fn check_well_known_ip_addrs(octets: [u8; 4]) -> String {
    if (octets == [8, 8, 8, 8]) || (octets == [8, 8, 4, 4]) {
        "Google Public DNS".to_string()
    }
    else if (octets == [1, 1, 1, 1]) || (octets == [1, 0, 0, 1]) {
        "Cloudflare DNS".to_string()
    }
    else if ([octets[0], octets[1], octets[2]] == [4, 2, 2]) && (octets[3] >= 1) && (octets[3] <= 6) {
        "Level 3 DNS".to_string()
    }
    else if octets == [9, 9, 9, 9] {
        "Quad9 DNS".to_string()
    }
    else {
        "Unknown".to_string()
    }
}


#[derive(Parser, Default, Debug)]
#[clap(about="Decode IP addresses.")]
struct Args {
    /// The IP Address or netmask/CIDR to decode
    in_ip: String,
}


fn main() {
    let args = Args::parse();
    println!(":: IP Address decoder ::");
    println!("IP: {}", args.in_ip);
    if is_bare_netmask(&args.in_ip) {
        println!("CIDR notation:   {}", args.in_ip);
        let mask_pre = parse_bare_netmask(&args.in_ip);
        let netmask_ip = convert_cidr_pfx_to_netmask(mask_pre);
        println!("Net mask IP:     {}", netmask_ip);
    } else if is_cidr_spec(&args.in_ip) {
        println!("CIDR notation:   {}", args.in_ip);
        let (ip, mask_pre) = parse_cidr_spec(&args.in_ip);
        println!("IP part:         {}", ip);
        println!("Net mask part:   {}", mask_pre);
        let netmask = calculate_netmask(mask_pre);
        println!("Net mask calc:   {}", netmask);
        println!("Net mask hex:    {}", format!("{:X}", netmask));
        let netmask_ip = convert_cidr_pfx_to_netmask(mask_pre);
        println!("Net mask IP:     {}", netmask_ip);
        interpret_ip(ip);
    } else {
        let ip: Result<IpAddr, _> = args.in_ip.parse();
        match ip {
            Ok(ip) => {
                interpret_ip(ip);
            }
            Err(err) => {
                panic!("Error: {}", err);
            }
        }
    }
}

