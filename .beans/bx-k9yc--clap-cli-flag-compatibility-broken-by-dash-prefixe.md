---
# bx-k9yc
title: Clap CLI flag compatibility broken by dash-prefixed subcommands
status: todo
type: bug
created_at: 2026-01-22T19:36:28Z
updated_at: 2026-01-22T19:36:28Z
---

Clap does not dispatch dash-prefixed subcommands (e.g., "-v", "-i", "-c", "-h"). As a result, legacy flags are treated as positional args or intercepted by Clap's built-ins, so version/help/init/cache handlers never run. This breaks existing CLI behavior and will cause new CLI tests to fail.\n\n## Checklist\n- [ ] Review src/cli/args.rs for dash-prefixed subcommand definitions\n- [ ] Replace dash-prefixed subcommands with proper flags/options or disable Clap default help/version as needed\n- [ ] Ensure legacy flags (-v/-h/-i/-c) route to correct handlers\n- [ ] Update/verify CLI tests for legacy flag compatibility