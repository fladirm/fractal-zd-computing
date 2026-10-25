# Benchmarky: férové moderní srovnání

[English](BENCHMARKS.md) · **Čeština**

Pro tutéž přesně definovanou úlohu porovnávej alespoň:

- **CSR / GraphBLAS:** sestavení a aplikaci kvalifikované řídké incidence při shodných konvencích a významu dotazu.
- **Strukturovaný / implicitní / Kroneckerův operátor:** uložení A₀ a použití blokových součtů místo materializace zděděné Aₜ.
- **Podepsané fraktální ZD orákulum:** generování ověřených ZD hran, znamének, původu a vyšších sond; tam, kde to odpovídá úloze, aplikaci téhož zděděného operátoru.

Samotná hustá matice nestačí jako výchozí konkurent. Odděluj klasifikaci dvojic, zděděný matvec, enumeraci nových hran, podepsané řídké součiny, asociátory a alternátory, paměť v K16/32/64/2048, persistenci a načtení i obnovu s redundantními pozorováními a bez nich.

Uváděj p50/p99 času, zahřívání, model CPU/GPU, OS, překladač a přepínače sestavení, maximální RSS/VRAM, počty správných výsledků, datasety a hash commitu. Význam porovnávaných operací musí být shodný. Binární incidence nezachycuje asociátor, ale hypergrafy a jiné reprezentace vyšších vztahů jej zachytit mohou. Porovnávej reprezentaci, složitost kódu a ověřené výsledky.

`src/python/benchmark.py` je **malá výuková reference**, nikoli úplný benchmark proti nejlepším dostupným implementacím. Konkurenční benchmarková sada je samostatný výzkumný úkol.
