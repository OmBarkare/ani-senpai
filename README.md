# ani-senpai

This project was inspired from my daily pain of scrolling through countless anime posters and banners,  
and trying to manually figure out what genre or type of anime I want to watch.

Now, I just tell it the mood I am in or what type of anime I want to watch (something like *x*), and it generates a list of anime, sorted in order of rating (and some special recommendation from the AI itself) that I can select from. It then also plays the anime directly in `mpv` using `ani-cli`.

## How it works

- It uses Gemini to understand your intent and extracts genres from it  
- Uses those genres to get a list of animes from AniList  
- Shows you the list in an interactive TUI to select from  
- Plays the selected anime in `mpv` using `ani-cli`

## Usage

Clone the project:

```bash
git clone https://github.com/OmBarkare/ani-senpai.git
```
build it:
```bash
cargo build --release
```
After building it, put the binary in the /bin directory.
(or wherever you want, but then include the path in .bashrc)
You will find it in:
```bash
/target/release/
```

## Dependencies
ani-cli should be installed first to use this
```bash
sudo apt install ani-cli
```

set your GEMINI_API_KEY in .bashrc

## known issues
- If the Gemini API key’s token limit is exhausted, it fails.