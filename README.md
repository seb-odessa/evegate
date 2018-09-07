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

Application details: https://developers.eveonline.com/applications/details/37869



STEP 1
https://login.eveonline.com/oauth/authorize/?response_type=code&redirect_uri=http://host/callback&client_id=<...>&scope=publicData&state=A

Authorization Code: 8pcDz97YMB0puiTbGBRchxpIAADOrd3SHqVm09oplGCOBBFa2QjoucAaQMUgUL060
Authorization State: A


#access token
curl --data "grant_type=authorization_code&code=8pcDz97YMB0puiTbGBRchxpIAADOrd3SHqVm09oplGCOBBFa2QjoucAaQMUgUL060" -H "Authorization: Basic MmUxMzA2YjA1OTg3NGEwMjgxNTkzZGVlMTBiMmJiZmM6SG9oRWlrUmFqV0ZlOGhONU5ld2hsTXVna1hsa0pqM2VpMlJJZHlkVA==" -H "Content-Type: application/x-www-form-urlencoded" -H "Host: login.eveonline.com" https://login.eveonline.com/oauth/token

{"access_token":"Z3WO-dmYNPfgwlpUpamNo5dadiP9dklcdg797MTAN4yOMjFRsLx2faoboGq0Tjevvp17vwN2RUAZVhRLmF2v3Q2","token_type":"Bearer","expires_in":1199,"refresh_token":"G98p5aMSccFDLnH-Ho3-waVju97GKA4tm3CYYIfxi0U1"}

#refresh token
curl --data "grant_type=refresh_token&refresh_token=G98p5aMSccFDLnH-Ho3-waVju97GKA4tm3CYYIfxi0U1" -H "Authorization: Basic MmUxMzA2YjA1OTg3NGEwMjgxNTkzZGVlMTBiMmJiZmM6SG9oRWlrUmFqV0ZlOGhONU5ld2hsTXVna1hsa0pqM2VpMlJJZHlkVA==" -H "Content-Type: application/x-www-form-urlencoded" -H "Host: login.eveonline.com" https://login.eveonline.com/oauth/token

{"access_token":"adjp_Xd9Uvp1EGcAyBxqVtV4iID_LG3QpaLE905myp9hfWtba1_UMU86hgvl6D1eu4PgkWdGxAxFDZjRZP24Uw2","token_type":"Bearer","expires_in":1199,"refresh_token":"G98p5aMSccFDLnH-Ho3-waVju97GKA4tm3CYYIfxi0U1"}


#Obtaining Character ID
curl -H "Authorization: Bearer adjp_Xd9Uvp1EGcAyBxqVtV4iID_LG3QpaLE905myp9hfWtba1_UMU86hgvl6D1eu4PgkWdGxAxFDZjRZP24Uw2" -H "Host: esi.tech.ccp.is" https://esi.tech.ccp.is/verify/

{"CharacterID":90306744,"CharacterName":"Pavel Sergeevich","ExpiresOn":"2018-09-06T12:31:03.2319974","Scopes":"publicData","TokenType":"Character","CharacterOwnerHash":"gHPs2tUO/A5RgN3Ufl4mkyIzzfI=","IntellectualProperty":"EVE"}

 
# Get character's public information
curl -H "Authorization: Bearer adjp_Xd9Uvp1EGcAyBxqVtV4iID_LG3QpaLE905myp9hfWtba1_UMU86hgvl6D1eu4PgkWdGxAxFDZjRZP24Uw2" -H "Host: esi.tech.ccp.is" -H "Content-Type: application/json" https://esi.tech.ccp.is/latest/characters/90306744/ 

{"ancestry_id":2,"birthday":"2011-01-19T15:11:00Z","bloodline_id":5,"corporation_id":1000066,"description":"\u2605\u2606\u2606 \u041a\u043e\u0440\u043f\u043e\u0440\u0430\u0446\u0438\u044f Air Force Special Ops \u043f\u0440\u0438\u043c\u0435\u0442 \u0432 \u0441\u0432\u043e\u0438 \u0440\u044f\u0434\u044b \u043f\u0438\u043b\u043e\u0442\u043e\u0432 \u0432\u0441\u0435\u0445 \u043d\u0430\u043f\u0440\u0430\u0432\u043b\u0435\u043d\u0438\u0439.\u2605\u2606\u2606.<br>\u041e\u043a\u0430\u0437\u044b\u0432\u0430\u0435\u043c \u0438\u043d\u0444\u043e\u0440\u043c\u0430\u0446\u0438\u043e\u043d\u043d\u0443\u044e \u043f\u043e\u0434\u0434\u0435\u0440\u0436\u043a\u0443 \u043f\u043e \u043e\u0431\u0443\u0447\u0435\u043d\u0438\u044e \u0432 \u0438\u0433\u0440\u0435.<br>\u041f\u0440\u043e\u0432\u043e\u0434\u0438\u043c \u043e\u0431\u0443\u0447\u0430\u044e\u0449\u0438\u0435 \u041f\u0412\u041f \u0432\u044b\u043b\u0435\u0442\u044b \u043f\u0440\u0438 \u0443\u0447\u0430\u0441\u0442\u0438\u0438 \u0430\u043b\u0438 \u0444\u043b\u043e\u0442\u0430 \u0432 \u043b\u043e\u0443.<br>\u041f\u043e\u0434\u043a\u043b\u044e\u0447\u0438\u0442\u0435\u0441\u044c \u043a \u0440\u0435\u043a\u0440\u0443\u0442 \u043a\u0430\u043d\u0430\u043b\u0443 AFSO \u0442\u0443\u0442 \u0432\u044b \u0441\u043c\u043e\u0436\u0435\u0442\u0435 \u0443\u0442\u043e\u0447\u043d\u0438\u0442\u044c \u0438\u043d\u0442\u0435\u0440\u0435\u0441\u0443\u044e\u0449\u0438\u0435 \u0432\u0430\u0441 \u043f\u043e\u0434\u0440\u043e\u0431\u043d\u043e\u0441\u0442\u0438.<br><br>** Targeting systems \/ \u0421\u0438\u0441\u0442\u0435\u043c\u044b \u041f\u0440\u0438\u0446\u0435\u043b\u0438\u0432\u0430\u043d\u0438\u044f **<br>Amarr - Radar    Caldary - Gravimetric     Gallente - Magnetometric   Minmatar - Ladar<br><br>** Main Damage \/ \u041e\u0441\u043d\u043e\u0432\u043d\u043e\u0439 \u0442\u0438\u043f \u0423\u0440\u043e\u043d\u0430 **<br><a href=\"showinfo:2\/\/1000127\">Guristas<\/a>      - Kinetic<br><a href=\"showinfo:2\/\/1000135\">Serpentis<\/a>     - Thermal<br><a href=\"showinfo:2\/\/98000190\">Mordus<\/a>        - Thermal &amp; Kinetic<br><a href=\"showinfo:2\/\/98014742\">Blood<\/a>          - EM &amp; Thermal<br><a href=\"showinfo:2\/\/98017295\">Sansha<\/a>        - EM &amp; Thermal<br>Angel          - Explosive<br><a href=\"showinfo:2\/\/469432667\">Mercenary<\/a>   - Thermal","gender":"male","name":"Pavel Sergeevich","race_id":4,"security_status":1.9016243340678807}

