# QML parser

This is a parser for QML ([Qt Modeling Language][1]) that allows providing own `import` processing rules.

[1]: https://en.wikipedia.org/wiki/QML

## Why not Serde?

_Serde_ is designed to deserialize a format into in-memory Rust structures. This is fine if you just want to restore the
data from some serialization format. But QML is mainly User Interface description format and parsing it should rather
and up in a series of calls creating UI elements. Using Serde would require deserializing QML to some intermediary
data structure and then processing it doing UI calls. I want to skip this intermediary step.

This project explores a way of supporting such use case in a flexible manner. For example it should also be possible to
use QML as a _prefab_ description language creating entities directly in a game engine ECS. It should be also possible
to build an intermediary AST for QML file and process it later.
