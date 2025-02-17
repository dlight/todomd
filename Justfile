set unstable

num_params := env('NUM_PARAMS', '8')

dev num='8':
    NUM_PARAMS={{num}} overmind start



backend:
    cd backend/tauri-bin && cargo tauri dev -- {{num_params}}

frontend:
    cd frontend/leptos-ui && trunk serve

debug params:
    #!/bin/bash
    set -e

    export ORIGINAL_PWD="$(pwd)"
    cd backend
    cargo run -- {{params}}

clean:
    cd backend && cargo clean
    cd frontend/leptos-ui && cargo clean && trunk clean
