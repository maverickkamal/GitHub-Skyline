# 🏙️ GitHub Skyline

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/yourusername/github-skyline)

> Transform your GitHub contributions into stunning ASCII art cityscapes! 🌆

GitHub Skyline is a command-line tool that visualizes your GitHub contribution history as a beautiful ASCII art city skyline. Each day becomes a building in your personal coding metropolis, with heights representing your daily contribution counts.

## ✨ Features

### 🎨 **Visual Excellence**
- **Braille-style ASCII Art**: Sophisticated Unicode patterns for detailed building textures
- **6 Stunning Themes**: Choose from Synthwave, Dracula, Solarized, Cyberpunk, Matrix, or Sunset
- **Dynamic Sky Elements**: Twinkling stars and phase-based moons that change with your total contributions
- **Smart Scaling**: Non-linear dramatic scaling makes all skylines visually interesting

### 🏆 **Gamification System** 
- **Achievement Unlocking**: Earn Bronze, Silver, Gold, and Legendary achievements
- **Multiple Categories**: Consistency, streaks, intensity, special patterns, and total contributions
- **Real-time Display**: See your accomplishments alongside your skyline

### 💻 **User Experience**
- **Dual Interface**: Interactive mode with splash screen + CLI mode for power users
- **Continuous Generation**: Generate multiple skylines without restarting the app
- **Smart Token Management**: Auto-detection, validation, and helpful setup guides
- **File Output**: Save your skylines as text files to share or archive

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) 1.70 or higher
- A GitHub account
- GitHub Personal Access Token (instructions provided in-app)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/github-skyline.git
cd github-skyline/github_skyline

# Build the project
cargo build --release

# Run it!
cargo run
```

### First Run - Interactive Mode

Simply run the application and follow the guided setup:

```bash
cargo run
```

The app will:
1. 🔑 Help you set up your GitHub token
2. 👤 Ask for the username to visualize  
3. 🎨 Let you choose a color theme
4. 💾 Offer to save the result to a file
5. 🏙️ Generate your beautiful skyline!

### Command Line Usage

For power users, skip the interactive setup:

```bash
# Basic usage
cargo run -- yourusername

# With theme selection
cargo run -- yourusername --theme matrix

# Save to file
cargo run -- yourusername --theme cyberpunk --output my-skyline.txt

# All options
cargo run -- yourusername --theme random --output skyline.txt --no-interactive
```

## 🎨 Themes Gallery

| Theme | Description | Colors |
|-------|-------------|---------|
| **Synthwave** | Neon cyber vibes (default) | 🩵💜 Cyan & Magenta |
| **Dracula** | Dark with vibrant accents | 🖤💜 Dark & Purple |
| **Solarized** | Warm, balanced earth tones | 🟨🟩 Yellow & Green |
| **Cyberpunk** | Electric future aesthetic | 💜🩵 Magenta & Cyan |
| **Matrix** | Digital green rain | 🟢⚪ Green & White |
| **Sunset** | Warm evening colors | 🔴🟡 Red & Yellow |
| **Random** | Surprise me! | 🎲 Randomized |

## 🏆 Achievement System

Unlock achievements across multiple categories:

### 📊 **Total Contributions**
- 🥉 **Active Developer**: 1,000+ contributions
- 🥈 **Prolific Coder**: 2,500+ contributions  
- 🥇 **Coding Machine**: 5,000+ contributions
- 🏆 **Code Titan**: 10,000+ contributions

### ⚡ **Consistency**
- 🏃 **Weekend Warrior**: Active 25%+ of days
- 📊 **Regular Contributor**: Active 50%+ of days
- 📈 **Steady Coder**: Active 70%+ of days
- ⚡ **Daily Grinder**: Active 90%+ of days

### 🔥 **Streaks**
- 📅 **Week Warrior**: 7+ day streak
- 🏁 **Monthly Marathon**: 30+ day streak
- 💯 **Centurion**: 100+ day streak
- 🔥 **Year-Long Dedication**: 365+ day streak

### ⚡ **Intensity**
- 📈 **Big Day**: 10+ contributions in one day
- 🎯 **High Roller**: 25+ contributions in one day
- ⚡ **Power User**: 50+ contributions in one day
- ⚙️ **Efficient Coder**: Average 10+ per active day
- 💎 **Quality over Quantity**: Average 20+ per active day

### 🌟 **Special Patterns**
- 📅 **Perfect Month**: 30 consecutive active days
- 🏃‍♂️ **Marathon Runner**: 3+ perfect months
- 🔄 **The Comeback**: Return after long break
- 🎯 **Strong Finish**: High activity in recent months

## 📖 Usage Examples

### Interactive Mode (Recommended for first-time users)
```bash
cargo run
```
Follow the beautiful ASCII splash screen and guided setup!

### Quick Generation
```bash
# Generate skyline for specific user
cargo run -- octocat

# Different theme
cargo run -- octocat --theme dracula

# Save to file
cargo run -- octocat --output octocat-skyline.txt
```

### Advanced Usage
```bash
# Random theme, save to file, skip interactive prompts
cargo run -- torvalds --theme random --output linus.txt --no-interactive

# Help and options
cargo run -- --help
```

## 🔧 Configuration

### GitHub Token Setup

The app needs a GitHub Personal Access Token to fetch contribution data:

1. Go to [GitHub Settings > Personal Access Tokens](https://github.com/settings/tokens)
2. Click "Generate new token" → "Generate new token (classic)"
3. Give it a name like "GitHub Skyline"
4. **No scopes needed** (public data only)
5. Copy the token

#### Set the token (choose one method):

**Option 1: Interactive Setup** (Recommended)
```bash
cargo run
# Follow the guided setup
```

**Option 2: Environment Variable**
```bash
# Windows (PowerShell)
$env:GITHUB_TOKEN="your_token_here"

# Windows (Command Prompt) 
set GITHUB_TOKEN=your_token_here

# macOS/Linux
export GITHUB_TOKEN="your_token_here"
```

**Option 3: Persistent Setup**
Add to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):
```bash
export GITHUB_TOKEN="your_token_here"
```

## 🎯 CLI Reference

```
github_skyline [OPTIONS] [USERNAME]

ARGUMENTS:
    [USERNAME]    GitHub username to generate skyline for

OPTIONS:
    --theme <THEME>       Color theme: synthwave, dracula, solarized, 
                         cyberpunk, matrix, sunset, random
    -o, --output <FILE>   Save skyline to file instead of terminal
    --no-interactive      Skip interactive mode and prompts
    -h, --help           Show help information
    -V, --version        Show version information
```

## 🔍 Troubleshooting

### Common Issues

**"GITHUB_TOKEN environment variable not set"**
- Follow the token setup instructions above
- Make sure you restart your terminal after setting environment variables

**"401 Unauthorized"**
- Check that your token is valid and properly set
- Try generating a new token if the old one expired
- Ensure no extra spaces or characters in the token

**"Network request failed"**
- Check your internet connection
- GitHub API might be temporarily unavailable
- Try again in a few minutes

**Build errors on Windows**
- Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Or use the GNU toolchain: `rustup default stable-x86_64-pc-windows-gnu`

**"User not found"**
- Verify the GitHub username is correct and public
- Private profiles won't work with public API access

## 🏗️ Project Structure

```
github_skyline/
├── src/
│   ├── main.rs              # Entry point and CLI setup
│   ├── api/                 # GitHub API integration
│   │   ├── client.rs        # HTTP client and auth
│   │   ├── queries.rs       # GraphQL queries
│   │   └── types.rs         # Data structures
│   ├── cli/                 # Command line interface
│   │   └── interactive.rs   # Interactive mode and prompts
│   ├── renderer/            # ASCII art generation
│   │   ├── skyline.rs       # Main rendering logic
│   │   ├── building.rs      # Building utilities
│   │   └── sky_elements.rs  # Stars, moons, sky effects
│   ├── achievements.rs      # Gamification system
│   └── output.rs           # File output handling
├── Cargo.toml              # Dependencies and metadata
└── README.md               # This file!
```

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b amazing-feature`
3. **Make your changes** and test thoroughly
4. **Follow Rust conventions**: `cargo fmt` and `cargo clippy`
5. **Write clear commit messages**
6. **Submit a pull request**

### Development Setup
```bash
git clone https://github.com/yourusername/github-skyline.git
cd github-skyline/github_skyline
cargo build
cargo test
cargo run
```

### Ideas for Contributions
- 🎨 New color themes
- 🏆 Additional achievement types  
- 🌍 Internationalization
- 📊 More visualization options
- 🔧 Performance improvements
- 📖 Documentation enhancements

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- 🦀 **Rust Community** - For the amazing ecosystem
- 🐙 **GitHub** - For the fantastic GraphQL API
- 🎨 **ASCII Art Pioneers** - For inspiration in terminal art
- 💜 **Open Source Contributors** - You make the world better!

## 🌟 Why GitHub Skyline?

Your GitHub contributions tell a story - late nights, creative breakthroughs, collaborative moments, and persistent dedication. GitHub Skyline transforms that story into a visual masterpiece you can share, save, and celebrate.

Whether you're a weekend warrior, a daily grinder, or somewhere in between, your coding journey deserves to be visualized in style. Each commit becomes a block in your personal metropolis, each streak a skyscraper reaching for the digital sky.

**Build your city. Share your story. Code your skyline.** 🏙️✨

---

<div align="center">

**[⭐ Star this repo](https://github.com/yourusername/github-skyline)** • **[🐛 Report Bug](https://github.com/yourusername/github-skyline/issues)** • **[💡 Request Feature](https://github.com/yourusername/github-skyline/issues)**

Made with 💜 by developers, for developers

</div>