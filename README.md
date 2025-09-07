```launchpad.c``` &rarr; ```rfPacketRx.c``` <br>
```sensortag.c``` &rarr; ```rfPacketTx.c```

# PewPew (IOT LAB Final) 

## How to Run

### Prerequisites

#### Flash Sensortag and Launchpad
Use the Node/Concentrator projects from the lab, add `sensortag.c` and `launchpad.c` to the corresponding projects and remember to start their main threads.

#### Install Dependencies
To build, you need sdl2, sdl2-ttf, sdl2-image and sdl2-mixer. On Windows, all the libraries are already in this repository, on Linux you have to install them yourself (on Debian or Ubuntu, install `libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev libsdl2-mixer-dev`)

#### Set the Serial Port
Then open `pewpew/src/user_settings.rs` and set the value of `SERIAL_PORT`.

Finally run `cargo run --bin pewpew` and the game should start

## DOD (Definition of Done)
### Modus 1
- [x] auf Bildschirm schießen mit HitReg durch BinSearch
### Modus 2
- [ ] Multiplayer Mohrhuhn
  - [x] Multiplayer auf gleichem Bildschirm
  - [ ] ~~custom, präzise Hitboxen~~
  - [x] statisches Bild (nicht rechts/links verschieben)
  - [ ] Figuren
    - [x] fliegende Hühner
    - [ ] bienendude
    - [ ] ~~rennende Hühner~~
    - [ ] ~~Pfannendude~~
  - [ ] ~~Jogger verschwinden hinter mind. einem Obj~~
  - [ ] Punktestand für Spieler und Timer
  - [ ] Scoreboard vom aktuellen Spiel
  - [x] **Reload per Gyro Flick** nach x Schuss --> Munitionanzeige
  - [ ] ~~Ton bei Schuss & Treffer~~
  - [x] **Pre-game lobby**, wo bei schüssen spielername angezeigt wird um spieler den Sensortags (Waffen) zuzuordnen
- [ ] HitReg
  - [x] ein ganz schwarzer Frame
  - [x] dann **binärsuche** alle Objekte flashen
  - [ ] **<200ms** detection
  - [x] Delay zwischen Schüssen von **1 sec** ~~--> LED rot~~
  - [ ] ~~während eines HitReg werden andere Schüsse durch selbe Frequenz registriert~~
  - [x] wenn keine HitReg möglich dann einfach kein Treffer (halt das Teil still!)
  - [ ] ~~wenn 2 dasselbe treffen bekommen beide Punkte (jeder die Hälfte)~~
- [ ] Waffe
  - [x] **3D gedruckte** Halterung für sensortag (länge, dicke, etc testen)
  - [x] Lauf vor lichtsensor
  - [ ] ~~trigger durch **eigenen Knopf --> anlöten**~~
  - [ ] ~~**LED** nach außen sichbar machen --> Glasfaser?~~
  - [x] Halterung für board --> klicken, schrauben, pins?
- [x] Sensorboard
  - GCU (Gun control Unit)
  - [x] Nachlade-Gyro-Flick Erkennung
  - [x] nach reload, Nachladenachricht senden (siehe Interfaces)
  - LED
    - [ ] ~~rot leuchten bei 1sec schussdelay~~
    - [ ] ~~blinken wenn magazin leer~~
  - [x] schickt brightness bei änderung mit timestamp (siehe Interfaces)
  - [x] generiert irgendwie ID
  - [x] schickt Nachricht bei Schuss (siehe Interfaces)
- [x] Pregame lobby
  - zeigt übrige schüsse aller spieler an
  - spiel geht los wenn alle volles mag haben
  - einmal schießen um lobby zu  betreten
- [x] launchpad
  - empfängt alle GCU daten (siehe Interfaces)
  - schickt daten per UART an Moorhuhn Control Unit (MCU) (PC) (siehe Interfaces)

### Interfaces
 - #### Sensortag $\overset{\text{RF}}\rightarrow$ Launchpad
    Frequenz undso sind wahrscheinlich egal, sollten halt beim Sensortag und Launchpad gleich sein. Kommunikation unterstützt drei verschiedene Nachrichten die über diesen Low-Freq RF shit versendet werden:
    - Schuss

      Wird immer dann gesendet, wenn ein Button am Sensortag gedrückt wird. Enthält die ID vom Sensortag, einen Timestamp, die Restmunition im Magazin und die Magazingröße
    - Helligkeitsänderung

      Wird immer dann gesendet, wenn das Sensortag einen Helligkeitswert misst, der sich vom vorherigen unterscheidet. Enthält die ID vom Sensortag, einen Timestamp und den neuen Helligkeitswert
    - Reload

      Wird immer dann gesendet, wenn das Sensortag die Reload-Geste erkannt hat und teilt der Desktop-Anwendung mit, dass die Munition wieder voll ist. Enthält ID vom Sensortag, Timestamp, Restmunition im Magazin und Magazingröße. Weil das hier ein Reload ist, sollte hier Restmunition == Magazingröße gelten.

- ####  Launchpad $\overset{\text{Serial}}\rightarrow$ PC
  Launchpad empfängt die Nachrichten vom Sensortag und leitet sie einfach 1:1 an den PC weiter.

- #### PC-Serial $\overset{\text{?}}\rightarrow$ PC-GUI
  PC-Serial ist obviously Teil von PC und parst die Nachrichten, die es als row Bytes empfängt in ein Rust-Enum. Nachrichten Schuss und Reload werden an PC-GUI weitergeleitet und Nachrichten vom Typ Helligkeitsänderungen gehen an PC-Hitreg

- #### PC-GUI $\overset{\text{?}}\leftrightarrow$ PC-Hitreg
  PC-GUI leitet Informationen über Flash-Sequence (welche Informationen genau?) and PC-Hitreg weiter. PC-Hitreg leitet nach Verarbeitung an PC-GUI weiter, wer, was getroffen hat.
