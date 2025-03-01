setlocal

:: Pass the sequence via an environment variable (without extra quotes)
set "SEQUENCE=VK A#_VK SHIFT#VK A#^VK SHIFT#VK A#SLEEPS 1#GOTO 0"
set "DELIMITER=#"

press-key-cheater.exe

endlocal
pause
