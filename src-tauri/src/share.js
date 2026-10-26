// Le client invité utilise un cookie HttpOnly ; aucun jeton n’est écrit dans localStorage.
// @ligne Prépare une valeur locale utilisée par les étapes suivantes.
const $ = (id) => document.getElementById(id);
// @ligne Prépare une valeur locale utilisée par les étapes suivantes.
const message = (value) => { $("message").textContent = value; };
// @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
async function request(url, options) {
    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
    const response = await fetch(url, options);
    // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
    if (!response.ok) throw new Error(await response.text());
    // @ligne Renvoie le résultat ou la vue correspondant à l’état courant.
    return response.json();
}
// @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
async function refresh() {
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
        // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
        const data = await request("/api/files");
        // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        $("room").hidden = false;
        // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        $("join").hidden = true;
        // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        $("files").replaceChildren();
        // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        for (const file of data.files) {
            // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
            const item = document.createElement("li");
            // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
            const link = document.createElement("a");
            // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            link.href = `/api/files/${encodeURIComponent(file.id)}`;
            // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            link.download = file.name;
            // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            link.textContent = `${file.name} · ${(file.size / 1048576).toFixed(2)} Mio`;
            // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            item.append(link);
            // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
            $("files").append(item);
        }
        // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        $("upload").hidden = data.role !== "contribute";
        // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        message("Salle connectée. Les droits sont contrôlés à chaque requête.");
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (error) { message(error.message); }
}
// @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
$("join").addEventListener("submit", async (event) => {
    // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    event.preventDefault();
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
        // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
        await request("/api/join", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ name: $("name").value, pin: $("pin").value }) });
        // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
        await refresh();
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (error) { message(error.message); }
});
// @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
$("refresh").addEventListener("click", refresh);
// @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
$("upload").addEventListener("submit", async (event) => {
    // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
    event.preventDefault();
    // @ligne Prépare une valeur locale utilisée par les étapes suivantes.
    const file = $("file").files[0];
    // @ligne Vérifie cette condition avant d’autoriser la suite du traitement.
    if (!file || file.size > 64 * 1024 * 1024) return message("Choisissez un fichier de 64 Mio maximum.");
    // @ligne Regroupe cette opération afin que son éventuel échec soit présenté dans le même parcours.
    try {
        // @ligne Fournit cet argument à l’expression de share ; le traitement reste lié à l’état et aux contrôles définis juste au-dessus.
        message("Envoi en cours…");
        // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
        await request("/api/upload", { method: "POST", headers: { "X-File-Name": encodeURIComponent(file.name) }, body: file });
        // @ligne Attend le résultat de cette action avant de poursuivre le parcours utilisateur.
        await refresh();
    // @ligne Transforme l’échec en retour lisible pour l’utilisateur.
    } catch (error) { message(error.message); }
});
