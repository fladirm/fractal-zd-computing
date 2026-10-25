# Fraktální výpočetní algebra dělitelů nuly

**[English](README.md) · Čeština**

**Adresovatelný podepsaný informační les odvozený z Cayley–Dicksonovy věže.**
**Autor: Vladimír Bíža · Nezávislá matematika a informatika · 2026**

Algebra z adres vypočítá, které vazby existují, které jsou zděděné, které vznikly nově a jaké informace pouhá binární hrana neuchovává.

## Vyzkoušej demo

**[Živé demo česky](https://fladirm.github.io/fractal-zd-computing/demo/?lang=cs)** · [English demo](https://fladirm.github.io/fractal-zd-computing/demo/?lang=en) · [Český web](https://fladirm.github.io/fractal-zd-computing/?lang=cs) · [Release v1.0.0: čtyři PDF](https://github.com/fladirm/fractal-zd-computing/releases/tag/v1.0.0)

**[Offline HYPERION-ZD](demo/index.html)** · [Český screenshot](assets/demo-desktop-cs.png) · [English screenshot](assets/demo-desktop-en.png)

Soubor `demo/index.html` otevři dvojklikem v moderním prohlížeči. Přepínač **EN / CZ** je přímo v horní liště a funguje také bez připojení. Oba jazyky i matematický motor jsou obsažené v jediném HTML. Není potřeba Node, npm, backend, účet, server, síť ani GPU. Přepnutí jazyka zachovává scénu, vstupy a výsledek obnovy.

Čtyři interaktivní scénáře:

1. **Adresové orákulum:** zvol dvě dvojice bázových adres a ověř přesnou podepsanou incidenci nulového součinu.
2. **Výpadek a obnova:** otestuj rekonstrukci při dostatečném počtu přeživších nezávislých pozorování i její selhání při nedostatku informace.
3. **Asociátor:** nulový párový součin může mít nenulový asociátor po přidání třetího kontextu.
4. **Vznik nových vazeb:** sleduj čtyřnásobné dědění starých hran a samostatný přírůstek nově vzniklých hran při zdvojování dimenze.

## Články

| Článek | Čeština | English |
|---|---|---|
| **A — Matematika:** ZD predikát, kořeny K16, four-lift, zákon nových hran, vyšší π a normové momenty | [PDF](papers/cs/Paper_A_Mathematics_CS.pdf) · [Markdown](papers/cs/Paper_A_Mathematics_CS.md) · [HTML](papers/cs/Paper_A_Mathematics_CS.html) | [PDF](papers/en/Paper_A_Mathematics_EN.pdf) · [Markdown](papers/en/Paper_A_Mathematics_EN.md) · [HTML](papers/en/Paper_A_Mathematics_EN.html) |
| **B — Výpočty:** implicitní operátory, GPU, persistence, podepsané sondy a testování | [PDF](papers/cs/Paper_B_Computing_CS.pdf) · [Markdown](papers/cs/Paper_B_Computing_CS.md) · [HTML](papers/cs/Paper_B_Computing_CS.html) | [PDF](papers/en/Paper_B_Computing_EN.pdf) · [Markdown](papers/en/Paper_B_Computing_EN.md) · [HTML](papers/en/Paper_B_Computing_EN.html) |

Anglická technická vydání zachovávají matematiku a číslované výrazy českého zdroje. Nejde o ověřené doslovné překlady; společným závazkem je matematická reprodukovatelnost. Původní rukopisy, důkazy i PDF zůstávají beze změny.

## Jádro ve třech bodech

1. `epsilon(k,a,b)` vypočítá znaménko součinu `e_a * e_b` bitovou rekurzí bez tabulky N×N.
2. Čtyřadresový podepsaný predikát klasifikuje vybranou dvoučlennou nulovou vazbu. Úplný klíč obsahuje obě dvojice assessorů, znaménka a orientaci.
3. Staré vazby se dědí čtyřnásobně, zatímco nové vazby vznikají samostatně. Zděděná komponenta splňuje `A_t = A_0 ⊗ J_(2^t)`; celý les tuto jedinou faktorizaci nemá.

Pro definovaný **nepodepsaný dvoučlenný assessor atlas** vychází v `K2048` přesně **533 208 060 nulových hran**. Je to kombinatorický počet tohoto atlasu; ne počet všech spojitých nulových dělitelů ani fyzicky vytvořených serverů, záznamů nebo uzlů.

Svědek v `K16` splňuje `x*y=0`, ale `[x,e₁,y] = −4e₇`. Samotná booleovská párová incidence tuto informaci o třetím kontextu neobsahuje.

## Reprodukce

```bash
python3 src/python/verify.py
python3 src/python/benchmark.py
cd src/rust && cargo test && cargo run --release
```

Python používá pouze standardní knihovnu. Rust reference se kompiluje místním toolchainem a kontroluje ji i GitHub CI. Výukový benchmark porovnává explicitní a faktorizovanou aplikaci na totožné strukturované úloze. Neprokazuje obecnou převahu nad optimalizovanými tensorovými, GraphBLAS nebo cuSPARSE implementacemi.

### Výpočetní rozsah

Přínosem je podepsané algebraické generování, rozlišené vyšší vztahy a genealogický vznik nových hran. Úspora paměti pro incidenci neodstraňuje potřebu ukládat aplikační data. Nulový součin není šifrování ani hardwarová izolace nájemců. Schurův doplněk může rekonstruovat chybějící část jen při dostatku nezávislé přeživší měřicí informace a pravých stran. Požadavky distribuovaného konsenzu zůstávají samostatné.

## Publikace, citace a další použití

- [Jazykový rozcestník EN/CZ](LANGUAGES.md)
- [Publikační postup](PUBLISHING_CS.md)
- [Benchmarky a jejich omezení](BENCHMARKS_CS.md)
- [Ověření před publikací](release/PREFLIGHT_CS.md)
- [Výsledky publikace](PUBLICATION_RESULTS_CS.md)
- [Ruční zveřejnění na LinkedInu](linkedin/MANUAL_PUBLISHING_CS.md)
- [Reprodukovatelný kód](src/) a [citační metadata](CITATION.cff)
- [Příspěvky a PDF dokumenty pro LinkedIn](linkedin/)

Copyright © 2026 Vladimír Bíža. **Všechna práva vyhrazena, dokud autor neudělí samostatnou licenci.** Veřejný GitHub sám o sobě neuděluje patentovou licenci ani licenci ke komerčnímu použití nebo šíření kódu.
