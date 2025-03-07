# Protocol

## Overview

## Repostory

```yaml
parser: string; the parser name
pkgs: [Package]; packages
files: [File]; files
deps: [Dependency]; dependencies
```

## Package

```yaml
id: string; unique package IDs
name: string; package name, commonly the folder name
path: string; the path of the package, the result of pwd
imports: [string]; packages which are imported by current package
exports: [string]; packages which import current package
```

## File

```yaml
id: string; unique file IDs
name: string; the file name with extension name
path: string; the path of the package, the result of pwd
pkg: ID: related package ID
imports: [string]; files which are imported by current file
exports: [string]; files which import current file
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
file: ID; the file
signature: string; the callable signature
abstract: ID; the related abstract
parameters: [string]; the parameters of current callable
results: [string]; the returns of current callable
comment: string; comments
method: bool; is this callable a method ?
private: bool; is this callable private ?
```

## Call

TODO

## Abstract

TODO

## Reference

TODO