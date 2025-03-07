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
pkg: string: related package ID
imports: [string]; files which are imported by current file
exports: [string]; files which import current file
```

## Dependency

```yaml
id: string; unique dependency IDs
name: string; the dependency name
```