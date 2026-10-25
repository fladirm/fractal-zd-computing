# Výsledky publikace

[English](PUBLICATION_RESULTS.md) · **Čeština**

Autor: **Vladimír Bíža**. GitHub, release a Pages byly zveřejněny a ověřeny 25. října 2026. Autor následně oznámil ruční zveřejnění na LinkedInu a požádal, aby tam již neprobíhaly žádné změny. Přímý odkaz na příspěvek nebyl dodán ani nezávisle ověřen.

| Výstup | Stav | Skutečný odkaz |
|---|---|---|
| GitHub účet | Ověřen: fladirm | https://github.com/fladirm |
| Veřejný repozitář | Zveřejněn | https://github.com/fladirm/fractal-zd-computing |
| Release v1.0.0 | Zveřejněn, čtyři původní PDF | https://github.com/fladirm/fractal-zd-computing/releases/tag/v1.0.0 |
| Web | Běží přes HTTPS | https://fladirm.github.io/fractal-zd-computing/ |
| Demo | Běží přes HTTPS | https://fladirm.github.io/fractal-zd-computing/demo/ |
| Anglický LinkedIn příspěvek | Autor hlásí zveřejnění; nezávisle neověřeno | Permalink nebyl dodán |
| Česká navazující varianta | Návrh, Codex ji nezveřejnil ani nenaplánoval | https://github.com/fladirm/fractal-zd-computing/blob/main/linkedin/POST_CS_READY.txt |

## Ověřené PDF přílohy release

Release vyšel 25. října 2026 v 03:40:12 UTC. Všechny čtyři veřejné soubory byly znovu staženy; HTTP 200 a SHA-256 odpovídaly originálům. [Strojový záznam](release/published-assets.json).

- [Článek A — anglicky](https://github.com/fladirm/fractal-zd-computing/releases/download/v1.0.0/Paper_A_Mathematics_EN.pdf), 86 068 bajtů.
- [Článek B — anglicky](https://github.com/fladirm/fractal-zd-computing/releases/download/v1.0.0/Paper_B_Computing_EN.pdf), 95 420 bajtů.
- [Článek A — česky](https://github.com/fladirm/fractal-zd-computing/releases/download/v1.0.0/Paper_A_Mathematics_CS.pdf), 99 109 bajtů.
- [Článek B — česky](https://github.com/fladirm/fractal-zd-computing/releases/download/v1.0.0/Paper_B_Computing_CS.pdf), 106 312 bajtů.

Tag `v1.0.0` ukazuje na `7c901268180e8a20e02ab0a7daa7cb7b1cf0677a`. Původní tag ani PDF se nepřepisují. Nové EN/CZ rozhraní je aktualizací větve `main` a Pages.

## EN/CZ aktualizace

- [English demo](https://fladirm.github.io/fractal-zd-computing/demo/?lang=en) · [České demo](https://fladirm.github.io/fractal-zd-computing/demo/?lang=cs).
- [English web](https://fladirm.github.io/fractal-zd-computing/?lang=en) · [Český web](https://fladirm.github.io/fractal-zd-computing/?lang=cs).
- [Jazykový rozcestník dokumentace a screenshotů](LANGUAGES.md).

Demo i web mají přepínač **EN / CZ**. Přeložené jsou čtyři scénáře, ovládací prvky, stavy, chyby, vysvětlení i popisky plátna. Přepnutí zachovává scénu, vstupy, vybranou sondu a výsledek obnovy. Offline HTML obsahuje oba jazyky a nevyžaduje CDN, server ani API klíč.

Prošlo **62 offline jazykových a funkčních kontrol**, bez chyb JavaScriptu a bez externí sítě. Ověřena je i mobilní šířka 390 px, zachování jazyka a přenos jeho volby z webu do dema. [Výsledky](release/bilingual-demo-checks.json).

Ověření nasazení EN/CZ: **ČEKÁ NA NASAZENÍ**.

## Testy a stav repozitáře

Python orákulum i výukový benchmark prošly. Rust má **5/5 úspěšných testů**; optimalizovaná a explicitní implementace pěti sond dávaly shodné výsledky. [CI původního vydání](https://github.com/fladirm/fractal-zd-computing/actions/runs/40159041343) a [jeho Pages nasazení](https://github.com/fladirm/fractal-zd-computing/actions/runs/40159041071) skončily úspěšně. [Aktuální historie CI](https://github.com/fladirm/fractal-zd-computing/actions/workflows/verify.yml).

Pages používá `main /docs`, veřejné zobrazení a vynucené HTTPS. Repozitář je veřejný, vzdálený `origin` je `git@github.com:fladirm/fractal-zd-computing.git`. Větev `main` sleduje `origin/main`.

K2048 má podle vyhodnoceného vzorce 533 208 060 hran definovaného dvoučlenného atlasu. Nejde o enumeraci všech spojitých nulových dělitelů. Obnova potřebuje osm nezávislých přeživších symbolů; při 70% požadovaném výpadku jich zbývá deset, při 85% jen pět a obnova správně selže.

Všechny původní rukopisy, rovnice, důkazy, autorství i PDF jsou zachovány. Opravy kompilace, metadat, vstupního predikátu a resetu i historické cesty v českém článku B jsou popsány v [předpublikační kontrole](release/PREFLIGHT_CS.md). Kontrolní součty publikačních souborů jsou v `SHA256SUMS.txt`. Přihlašovací údaje a soukromé záznamy prohlížeče se nepublikují.

Copyright © 2026 Vladimír Bíža. **Všechna práva vyhrazena; otevřená licence nebyla udělena.**

## LinkedIn

Potvrzený cílový profil: https://www.linkedin.com/in/vladim%C3%ADr-b%C3%AD%C5%BEa-84bb11102/

Codex příspěvek nezveřejnil. Autor oznámil, že jej zveřejnil ručně, a výslovně si přeje ponechat LinkedIn beze změn. Žádné další přístupy, úpravy nebo pokusy o zveřejnění nejsou součástí této aktualizace. Bez skutečného permalinku se zde žádná adresa příspěvku nedoplňuje.

Připravené anglické a české texty, PDF a ruční návody zůstávají v adresáři `linkedin/` jako podklady. Česká varianta není službou Codex zveřejněna ani naplánována.
