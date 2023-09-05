# statusbar
An application that starts up a 'bar that handles battery statistics, time, date and a generic start button.

* [Download (.MSI)](https://github.com/tnhung2011/statusbar/releases)
* [Download (source code)](https://github.com/tnhung2011/statusbar/archive/refs/heads/master.zip)

### Compatibility
Due to specialisations in Windows as par `winapi` and Tauri, statusbar only support Windows 7 and above.

### Performance
Since this program utilizes Microsoft Edge WebView2 (through Tauri), it can consume up to ~100 MB of memory, which is light compared to Electron's consumption of ~410 MB.

### Usage
Since this program's window is transparent, it may overlap the main shell's taskbar. To prevent this, simply run
```batch
taskkill /f /im explorer.exe
```
You may also want to edit the `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon` key if you wish to deplace the main shell.

### Development
To start developing, run
```batch
cd src-tauri
cargo update
cargo tauri dev
```

To compile this program (assuming you're in the folder `src-tauri`), run
```batch
cargo tauri compile
```