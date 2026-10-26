// @ligne Importe les dépendances explicites utilisées par cet espace.
import React from "react";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import ReactDOM from "react-dom/client";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import App from "./App";
// @ligne Importe les dépendances explicites utilisées par cet espace.
import "./style.css";

// @ligne Fournit cet argument à l’expression de main ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
ReactDOM.createRoot(document.getElementById("root")!).render(
  // @ligne Compose cet élément de la vue main à partir de l’état courant.
  <React.StrictMode>
    {/* @ligne Compose cet élément de la vue main à partir de l’état courant. */}
    <App />
  {/* @ligne Termine ce groupe visuel en conservant la hiérarchie de la vue. */}
  </React.StrictMode>,
);
