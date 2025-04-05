# Protocol

## Overview

## Repostory

```yaml
name: string; repository name
lang: string; package programming language
parser: string; the parser name
protocol: string: the output protocol version
timestamp: string; parse time
repository: string; repository URL
version: string; version
pkgs: [Package]; packages
files: [File]; files
fns: [Callable]; files
calls: [Call]; files
absts: [Abstract]; files
refs: [Reference]; files
deps: [Dependency]; dependencies
```

## Package

```yaml
name: string; package name, commonly the folder name
fullName: string; package full name
path: string; package path
deps: [string]; deps of current package
imports: [string]; packages which are imported by current package
```

## File

```yaml
id: string; unique file IDs
name: string; the file name with extension name
path: string; the path of the package, the result of pwd
source: boolean: show whether the current file is source code of this language
pkg: ID: related package ID, this field should be empty string if all the files are not source code.
deps: [string]; deps of current file
imports: [string]; files which are imported by current file
```

## Dependency

```yaml
id: string; unique dependency IDs
name: string; the dependency name
typ: string; the dependency typ
ref: string; the dependency reference
```

## Callable

callables could be common functions, anonymous functions, arrow functions, lambda expression or other possible callable expressions.

```yaml
id: string; unique callable IDs
name: string; the callable name
pos: string; the pos of the expression or declaration
file: ID; the file ID
signature: string; the callable signature
abstract: ID; the related abstract
parameters: [string]; the parameters of current callable
results: [string]; the returns of current callable
comment: string; comments
method: bool; is this callable a method ?
private: bool; is this callable private ?
```

## Call

```yaml
id: string; unique call IDs
pos: string; the pos of the expression or declaration
caller: ID; the caller ID
callee: ID; the callee ID
file: ID; the file ID
typ: string; the type of the call
signature: string; the callable signature
dep: ID; the related dependency
```

## Abstract

```yaml
id: string; unique abstract IDs
name: string; the abstract name
pos: string; the pos of the expression or declaration
file: ID; the file ID
comment: string; comments
fields: [string]; the fields of current abstract
```

## Reference

```yaml
id: string; unique reference IDs
pos: string; the pos of the expression or declaration
file: ID; the file ID
```