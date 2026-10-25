# Český LinkedIn příspěvek

**533 208 060 algebraických vazeb. Bez tabulky s 533 miliony hran.**

Pracuji na neobvyklé výpočetní struktuře odvozené z Cayley–Dicksonových algeber: na podepsaném, rekurzivně adresovatelném stromu vztahů mezi děliteli nuly.

Nezajímá mě jen komprese grafu. Tu dnes umějí také dobře navržené implicitní operátory. Zde ale z jednoho algebraického pravidla získávám **exaktní rozhodování vazeb ze čtyř adres**, jejich **čtyřnásobnou genealogickou dědičnost**, **vznik nových vztahů při každém zdvojení** a vyšší informace, které jednoduchý graf „hrana/nehrana“ neumí vyjádřit.

Konkrétní příklad: dva nenulové výrazy se přímo vynásobí na nulu, ale po vložení třetího členu vyjde **nenulový asociátor**. Párový graf tuto informaci neobsahuje.

Připravil jsem dva samostatné matematicko-výpočetní papery, Rust/Python reference a offline HTML demo na dvojklik. Můžeš si v něm sám měnit vstupy, způsobit výpadek, spustit rekonstrukci nebo sledovat, jak v další vrstvě vznikají nové vztahy.

Publikace a zdrojový kód: {REPO_URL}  
Živé demo: {DEMO_URL}

Uvítám diskusi s lidmi, kteří řeší řídkou lineární algebru, GPU, hypergrafy, databáze a generativní datové struktury — zejména o férovém benchmarku proti dnešním strukturovaným operátorům.

#Rust #Algoritmy #Matematika #GPU #SoftwareArchitecture
