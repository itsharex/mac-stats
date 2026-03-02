# Browser automation (BROWSER_* tools)

BROWSER_NAVIGATE, BROWSER_CLICK, BROWSER_INPUT, and BROWSER_SCREENSHOT use **Chrome** via the Chrome DevTools Protocol (CDP) on port **9222**.

## What mac-stats needs to use the browser

1. **Chrome installed**
   - **macOS:** `/Applications/Google Chrome.app` (standard install).
   - **Linux:** `google-chrome` on PATH.

2. **Chrome on port 9222** — either:

   - **You start Chrome:** run Chrome with remote debugging so mac-stats can attach:
     ```bash
     /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222
     ```
     Leave that window running. mac-stats will connect to it when you use BROWSER_* tools.

   - **mac-stats starts Chrome:** if nothing is listening on 9222, mac-stats will try to launch Chrome with `--remote-debugging-port=9222`, wait ~3 seconds, then connect. No manual step needed if Chrome is installed in the default location.

3. **If you see “Chrome isn’t running on port 9222” or connection errors**
   - Start Chrome manually with the command above, then retry.
   - If Chrome is not installed at the path above, install it or create a symlink; mac-stats does not install Chrome.
   - After a timeout or crash, mac-stats clears the cached session; the next BROWSER_* use will reconnect or relaunch.

## Summary

| Requirement | What mac-stats does |
|------------|----------------------|
| Chrome on 9222 | If port is free, **launches** Chrome with `--remote-debugging-port=9222` (macOS/Linux). If port is in use, **connects** to existing Chrome. |
| Chrome not installed | Cannot launch; you must install Chrome and/or start it manually on 9222. |
| Connection dies (timeout, crash) | Session is cleared on error; next use will reconnect to 9222 or relaunch. |

See also: `docs/026_light_browser_agent_plan.md`, `src-tauri/src/bin/test_cdp_browser.rs`.
