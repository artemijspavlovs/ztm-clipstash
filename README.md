## ClipStash

This is a project provided by the [Zero To Mastery Rust Course](https://zerotomastery.io/courses/learn-rust/) as the final part of the
course that solidifies learning material.

According to the course description, `ClipStash` is a web service that allows the user to paste and share clipboard content. The
features include:

- Ling Sharing
- Clip Expiration
- JSON API
- Password Protected clips

The Topics covered by the projects include:

- Async web server
- Template rendering
- CLI client with API key generation and revocation
- SQLite Database
    - Deferred database writes
    - Migrations
- Background service - routine cleanup tasks
- Multilayered architecture
- Tests

### Architecture

The `ClipStash` is composed of 4 layers:

`web` - communicates with the user and manages application state.
This service does not have a direct access to the data layer, it
can only communicate with the `service` layer and application users.

- Reports errors
- Renders pages
- Exposes an RESTful API
- Establishes database connections
- Binds ports
- Spawns background tasks

`service` - intermediate layer between the `web` and the `database`.
Implements application logic and abstracts user requests and
data access.

`data` - manages data storage and retrieval.

- Incoming and outgoing data is unmodified
- Maintains internal consistency when applicable

`domain` - provides the data types that are shared with all layers
and components, additionally - enforces business rules on data.

- domain objects cannot be created unless all rules pass
- accessible from all layers above
