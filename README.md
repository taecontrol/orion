![Orion Logo](/docs/images/orion.png?raw=true)
# Orion

Introducing Orion - the desktop app that lets you create multiple assistants with specific goals. With chatGPT technology behind it, Orion's assistants are capable of helping you with anything you need, based on the goals you've defined for them.

Built with Tauri, Orion is a lightweight app that takes up only 11MB of space on your computer. It's the perfect tool for anyone who wants to define a unique profile for their assistant, complete with a list of instructions, and have personalized conversations with it.

Orion's features include a local SQLite database that stores the chat history for each assistant and allows you to easily organize your conversations by assistant. You can also use your own OpenAI key and general settings are stored in local.

Whether you're looking for a more productive workday or simply want to chat with a friendly virtual assistant, Orion has got you covered. Try it out today and see how it can simplify your life.

## Installation

- Download the installation file and support us by paying what you want (even 0$) from [here](https://taecontrol.lemonsqueezy.com/checkout/buy/0b6e6aea-457b-42ea-a14b-c4371dd1a3fa).
- You can also clone the repo, install dependencies, and run `npm run tauri build`. The installation file is going to be created on `./src-tauri/target/release/bundle` directory.

## Setup

1. Start by opening the app and selecting the settings icon. From there, you can add your OpenAI API key and save the changes. If you don't have an API key yet, you can create one from the OpenAI dashboard.
1. Once you've added your API key, close the settings page and select the assistant icon to create a new assistant. By default, there is a "ChatGPT" assistant.
1. When creating a new assistant, you can provide as much information as you want about how you want your assistant to behave and what kind of information it should provide. You can do this by filling in the description input field.
1. Select the assistant you want to use.
1. Close the assistants page and start a new chat. That's it! You're ready to start using your new assistant.

## Screenshots
### Chat view
Here is where you can chat with you selected assistant:
![Chat view](/docs/images/assistant-chat.png?raw=true)

### Assistants view
You can see and select you assistants from here:
![Assistants view](/docs/images/assistants.png?raw=true)

### Assistant information view
Define how you want your assistant behaves in this view:
![Assistant information view](/docs/images/assistant-config.png?raw=true)

### General settings view
Set you Open AI key and model on this view:
![General settings view](/docs/images/settings.png?raw=true)


## Get our book - MoonGuard: The Software Creator's Journey

Our team has written a book, "MoonGuard: The Software
Creator's Journey." In this book, we document and explain the entire process
of creating, developing, publishing a [Filament plugin](https://filamentphp.com/). Get your own digital copy on [https://moonguard.dev](https://moonguard.dev/book)

## License

Orion desktop app is open-sourced software licensed under the [MIT license](https://opensource.org/licenses/MIT)
