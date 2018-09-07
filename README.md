# evegate



- Create ~/.local/share/applications/eveauth-evegate.desktop file with content:
[Desktop Entry]
Version=1.0
StartupNotify=true
Terminal=true
Type=Application
Name=Eve Online Callback Application
Comment=Provides callback handler for the EVE Single Sign-On (SSO) technology
TryExec=evegate
Exec=evegate --auth %u
MimeType=x-scheme-handler/eveauth-evegate;
Icon=evegate

- Check it with
$ cat ~/.local/share/applications/eveauth-evegate.desktop

- Register 
$ xdg-mime default eveauth-evegate.desktop  x-scheme-handler/eveauth-evegate

-Test 
$ xdg-open eveauth-evegate://foo/

-Try to receive external request
$ xdg-open https://login.eveonline.com/oauth/authorize/?response_type=code&redirect_uri=eveauth-evegate://auth/&client_id=<...>&scope=publicData&state=A