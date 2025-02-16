backend:
    cd backend/tauri-bin && cargo tauri dev

frontend:
    cd frontend/leptos-ui && trunk serve

dev:
    overmind start

debug *params:
    #!/bin/bash
    set -e

    export ORIGINAL_PWD="$(pwd)"
    cd backend
    cargo run -- {{params}}

clean:
    cd backend && cargo clean
    cd frontend && cargo clean
