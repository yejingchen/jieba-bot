=========
Jieba-bot
=========
This is a simple Telegram inline bot providing Chinese word segmentation powered
by the Jieba algorithm.

The official instance is `@jiebabot <https://t.me/jiebabot>`_.

.. image:: example.png

Build
-----
Requries rustc 1.39+ for async/await.

.. code:: sh

    # debug build
    cargo build
    # or, optimized build
    cargo build --release

Running
-------
Configurations including bot token is provided at runtime via environment
variables.

- ``BOT_TOKEN``: required. This is what you get from botfather when you create
  a bot.

- ``RUST_LOG``: controls the logging behavior, for example: ``info``,
  ``debug``, ``jieba_bot=debug``, ``tbot``, etc. See |log_crate|_.

- ``HTTPS_PROXY``: in the form of ``http[s]://host:port``. Currently only http
  and https proxy is supported.

.. |log_crate| replace:: the `log` crate
.. _log_crate: https://docs.rs/log

.. code:: sh

    RUST_LOG=info BOT_TOKEN=xxxxxx cargo run --release

Alternatively, you can use systemd to manage the service. A demo systemd
service and an EnvironmentFile are included. Place the service file in
``/etc/systemd/system``, put the executable at ``/usr/local/bin/jieba-bot``,
put ``jieba-bot.conf`` in ``/etc``, and issue a ``systemctl daemon-reload``.
Then you can start and enable the ``jieba-bot`` service using normal systemd
commands.

Miscellaneous
-------------
Q: Why not write this in Python?

A: Because I'm afraid of programming in dynamically-typed languages :)

Privacy
-------
This bot records nothing at all. All messages are discarded once processed.

License
-------
This work is released under the WTFPL license. A copy of the license is provided
in the LICENSE file.
