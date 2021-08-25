# Omae Wo Miteiru
Omae Wo Miteiru (お前を見ている) means "I am watching at you" is a Discord Text-To-Speech bot taht uses [Google Text-To-Speech API](https://cloud.google.com/text-to-speech).

The bot only reads up and speak it in specified channel. Currently heavily in development, but can used right now.

## How to use
### Prerequisite
- Docker
- Docker Compose
- amd64 based architecture OS
- Discord bot token
- Google's IAM Service account credentials

### Install
Once you got all prerequisite, you need to set these environment variables.
```
export DISCORD_TOKEN="XXXXXXXXXX.YYYYYYY.ZZZZZZZZ_AAAAAAAAAAAAA"
export GOOGLE_APPLICATION_CREDENTIALS="secret.json"
```

`DISCORD_TOKEN`: is your bot's token. Can obtained from [Discord Developer Portal](https://discord.com/developers/applications).
`GOOGLE_APPLICATION_CREDENTIALS`: is a credential file **path** to use Google's application API. You need to create an IAM Service Account and create a key in the account.

`GOOGLE_APPLICATION_CREDENTIALS` must be **path**, and also should be included in the project file. I recommend rename the credential JSON file to `secret.json` since it has added in `.gitignore`.

Then run `docker-compose up` and wait for awhile to initial build, and you will get `"[YourBotName] is connected!"` then you are ready to go!
