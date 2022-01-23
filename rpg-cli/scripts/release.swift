#!/usr/bin/swift
// Environment variables
// BUILD=BOOL (default=false)

import Foundation

/// Runs a shell command and returns the output
func shell(_ command: String) -> String {
    let task = Process()
    let pipe = Pipe()

    task.standardOutput = pipe
    task.standardError = pipe
    task.arguments = ["-c", command]
    task.launchPath = "/bin/zsh"
    task.launch()

    let data = pipe.fileHandleForReading.readDataToEndOfFile()
    let output = String(data: data, encoding: .utf8)!

    return output
}

/// Runs a shell command and prints the output
func pshell(_ command: String) {
    print(shell(command))
}

let build: String = ProcessInfo.processInfo.environment["BUILD"] ?? "false"

if build == "true" {
    pshell("./scripts/build.sh")
}

pshell("./scripts/package.sh")