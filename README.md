# TestBot

TestBot is a Discord bot built with Rust using the Serenity framework. It responds to the "!ping" command with a "Pong!" message.

## Features

- Responds to the "!ping" command with "Pong!"
- Built with the Serenity framework for Discord bots
- Written in Rust programming language

## Getting Started

To get started with TestBot, follow these steps:

1. Clone the repository:
   ``git clone https://github.com/Ezdies/TestBot.git``

2. Install Rust and Cargo. You can find the installation instructions at: https://www.rust-lang.org/tools/install

3. Set up your Discord bot in the Discord Developer Portal:
- Create a new application and add a bot to it.
- Copy the bot token for later use.

4. Create a `.env` file in the root directory of the project and add the following line:
   ``BOT_TOKEN=your_bot_token_here``
   
   Replace `your_bot_token_here` with the token you copied from the Discord Developer Portal.

5. Build and run the bot:
   ``cargo run``

6. Test the bot by sending `!ping` in a Discord server where the bot is present.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License.