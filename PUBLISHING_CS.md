# Publikační postup

[English](PUBLISHING.md) · **Čeština**

1. Spusť `python3 src/python/verify.py`, `python3 src/python/benchmark.py` a `cargo test --manifest-path src/rust/Cargo.toml`, je-li dostupný Cargo.
2. Prohlédni čtyři PDF v `papers/{cs,en}`. Otevři `demo/index.html` přímo v prohlížeči bez serveru, ověř všechny čtyři scény v EN i CZ a náhled `docs/`.
3. Použij veřejný repozitář `fractal-zd-computing`. Commitni zdroje, dokumenty, demo a záznam ověření. Release `v1.0.0` obsahuje čtyři původní PDF článků.
4. GitHub Pages publikuj z větve `main`, adresáře `/docs`. Ověř HTTPS a skutečně vyzkoušej obě jazykové varianty dema. Jazyk přepíná EN/CZ nebo parametr `?lang=en` / `?lang=cs`.
5. Anglický příspěvek zveřejni pod potvrzeným osobním profilem z `linkedin/PROFILE_TARGET.md`. Přilož jediný dokument `linkedin/LINKEDIN_CAROUSEL_EN.pdf` a vlož skutečné odkazy na repozitář a demo. Aktuálně se autor rozhodl zveřejnit příspěvek ručně: [postup krok za krokem](linkedin/MANUAL_PUBLISHING_CS.md).
6. Český příspěvek s `linkedin/LINKEDIN_CAROUSEL_CS.pdf` ponech pro pozdější navazující publikaci. Nezveřejňuj obě téměř shodné varianty bezprostředně po sobě.
7. Skutečné adresy repozitáře, Pages, release a příspěvků zapisuj do `PUBLICATION_RESULTS.md` a českého přehledu `PUBLICATION_RESULTS_CS.md`. Bez ověřeného veřejného odkazu netvrď, že příspěvek vyšel.

Přihlášení, 2FA, CAPTCHA a potvrzovací dialogy platformy musí proběhnout běžným způsobem. Do repozitáře nepatří hesla, tokeny, cookies ani soukromá data profilu. Autorství Vladimíra Bíži, matematika a důkazy musí zůstat zachovány. Veřejná publikace neuděluje otevřenou licenci.

LinkedIn přijímá jeden PDF dokument do 100 MB a 300 stran; dodané dokumenty mají každý šest stran a přibližně 1,48 MB. Omezení byla ověřena 25. října 2026 v [oficiální nápovědě LinkedInu](https://www.linkedin.com/help/linkedin/answer/a518909).
