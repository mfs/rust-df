from fabric.api import local

def build_release():
    "standard release build"
    local("cargo build --release")

def build_debug():
    "standard debug build"
    local("cargo build")

def build_release_musl():
    "static musl release build"
    local("cargo build --release --target=x86_64-unknown-linux-musl")
    local("strip target/x86_64-unknown-linux-musl/release/rdf")
