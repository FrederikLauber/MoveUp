[Setup]
AppName=MoveUp
AppVersion={#GetEnv('VERSION')}
DefaultDirName={pf}\MoveUp
OutputBaseFilename=setup
DefaultGroupName=MoveUp
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64
SetupIconFile=assets\ico.ico
UninstallDisplayIcon={app}\MoveUp.exe

[Files]
Source: "target\release\MoveUp.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "LICENSE.md"; Flags: dontcopy

[Setup]
LicenseFile=LICENSE.md

[Registry]
Root: HKLM; Subkey: "SOFTWARE\Classes\Directory\shell\MoveUp"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\MoveUp.exe"; Flags: uninsdeletekeyifempty
Root: HKLM; Subkey: "SOFTWARE\Classes\Directory\shell\MoveUp"; ValueType: string; ValueName: "MultiSelectModel"; ValueData: "player"; Flags: uninsdeletekeyifempty
Root: HKLM; Subkey: "SOFTWARE\Classes\Directory\shell\MoveUp"; ValueType: string; ValueName: "NoWorkingDirectory"; ValueData: ""; Flags: uninsdeletekeyifempty
Root: HKLM; Subkey: "SOFTWARE\Classes\Directory\shell\MoveUp\command"; ValueType: string; ValueName: ""; ValueData: "{app}\MoveUp.exe ""%V"""; Flags: uninsdeletekeyifempty
