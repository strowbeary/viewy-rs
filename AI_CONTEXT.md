# AI Context - Viewy Interactive Components

Dernière mise à jour: 14 février 2026.

## Objectif implémenté
- Passer d'un modèle manuel à un modèle **composants interactifs hypermédia**.
- Le state UI est transporté en HTML (`_v_component_state`).
- Une seule route gère les messages interactifs (`/interactive-components/event`).
- Côté DX: pouvoir écrire directement `.append_child(CounterComponent { ... })`.

## Architecture actuelle

### 1) Trait `InteractiveComponent`
Fichier: `viewy/lib/src/core/component.rs`
- Contrat:
  - `type Message`
  - `on_message(self, message) -> Self`
  - `render(self) -> Node`
- Helpers Rocket embarqués dans le trait:
  - `into_interactive_host(component_id)`
  - `into_interactive_host_auto()`
- Conversion appendable:
  - `impl<T> From<T> for Node where T: InteractiveComponent`
  - Permet `append_child(MyInteractiveComponent { ... })` directement.

### 2) Messages interactifs
Fichiers:
- `viewy/lib/src/core/component.rs`
- `viewy/codegen/src/lib.rs`

- Trait `InteractiveComponentMessage` (encode/decode transport `hex:<json>`).
- Derive macro: `#[derive(InteractiveComponentMessage)]`.

### 3) Derive macro composant interactif
Fichier: `viewy/codegen/src/lib.rs`
- `#[derive(InteractiveComponent)]` + `#[component(messages = MyMessage)]`.
- Génère un handler enregistré dans `inventory`.
- Le rerender renvoie toujours:
  - hidden `_v_component_state`
  - + vue du composant
- Cela corrige le bug du 2e clic (`_v_component_state` manquant).

### 4) Binding Rocket (caché dans Viewy)
Fichier: `viewy/lib/src/bindings/rocket/component.rs`
- Registre global par `inventory`.
- Constante de route unique:
  - `INTERACTIVE_COMPONENT_EVENT_ROUTE = "/interactive-components/event"`
- Route Rocket interne:
  - `interactive_components_event` (pub(crate))
- Helpers:
  - `interactive_component_content`
  - `interactive_component_host_with_id`
  - `interactive_component_host` (id auto)

### 5) Montage automatique via fairing
Fichier: `viewy/lib/src/bindings/rocket/static_assets.rs`
- Le fairing Viewy monte maintenant aussi la route interactive globale.
- L'app n'a plus besoin de déclarer la route interactive manuellement.

### 6) Action DX
Fichier: `viewy/lib/src/modifiers/actionnable.rs`
- API disponible:
  - `Action::TriggerMessage(MyMessage::...)`
- Le runtime place `data-v-component-msg`.

### 7) Runtime JS
Fichier: `viewy/lib/static/js/src/widgets/interactive_component.js`
- Mode hypermédia uniquement.
- Envoie les champs du host en `application/x-www-form-urlencoded`.
- Remplace le fragment HTML retourné puis réinitialise Viewy.

## Démo site
Fichier: `site/src/interactive_component_poc.rs`
- `CounterComponent` utilise `InteractiveComponent`.
- Les boutons utilisent `Action::TriggerMessage(...)`.
- Le composant est rendu via:
  - `.append_child(CounterComponent { value: 0 })`
- Plus de route interactive locale.

## Ce qui a été retiré
- Route `interactive_components_event` côté `site`.
- `manage(...)` spécifique au PoC pour ce flux.
- Le contexte/port fake du PoC qui n'était plus nécessaire pour la démo minimale.

## État build
- `cargo check`: OK
- Warnings existants: nombreux warnings préexistants hors scope du PoC.

## Prochaine session (BDD + tests réels)
Objectif: valider en conditions réelles avec logique métier/BDD.

1. Réintroduire un contexte métier réel dans l'app (`manage(...)`) avec ports/adapters.
2. Dans `on_message`, déléguer les effets à une couche service/use-case.
3. Créer des messages orientés lecture/écriture BDD (ex: pagination, filtres, tri).
4. Ajouter des tests d'intégration Rocket:
   - POST sur `/interactive-components/event`
   - vérification HTML retour + `_v_component_state`
   - vérification effets BDD.
5. Ajouter une démo avec vrai flux CRUD/pagination sur table SQL.

## Notes de design
- Philosophie retenue: HTTP transporte l'état via HTML (hypermédia).
- Pas d'état UI en mémoire serveur entre requêtes.
- Les effets de bord sont externalisés (BDD/services), pilotés par messages.
