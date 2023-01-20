# xresloader-dump-bin

[![Build](https://github.com/xresloader/xresloader-dump-bin/actions/workflows/main.yml/badge.svg)](https://github.com/xresloader/xresloader-dump-bin/actions/workflows/main.yml)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/xresloader/xresloader-dump-bin)](https://github.com/xresloader/xresloader-dump-bin/releases)

![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/xresloader/xresloader-dump-bin)
![GitHub repo size](https://img.shields.io/github/repo-size/xresloader/xresloader-dump-bin)
![GitHub All Releases](https://img.shields.io/github/downloads/xresloader/xresloader-dump-bin/total)
![GitHub forks](https://img.shields.io/github/forks/xresloader/xresloader-dump-bin?style=social)
![GitHub stars](https://img.shields.io/github/stars/xresloader/xresloader-dump-bin?style=social)

A tool to dump human readable text from binary output of [xresloader][1] .

## Usage

```bash
./xresloader-dump-bin --help

./xresloader-dump-bin -p ./sample-conf/kind.pb -b ./sample-data/role_upgrade_cfg.bin --pretty

xresloader-dump-bin.exe --help
```

You can use environment `RUST_LOG=<level>` to control log level and `RUST_LOG_STYLE=style` to set log style.

Example output: `./xresloader-dump-bin -p ./sample-conf/kind.pb -b ./sample-data/role_upgrade_cfg.bin` (Without `--pretty`)

```bash
======================== Header: .\sample-data\role_upgrade_cfg.bin ========================
xresloader version: 2.8.0
data version: 2.8.0.20200609192757
data count: 11
hash code: sha256:718d22f78006e5d34d6b68eb56e204a00f4174737b6864e247b661d8963c7df3
description:
============ Body: .\sample-data\role_upgrade_cfg.bin -> role_upgrade_cfg ============
[
    {"Id": 10001, "Level": 1},
    {"Id": 10001, "Level": 2, "CostType": 10001, "CostValue": 50},
    {"Id": 10001, "Level": 3, "CostType": 10001, "CostValue": 100},
    {"Id": 10001, "Level": 4, "CostType": 10001, "CostValue": 150},
    {"Id": 10001, "Level": 5, "CostType": 10001, "CostValue": 200},
    {"Id": 10001, "Level": 6, "CostType": 10001, "CostValue": 250},
    {"Id": 10001, "Level": 7, "CostType": 10001, "CostValue": 300},
    {"Id": 10001, "Level": 8, "CostType": 10001, "CostValue": 350},
    {"Id": 10001, "Level": 9, "CostType": 10101, "CostValue": 400},
    {"Id": 10001, "Level": 10, "CostType": 10101, "CostValue": 450},
    {"Id": 10001, "Level": 11, "CostType": 10101, "CostValue": 500},
]
```

https://doc.rust-lang.org/cargo/reference/config.html

[1]: https://github.com/xresloader/xresloader
