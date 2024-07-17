# Generating Entities CLI

Generate entities from database

```sh
# You need to be on the repo root folder
sea-orm-cli generate entity -o entity/src --lib --with-serde both --model-extra-derives 'endovelicus_macros::OptionalModel','endovelicus_macros::IntoActive' --serde-skip-hidden-column
```
