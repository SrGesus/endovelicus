# Generating Entities CLI

Generate entities from database

```sh
# You need to be on the repo root folder
sea-orm-cli generate entity -o entity/src -l --model-extra-derives 'serde::Deserialize','serde::Serialize' 
```
