# A helper to print messages in a consistent style
def "print-info" [message: string] {
    print $"✅ ($message)"
}

# A helper to print error messages
def "print-error" [message: string] {
    print -e $"❌ ERROR: ($message)"
}

# Advent of Code runner
export def "aoc" [year: int, day: int] {
    cargo run --release -q -- -y $year -d $day
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
    if not ("runner" | path exists) or not ("solutions" | path exists) {
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
"

    $new_cargo_toml | save --force $"($crate_path)/Cargo.toml"
    print-info $"Configured '($crate_path)/Cargo.toml'"

    # --- 4. Add boilerplate to the new crate's lib.rs and create inputs folder ---
let new_lib_rs = ' 
pub fn run(day: u8) -> anyhow::Result<()> {
    match day {
        _ => println!("Day {{day}} not yet implemented for ''' + $year_str + '''."),
    }
    Ok(())
}
'

    $new_lib_rs | save --force $"($crate_path)/src/lib.rs"
    mkdir $"($crate_path)/src/inputs"
    print-info $"Added boilerplate to '($crate_path)/src/lib.rs' and created inputs directory"

    # --- 5. Update the runner's Cargo.toml ---
    let runner_toml_path = "runner/Cargo.toml"
    let dep_line = $"($crate_name) = \{ path = \"../($crate_path)\" }"
    let runner_toml_content = open --raw $runner_toml_path
    let updated_runner_toml = $runner_toml_content | str replace "[dependencies]" $"[dependencies]\n($dep_line)"
    $updated_runner_toml | save --force $runner_toml_path
    print-info $"Updated '($runner_toml_path)' with new dependency"

    # --- 6. Update the runner's main.rs ---
    let runner_main_path = "runner/src/main.rs"
    let match_arm_line = "        " + $year_str + " => " + $crate_name + "::run(args.day)?,"
    # We target the line with the catch-all arm to insert our new line before it.
    let insertion_target = "        _ => println!"
    let runner_main_content = open $runner_main_path
    let updated_runner_main = $runner_main_content | str replace $insertion_target $"($match_arm_line)\n($insertion_target)"
    $updated_runner_main | save --force $runner_main_path
    print-info $"Updated '($runner_main_path)' with new match arm"

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
    let src_path = $"($crate_path)/src"
    let day_mod = (if $day < 10 { $"day0($day)" } else { $"day($day)" })
    let day_file = $"($src_path)/($day_mod).rs"

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
    let day_boiler = 'const INPUT: &' + "'" + 'static str = include_str!("inputs/day01.txt");

pub fn p1() -> anyhow::Result<()> {
    println!("p1: not implemented");
    Ok(())
}

pub fn p2() -> anyhow::Result<()> {
    println!("p2: not implemented");
    Ok(())
}

pub fn solve() -> anyhow::Result<()> {
    p1()?;
    p2()?;
    Ok(())
}
'
    $day_boiler | save --force $day_file
    print-info $"Created boilerplate for ($day_mod) at '($day_file)'"

    # --- 3. Update the year lib.rs to include the new module and match arm ---
    let lib_path = $"($src_path)/lib.rs"
    let lib_content = open --raw $lib_path
    # Add mod if not present (always add at the top)
    let mod_decl = $"pub mod ($day_mod);"
    let new_lib_content = if ($lib_content | str contains $mod_decl) {
        $lib_content
    } else {
        $mod_decl + "\n" + $lib_content
    }
    # Add match arm after 'match day {'
    let match_arm = $"        ($day) => ($day_mod)::solve" + "()?,"
    let new_lib_content = $new_lib_content | str replace "match day {" ("match day {\n" + $match_arm)
    $new_lib_content | save --force $lib_path
    print-info $"Updated '($lib_path)' to include ($day_mod) and match arm"
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
    let output_path = $"($crate_path)/src/inputs/($day_str_padded).txt"

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


