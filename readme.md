# ipdec

`ipdec` is a CLI tool to display information about IP addresses.


## References

- https://en.wikipedia.org/wiki/Private_network
- https://en.wikipedia.org/wiki/Link-local_address
- https://en.wikipedia.org/wiki/Google_Public_DNS
- https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing
- https://superuser.com/questions/708777/if-google-provides-public-dns-8-8-8-8-and-8-8-4-4-who-provides-4-2-2-2


## Examples


### Localhost

```shell
$ ipdec 127.0.0.1
:: IP Address decoder ::
IP: 127.0.0.1
Loopback:         true
Unspecified:      false
Multicast:        false
Canonicalized:    127.0.0.1
IPv4:             true
IPv6:             false
Well-known:       Unknown
Private:          false
Link-local:       false
Broadcast:        false
Documentation:    false
IPv6 compatible:  ::7f00:1
IPv6 mapped:      ::ffff:127.0.0.1
CIDR Class A
```


### CIDR

```shell
$ ipdec 198.51.100.14/18
:: IP Address decoder ::
IP: 198.51.100.14/18
CIDR notation:   198.51.100.14/18
IP part:         198.51.100.14
Net mask part:   18
Net mask calc:   262143
Net mask hex:    3FFFF
Net mask IP:     255.255.192.0
Loopback:         false
Unspecified:      false
Multicast:        false
Canonicalized:    198.51.100.14
IPv4:             true
IPv6:             false
Well-known:       Unknown
Private:          false
Link-local:       false
Broadcast:        false
Documentation:    true
IPv6 compatible:  ::c633:640e
IPv6 mapped:      ::ffff:198.51.100.14
CIDR Class C
```


### Well-known IP Addresses

ipdec includes information on a few well-known IP addresses.

```shell
$ ipdec 8.8.8.8
:: IP Address decoder ::
IP: 8.8.8.8
Loopback:         false
Unspecified:      false
Multicast:        false
Canonicalized:    8.8.8.8
IPv4:             true
IPv6:             false
Well-known:       Google Public DNS
Private:          false
Link-local:       false
Broadcast:        false
Documentation:    false
IPv6 compatible:  ::808:808
IPv6 mapped:      ::ffff:8.8.8.8
CIDR Class A
```

