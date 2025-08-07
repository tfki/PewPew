```launchpad.c``` &rarr; ```rfPacketRx.c``` <br>
```sensortag.c``` &rarr; ```rfPacketTx.c```

# PewPew (IOT LAB Final) 

##### Sebb's special place (for special people)
- ```cargo run --bin ur_sandbox```
- ```cargo run``` if src/main.rs exists (with main func (```pub fn main(){}```))

## DOD (Definition of Done)
### Modus 1
- [ ] auf Bildschirm schießen mit HitReg durch BinSearch
### Modus 2
- [ ] Multiplayer Mohrhuhn
  - [ ] Multiplayer auf gleichem Bildschirm
  - [ ] custom, präzise Hitboxen
  - [ ] statisches Bild (nicht rechts/links verschieben)
  - [ ] Figuren
    - [ ] fliegende Hühner
    - [ ] bienendude
    - [ ] rennende Hühner
    - [ ] Pfannendude
  - [ ] Jogger verschwinden hinter mind. einem Obj
  - [ ] Punktestand für Spieler und Timer
  - [ ] Scoreboard vom aktuellen Spiel
  - [ ] **Reload per Gyro Flick** nach x Schuss --> Munitionanzeige
  - [ ] Ton bei Schuss & Treffer
  - [ ] **Pre-game lobby**, wo bei schüssen spielername angezeigt wird um spieler den Sensortags (Waffen) zuzuordnen
- [ ] HitReg
  - [ ] ein ganz schwarzer Frame
  - [ ] dann **binärsuche** alle Objekte flashen
  - [ ] **<200ms** detection
  - [ ] Delay zwischen Schüssen von **1 sec** --> LED rot
  - [ ] während eines HitReg werden andere Schüsse durch selbe Frequenz registriert
  - [ ] wenn keine HitReg möglich dann einfach kein Treffer (halt das Teil still!)
  - [ ] wenn 2 dasselbe treffen bekommen beide Punkte (jeder die Hälfte)
- [ ] Waffe
  - [ ] **3D gedruckte** Halterung für sensortag (länge, dicke, etc testen)
  - [ ] Lauf vor lichtsensor
  - [ ] trigger durch **eigenen Knopf --> anlöten**
  - [ ] **LED** nach außen sichbar machen --> Glasfaser?
  - [ ] Halterung für board --> klicken, schrauben, pins?
- [ ] Sensorboard
  - GCU (Gun control Unit)
  - [ ] Nachlade-Gyro-Flick Erkennung
  - [ ] nach reload, Nachladenachricht senden (siehe Interfaces)
  - LED
    - [ ] rot leuchten bei 1sec schussdelay
    - [ ] blinken wenn magazin leer
  - [ ] schickt brightness bei änderung mit timestamp (siehe Interfaces)
  - [ ] generiert irgendwie ID
  - [ ] schickt Nachricht bei Schuss (siehe Interfaces)
- [ ] Pregame lobby
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
