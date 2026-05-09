# Analyse Comparative : Viewy vs Elm

## Introduction
Ce document présente une analyse comparative entre Viewy, une nouvelle boîte à outils pour UI en Rust, et Elm, un langage reconnu pour ses fondations robustes en matière de développement web. En outre, nous explorerons comment Viewy s’aligne avec les principes hypermédias, ainsi que la philosophie sous-jacente de ce projet.

---

## Présentation des Architectures

### Viewy
**Viewy**, comme décrit dans sa documentation, est une boîte à outils innovante spécialement conçue pour développer des interfaces utilisateur en Rust. 
- Intégration avec Rust : Viewy intègre des macros dérivées et des structures idiomatiques au langage pour encapsuler des composants UI, comme illustré dans cet exemple de code :
```rust
impl Component for MyPage {
    fn render(self) -> Node {
        View::new().append_child({
            Button::new(&self.btn_label, ButtonStyle::Filled)
        })
        .into()
    }
}
```
- Configurations : Les développeurs peuvent ajuster les composants en personnalisant un fichier `Viewy.toml`.

### Elm
Elm, en revanche, se distingue par sa nature fonctionnelle et réactive :
- **Architecture Elm** : Une structure unifiée où tout programme suit les mêmes trois principes : Modèle, Vue et Mise à jour (Model / View / Update).
- **Écosystème Typesafe** : Comme Rust, Elm compile directement en JavaScript tout en garantissant des erreurs d’exécution minimes.
- **Reactif et sans side-effects** : Les mises à jour UI sont gérées de manière fonctionnelle, rendant les données et le rendu UI prévisibles.

---

## Vuey et les Principes Hypermédias
Les principes hypermédias garantissent que les systèmes utilisent des hyperliens pour la gestion dynamique des ressources. Examinons Viewy sous cet angle :
### Concordances
1. **Navigabilité dynamique** : Avec Viewy, les composants riches permettent de structurer les pages sous forme d’arbres DOM complexes mais navigables via des configurations.
2. **Auto-découverte** : Bien que cela ne soit pas explicite, la personnalisation via Viewy.toml offre une modularité exploratoire à ses utilisateurs. 

### Écart
Cependant, Viewy ne semble pas implémenter directement la navigation basée sur des hyperliens comme le ferait un moteur REST/API-driven.

---

## Philosophie Derrière Viewy
- **“Developer-centric” Approach** : L'API intuitive met en avant le développeur.
- **Flexibilité** : Le choix d'un fichier `Viewy.toml` témoigne d'une prise de décision orientée vers le design scalable.
- **Rusty Foundation** : L'association avec Rust confère une fiabilité de bas niveau, en adéquation avec la philosophie "sûreté avant tout".

---

## Conclusion
Viewy, tout en étant encore jeune, privilégie une approche moderne et interconnectée des UI tout en gravitant autour de solides principes de personnalisation et modularité. Comparé à Elm, il présente des distinctions intéressantes mais semble manquer un degré d'alignement sur les véritables principes du web hypermédia. Néanmoins, sa philosophie centré-développeur et sa fondation Rusty lui donnent un avenir prometteur dans l’écosystème outil UI moderne.

---

💡 **Notes** : À mesure que Viewy évolue, étendre son cadriciel avec davantage de guidance hypermédia pourrait compléter sa robustesse intrinsèque.