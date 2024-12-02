work day:
    watchexec -f "./{{day}}" "cargo check && cargo test -p {{day}}"

create day:
    cargo generate --path ./daily-template --name {{day}}
