# tiny-http-server

[![Build Status](https://travis-ci.org/livioribeiro/rust-tiny-http.svg?branch=master)](https://travis-ci.org/livioribeiro/rust-tiny-http)

Un petit serveur HTTP écrit en Rust.

Ce projet est une expérience commencée pour apprendre le langage Rust. Il s'agit d'un serveur HTTP multithread capable de servir des fichiers statiques et de lister le contenu des répertoires.

## Fonctionnalités

- **Multithread** : Utilise un pool de threads pour gérer plusieurs connexions simultanément.
- **Analyse HTTP** : Analyse les requêtes HTTP/1.1 (méthodes, URL, en-têtes, paramètres de requête).
- **Service de fichiers statiques** : Distribue les fichiers à partir d'un répertoire racine défini.
- **Listing de répertoires** : Option pour afficher le contenu des répertoires sous forme de liste HTML.
- **Support MIME** : Détecte automatiquement le type MIME des fichiers servis.
- **Configurable** : Adresse et port configurables via la ligne de commande.
- **Gestion des erreurs** : Pages d'erreur basiques pour les codes 404 (Non Trouvé) et 500 (Erreur Interne).

## Installation

Assurez-vous d'avoir [Rust](https://www.rust-lang.org/) installé sur votre système.

1. Clonez ce dépôt :

   ```bash
   git clone https://github.com/livioribeiro/rust-tiny-http.git
   cd rust-tiny-http
   ```

2. Compilez le projet :
   ```bash
   cargo build
   ```

## Utilisation

Pour démarrer le serveur avec les paramètres par défaut (écoute sur `127.0.0.1:9000`) :

```bash
cargo run
```

### Options de ligne de commande

Vous pouvez personnaliser le comportement du serveur avec les options suivantes :

- `-a, --addr <adresse>` : Définit l'adresse et le port sur lesquels le serveur doit écouter (ex: `127.0.0.1:80`).
- `-d, --dir` : Active le listing des répertoires au sein de la racine du serveur.

**Exemple :**

```bash
cargo run -- --addr=127.0.0.1:8080 --dir
```

### Configuration de la racine du serveur

Actuellement, la racine du serveur est définie par défaut sur le répertoire personnel de l'utilisateur (`home_dir`). Pour changer cela, vous devez modifier manuellement le fichier `src/main.rs` :

```rust
// Modifiez ici pour changer la racine du serveur
let path = env::home_dir().unwrap();
```

## Dépendances principales

- `threadpool` : Pour la gestion des threads.
- `conduit-mime-types` : Pour la gestion des types MIME.
- `argparse` : Pour l'analyse des arguments de la ligne de commande.
- `regex` : Pour l'analyse des requêtes.

## Licence

Ce projet est disponible sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.
