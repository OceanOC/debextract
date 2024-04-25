# Deb Extract
A small tool to extract .deb files,

It automaically downloads .deb files from the [Debian Package Mirror](https://deb.debian.org)

###### I am not related to Debian in any way or form.

## Usage
```
debextract [Package Name] [Package Version] [Architecture] [Arguments]
    -c          cleanup useless files after decompressing
    -m          option to install the deb file into /usr/bin/ and add it the PATH (requires root)
```

## Planned Features
- [ ] Use custom mirror
- [ ] Install dependencies with package
- [x] Option to not clean up
- [x] Automated extract

## Building
1. Clone the repo
2. `` cargo build `` or `` cargo build -r ``
