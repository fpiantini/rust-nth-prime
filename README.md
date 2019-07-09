# Confronto di algoritmi per il calcolo di numeri primi

In questo progetto viene definito un modulo Rust `prime_numbers` che contiene al suo interno differenti implementazioni di una possibile soluzione per il problema:

_dato un intero `n` determinare l'n-esimo numero primo (con indice  che parte da 0)_

Per esempio se n = 0 l'algoritmo deve ritornare 2 (il primo numero primo), se n = 1 deve ritornare 3, se n = 2 deve ritornare 5 è così via.

Vengono definite due funzioni:

```rust
fn is_prime(val: u32) -> bool
```
che ritorna `true` se `val` è un numero primo o `false` in caso contrario

```rust
fn nth_prime(n: u32) -> u32
```

ritorna l'n-esimo numero primo. La seconda funzione fa uso della prima per determinare se un numero è primo. Nella seconda funzione volutamente si evita di fare uso di uno stato interno (per cercare di ricordarsi i numeri primi calcolati fino a quel momento).

## Prove di performance con differenti implementazioni

Per risolvere il problema si propongono differenti implementazioni.

### Soluzione 1: Programmazione "convenzionale"

Fa uso dei costrutti dei programmi imperativi (più che altro `while`). Questo tipo di soluzione viene indicata con il suffisso `_oldf`:

```rust
fn is_prime_oldf(val: u32) -> bool
fn nth_prime_oldf(n: u32) -> u32
```

Per "stressare questo algoritmo" è stato scritto il test `test_nth_prime_very_big_prime_oldf`. Eseguendo il comando:

```
cargo test test_nth_prime_very_big_prime_oldf
```

E' possibile fare un benchmark o usando semplicemente il comando `time` o usando qualcosa di più sofisticato come ad esempio [hyperfine](https://github.com/sharkdp/hyperfine)).

Sul mio PC (un buon vecchio PC del 2012), questa implementazione ci mette circa **8 secondi** ad eseguire il test:

```
$ hyperfine 'cargo test test_nth_prime_very_big_prime_oldf'
Benchmark #1: cargo test test_nth_prime_very_big_prime_oldf
  Time (mean ± σ):      8.140 s ±  0.022 s    [User: 8.133 s, System: 0.006 s]
  Range (min … max):    8.105 s …  8.183 s    10 runs
```

### Soluzione 2: Primo esempio con iteratore

Una seconda soluzione, riconoscibile dal suffisso `_iter1`, comincia a far uso di tecnologie più moderne basate su iteratori. In particolare questa soluzione fa uso della funzione [`from_fn`](https://doc.rust-lang.org/std/iter/fn.from_fn.html).

Per "stressare" questa soluzione si può eseguire il test `test_nth_prime_very_big_prime_iter1`:

```
cargo test test_nth_prime_very_big_prime_iter1
```

Questo test sul mio PC è molto meno efficiente della Soluzione 1: per completare il test occorrono circa **21 secondi**. Come si vede dal punto di vista della performance questa soluzione è molto peggiore di quella "vecchio stile".

```
$ hyperfine 'cargo test test_nth_prime_very_big_prime_iter1'
Benchmark #1: cargo test test_nth_prime_very_big_prime_iter1
  Time (mean ± σ):     20.837 s ±  0.140 s    [User: 20.812 s, System: 0.020 s]
  Range (min … max):   20.704 s … 21.148 s    10 runs
```

### Soluzione 3: Elegante ma inefficiente (sempre con iteratore)

[Sul sito](https://exercism.io/tracks/rust) dove trovo kata in rust e dove ho trovato questo esercizio, ho visto che la soluzione più popolare tra gli studenti è quella che fa uso del metodo [`any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) applicato a un iteratore.

Questa soluzione che ho chiamato "elegante ma inefficiente" è implementata nei metodi con il suffisso `_ebi`. La performance è catastrofica: per completare il test occorrono circa **227 secondi**.

### Soluzione 4: `try_for_each`

Incuriosito da questa presunta inefficienza delle soluzioni con gli iteratori, mi sono messo a cercare di capire qualcosa di più e mi sono imbattutto in [un articolo](https://medium.com/@veedrac/rust-is-slow-and-i-am-the-cure-32facc0fdcb) (a dire il vero piuttosto datato) che parla dell'inefficienza degli iteratori Rust. Lasciando [all'articolo](https://medium.com/@veedrac/rust-is-slow-and-i-am-the-cure-32facc0fdcb) per gli approfondimenti mi limito a rilevare che sembra che tutti i problemi degli iteratori Rust siano stati risolti tramite i metodi [`try_fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold) e [`try_for_each`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_for_each).

Ho implementato quindi una soluzione basata sull'uso di `try_for_each` che è riconoscibile dal suffisso `_try_for_each`. Le sue performance purtroppo sono simili alla Soluzione 3: il test `test_nth_prime_very_big_prime_try_for_each` viene eseguito in circa *227 secondi*.

Io per il momento mi fermo qua... Tutto questo probabilmente dimostra che io non ho ancora capito come usare al meglio gli iteratori in un linguaggio come Rust. Resto a disposizione per consigli e implementazioni alternative più efficienti... (naturalmente senza cambiare il tipo di algoritmo per il test di primalità: qui il confronto lo stiamo facendo tra funzionalità del linguaggio Rust).




