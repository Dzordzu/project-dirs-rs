# Strategies structure tables

## `FHS` (shared) structure

| Directory   | Path                            |
| ----------- | ------------------------------- |
| Bin         | `/usr/bin/`                     |
| Cache       | `/var/cache/<project-name>`     |
| Config      | `/etc/<project-name>`           |
| Data        | `/var/lib/<project-name>`       |
| Include     | `/usr/include/<project-name>`   |
| Lib         | `/usr/lib/<project-name>`       |
| Log         | `/var/log/<project-name>`       |
| ProjectRoot | -                               |
| Runtime     | `/run/<project-name>`           |
| State       | `/var/lib/<project-name>/state` |

## `FHS` (local) structure

| Directory   | Path                                |
| ----------- | ----------------------------------- |
| Bin         | `/usr/local/bin/`                   |
| Cache       | `/var/cache/<project-name>`         |
| Config      | `/usr/local/etc/<project-name>`     |
| Data        | `/var/lib/<project-name>`           |
| Include     | `/usr/local/include/<project-name>` |
| Lib         | `/usr/local/lib/<project-name>`     |
| Log         | `/var/log/<project-name>`           |
| ProjectRoot | -                                   |
| Runtime     | `/run/<project-name>`               |
| State       | `/var/lib/<project-name>/state`     |

## `Unix` structure

| Method                     | Base Path          |
| -------------------------- | ------------------ |
| `unix_home`                | `$HOME`            |
| `unix` and `unix_prefixed` | Defined path       |
| `unix_binary`              | Path of the binary |
| `unix_pwd`                 | `$PWD`             |

**NOTE**: `unix_prefixed` additionally prepends a custom prefix to the project\_name.

| Directory   | Path                                  |
| ----------- | ------------------------------------- |
| Bin         | `<base-path>/<project-name>/bin/`     |
| Cache       | `<base-path>/<project-name>/cache/`   |
| Config      | `<base-path>/<project-name>/`         |
| Data        | `<base-path>/<project-name>/data/`    |
| Include     | `<base-path>/<project-name>/include/` |
| Lib         | `<base-path>/<project-name>/lib/`     |
| Log         | `<base-path>/<project-name>/logs/`    |
| ProjectRoot | `<base-path>/<project-name>/`         |
| Runtime     | `<base-path>/<project-name>/tmp/`     |
| State       | `<base-path>/<project-name>/state/`   |

## `XDG` structure

| Directory   | Path                                    | Fallback                                    |
| ----------- | --------------------------------------- | ------------------------------------------- |
| Bin         | `$HOME/.local/bin/`                     |                                             |
| Cache       | `$XDG_CACHE_HOME/<project-name>`        | `$HOME/.cache/<project-name>`               |
| Config      | `$XDG_CONFIG_HOME/<project-name>`       | `$HOME/.config/<project-name>`              |
| Data        | `$XDG_DATA_HOME/<project-name>`         | `$HOME/.local/share/<project-name>`         |
| Include     | `$XDG_DATA_HOME/<project-name>/include` | `$HOME/.local/share/<project-name>/include` |
| Lib         | `$XDG_DATA_HOME/<project-name>/lib`     | `$HOME/.local/share/<project-name>/lib`     |
| Log         | `$XDG_STATE_HOME/<project-name>/log`    | `$HOME/.local/state/<project-name>/log`     |
| ProjectRoot | -                                       | -                                           |
| Runtime     | `$XDG_RUNTIME_DIR/<project-name>`       | -                                           |
| State       | `$XDG_STATE_HOME/<project-name>`        | `$HOME/.local/state/<project-name>`         |

## `Windows` structure

| Type           | Static data        | Changing data      | Project root       |
| -------------- | ------------------ | ------------------ | ------------------ |
| System wide    | `%ProgramFiles%`   | `%ProgramData%`    | -                  |
| User (default) | `%RoamingAppData%` | `%LocalAppData%`   | -                  |
| User (local)   | `%LocalAppData%`   | `%LocalAppData%`   | `%LocalAppData%`   |
| User (shared)  | `%RoamingAppData%` | `%RoamingAppData%` | `%RoamingAppData%` |

| Directory   | Path                                    |
| ----------- | --------------------------------------- |
| Bin         | `<static-data>/<project-name>/bin/`     |
| Cache       | `<changing-data>/<project-name>/cache/` |
| Config      | `<static-data>/<project-name>/config/`  |
| Data        | `<changing-data>/<project-name>/data/`  |
| Include     | `<static-data>/<project-name>/include/` |
| Lib         | `<static-data>/<project-name>/lib/`     |
| Log         | `<changing-data>/<project-name>/logs/`  |
| ProjectRoot | `<project-root>`                        |
| Runtime     | `<changing-data>/<project-name>/tmp/`   |
| State       | `<changing-data>/<project-name>/state/` |
