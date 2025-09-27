# advent-of-code

My solutions to https://adventofcode.com

## Nushell overlay (advent-of-code.nu)

This repository includes `advent-of-code.nu` — a Nushell script that adds helpers and shortcuts for working with the Advent of Code workspace.

You can load those helpers into your current Nushell session temporarily, or install them as a persistent overlay so they're available every time you start Nushell.

### Quick (temporary) — load into the current session
From Nushell, run:
```nu
source ./advent-of-code.nu
```
After that you can call the helpers (examples below) in the same shell session.

### Persistent (recommended) — install as an overlay
Copy the file into Nushell's overlays directory so it is automatically available.

- Linux / macOS:
```sh
mkdir -p ~/.config/nushell/overlays
cp advent-of-code.nu ~/.config/nushell/overlays/advent-of-code.nu
```

- Windows (PowerShell):
```powershell
New-Item -ItemType Directory -Force $env:USERPROFILE\.config\nushell\overlays
Copy-Item .\advent-of-code.nu -Destination "$env:USERPROFILE\.config\nushell\overlays\advent-of-code.nu"
```

After copying, start a new Nushell session (or `source` the file) to load the overlay.

Note: overlay placement is the conventional location; if your Nushell config directory differs, place the file into the `overlays` subdirectory of your Nushell config directory.

## Important prerequisites
- Nushell installed (nu)
- Rust toolchain + Cargo (for building/running solutions)
- For `get-input` to work you must set the `AOC_SESSION` environment variable with your Advent of Code session cookie.
- For `upload-gist` you must have the GitHub CLI (`gh`) configured & authenticated.

## Example helper commands
After loading the overlay (see above), here are common examples:

- Run a solution (runner crate): run Advent of Code 2015 day 1
```nu
aoc 2015 1
```

- Run tests for a specific day (quiet):
```nu
aoc test 2015 1
```

- Watch source files and re-run a day on change:
```nu
aoc watch 2015 1
```

- Download puzzle input for a day (requires AOC_SESSION):
```nu
get-input 2015 1
```

- Create a new year crate skeleton:
```nu
new-year 2025
```

- Create a new day module for a year (also fetches the input):
```nu
new-day 2025 1
```

- Upload a day's solution file to a GitHub Gist:
```nu
upload-gist 2015 1
```

- Generate a YouTube description from a stage progress JSON:
```nu
youtube-desc path/to/2015-13.json
```

## Troubleshooting
- If a command fails with authentication errors, double-check `AOC_SESSION` (for `get-input`) or that `gh auth status` shows you are logged in (for `upload-gist`).
- If a helper is not found after installing the overlay, either `source` the file manually in your session or restart Nushell.

Enjoy — and happy coding!
