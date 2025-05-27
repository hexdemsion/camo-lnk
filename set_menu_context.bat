@echo off
setlocal ENABLEDELAYEDEXPANSION
:: Prompt menu
echo [1] Install context menu
echo [2] Uninstall context menu
set /p choice=Choose an option (1/2):
if "%choice%"=="1" goto :install
if "%choice%"=="2" goto :uninstall
echo Invalid choice.
goto :eof

:install
echo Installing context menu entries...
:: Add Camouflage_this_file to all file types (*)
reg add "HKCU\Software\Classes\*\shell\Camouflage_this_file" /ve /d "" /f
reg add "HKCU\Software\Classes\*\shell\Camouflage_this_file\command" /ve /t REG_EXPAND_SZ /d "\"C:\Users\%%USERNAME%%\Desktop\camo\target\release\hider.exe\" \"%%1\"" /f

:: Add Camouflage_entire_folder to folders
reg add "HKCU\Software\Classes\Directory\shell\Camouflage_entire_folder" /ve /d "" /f
reg add "HKCU\Software\Classes\Directory\shell\Camouflage_entire_folder\command" /ve /t REG_EXPAND_SZ /d "\"C:\Users\%%USERNAME%%\Desktop\camo\target\release\hider-folder.exe\" \"%%1\"" /f

:: Add Recover_this_file to shortcut files (.lnk)
reg add "HKCU\Software\Classes\lnkfile\shell\Recover_this_file" /ve /d "" /f
reg add "HKCU\Software\Classes\lnkfile\shell\Recover_this_file\command" /ve /t REG_EXPAND_SZ /d "\"C:\Users\%%USERNAME%%\Desktop\camo\target\release\recover.exe\" \"%%L\"" /f
echo Done.
goto :eof

:uninstall
echo Uninstalling context menu entries...
:: Remove Camouflage
reg delete "HKCU\Software\Classes\*\shell\Camouflage_this_file" /f
:: Remove Camouflage for folders
reg delete "HKCU\Software\Classes\Directory\shell\Camouflage_entire_folder" /f
:: Remove Recover
reg delete "HKCU\Software\Classes\lnkfile\shell\Recover_this_file" /f
echo Removed.
goto :eof