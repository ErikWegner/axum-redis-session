# Rust Axum session with redis store

## Description

The project simply shows how to integrate:

* [axum](https://crates.io/crates/axum) as http server
* [axum-sessions](https://crates.io/crates/axum-sessions) as session middleware
* [async-redis-session](https://crates.io/crates/async-redis-session) as session storage

[![Open in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/ErikWegner/axum-redis-session)

A **development container** is a running [Docker](https://www.docker.com) container with a well-defined tool/runtime stack and its prerequisites. You can try out development containers with **[GitHub Codespaces](https://github.com/features/codespaces)** or **[Visual Studio Code Dev Containers](https://aka.ms/vscode-remote/containers)**.

## Requirements

* Provide a session mechanism with cookies
* Cookie must have http-only and secure flags set, the name must be configurable
* Cookie must only be set when session is started
* Cookie must be deleted from client browser on session end
