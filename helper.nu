#!/usr/bin/env nu

source ~/.config/nushell/config.nu

def get-input [workspace: string, name: string] {
    if not ("AOC_SESSION" in $env) {
        print -e "AOC_SESSION environment variable not set."
        return
    }

    mkdir $"($workspace)/src/bin/inputs" 
    let output_path = $"($workspace)/src/bin/inputs/($name).txt"
    if ($output_path | path exists) {
        print-info $"Input for ($workspace) ($name) already exists at '($output_path)'. Skipping."
        return
    }

    let year = ($workspace | split row '_' | last)
    let day = ($name | str replace 'day' '' | into int)

    let url = $"https://adventofcode.com/($year)/day/($day)/input"
    let session_cookie = $env.AOC_SESSION

    # Advent of Code requests that automated scripts have a custom User-Agent
    # to help them identify traffic.
    let user_agent = "github.com/icub3d/advent-of-code by joshua.marshian@gmail.com"

    print $"🚀 Downloading input for ($workspace)-($name) from ($url)..."

    try {
        let response = http get --headers {
                Cookie: $"session=($session_cookie)"
                "User-Agent": $user_agent
            } $url

        # The response from AoC might end in a newline, which we usually want to keep.
        $response | save --force $output_path
        print $"Successfully saved input to '($output_path)'"
    } catch { |error|
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

def get-target [workspace: string, name: string, part="1": string] {
  # Right now we don't do anything here.
}

def youtube [year: int, day: int] {
  let day_str = if $day < 10 { $"0($day)" } else { $"($day)" };

  let url = $"https://adventofcode.com/($year)/day/($day)" 
  let times = $"~/Videos/($year)-($day_str).json" 
  let desc = $"Solution for Advent of Code ($year) Day ($day)";
  let file = $"aoc_($year)/src/bin/day($day_str).rs";

  youtube-description $url $times $desc $file
}

def main [command?: string, ...args] {
  if ($command | is-empty) {
    print "Usage: helper.nu <command> [args...]"
    print "Available commands:"
    print "  get-input <workspace> <name> [part]"
    print "  get-target <workspace> <name> [part]"
    print "  youtube <workspace> <name>"
    return
  }

  let func_name = $command | str replace "-" "_"
  
  match $command {
    "get-input" => {
      if ($args | length) < 2 {
        print "Usage: get-input <workspace> <name>"
        return
      }
      let workspace = ($args | get 0)
      let name = ($args | get 1)
      get-input $workspace $name 
    }
    "get-target" => {
      if ($args | length) < 2 {
        print "Usage: get-target <workspace> <name> [part]"
        return
      }
      let workspace = ($args | get 0)
      let name = ($args | get 1)
      let part = (if ($args | length) >= 3 { $args | get 2 } else { "1" })
      get-target $workspace $name $part
    }
    "youtube" => {
      if ($args | length) < 2 {
        print "Usage: youtube <year> <quest>"
        return
      }
      let year = ($args | get 0 | into int)
      let quest = ($args | get 1 | into int)
      youtube $year $quest
    }
    _ => {
      print $"Unknown command: ($command)"
      print "Available commands:"
      print "  get-input <workspace> <name> [part]"
      print "  get-target <workspace> <name> [part]"
      print "  youtube <year> <quest>"
    }
  }
}

