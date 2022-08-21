# domain-scanner
Subdomain scanner

# How to use
## Scan only subdomains and print them
```shell
domain-scanner -d example.com
```
or
```shell
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
```
or
```shell
domain-scanner --domain example.com --ports
```
output
```shell
www.example.com:
    80
    443
example.com:
    80
    443
m.testexample.com:
    80
    443
    25
    3389
    110
    143
    3306
    8080
    995
    993
    5900
    587
    465
    8081
    389
    5432
    9100
    119
```