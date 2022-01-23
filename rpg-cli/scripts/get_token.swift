#!/usr/bin/swift
import Foundation

do {
    let token = try String(contentsOfFile: "scripts/token.txt")
    print("\(token.trimmingCharacters(in: .whitespacesAndNewlines))")
} catch {
    print("\(error)")
}
