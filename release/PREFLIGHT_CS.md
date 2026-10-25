# Ověření publikačního balíčku — 25. října 2026

[English](PREFLIGHT.md) · **Čeština**

Autor: **Vladimír Bíža**. Původní rovnice, důkazy, rukopisy a PDF článků zůstávají beze změny.

## Výpočty a testy

- `python3 src/python/verify.py`: PASS. K16 má 84 nulových hran z 315 kandidátů, K32 má 1260 hran a K64 má 13020 hran. Ověřen byl podepsaný svědek v obou pořadích, oba kontexty asociátoru, čtyřnásobné dědění v obou pořadích a přesné zlomky vyšších π.
- `python3 src/python/benchmark.py`: PASS. Explicitní a faktorizovaný operátor vrací shodné celočíselné výsledky pro m = 1, 2, 4, 8, 16, 32, 64, 128.
- `cargo test --manifest-path src/rust/Cargo.toml`: PASS, pět testů. Predikát je porovnán se skutečnými podepsanými součiny všech kandidátů K16/K32/K64. Další kontroly zahrnují svědka, dědění, sondy, momenty, neplatné adresy a shodu explicitní/faktorizované aplikace s lokální změnou i bez ní.
- `cargo run --release --manifest-path src/rust/Cargo.toml`: PASS; všech pět sond pro m = 32 má maximální odchylku od explicitní reference 0.
- K2048: `(2048−2)(2048−4)(2048−8)/16 = 533 208 060`, nezávisle vyhodnoceno v Pythonu, Rustu a prohlížeči. Jde o kontrolu vzorce pro definovaný dvoučlenný assessor atlas, nikoli úplnou enumeraci 533 milionů hran nebo všech spojitých nulových dělitelů.

Prostředí: Python 3.14.4, rustc 1.96.0, Cargo 1.96.0, Linux 7.1.3. Jsou to kontroly správnosti, nikoli konkurenční hardwarový benchmark. Moderní srovnání s CSR/GraphBLAS a optimalizovanými implicitními operátory popisuje [benchmarkový kontrakt](../BENCHMARKS_CS.md).

## Offline demo a EN/CZ

Chromium otevřelo `demo/index.html` přímo pomocí `file:`. Při interakcích byla síť vypnuta. Původních 9 interních a 24 interakčních kontrol prošlo; po doplnění EN/CZ prošlo **62 kontrol obou jazyků**, bez chyb JavaScriptu a bez externích síťových zdrojů. [Výsledky EN/CZ](bilingual-demo-checks.json).

1. Orákulum: start K2048, všech pět sond, neplatné shodné adresy, nové adresy, skutečné měření 3000 dotazů a reset χ = 1025.
2. Obnova: přesná při požadovaném výpadku 70 % (10 přeživších) a na hranici 75 % (8 přeživších); správně nemožná při 85 % (5 přeživších). Počty výpadků se zaokrouhlují na celé symboly z 32 uzlů. Používá se běžné erasure coding nad GF(257).
3. Asociátor: `XY = YX = 0`, `[X,e1,Y] = −4e7`, `[X,e8,Y] = 0`.
4. Nové hrany: K16 = 84; K32 = 1260 celkem a 924 nových; K2048 = 533 208 060 celkem.

Přepínač EN/CZ zachovává scénu, vstupy, vybranou sondu i výsledek obnovy. Jazyk je v URL a pamatuje se při opětovném načtení. Ověřeno bylo desktopové zobrazení 1440×1080 i mobilní 390×844 bez vodorovného přetékání. Obě jazykové varianty jsou v jediném offline HTML a kopie v `docs/demo/` je totožná.

Screenshoty: [EN desktop](../assets/demo-desktop-en.png), [CZ desktop](../assets/demo-desktop-cs.png), [EN mobil](../assets/demo-mobile-en.png), [CZ mobil](../assets/demo-mobile-cs.png).

## Dokumenty a veřejné odkazy

Všechny strany čtyř vědeckých PDF a obou šestistránkových LinkedIn dokumentů byly vykresleny a prohlédnuty. Původních 30 souborů rukopisů, formátů a PDF prezentací odpovídalo vstupním SHA-256 součtům. Článek A má EN 7 / CS 8 stran; článek B EN 5 / CS 5 stran. LinkedIn dokumenty mají každý přibližně 1,48 MB a jednotný formát 720×900 bodů. Ve viditelném obsahu je správně uveden Vladimír Bíža; původní PDF metadata prezentací obsahují `anonymous` a nebyla přepisována.

Pages kopie článků odpovídají zdrojům a HTML článků neodkazují na externí zdroje. `docs/.nojekyll` zachovává Markdownové soubory. Veřejná stránka, demo a všech 12 URL formátů článků vracely HTTP 200. Všechny čtyři PDF stažené z release se shodují s originály. Aktuální odkazy a stav jsou v [publikačních výsledcích](../PUBLICATION_RESULTS_CS.md).

## Opravené a zdokumentované nesrovnalosti

- Opravena Rust syntaxe `weight:.75` na `weight:0.75`, která bránila kompilaci.
- Odstraněn starý název/bannery TOE a neautorizované MIT metadata. Balíček je `fractal-zd-computing` v1.0.0, autor je Vladimír Bíža, publikace do registru je vypnuta. Práva zůstávají vyhrazena.
- Rust a prohlížeč před predikátem ověřují platné disjunktní assessory a shodnou XOR barvu podle článku A.
- Opraven reset K2048 na χ = 1025 a první sondu. Zavádějící popisek `0 × 0` nahrazen `XY = 0`; počet hran je vymezen na zvolený atlas.
- Doplněny matematické regresní kontroly a následně EN/CZ celé interaktivní aplikace, včetně popisků plátna, stavů a chybových výsledků.
- Původní český článek B uvádí historické cesty `code/verify.py`, `code/benchmark.py`, `code/src/main.rs` a tehdejší nedostupnost Rustu. Aktuální cesty jsou `src/python/verify.py`, `src/python/benchmark.py` a `src/rust/`; současná kompilace je ověřena. Historické znění PDF i Markdownu zůstalo zachováno.

Kontroly neprokazují recenzní status, obecnou rychlostní převahu, šifrování, izolaci nájemců ani konsenzus.
