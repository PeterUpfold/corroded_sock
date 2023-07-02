# Corroded Sock

A horrible name for a Rust version of [Socky](https://github.com/PeterUpfold/Socky), a simple
tool to listen on TCP sockets to observe connection attempts to servers on the local host.

This is designed to provide some level of visibility into any activities on your system where local servers running on Well-Known (and merely well-known, if you catch my drift) ports are being enumerated.

I would like to be notified when web pages think this behaviour is justified.

Inspired by the proof of concept of this demonstrated by https://wybiral.github.io/wtf/

## Disclaimer

I do not yet know how to write **good** Rust code. This is experimental and not for production use. It may
cause harm. It may be a Bad Idea. You have been warned.

## Building

    cargo build -r

The executable is built in `target/release/corroded_sock`.

## Usage

For example, to listen on the Steam port, `27060`:

    ./corroded_sock 27060

For `systemd` systems, you can make this start upon login as follows.

Copy `systemd/listen-steam-port.example.service` to `~/.config/systemd/user/listen-steam-port.service`.

Alter the `ExecStart` line to point to where the `corroded_sock` executable is held.

Enable the service:

    systemctl --user enable --now listen-steam-port.service

Test that you receive a desktop notification:

    nc 127.0.0.1 27060