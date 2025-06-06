# justfile for rust

set unstable
set dotenv-load := true
set positional-arguments

# Variables

@run *msg="This new software is absolute bullshit.":
    just _api "{{ msg }}"

@_api body:
    export API_KEY="$TOKEN" BODY="$1" && cargo run