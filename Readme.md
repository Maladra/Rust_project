## Project RUST

Faire un système de messagerie en Rust, avec un côté client et un serveur.

Rendre les échanges chiffré (méthode libre)

Implémentation d'une messagerie en Rust en suivant les bonnes pratiques.

Le programme doit:

 - Etre lancé et va soit chercher un autre client auquel se connecter soit un serveur

 - Une fois connecté on doit pouvoir voir des messages s'afficher

 - On peut envoyer des messages et l'autre client (ou les autres clients) doivent pouvoir voir ces
messages

 - Il n'est pas obligé de pouvoir voir les anciens messages lorsqu'un client se connecte

 - un systeme de chiffrement est demandé (symétrique ou asymétrique) et le choix du l'algorithme est libre

Il doit y avoir dans le projet:

 - De l'ownership

 - Du borrowing

 - Des collections

 - Des tests

 - De la propagation d'erreur

 - Des structures

 - Des Enums

 - Des threads

Un plus:

 - Présence de Traits

 - Présence de Generics

Les malus:

 - bloc Unsafe

 - trop d'unwrap/expect (remplacer par des unwrap_or/unwrap_or_else ou des match)

 - pas de test

Idées Bonus:

 - Connexion BDD

 - Interface graphique

 - gestion des utilisateur poussé

 - système anti-usurpation d'identité

 - Système de chiffrement avancé



The Rust Programming Language Book: https://doc.rust-lang.org/book/

Learn rust: https://learning-rust.github.io/

Rust by Example: https://doc.rust-lang.org/stable/rust-by-example/

Gentle rust intro: https://stevedonovan.github.io/rust-gentle-intro/

Exercises: https://github.com/rust-lang/rustlings/

The summary of the book: https://tourofrust.com/

Examples of code: http://rosettacode.org/wiki/Category:Rust

Rust Cheatsheet: https://www.cheats.rs/

Minimal Cheatsheet: https://upsuper.github.io/rust-cheatsheet/

Cours de Rust interactifs gratuits: https://www.educative.io/courses/learn-rust-from-scratch

https://www.educative.io/courses/ultimate-guide-to-rust-programming
