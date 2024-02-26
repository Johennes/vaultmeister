# Vaultmeister

![Build](https://github.com/Johennes/vaultmeister/actions/workflows/build.yml/badge.svg?branch=main)

Vaultmeister is a [Matrix]-based password manager that implements [MSC4114].
The application is a proof-of-concept and in experimental state. It should
not be used productively.

## Development

Vaultmeister uses [Tauri] as multi-platform application framework with
[Next.js] on the frontend. Note that during development, Next.js collects
[anonymous telemetry] data. You may opt out of this process by running

```
yarn next telemetry disable
```

To setup a development environment, make sure you have Go, Rust and Node.js
installed. Then run

```
yarn install && yarn dendrite:build
```

to install the dependencies and build [dendrite].

Next, you'll need to create a user account. Start dendrite with

```
yarn dendrite:start
```

and then in another terminal run

```
yarn dendrite:create-account <USERNAME>
```

Follow the interactive steps and afterwards switch back to the first terminal
to stop dendrite again.

Now, finally, you can start the app using

```
yarn tauri dev
```

## License

Vaultmeister is licensed under the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at your
option) any later version.

[anonymous telemetry]: https://nextjs.org/telemetry
[dendrite]: https://github.com/matrix-org/dendrite
[Matrix]: https://matrix.org/
[MSC4114]: https://github.com/matrix-org/matrix-spec-proposals/pull/4114
[Next.js]: https://nextjs.org/
[Tauri]: https://tauri.app/
