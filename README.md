<!-- lint disable no-undefined-references -->

# Project Directories

A library for retrieving common project directories for your projects.

* Cross-platform - supports linux, windows, freebsd, macos and more
* Standard agnostic - get directories using multiple stanards like e.g. FHS, XDG, Known Folders
* Language agnostic - supports rust, python, c, bash
* Easy to use - plug and go
* Universal - single builder configuration that works for all supported systems and languages

<!-- vim-markdown-toc GFM -->

* [Usage](#usage)
    * [Rust](#rust)
    * [Bash (cli tool)](#bash-cli-tool)
    * [Python](#python)
    * [C and C++](#c-and-c)
* [Project name](#project-name)
* [Supported standards](#supported-standards)
* [Supported directories](#supported-directories)
* [Fully supported systems](#fully-supported-systems)
* [Future plans](#future-plans)

<!-- vim-markdown-toc -->

## Usage

### Rust

```rust
use project_dirs::{Directory, Project};

pub fn main() {
    // Yep. That's all you need in most cases
    let log_dir = Project::new("org", "My Super Company", "My app")
        .project_dirs()
        .local
        .get(&Directory::Log)
        .unwrap()
        .clone();

    println!("log dir: {:#?}", log_dir);
}
```

### Bash (cli tool)
```bash
> cat manifest.json
{
   "qualifier": "q",
   "organization": "o",
   "application": "a",
   "spec": "system-default"
}

> project-dirs-bin manifest.json \
    --application "my-project" \
    --organization "My Super Company" \
    --qualifier "org"

{
  "application_name": "my-project",
  "dirs": {
    "local": {
        ...
    },
    "user": {
        ...
    },
    "system": {
        ...
    }
  }
}
```

### Python

```python
import project_dirs_py

dirs = project_dirs_py.BuilderResult.from_default('app', 'mycorp', 'org')
__import__('pprint').pprint(dirs)
```

### C and C++

```c
#include <project-dirs-c.h>

void project_dirs() {
  char *project_dirs = project_dirs__project_dirs("my-app", "ultracorp", "org");
  printf("%s\n", project_dirs);
  free(project_dirs);
}
```

## Project name

For the triplet the following values are used as an example:

* `qualifier` is `com`
* `organization` is `Example Org`
* `application` is `Magic App-Name`

| System  | Project name (`<project-name>`)            | Example                          |
| ------- | ------------------------------------------ | -------------------------------- |
| Linux   | `<application>`                            | `magic-app-name`                 |
| macOS   | `<qualifier>.<organization>.<application>` | `com.example-org.magic-app-name` |
| Windows | `<<organization>\<application>`            | `Example Org\Magic App-Name`     |

## Supported standards

* **FHS** – Follow the [Filesystem Hierarchy Standard (FHS)](https://refspecs.linuxfoundation.org/FHS_3.0/fhs-3.0.pdf), common on Linux systems.
* **Xdg** – Implements the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html) for organizing user-specific config, cache, and data directories.
* **Unix** – Uses [Unix-style "dotted" directories](https://unix.stackexchange.com/questions/21778/whats-so-special-about-directories-whose-names-begin-with-a-dot) (e.g., `~/.config`) in the user’s home directory.
* **Windows** – Leverage [Windows known directories](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_Profile) via [`SHGetKnownFolderPath`](https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath).

## Supported directories

* bin - project executables
* cache - non-essential data, usually used to speed up the application
* config - project configuration
* data - essential data
* include - C/C++ headers
* lib - shared libraries
* log - project logs
* runtime - non-essential data that do not persist
* state - non-essential data that should persist
* project-root - project root directory (if applicable)

## Fully supported systems

* Linux
* Windows
* FreeBSD
* MacOS (planned)


## Future plans

We are planning to support the following languages:

- [ ] Java / Kotlin
- [ ] Golang
- [ ] C#
