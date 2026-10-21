#!/usr/bin/env bash
# bash unofficial strict mode:
set -euo pipefail
IFS=$'\n\t'
[[ -n ${DEBUG:-""} ]] && set -x

for HOST; do
    ssh "$HOST" <<'EOT'
        set -euxo pipefail
        apt install -y matchbox-window-manager libclang-dev libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev pkg-config
        [[ -f /root/.cargo/bin/cargo ]] || curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal
        # kiosk: no desktop, so no Firefox or screensaver; `systemctl start gdm3` brings the desktop back
        systemctl set-default multi-user.target
        systemctl disable gdm3
        rm -f /home/kaleidoscopist/.config/autostart/{firefox_firefox-1,Kaleidoscope}.desktop
        usermod -aG video kaleidoscopist
        systemctl mask sleep.target suspend.target hibernate.target hybrid-sleep.target
        [[ -f /opt/kaleidoscope/config/local.json ]] || cp /home/kaleidoscopist/src/webcam-kaleidoscope/config/local.json /opt/kaleidoscope/config/
        /root/.cargo/bin/cargo build --release --manifest-path /opt/kaleidoscope/Cargo.toml
        systemctl enable /opt/kaleidoscope/kaleidoscope.service
        systemctl reboot
EOT
done
