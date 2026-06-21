#!/bin/sh

# This script is designed for Docker usage only! It must be run as root, and will install bx directly to `/bin`.

# Get the URL to the latest version of the musl binary
url=$(curl -s https://api.github.com/repos/codemonument/bx/releases/latest | grep 'browser_' | cut -d\" -f4 | grep musl)

# Download the latest release of bx for musl and put it in `/bin`
curl -L $url > /bin/bx
chmod +x /bin/bx
