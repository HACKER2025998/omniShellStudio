// Les observations locales et la collecte WAN facultative sont deux actions explicites distinctes.
import { useEffect, useMemo, useState } from "react";
// @ligne Importe les outils nécessaires à la mise en œuvre des observations réseau et du choix des sondes.
import { open } from "@tauri-apps/plugin-dialog";
// @ligne Importe les outils nécessaires à la mise en œuvre des observations réseau et du choix des sondes.
import { api, download } from "../api";
// @ligne Importe les outils nécessaires à la mise en œuvre des observations réseau et du choix des sondes.
import { GraphCanvas, type GraphSpec } from "./GraphCanvas";
// Les types reflètent les informations effectivement présentes, sans inventer un nom ou une MAC.
export function NetworkWorkspace({
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
  initial,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
  onUpdate,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
  interfaces,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
  onError,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
}: {
  // @ligne Renseigne le champ « initial » de la structure utilisée ici des observations réseau et du choix des sondes.
  initial: any;
  // @ligne Renseigne le champ « onUpdate » de la structure utilisée ici des observations réseau et du choix des sondes.
  onUpdate: (g: any) => void;
  // @ligne Renseigne le champ « interfaces » de la structure utilisée ici des observations réseau et du choix des sondes.
  interfaces: any[];
  // @ligne Renseigne le champ « onError » de la structure utilisée ici des observations réseau et du choix des sondes.
  onError: (s: string) => void;
// @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
}) {
  // @ligne Prépare les valeurs locales pour la prochaine étape des observations réseau et du choix des sondes.
  const [cidr, setCidr] = useState("192.168.1.0/24"),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    [iface, setIface] = useState(""),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    [raw, setRaw] = useState(false),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    [community, setCommunity] = useState(""),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    [busy, setBusy] = useState(false),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    [wan, setWan] = useState<any>(null),
    [selected, setSelected] = useState<any>(null); // Les secrets SNMP ne sont jamais stockés dans les préférences.
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
  useEffect(() => {
    // @ligne Prépare « first » pour la prochaine étape des observations réseau et du choix des sondes.
    const first = interfaces.find((i) => /^\d+\./.test(i.ip));
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des observations réseau et du choix des sondes.
    if (first) {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      setIface(first.name);
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      setCidr(first.ip.split(".").slice(0, 3).join(".") + ".0/24");
    // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
    }
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    api<any[]>("wan_cache")
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      .then((cache) => {
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des observations réseau et du choix des sondes.
        if (cache[0]) setWan(cache[0].data);
      // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
      })
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      .catch(() => {});
  }, [interfaces]); // Le cache WAN n'effectue aucune requête Internet.
  // @ligne Prépare « spec » pour la prochaine étape des observations réseau et du choix des sondes.
  const spec = useMemo<GraphSpec>(
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    () => ({
      // @ligne Renseigne le champ « nodes » de la structure utilisée ici des observations réseau et du choix des sondes.
      nodes: (initial?.nodes || []).map((n: any) => ({
        // @ligne Renseigne le champ « id » de la structure utilisée ici des observations réseau et du choix des sondes.
        id: n.id,
        // @ligne Renseigne le champ « title » de la structure utilisée ici des observations réseau et du choix des sondes.
        title: n.name || n.ip || n.mac || n.id,
        // @ligne Renseigne le champ « kind » de la structure utilisée ici des observations réseau et du choix des sondes.
        kind: n.kind,
        // @ligne Renseigne le champ « lines » de la structure utilisée ici des observations réseau et du choix des sondes.
        lines: [
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          n.kind,
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          n.ip,
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          n.mac,
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          n.vendor,
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          "Ports : " + (n.ports || []).join(", "),
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          n.evidence,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        ].filter(Boolean),
      // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
      })),
      // @ligne Renseigne le champ « edges » de la structure utilisée ici des observations réseau et du choix des sondes.
      edges: (initial?.edges || []).map((e: any) => ({
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        ...e,
        // @ligne Renseigne le champ « logical » de la structure utilisée ici des observations réseau et du choix des sondes.
        logical: e.kind !== "physical",
        // @ligne Renseigne le champ « label » de la structure utilisée ici des observations réseau et du choix des sondes.
        label: e.kind === "logical" ? "relation logique" : e.evidence,
      // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
      })),
    // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
    }),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    [initial],
  ); // Les preuves restent accessibles dans l'inspecteur.
  // @ligne Prépare « scan » pour la prochaine étape des observations réseau et du choix des sondes.
  const scan = async () => {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    setBusy(true);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    try {
      // @ligne Prépare « graph » pour la prochaine étape des observations réseau et du choix des sondes.
      const graph = await api("net_scan", {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        cidr,
        // @ligne Renseigne le champ « interface » de la structure utilisée ici des observations réseau et du choix des sondes.
        interface: iface,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        raw,
        // @ligne Renseigne le champ « mdns » de la structure utilisée ici des observations réseau et du choix des sondes.
        mdns: true,
        // @ligne Renseigne le champ « snmp_community » de la structure utilisée ici des observations réseau et du choix des sondes.
        snmp_community: community,
      // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
      });
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      onUpdate(graph);
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      setSelected(null);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    } catch (error) {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      onError(String(error));
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    } finally {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      setBusy(false);
    // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
    }
  }; // Les erreurs de raw sockets sont gérées comme replis par Rust.
  // @ligne Prépare « copy » pour la prochaine étape des observations réseau et du choix des sondes.
  const copy = async (command: string) => {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    try {
      // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des observations réseau et du choix des sondes.
      await navigator.clipboard.writeText(command);
      // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des observations réseau et du choix des sondes.
      await api("capture", { raw_command: command, shell: "generated" });
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
    } catch (error) {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      onError(String(error));
    // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
    }
  }; // Les actions de cartographie rejoignent le coffre sans être exécutées.
  // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel des observations réseau et du choix des sondes.
  return (
    // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
    <>
      {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
      <div className="toolbar">
        {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
        <input
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          aria-label="Sous-réseau privé à scanner"
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          value={cidr}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          onChange={(e) => setCidr(e.target.value)}
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        />
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button className="primary" disabled={busy} onClick={scan}>
          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
          {busy ? "Observation en cours…" : "Observer ce /24"}
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          disabled={!initial}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          onClick={() =>
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            download("omnishell-reseau.json", JSON.stringify(initial, null, 2))
          // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
          }
        // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
        >
          JSON
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button disabled={!initial} onClick={() => window.print()}>
          Imprimer / PDF
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
      {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
      </div>
      {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
      <details className="panel">
        {/* @ligne Construit ou ferme l’élément « summaryOptions » de l’interface ; son contenu reste géré par React. */}
        <summary>Options de découverte</summary>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          onClick={async () => {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            try {
              // @ligne Prépare « path » pour la prochaine étape des observations réseau et du choix des sondes.
              const path = await open({
                // @ligne Renseigne le champ « multiple » de la structure utilisée ici des observations réseau et du choix des sondes.
                multiple: false,
                // @ligne Renseigne le champ « filters » de la structure utilisée ici des observations réseau et du choix des sondes.
                filters: [{ name: "IEEE OUI CSV", extensions: ["csv"] }],
              // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
              });
              // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : des observations réseau et du choix des sondes.
              if (typeof path === "string") {
                // @ligne Prépare « result » pour la prochaine étape des observations réseau et du choix des sondes.
                const result = await api("net_oui_import", { path });
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
                window.alert(
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
                  `${result.vendors} préfixes constructeur importés pour les prochains scans.`,
                // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
                );
              // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
              }
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            } catch (error) {
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
              onError(String(error));
            // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
            }
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          }}
        // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
        >
          Importer un fichier OUI IEEE
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        <label>
          Interface
          {/* @ligne Construit ou ferme l’élément « select » de l’interface ; son contenu reste géré par React. */}
          <select value={iface} onChange={(e) => setIface(e.target.value)}>
            {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
            {interfaces.map((i) => (
              // @ligne Construit ou ferme l’élément « option » de l’interface ; son contenu reste géré par React.
              <option key={i.name + i.ip} value={i.name}>
                {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
                {i.name} · {i.ip}
              {/* @ligne Construit ou ferme l’élément « option » de l’interface ; son contenu reste géré par React. */}
              </option>
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            ))}
          {/* @ligne Construit ou ferme l’élément « select » de l’interface ; son contenu reste géré par React. */}
          </select>
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        </label>
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        <label className="inline-label">
          {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
          <input
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            type="checkbox"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            checked={raw}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            onChange={(e) => setRaw(e.target.checked)}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          />{" "}
          Essayer ARP actif et écoute LLDP/CDP (permissions système requises)
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        </label>
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        <label>
          Communauté SNMPv2c de la passerelle, facultative
          {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
          <input
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            type="password"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            value={community}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            onChange={(e) => setCommunity(e.target.value)}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            autoComplete="off"
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          />
        {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
        </label>
        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
        <p className="help">
          La communauté n'est pas conservée. Sans droits bruts, le scan garde
          TCP, mDNS et les tables système.
        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
        </p>
      {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
      </details>
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
      {initial?.warnings?.map((warning: string) => (
        // @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React.
        <p key={warning} className="alert">
          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
          {warning}
        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
        </p>
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      ))}
      {/* @ligne Construit ou ferme l’élément « GraphCanvas » de l’interface ; son contenu reste géré par React. */}
      <GraphCanvas
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        name={"reseau-" + (initial?.cidr || cidr)}
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        spec={spec}
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        onError={onError}
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        onSelect={(id) =>
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          setSelected(initial?.nodes.find((n: any) => n.id === id))
        // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
        }
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      />
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
      {!spec.nodes.length && (
        // @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React.
        <p className="help">
          Aucun service observé. Un équipement silencieux peut rester invisible.
        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
        </p>
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      )}
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
      {selected && (
        // @ligne Construit ou ferme l’élément « section » de l’interface ; son contenu reste géré par React.
        <section className="panel">
          {/* @ligne Construit ou ferme l’élément « h2{selected.name » de l’interface ; son contenu reste géré par React. */}
          <h2>{selected.name || selected.ip || selected.id}</h2>
          {/* @ligne Construit ou ferme l’élément « pre{JSON.stringify(selected, » de l’interface ; son contenu reste géré par React. */}
          <pre>{JSON.stringify(selected, null, 2)}</pre>
          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
          {selected.ip && (
            // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
            <div className="actions">
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              <button onClick={() => copy("ssh utilisateur@" + selected.ip)}>
                Copier SSH
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              </button>
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              <button onClick={() => copy("nmap -sV " + selected.ip)}>
                Copier Nmap
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              </button>
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              <button onClick={() => copy("ping " + selected.ip)}>
                Copier Ping
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              </button>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          )}
        {/* @ligne Construit ou ferme l’élément « section » de l’interface ; son contenu reste géré par React. */}
        </section>
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
      )}
      {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
      <p className="help">
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
        {initial?.notice ||
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          "Découverte locale bornée au /24 : TCP, tables du système et mDNS. Aucun switch ou lien physique n’est inventé."}
      {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
      </p>
      {/* @ligne Construit ou ferme l’élément « section » de l’interface ; son contenu reste géré par React. */}
      <section className="panel">
        {/* @ligne Construit ou ferme l’élément « h3FAI » de l’interface ; son contenu reste géré par React. */}
        <h3>FAI et adresse WAN</h3>
        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
        <p className="help">
          Action facultative : le service ipapi.co recevra votre adresse IP
          publique. Le mode local reste disponible sans cet appel.
        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
        </p>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          disabled={busy}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          onClick={async () => {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            setBusy(true);
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            try {
              // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours des observations réseau et du choix des sondes.
              setWan(await api("net_wan"));
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            } catch (error) {
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
              onError(String(error));
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
            } finally {
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
              setBusy(false);
            // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
            }
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
          }}
        // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
        >
          Interroger le WAN sur Internet
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
        {wan && (
          // @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React.
          <p>
            {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
            {wan.isp || "FAI inconnu"} · {wan.ip} · {wan.asn} · cache du{" "}
            {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
            {new Date(wan.observed_at * 1000).toLocaleString("fr-FR")}
          {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
          </p>
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        )}
      {/* @ligne Construit ou ferme l’élément « section » de l’interface ; son contenu reste géré par React. */}
      </section>
      {/* @ligne Construit ou ferme l’élément « section » de l’interface ; son contenu reste géré par React. */}
      <section className="panel">
        {/* @ligne Construit ou ferme l’élément « h3Interfaces » de l’interface ; son contenu reste géré par React. */}
        <h3>Interfaces de cette machine</h3>
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes. */}
        {interfaces.map((i) => (
          // @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React.
          <p key={i.name + i.ip}>
            {/* @ligne Construit ou ferme l’élément « code{i.name}code » de l’interface ; son contenu reste géré par React. */}
            <code>{i.name}</code> · {i.ip} / {i.netmask}
          {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
          </p>
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre des observations réseau et du choix des sondes.
        ))}
      {/* @ligne Construit ou ferme l’élément « section » de l’interface ; son contenu reste géré par React. */}
      </section>
    {/* @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React. */}
    </>
  // @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
  );
// @ligne Termine cette construction et revient au contexte parent des observations réseau et du choix des sondes.
}
