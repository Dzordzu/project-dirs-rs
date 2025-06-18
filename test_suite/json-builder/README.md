# Builder tests

Single test consists of

* env.json (optional)
* input.json (required)
* output.output.json (required)
* output.<`__RUN_ONLY_ON__>`.json (optional). Will override the default output for certain OS.

## Env (env.json)

Env is a map of environment variables. Additionally these magic variables are available:

* `__RUN_ONLY_ON__` - run the test only on the specified OS or FAMILY (`windows` or `unix`). Special values: 
    * `unix-not-mac` (freebsd, linux etc.)
    * `docker` (run only inside docker for testing)
    * `never` (example usage only)
* `__RESOLVE_PROJECT_ROOT__` - resolve the project root to the actual path. It works only for custom unix-like strategies with `/PROJECT_ROOT` as path
* `__CHDIR__` - change the current working directory for the single test

**NOTE**: Magic variables cannot be used with a "standard" env variables. They need to be specified in the env.json file
