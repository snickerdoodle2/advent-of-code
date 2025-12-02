@_default:
    just --list

create day:
    cargo generate --path daily-template --name {{day}}
    just input {{day}}

work day:
    @watchexec -r just _pass {{day}}

_pass day:
    just check {{day}}
    just lint {{day}}
    just test {{day}}

check day:
    cargo check -p {{day}}

lint day:
    cargo clippy -p {{day}}

test day:
    cargo test -p {{day}}

part-one day:
    cargo run -p {{day}} --bin part1

part-two day:
    cargo run -p {{day}} --bin part2

bench day part="":
    cargo bench --bench {{day}}-bench {{ if part != "" { "part" + part + "_bench" } else { "" } }}

input day:
    ./input.rs --day {{day}} --cwd {{justfile_directory()}}
