test day part:
    cargo nextest run -p {{day}} {{part}}
testday day:
    cargo nextest run -p {{day}}

create day:
    cargo generate --path ./daily-template --name {{day}}
run day part:
    cargo run --release --package {{day}} --bin {{part}}
