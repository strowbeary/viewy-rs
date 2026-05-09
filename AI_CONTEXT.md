# AI Context - Viewy Interactive Components

Dernière mise à jour: 14 février 2026.

## État actuel

### Modèle interactif
- Architecture hypermédia: le state est transporté en HTML.
- Attributs runtime posés sur la **racine du composant** (`data-v-component-*`).
- Le state est stocké en `data-v-component-state`.
- Le payload POST continue d'envoyer `_v_component_state` (copié depuis l'attribut).
- Une route unique gère les messages: `/interactive-components/event`.

### Route unique cachée dans Viewy
Fichiers:
- `viewy/lib/src/bindings/rocket/component.rs`
- `viewy/lib/src/bindings/rocket/static_assets.rs`

- La route Rocket interactive est interne à Viewy.
- Elle est montée automatiquement par `viewy_static_assets_fairing()`.
- L'app n'a plus besoin de définir `interactive_components_event`.

### Trait et macros
Fichiers:
- `viewy/lib/src/core/component.rs`
- `viewy/codegen/src/lib.rs`

- `InteractiveComponent`:
  - `on_message(self, message) -> Self`
  - `on_message_with_services(self, message, services)` (hook Rocket)
  - `render(self) -> Node`
- `InteractiveComponentMessage` pour les messages transportables.
- Derive macros:
  - `#[derive(InteractiveComponent)]`
  - `#[derive(InteractiveComponentMessage)]`

### DX
- API message côté widget: `Action::TriggerMessage(...)`.
- `InteractiveComponent` est appendable directement:
  - `.append_child(CounterComponent { ... })`

### Runtime JS
Fichier: `viewy/lib/static/js/src/widgets/interactive_component.js`
- Collecte des champs du composant racine.
- POST en `application/x-www-form-urlencoded`.
- Remplacement du **nœud racine complet** à chaque interaction.

### Services externes injectables
Fichier: `viewy/lib/src/bindings/rocket/component.rs`
- Registre `InteractiveServices` (clé string -> service typé via `Any`).
- API:
  - `InteractiveServices::new()`
  - `insert(key, service)`
  - `get::<T>(key)`
- La route unique lit ce registre via `Option<&State<InteractiveServices>>`.
- Le derive appelle `on_message_with_services(...)` automatiquement.

## Démo site actuelle
Fichiers:
- `site/src/interactive_component_poc.rs`
- `site/src/main.rs`

- `CounterComponent` interactif appendé directement.
- Démo d'un service externe injecté:
  - `CounterPolicyPort`
  - `demo_interactive_services()`
  - `.manage(interactive_component_poc::demo_interactive_services())`
- Message `IncrementByPolicy` lit la valeur depuis ce service.

## État build
- `cargo check`: OK
- Warnings existants: préexistants hors scope.

## Reprise prochaine session (BDD réelle)
1. Remplacer `StaticCounterPolicy` par un adapter BDD réel.
2. Injecter ports/adapters métier dans `InteractiveServices`.
3. Brancher des messages orientés BDD (pagination, filtres, tri, refresh).
4. Ajouter tests d'intégration Rocket:
   - POST `/interactive-components/event`
   - vérification root HTML + `data-v-component-state`
   - vérification effets BDD.
