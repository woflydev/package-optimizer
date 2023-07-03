# Package Optimizer
A rudimentary number-cruncher in Rust that optimizes rectangular packaging.

Early tests conclude that with a Ryzen 5950X, this program can crunch around 1 billion geometric algorithms in 40 seconds flat. There's still a lot of different optimizations that can be done (this was thrown together in half a day or so) such as taking it completely multithreaded but at the moment this is sufficient as far as I am concerned.

Simply unzip, navigate to folder through terminal of your preference, and run:
``cargo run --release``.

Or, if you would simply like to build the executable:
``cargo build --release``.

Have fun :)

``main.rs`` is not operational at this point in time, but ``graph.rs`` works fine.
