#!/usr/bin/ruby

input = ARGV[0]
regex = /https:\/\/[a-zA-Z0-9\-._\/]+/
puts input[regex]