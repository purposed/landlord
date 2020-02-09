# Landlord
Landlord is a powerful CLI tool for automating the 
packaging & deployment of purposed-format Rust & Go projects.

## Spec
### Publishing Flow (Creating a release from local computer)
* `landlord publish [-v] major|minor|patch`
    0. Makes sure current branch is `master` & no changes are pending.
    1. Bumps the application version in local code.
        * `Lease.toml` & `README.md` for all stacks
        * `VERSION.txt` for Go stack
        * `Cargo.toml` for Rust stack
    2. Builds the application (according to build spec)
        * `cargo build --all-features` for Rust stack
        * `go build [static flags]` for go stack
    3. *[If build passes]* Runs all validations on the application
        * UTs (`cargo test` and `go test -race ./...`)
        * Formatting
            * `rustfmt`, `clippy` for rust
            * `gofmt`, `golint` & `govet` for Go
        * Custom-defined validations  in `Lease.toml`
    4. *[If validations passed]* Commit pending version changes to master & push to `origin`
    5. *[If validations passed]* Creates a git tag with new version & push tag to trigger CI & GitHub Release.

### Release Flow (Producing Release Artifacts)
* `landlord release [-v] [--zip]`
    0. Create the artifacts directory (defined in `Lease.toml`) (defaults to `artifacts`)
    1. Builds the application (same code than publish flow)
    2. Process all artifacts (renames, checksums, etc.)
    3. *[Optional]* Zip artifacts per defined build
    4. Move final output files to artifact directory
