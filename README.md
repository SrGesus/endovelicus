# Endovelicus

Endovelicus is (or rather will be) an open-source Application to ease the management of your household finances. I started this as a personal project it is currently not functional as of yet, much work is still to be done.

Its backend is written in rust with the [axum](https://extism.org/docs/overview) framework and [sea-orm](https://github.com/SeaQL/sea-orm/).

## TO DO:

### Backend
- [ ] Currency
  - [x] Entity
  - [x] Create
  - [x] Read
  - [x] Update
  - [x] Delete
  - [ ] ~~Endpoint for updating currencies' exchange rates~~ (this should be instead be implemented by a plugin, checkout [this repo](https://github.com/SrGesus/endovelicus-plugins/tree/main))
- [ ] Plugins
  - [x] Solve concurrency issue (because of the rwlock only one plugin call possible at the same time)
  - [x] CR~~U~~D API for plugins 
  - [x] Loading plugins (with [Extism](https://extism.org/docs/overview))
  - [ ] Endpoints
    - [x] Currently just calls a single function at /api/:plugin_endpoint
  - [ ] Consider more functionality
  - [ ] Make host functions
    - [ ] Currency
    - [ ] Account
    - [ ] Transaction
- [ ] Account
  - [x] Entity
  - [ ] Create
  - [ ] Read
  - [ ] Update
  - [ ] Delete
- [ ] Transaction
  - [ ] Entity
  - [ ] Create
  - [ ] Read
  - [ ] Update
  - [ ] Delete

### Frontend
- [ ] Frontend