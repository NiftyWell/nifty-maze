# Nifty Maze Smart Contract & Ping Proxy

🦖 Welcome to the Nifty Maze Smart Contract and Ping Proxy! 🦖
![output](https://github.com/user-attachments/assets/f5f1482d-3d99-4169-9234-802136fd1758)

## Nifty Maze Smart Contract
🌐 The Nifty Maze Smart Contract is a part of the Nifty Rex project and is designed for the MultiversX blockchain (formerly known as Elrond). It allows players to participate in an exciting maze game and win rewards by navigating through a maze matrix filled with various blocks, including prizes, traps, doors, and keys.

🎮 Players can register moves and progress through the maze, but they must be careful as walls, traps, and locked doors can obstruct their path. To claim the title of Maze MVP, players must reach the finish line, and the MVP (Player spending the most ESDTs) will receive a share of the collected tokens as a reward.

💡 The contract is open-source under the GNU General Public License (GPL). Feel free to explore the code, contribute improvements, or create exciting variations of the game!

➡️ Whenever a new maze is open you can play on https://nifty-maze.netlify.app/.

## Ping Proxy

🔗 The Ping Proxy is a simple yet essential component of the Nifty Maze Smart Contract. It acts as a proxy to call the `ping` method in the main nifty-maze contract when the caller contract calls the `pong` method in the ping proxy.

⏱️ The Ping Proxy is used together with the main contract to create a "clock" that will allow users, during each game round, to register moves for a set period of time before the contract will automatically pick a random winner.

## Contribution & Ideas

🤝 We welcome contributions and ideas from the community to enhance the Nifty Maze Smart Contract and Ping Proxy! If you have suggestions, bug fixes, or exciting variations to propose, feel free to join the project and help shape the future of Nifty Rex.

🔧 Whether you are a developer, designer, or blockchain enthusiast, your expertise is valuable, and we encourage you to participate in the development of Nifty Maze and its supporting components.

Let's navigate the maze together and unlock the potential of Nifty Rex! 🗝️🎉
