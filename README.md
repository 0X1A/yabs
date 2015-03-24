# yabs

[![Join the chat at https://gitter.im/0X1A/yabs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/0X1A/yabs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
Yet another build system

[![Build Status](https://travis-ci.org/0X1A/yabs.svg)](https://travis-ci.org/0X1A/yabs) 
[![Coverage Status](https://img.shields.io/coveralls/0X1A/yabs.svg)](https://coveralls.io/r/0X1A/yabs?branch=master)
[![Build Status on 0X1A](http://0x1a.us/img/yabs-build.png)](http://0x1a.us/ci/yabs)
[![Coverage Status on 0X1A](http://0x1a.us/img/yabs-coverage.png)](http://0x1a.us/ci/yabs)

A build system for multiple architectures and operating systems. Project files 
define Makefiles to be written that can be copied to a remote system or be 
completely built on a remote system.

## Status
yabs can currently self host. Major things that are left to implement are:

- [ ] Remote jobs (ssh and or git)
- [ ] Better language detection (& add more languages)
- [ ] Better distribution tarballs
- [ ] Pre/Post scripting
- [x] Direct building (omitting Make)

Yabs currently only works well with C and C++ and only with certain file 
extensions

## Building

You may use the `Makefile` provided, a qmake profile is available if you wish to 
 compile with clang:
```
qmake "CONFIG+=clang"
```

## Using
```bash
~/Project $ cat hello-yabs.c
#include <stdio.h>

int main()
{
	printf("Hello yabs!\n");
	return 0;
}
~/Project $ yabs -n
New build file written as: Project.ybf
```
**Edit the build file to reflect the language used**
```bash
---
lang: c
...
```
**Build it**
```bash
~/Project $ yabs -b Project.ybf
g++ -c -I/usr/include -I/usr/local/include -o hello-yabs.o hello-yabs.c
g++ -o Project hello-yabs.o -L/usr/lib -L/usr/local/lib
~/Project $ ./Project
Hello yabs!

```

## Defining a Project

To learn how projects are defined please see the [projects.md](projects.md)

## Dependencies
- `libyaml`
- `libgit2`
- `libssh2`
- `libarchive`
- C++11 compliant compiler

## Contributing
Commits must be **signed**, gpg signing is optional. Patches mailed to the 
mailing list is preferable.

### Mailing list
`yabs@librelist.com`

## Releases
All releases are signed with keyid `639DEF0A`
