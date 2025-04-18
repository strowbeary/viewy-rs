# 🚀 Viewy – Guide de publication

Ce document explique comment publier de nouvelles versions de Viewy, en suivant les conventions **SemVer** et en utilisant [`cargo-release`](https://crates.io/crates/cargo-release) pour automatiser le processus.

---

## 🧬 Stratégie de version et de branches

### 📦 Versionnage – SemVer

Viewy utilise **Semantic Versioning** :

```
MAJOR.MINOR.PATCH
```

- **MAJOR** : changements incompatibles
- **MINOR** : nouvelles fonctionnalités, rétrocompatibles
- **PATCH** : corrections de bugs

### 🌿 Branches principales

| Branche         | Rôle                                                |
|-----------------|-----------------------------------------------------|
| `main`          | Développement de la prochaine version majeure       |
| `release/X.Y`   | Maintenance d’une version mineure stable            |
| Tags `vX.Y.Z`   | Chaque version publiée est taggée automatiquement   |

### 📆 Maintenance des versions

- On maintient les **3 ou 4 dernières versions mineures** de la version majeure courante.
- On garde aussi la **dernière version mineure** de la version majeure précédente (LTS ou correctifs critiques).

#### Exemple :

Si la version actuelle est `3.5` :
- Maintenues : `release/3.5`, `3.4`, `3.3`, `3.2`
- Dernière v2 maintenue : `release/2.5`

---

## 🛠️ Préparation d’une release

1. S'assurer que le code est stable et testé :
   ```bash
   cargo test
   ```

2. Vérifier que l’index Git est propre :
   ```bash
   git status
   ```

3. Mettre à jour le changelog si nécessaire (`CHANGELOG.md`)

---

## 🚀 Publier avec cargo-release

### 🔸 1. Choisir la bonne branche

| Type de release | Branche de départ        |
|------------------|--------------------------|
| Patch            | `release/X.Y`            |
| Minor            | `main` → crée `release/X.Y` |
| Major            | `main`                   |

---

### 🔹 2. Lancer la release

#### ➕ PATCH (ex : `3.5.1`)
```bash
git checkout release/3.5
cargo release patch
```

#### 🆕 MINOR (ex : `3.6.0`)
```bash
git checkout main
cargo release minor
```

#### 🔥 MAJOR (ex : `4.0.0`)
```bash
git checkout main
cargo release major
```

---

### 🔁 Que fait `cargo release` ?

Selon la config dans `Release.toml`, il va automatiquement :

- Mettre à jour la version dans `Cargo.toml`
- Committer avec un message `release vX.Y.Z`
- Créer un tag `vX.Y.Z`
- Pusher commit et tag vers `origin`
- (Optionnel) Publier sur [crates.io](https://crates.io)

---

## 🧪 Simulation (dry-run)

Pour tester sans rien publier :
```bash
cargo release minor --dry-run
```

---

## 🧬 Exemple complet : publier `v3.5.0`

```bash
git checkout -b release/3.5 main
cargo release minor
```

Ce qui va :
- créer `v3.5.0`
- pousser `release/3.5`
- taguer `v3.5.0`
- publier le crate

---

## 🧩 Bonnes pratiques

- Toujours tester localement avant de release
- Ne jamais publier depuis une branche non officielle
- Documenter chaque version dans `CHANGELOG.md`

---

## 📂 Branches actives

| Branche        | État       | Remarques                    |
|----------------|------------|------------------------------|
| `release/3.5`  | ✅ Active   | Version courante             |
| `release/3.4`  | ✅ Active   |                              |
| `release/3.3`  | ✅ Active   |                              |
| `release/3.2`  | ✅ Active   |                              |
| `release/2.5`  | ⚠️ LTS      | Dernière v2 maintenue        |

---
