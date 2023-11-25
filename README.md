# statusbar
An application that starts up a 'bar that handles battery statistics, time, date and a generic start button.

* [Download (.MSI)](https://github.com/tnhung2011/statusbar/releases)
* [Download (source code)](https://github.com/tnhung2011/statusbar/archive/refs/heads/master.zip)

### Compatibility
As per Tauri restrictions (and its focus on Windows), statusbar only support Windows 7 and above.

<!-- Performance
Since this program utilizes Microsoft Edge WebView2 (through Tauri), it can consume up to ~100 MB of memory, which is lightweight compared to Electron's consumption of ~410 MB. -->

### Usage
This program, on default, overlaps the shell's taskbar. To prevent this, simply run
```batch
taskkill /f /im explorer.exe
```
You may also edit the `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon` key if you wish to replace the main shell.

### Deployment
*(assuming you're in the `src-tauri` folder)*

To initialise this program, run
```batch
cargo update
cargo tauri dev
```

To compile this program, run
```batch
cargo tauri compile
```
