# cadmium-yellow

[![CI](https://github.com/DanNixon/cadmium-yellow/actions/workflows/ci.yml/badge.svg)](https://github.com/DanNixon/cadmium-yellow/actions/workflows/ci.yml)
![Crates.io Version](https://img.shields.io/crates/v/cadmium-yellow)
![docs.rs](https://img.shields.io/docsrs/cadmium-yellow)

A client library for the [unofficial Nexus Tyne and Wear Metro real time information API](https://github.com/danielgjackson/metro-rti).

## Usage notes

Keep in mind that this API is for the Pop app and not a public API.
When using it, poll the API in a manner that you would expect to come from the Pop app, i.e. quite sporadic, mostly no traffic apart from a few requests over a few minutes.

Things you could use this API for:

- A better mobile app
- A command line tool
- Home automation integration, tied into e.g. a "leaving home" routine

Things you bloody well should not use this API for:

- A departure board for your living room
- A widget that tells you when to leave to catch the next Metro (regardless of if you actually have to leave or not)
- Anything else that wastes requests collecting data that is of zero practical use to anyone
