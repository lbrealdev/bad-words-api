# justfile for rust

set unstable
set dotenv-load := true
set positional-arguments

@run *msg="Go fuck yourself!!":
    just _api "{{ msg }}"

@_api body:
    export API_KEY="$TOKEN" BODY="$1" && cargo run