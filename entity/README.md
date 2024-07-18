# Generating Entities CLI

Generate entities from database

```sh
# You need to be on the repo root folder
sea generate entity -o entity/src --lib --with-serde both --model-extra-derives 'endovelicus_macros::OptionalModel' --serde-skip-hidden-column
```
