work day:
    watchexec -e rs -f "./{{day}}" "cargo check -p {{day}} && cargo test -p {{day}}"

create day:
    cargo generate --path ./daily-template --name {{day}}
