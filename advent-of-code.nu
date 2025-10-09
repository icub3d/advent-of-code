# A helper to print messages in a consistent style
def "print-info" [message: string] {
    print $"✅ ($message)"
}

# A helper to print error messages
def "print-error" [message: string] {
    print -e $"❌ ERROR: ($message)"
}

# Normalize and convert a debug-formatted Rust Duration string into a Nushell duration
def "parse-duration" [value: string] {
    $value
    | str trim
    | split row " "
    | where {|part| not ($part | str trim | is-empty) }
    | each {|part|
        let normalized = (
            if ($part | str ends-with "ms") {
                $part
            } else if ($part | str ends-with "µs") {
                $part
            } else if ($part | str ends-with "us") {
                $part | str replace --regex "us$" "µs"
            } else if ($part | str ends-with "ns") {
                $part
            } else if ($part | str ends-with "s") {
                $part | str replace --regex "s$" "sec"
            } else if ($part | str ends-with "m") {
                $part | str replace --regex "m$" "min"
            } else if ($part | str ends-with "h") {
                $part | str replace --regex "h$" "hr"
            } else if ($part | str ends-with "d") {
                $part | str replace --regex "d$" "day"
            } else if ($part | str ends-with "w") {
                $part | str replace --regex "w$" "wk"
            } else {
                $part
            }
        )

        try { $normalized | into duration } catch { 0ns }
    }
    | math sum
}

# run all days for a given year
export def "aoc all" [year: int] {
    let crate_name = $"aoc_($year)"
    let bin_path = $"solutions/($crate_name)/src/bin"

    let results = (
        1..25
        | each {|day|
            let day_mod = (if $day < 10 { $"day0($day)" } else { $"day($day)" })
            let day_file = $"($bin_path)/($day_mod).rs"

            if not ($day_file | path exists) {
                []
            } else {
                let run = (cargo run --release -q -p $crate_name --bin $day_mod | complete)

                if $run.exit_code != 0 {
                    print-error $"Day ($day_mod) failed with exit code ($run.exit_code)"
                    let stderr = ($run.stderr | default "" | str trim)
                    if not ($stderr | is-empty) {
                        print $stderr
                    }
                    []
                } else {
                    $run.stdout
                    | str trim
                    | lines
                    | parse "{part} {time} {solution}"
                    | insert day $day
                    | insert year $year
                }
            }
        }
        | flatten
    )

    if ($results | is-empty) {
        print-info "No puzzle output detected."
    } else {
        print $"🎄 Advent of Code ($year) Summary 🎄"
        let table = (
            $results
            | select year day part time solution
            | sort-by year day part
        )

        let total_duration = (
            $results
            | each {|row| parse-duration ($row.time | default "0ns") }
            | math sum
        )

        print $"total time: ($total_duration)"
        $table
    }
}

# Advent of Code runner
export def "aoc" [year: int, day: int] {
    let crate_name = $"aoc_($year)"
    let day_mod = (if $day < 10 { $"day0($day)" } else { $"day($day)" })
    print $"🎄 Advent of Code ($year) - Day ($day) 🎄"
    cargo run --release -q -p $crate_name --bin $day_mod
}

export def "aoc test" [year: int, day: int] {
    let crate_name = $"aoc_($year)"
    let day_mod = (if $day < 10 { $"day0($day)" } else { $"day($day)" })
    
    print $"🎄 Test Advent of Code ($year) - Day ($day) 🎄"
    cargo test -p $crate_name --bin $day_mod
}

export def "aoc watch" [
    year: int, 
    day: int,
    --test # Run tests instead of the solution
] {
    watch --quiet . --glob=**/*.rs {||
        clear
        try { 
            if $test {
                aoc test $year $day
            } else {
                aoc $year $day 
            }
        } catch { |err| 
            print-error $"Compilation failed: ($err.msg)"
            print "🔄 Watching for changes..."
        }
    }
}

###*
# Sets up a new solution crate for a given year.
#
# This command will:
# 1. Create a new crate like `solutions/aoc_YYYY`.
# 2. Add boilerplate code to the new crate.
# 3. Update the `runner` crate to depend on and call the new crate.
#
# Usage:
# > new-year 2025
###
export def "new-year" [
    year: int, # The Advent of Code year to set up (e.g., 2025)
] {
    let year_str = $"($year)"
    let crate_name = $"aoc_($year_str)"
    let crate_path = $"solutions/($crate_name)"

    # --- 1. Validation ---
    if not ("solutions" | path exists) {
        print-error "This script must be run from the root of your AoC workspace."
        return
    }

    if ($crate_path | path exists) {
        print-error $"Solution crate for year ($year_str) already exists at '($crate_path)'."
        return
    }

    print $"🚀 Setting up Advent of Code ($year_str)..."

    # --- 2. Create the new solution crate ---
    # We use `--vcs none` to avoid creating a nested Git repository.
    cargo new --lib --vcs none $crate_path
    print-info $"Created new crate at '($crate_path)'"

    # --- 3. Configure the new crate's Cargo.toml ---
    let new_cargo_toml = $"
[package]
name = "($crate_name)"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = { workspace = true }
pathfinding = { workspace = true }
serde_json = { workspace = true }
rustc-hash = { workspace = true }
itertools = { workspace = true }
rayon = { workspace = true }
"

    $new_cargo_toml | save --force $"($crate_path)/Cargo.toml"
    print-info $"Configured '($crate_path)/Cargo.toml'"

    rm $"($crate_path)/src/lib.rs"
    mkdir $"($crate_path)/src/bin/inputs"
    print-info $"Created inputs directory"

    print $"\n🎉 Successfully set up year ($year)! You can now add daily solutions."
}

# Adds a new day module to a given year crate and creates boilerplate for that day.
# Usage: new-day 2025 1
export def "new-day" [
    year: int, # The Advent of Code year (e.g., 2025)
    day: int   # The day number (e.g., 1)
] {
    let year_str = $"($year)"
    let day_str = $"($day)"
    let crate_name = $"aoc_($year_str)"
    let crate_path = $"solutions/($crate_name)"
    let src_path = $"($crate_path)/src/bin"
    let day_mod = (if $day < 10 { $"day0($day)" } else { $"day($day)" })
    let day_file = $"($src_path)/($day_mod).rs"

    get-input $year $day

    # --- 1. Validation ---
    if not ($crate_path | path exists) {
        print-error $"Crate for year ($year_str) does not exist at '($crate_path)'!"
        return
    }

    # --- 2. Create the day module file ---
    if ($day_file | path exists) {
        print-error $"Day ($day) already exists for year ($year_str) at '($day_file)'!"
        return
    }
    let day_boiler = "use std::time::Instant;\n\nconst INPUT: &str = include_str!(\"inputs/" + $day_mod + ".txt\");

" + "type Input<'a> = Vec<&'a str>;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.lines().collect()
}

" + 'fn p1(_input: &Input) -> usize {
    0
}

fn p2(_input: &Input) -> usize {
    0
}

fn main() {
    let now = Instant::now();
    let input = parse_input(INPUT);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
'
    $day_boiler | save --force $day_file
    print-info $"Created boilerplate for ($day_mod) at '($day_file)'"
}

###*
# Downloads the puzzle input for a given day and year.
#
# It requires the AOC_SESSION environment variable to be set with your
# session cookie from the Advent of Code website.
#
# Usage:
# > get-input 2025 1
###
export def "get-input" [
    year: int, # The Advent of Code year (e.g., 2025)
    day: int,  # The Advent of Code day (e.g., 5)
] {
    # --- 1. Validation ---
    if not ("AOC_SESSION" in $env) {
        print-error "AOC_SESSION environment variable not set."
        print -e "Please get your session cookie from the website and set it."
        return
    }

    let year_str = $"($year)"
    let crate_name = $"aoc_($year_str)"
    let crate_path = $"solutions/($crate_name)"

    if not ($crate_path | path exists) {
        print-error $"Solution crate for year ($year_str) does not exist."
        print -e $"Run `new-year ($year_str)` first."
        return
    }

    # Format day with a leading zero for consistent filenames (e.g., 1 -> 01)
    let day_str_padded = (if $day < 10 { $"day0($day)" } else { $"day($day)" })
    let output_path = $"($crate_path)/src/bin/inputs/($day_str_padded).txt"

    if ($output_path | path exists) {
        print-info $"Input for ($year)-($day_str_padded) already exists at '($output_path)'. Skipping."
        return
    }

    # --- 2. Download Logic ---
    let url = $"https://adventofcode.com/($year)/day/($day)/input"
    let session_cookie = $env.AOC_SESSION

    # Advent of Code requests that automated scripts have a custom User-Agent
    # to help them identify traffic.
    let user_agent = "github.com/icub3d/advent-of-code by joshua.marshian@gmail.com"

    print $"🚀 Downloading input for ($year)-($day_str_padded) from ($url)..."

    try {
        let response = http get --headers {
                Cookie: $"session=($session_cookie)"
                "User-Agent": $user_agent
            } $url

        # The response from AoC might end in a newline, which we usually want to keep.
        $response | save --force $output_path
        print-info $"Successfully saved input to '($output_path)'"
    } catch { |error|
        print-error "Failed to download input."
        # The error variable in Nushell holds details about the exception
        if ($error.msg | str contains "404") {
            print -e "Reason: Received status 404. The puzzle for this day might not be unlocked yet."
        } else if ($error.msg | str contains "401") {
            print -e "Reason: Received status 401. Your AOC_SESSION cookie might be invalid or expired."
        } else if ($error.msg | str contains "500") {
            print -e "Reason: Received status 500. Advent of Code might be having server issues."
        } else {
            print -e $"Reason: ($error.msg)"
        }
        return
    }
}

# Uploads a day's solution file to a GitHub Gist using the gh CLI.
# Usage: upload-gist 2015 1 [--public]
export def "upload-gist" [
    year: int, # The Advent of Code year (e.g., 2015)
    day: int,  # The day number (e.g., 1)
] {
    let year_str = $"($year)"
    let day_str = (if $day < 10 { ("day0" ++ ($day | into string)) } else { ("day" ++ ($day | into string)) })
    let file_path = ("solutions/aoc_" ++ $year_str ++ "/src/bin/" ++ $day_str ++ ".rs")

    if not ($file_path | path exists) {
        print-error ("Solution file not found: " ++ $file_path)
        return
    }

    let gist_desc = ($"Advent of Code ($year_str) Day ($day) Solution")
    let public_flag = "--public"

    print ("🚀 Uploading " ++ $file_path ++ " to GitHub Gist...")
    let cmd = ["gh" "gist" "create" $file_path "--desc" $gist_desc $public_flag]
    let result = do -i { ^$cmd }
    if ($result | describe) == 'string' {
        print-info "Gist uploaded successfully!"
        $result
    } else if ($result.exit_code? | default 1) == 0 {
        print-info "Gist uploaded successfully!"
        $result.stdout? | default ""
    } else {
        print-error "Failed to upload Gist."
        $result.stderr? | default $result
    }
}

# Generate a YouTube description with timestamps from a stage progress JSON file.
# Usage: youtube-desc path/to/2015-13.json
export def "youtube-desc" [
    file: string # The path to the JSON file (e.g., '2015-13.json')
] {
    let file = ($file | path expand)

    # Validate file exists
    if not ($file | path exists) {
        print-error $"JSON file not found: '($file)'"
        return
    }

    # Derive year and day from the filename (e.g., '2015-13.json')
    let base = ($file | path basename)
    let base_no_ext = ($base | str replace ".json" "")
    let parts = ($base_no_ext | split row "-")
    if ($parts | length) < 2 {
        print-error "Filename must be in the format 'YEAR-DAY.json' (e.g., '2015-13.json')."
        return
    }
    let year = ($parts | get 0 | into int)
    let day = ($parts | get 1 | into int)

    # --- Find or Create Gist ---
    let filter_str = $"($year) Day ($day)"
    let gist_id = (gh gist list --limit 1 --filter $filter_str | split column "\t" | get column1 | first)

    let solution_url = if not ($gist_id | is-empty) {
        $"https://gist.github.com/icub3d/($gist_id)"
    } else {
        # No gist found, so create one and capture the output URL
        upload-gist $year $day
    }

    # Parse JSON
    let data = (open --raw $file | from json)

    # Build problem URL
    let problem_url = $"https://adventofcode.com/($year)/day/($day)"

    # Print header for description
    print "[TODO]"
    print ""
    print $"Problem: ($problem_url)"
    print $"Solution: ($solution_url)"
    print ""

    # Get stage times (fall back to empty if missing)
    let stages = ($data | get stageTimes | default [])

    # Sort stages by startMs to ensure order
    let stages = ($stages | sort-by startMs)

    if ($stages | is-empty) {
        print-info "No 'stageTimes' found in JSON."
        return
    }

    # Print timestamp lines, converting startMs (milliseconds) to a 'M:SS' format.
    for $st in $stages {
        let start_ms = ($st | get startMs | default 0)
        let mins = ($start_ms / 60000 | into int)
        let secs = (($start_ms mod 60000) / 1000 | into int)
        let time_str = (if $secs < 10 { $"($mins):0($secs)" } else { $"($mins):($secs)" })
        let name = ($st | get stageName | default "Unnamed Stage")
        print $"($time_str) ($name)"
    }
}
