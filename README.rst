=========
Jieba-bot
=========
This is a simple Telegram inline bot providing Chinese word segmentation powered
by the Jieba algorithm.

The official instance is `@jiebabot <https://t.me/jiebabot>`_.

.. image:: example.png

Build
-----
A stable Rust compiler toolchain should be fine.

.. code:: sh

    # debug build
    cargo build
    # or, optimized build
    cargo build --release

Running
-------
Set the environment variable ``TELEGRAM_BOT_TOKEN=<your bot's secret token>``,
then run it!

.. code:: sh

    TELEGRAM_BOT_TOKEN=xxxx cargo run --release

Miscellaneous
-------------
Why not write this in Python?

Because I'm afraid of programming in dynamic type languages :)

Privacy
-------
This bot records nothing at all. All messages are discarded once processed.

License
-------
This work is released under the WTFPL license. A copy of the license is provided
in the LICENSE file.
