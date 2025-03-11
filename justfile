default:
    @just --list

set windows-powershell

alias rd := r_dev
alias wd := w_dev
alias dtt := dev_to_test
alias td := t_dev

#* DEV scripts

[windows]
r_dev soft:
    cd {{ soft }}; cargo watch -x run

[linux]
r_dev soft:
    cd {{ soft }} && cargo watch -x run

[windows]
w_dev:
    cd web; pnpm dev

[linux]
w_dev:
    cd web && pnpm dev

#* pNPM install scripts

[windows]
w_i:
    cd web; pnpm install

[linux]
w_i:
    cd web && pnpm install

#* Docker image build scripts

d_build soft tag:
    podman build --platform linux/amd64 -f {{soft}}/Dockerfile . -t ghcr.io/sckk-apms-dev/saes-{{ soft }}:{{ tag }}

[windows]
w_build:
    cd web; pnpm run build

[linux]
w_build:
    cd web && pnpm run build

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
    cd app; cargo tauri dev

[linux]
t_dev:
    cd app && cargo tauri dev