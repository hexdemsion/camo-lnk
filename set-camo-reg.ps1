# type this into privileged powershell window
# powershell -ExecutionPolicy Bypass -File .\set-camo-reg.ps1


# Set PowerShell to stop on error
$ErrorActionPreference = "Stop"

# Base path (gunakan HKCU agar tidak butuh admin)
$root = "Registry::HKEY_CURRENT_USER\Software\Classes"

# Define registry values with environment variables (ExpandString)
$camoPath = "`"C:\Users\$env:USERNAME\Desktop\camo\target\debug\hider.exe`" `"%1`""
$recoverPath = "`"C:\Users\$env:USERNAME\Desktop\camo\target\debug\recover.exe`" `"%L`""

# Camouflage_this_file for all files
New-Item -Path "$root\*\shell\Camouflage_this_file" -Force | Out-Null
New-Item -Path "$root\*\shell\Camouflage_this_file\command" -Force | Out-Null
Set-ItemProperty -Path "$root\*\shell\Camouflage_this_file\command" -Name '(default)' -Value $camoPath -Type ExpandString

# Recover_this_file for shortcuts
New-Item -Path "$root\lnkfile\shell\Recover_this_file" -Force | Out-Null
New-Item -Path "$root\lnkfile\shell\Recover_this_file\command" -Force | Out-Null
Set-ItemProperty -Path "$root\lnkfile\shell\Recover_this_file\command" -Name '(default)' -Value $recoverPath -Type ExpandString

Write-Host "✅ Registry entries created successfully under HKCU (user-level)."
