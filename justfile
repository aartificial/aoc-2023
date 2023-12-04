test day part:
    cargo nextest run -p {{day}} {{part}}
create day:
    cargo generate --path ./daily-template --name {{day}}
run day part:
    cargo run --package {{day}} --bin {{part}}
