# Fractal Zero-Divisor Computing v1.0.0

[English](RELEASE_NOTES.md) · **Čeština**

Nezávislá matematika a informatika — **Vladimír Bíža**.

Release obsahuje článek A o podepsaném Cayley–Dicksonově informačním lese a vyšších π a článek B o generativních operátorech a výpočtech, oba v angličtině a češtině. Přiložena jsou čtyři původní PDF; Markdown, HTML a TeX jsou v repozitáři.

- Repozitář: https://github.com/fladirm/fractal-zd-computing
- Web: https://fladirm.github.io/fractal-zd-computing/?lang=cs
- České demo: https://fladirm.github.io/fractal-zd-computing/demo/?lang=cs
- English demo: https://fladirm.github.io/fractal-zd-computing/demo/?lang=en
- [Ověření](PREFLIGHT_CS.md)

Reference implementuje podepsané bázové násobení, kvalifikovanou dvoučlennou incidenci, faktorizované zděděné operátory a sondy vyšších vztahů. Definovaný nepodepsaný dvoučlenný atlas K2048 má **533 208 060** hran. Tento počet nezahrnuje všechny spojité nulové dělitele. Čtyřnásobné dědění a samostatný přírůstek nových hran jsou odlišné vrstvy; jediná Kroneckerova inflace reprezentuje jen zděděnou komponentu.

Offline demo nepotřebuje CDN, server ani API klíč. Jeho scény ukazují adresové dotazy, obnovu při dostatečné redundanci, svědka `XY=0` s `[X,e1,Y]=−4e7` a vznik hran při zdvojování.

Při vydání prošlo Python ověření a výukový benchmark, pět Rust testů, devět interních kontrol prohlížeče a 24 offline interakcí. Pozdější EN/CZ aktualizace větve `main` prošla 62 jazykovými a funkčními kontrolami. Původní tag v1.0.0 ani čtyři PDF se nepřepisují; aktuální bilingvní aplikace je na `main` a Pages.

Výsledky neurčují obecnou převahu nad CSR/GraphBLAS nebo optimalizovanými implicitními operátory. Nulový součin není šifrování, izolace ani konsenzus. Obnova vyžaduje dost přeživší informace.

Copyright © 2026 Vladimír Bíža. **Všechna práva vyhrazena.** Není udělena otevřená licence ani neomezené právo dalšího použití.
