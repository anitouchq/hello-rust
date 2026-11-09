# 🦀 hello-rust — Docker image → GHCR

Учебный проект CI/CD для Rust. GitHub Actions проверяет код, запускает тесты и собирает release-версию. После успешных проверок при push в `main` Docker-образ публикуется в GitHub Container Registry.

## Pipeline

1. `cargo fmt --all -- --check` — проверка форматирования.
2. `cargo clippy --all-targets --all-features -- -D warnings` — статический анализ.
3. `cargo test --all-features` — unit- и integration-тесты.
4. `cargo build --release` — release-сборка.
5. Docker Buildx собирает контейнер и публикует его в GHCR.

Публикация выполняется только после успешного job `test` и только для push в `main`. Pull request запускает проверки, но не публикует образ.

## Локальный запуск

Требуется установленный Rust toolchain:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo run
```

## Docker

После первого успешного запуска workflow образ будет доступен по адресу:

```bash
docker pull ghcr.io/anitouchq/hello-rust:latest
docker run --rm ghcr.io/anitouchq/hello-rust:latest
```

Пакет можно сделать публичным в GitHub → Packages → Package settings.


## GHCR access troubleshooting

Если публикация завершается ошибкой `permission_denied: write_package`, откройте GitHub Profile → Packages → нужный пакет → **Package settings → Manage Actions access → Add repository**. Добавьте `hello-rust` и установите роль **Write**. Затем повторно запустите неуспешный workflow во вкладке Actions.

## Permissions

Workflow использует `contents: read` для checkout и `packages: write` для публикации контейнера через встроенный `GITHUB_TOKEN`. Внешние секреты не требуются.

## Структура

```text
.
├── .github/workflows/ci.yml
├── src/
│   ├── lib.rs
│   └── main.rs
├── tests/integration.rs
├── Cargo.toml
├── Dockerfile
└── README.md
```

[Главный каталог CI/CD проектов](https://github.com/anitouchq/CI-CD-Pipelines)
