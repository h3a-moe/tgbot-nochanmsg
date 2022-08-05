# `tgbot-nochanmsg` By Asuna



## Right Control

When creating your bot from botfather, it's not bad to disable privacy mode, because it needs to access the messages to detect channel messages. 

This bot needs the admin right "Delele messages" to be fully functional. If not granted, an error message will pop out in the chat. 



## Compile 

```bash
$ cargo build --release 
```

Remember to move the target build to your desired path, like `/usr/bin` .



## Usage

Very simple.

To use this bot, please set your environment variable first. 

```bash
$ export TELOXIDE_TOKEN="your bot token here"
```

Then execute it. 

Or just execute it like this:

```bash
$ TELOXIDE_TOKEN="your bot token here" /usr/bin/tgbot-nochanmsg
```

**Caution! This will expose your bot token to your shell history.** 

**Don't forget to clean up.** 



## Run as a Service

Here is a typical template. 

Similarly, the bot token is set as an environment variable in the file. So, keep it safe. 

```properties
# /etc/systmd/system/tgbot-nochanmsg.service

[Unit]
Description=tgbot-nochanmsg
Wants=network.target
After=network.target

[Service]
Type=simple
Restart=on-failure
RestartSec=10s
Environment="RUST_LOG=debug"
Environment="TELOXIDE_TOKEN=<Your_Token_here>"
ExecStart=/usr/bin/tgbot-nochanmsg

[Install]
WantedBy=multi-user.target
```



