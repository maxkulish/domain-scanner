# domain-scanner
Subdomain scanner

# How to use
## Scan only subdomains and print them
```shell
domain-scanner -d example.com
or
domain-scanner --domain example.com
```
output
```shell
example.com
m.testexample.com
www.example.com
```

## Scan subdomains and check alive ports
```shell
domain-scanner -d example.com -p
or
domain-scanner --domain example.com --ports
```
