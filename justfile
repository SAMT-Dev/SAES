default:
    @just --list

set windows-powershell

alias rd := r_dev
alias web := w_dev
alias dtt := dev_to_test
alias app := t_dev
alias dbg := db_gen
alias cdn := cdn_dev

#* DEV scripts

[windows]
r_dev soft:
    cd {{ soft }}; cargo watch -x run

[linux]
r_dev soft:
    cd {{ soft }} && cargo watch -x run

api:
    just rd api

[windows]
w_dev:
    cd web; bun dev

[linux]
w_dev:
    cd web && bun dev

#* deno install scripts

[windows]
w_i:
    cd web; bun install

[linux]
w_i:
    cd web && bun install

#* Docker image build scripts

d_build soft tag:
    podman build --platform linux/amd64 -f {{soft}}/Dockerfile . -t ghcr.io/sckk-apms-dev/saes-{{ soft }}:{{ tag }}

[windows]
w_build:
    cd web; deno run build

[linux]
w_build:
    cd web && deno run build

[windows]
r_build soft:
    cd {{ soft }}; cargo build --release

[linux]
r_build soft:
    cd {{ soft }} && cargo build --release

#* Merge changes from devel to test with rebase

dev_to_test:
    git switch test
    git pull
    git rebase devel
    git push
    git switch devel

#* Tauri dev for app

[windows]
t_dev:
    cd app; bun tauri dev

[linux]
t_dev:
    cd app && bun tauri dev

#* DB Type gen based on api's .env

[windows]
db_gen:
    cd api; sea-orm-cli generate entity -o ../shared/src/db

[linux]
db_gen:
    cd api && sea-orm-cli generate entity -o ../shared/src/db

#* CDN Server
[windows]
cdn_dev:
    cd cdn; air

[linux]
cdn_dev:
    cd cdn && air