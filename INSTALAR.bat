@echo off
echo ========================================
echo    MATRIX TERMINAL - INSTALADOR
echo ========================================
echo.

echo Copiando ejecutable a la carpeta de programas...
if not exist "C:\Program Files\MatrixTerminal" mkdir "C:\Program Files\MatrixTerminal"
copy "target\release\matrix-terminal.exe" "C:\Program Files\MatrixTerminal\"

echo.
echo Creando acceso directo en el escritorio...
powershell "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%USERPROFILE%\Desktop\Matrix Terminal.lnk'); $Shortcut.TargetPath = 'C:\Program Files\MatrixTerminal\matrix-terminal.exe'; $Shortcut.Save()"

echo.
echo ========================================
echo    INSTALACION COMPLETADA
echo ========================================
echo.
echo La terminal Matrix ha sido instalada en:
echo C:\Program Files\MatrixTerminal\matrix-terminal.exe
echo.
echo Se ha creado un acceso directo en el escritorio.
echo.
echo Presiona cualquier tecla para salir...
pause > nul
