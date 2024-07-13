# Endovelicus

Endovelicus is (or rather will be) an open-source Application written in pure rust to ease the management of your household finances. I started this as a personal project it is currently not functional as of yet, much work is still to be done.

## TO DO:

- [ ] Currency
  - [x] Entity
  - [x] Create
  - [x] Read
  - [ ] Update
  - [ ] Delete
  - [ ] ~~Endpoint for updating currencies' exchange rates~~ (this should be instead be implemented by a plugin, checkout [this repo](https://github.com/SrGesus/endovelicus-plugins/tree/main))
- [ ] Plugins
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
