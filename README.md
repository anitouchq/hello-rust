# hello-rust

[![CI](https://github.com/anitouchq/hello-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/anitouchq/hello-rust/actions)

Учебный проект на Rust: GitHub Actions проверяет исходный код, запускает тесты и собирает release-версию. После успешного CI при push в `main` Docker-образ публикуется в GitHub Container Registry (GHCR).

## Что делает pipeline

1. `cargo fmt --all -- --check` — проверяет форматирование.
2. `cargo clippy --all-targets --all-features -- -D warnings` — запускает статический анализ.
3. `cargo test --all-features` — выполняет unit- и integration-тесты.
4. `cargo build --release` — собирает release-версию.
5. Docker Buildx собирает образ и отправляет его в GHCR.

Pull request запускает проверки, но не публикует контейнер. Публикация выполняется отдельным job только после успешного job `test` на ветке `main`.

## Локальный запуск

Требуется установленный Rust toolchain:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo run
```

## Docker image

Образ публикуется по адресу `ghcr.io/anitouchq/hello-rust`:

```bash
docker pull ghcr.io/anitouchq/hello-rust:latest
docker run --rm ghcr.io/anitouchq/hello-rust:latest
```

[Открыть пакет в GHCR](https://github.com/users/anitouchq/packages/container/hello-rust)

## Структура проекта

| Файл или каталог | Назначение |
|---|---|
| `.github/workflows/ci.yml` | Проверки Rust и публикация Docker-образа |
| `src/main.rs` | Точка входа приложения |
| `src/lib.rs` | Основная логика, доступная приложению и тестам |
| `tests/integration.rs` | Интеграционные тесты |
| `Cargo.toml` | Метаданные пакета и зависимости Rust |
| `Dockerfile` | Сборка контейнерного образа |
| `.dockerignore` | Исключения из Docker build context |
| `.gitignore` | Локальные файлы, которые не попадают в Git |

## Результаты

- [Последние запуски GitHub Actions](https://github.com/anitouchq/hello-rust/actions)
- [Docker package в GHCR](https://github.com/users/anitouchq/packages/container/hello-rust)

[Главный каталог CI/CD проектов](https://github.com/anitouchq/CI-CD-Pipelines)
