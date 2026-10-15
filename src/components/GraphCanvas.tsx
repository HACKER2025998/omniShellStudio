// Le canevas est partagé entre réseau et SQL ; aucune bibliothèque n'est chargée depuis un CDN.
import { useEffect, useMemo, useRef, useState, type ReactNode } from "react";
// @ligne Importe les outils nécessaires à la mise en œuvre du canevas déplaçable et des exports.
import {
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  ReactFlow,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  Background,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  Controls,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  MiniMap,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  useNodesState,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  useEdgesState,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  type Node,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  type Edge,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
} from "@xyflow/react";
// @ligne Importe les outils nécessaires à la mise en œuvre du canevas déplaçable et des exports.
import dagre from "@dagrejs/dagre";
// @ligne Importe les outils nécessaires à la mise en œuvre du canevas déplaçable et des exports.
import { toPng } from "html-to-image";
// @ligne Importe les outils nécessaires à la mise en œuvre du canevas déplaçable et des exports.
import { download } from "../api";
// @ligne Importe les outils nécessaires à la mise en œuvre du canevas déplaçable et des exports.
import "@xyflow/react/dist/style.css";
// Une représentation neutre garde les exports indépendants du rendu HTML des nœuds.
export type GraphNode = {
  // @ligne Renseigne le champ « id » de la structure utilisée ici du canevas déplaçable et des exports.
  id: string;
  // @ligne Renseigne le champ « title » de la structure utilisée ici du canevas déplaçable et des exports.
  title: string;
  // @ligne Renseigne le champ « lines » de la structure utilisée ici du canevas déplaçable et des exports.
  lines: string[];
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  kind?: string;
// @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
};
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
export type GraphSpec = {
  // @ligne Renseigne le champ « nodes » de la structure utilisée ici du canevas déplaçable et des exports.
  nodes: GraphNode[];
  // @ligne Renseigne le champ « edges » de la structure utilisée ici du canevas déplaçable et des exports.
  edges: {
    // @ligne Renseigne le champ « source » de la structure utilisée ici du canevas déplaçable et des exports.
    source: string;
    // @ligne Renseigne le champ « target » de la structure utilisée ici du canevas déplaçable et des exports.
    target: string;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    label?: string;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    logical?: boolean;
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  }[];
// @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
};
// Les types de données n'acceptent aucun HTML brut issu d'un équipement ou d'un script SQL.
type CanvasNode = Node<{ label: ReactNode; spec: GraphNode }>;
// Dagre calcule une disposition lisible, modifiable ensuite par glisser-déposer.
function layout(spec: GraphSpec): CanvasNode[] {
  // @ligne Prépare « graph » pour la prochaine étape du canevas déplaçable et des exports.
  const graph = new dagre.graphlib.Graph();
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  graph.setGraph({ rankdir: "TB", ranksep: 90, nodesep: 50 });
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  graph.setDefaultEdgeLabel(() => ({}));
  // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du canevas déplaçable et des exports.
  for (const node of spec.nodes)
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    graph.setNode(node.id, { width: 260, height: 70 + node.lines.length * 23 });
  // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du canevas déplaçable et des exports.
  for (const edge of spec.edges)
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du canevas déplaçable et des exports.
    if (graph.hasNode(edge.source) && graph.hasNode(edge.target))
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
      graph.setEdge(edge.source, edge.target);
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  dagre.layout(graph);
  // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du canevas déplaçable et des exports.
  return spec.nodes.map((node) => {
    // @ligne Prépare « position » pour la prochaine étape du canevas déplaçable et des exports.
    const position = graph.node(node.id);
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du canevas déplaçable et des exports.
    return {
      // @ligne Renseigne le champ « id » de la structure utilisée ici du canevas déplaçable et des exports.
      id: node.id,
      // @ligne Renseigne le champ « position » de la structure utilisée ici du canevas déplaçable et des exports.
      position: {
        // @ligne Renseigne le champ « x » de la structure utilisée ici du canevas déplaçable et des exports.
        x: position.x - 130,
        // @ligne Renseigne le champ « y » de la structure utilisée ici du canevas déplaçable et des exports.
        y: position.y - (70 + node.lines.length * 23) / 2,
      // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
      },
      // @ligne Renseigne le champ « style » de la structure utilisée ici du canevas déplaçable et des exports.
      style: {
        // @ligne Renseigne le champ « width » de la structure utilisée ici du canevas déplaçable et des exports.
        width: 260,
        // @ligne Renseigne le champ « padding » de la structure utilisée ici du canevas déplaçable et des exports.
        padding: 0,
        // @ligne Renseigne le champ « borderRadius » de la structure utilisée ici du canevas déplaçable et des exports.
        borderRadius: 10,
        // @ligne Renseigne le champ « border » de la structure utilisée ici du canevas déplaçable et des exports.
        border: "1px solid #94a3b8",
        // @ligne Renseigne le champ « background » de la structure utilisée ici du canevas déplaçable et des exports.
        background: "white",
      // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
      },
      // @ligne Renseigne le champ « data » de la structure utilisée ici du canevas déplaçable et des exports.
      data: {
        // @ligne Renseigne le champ « spec » de la structure utilisée ici du canevas déplaçable et des exports.
        spec: node,
        // @ligne Renseigne le champ « label » de la structure utilisée ici du canevas déplaçable et des exports.
        label: (
          // @ligne Construit ou ferme l’élément « article » de l’interface ; son contenu reste géré par React.
          <article
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
            className={
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
              node.kind === "table" ? "entity graph-entity" : "graph-node-body"
            // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
            }
          // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
          >
            {/* @ligne Construit ou ferme l’élément « h3{node.title}h3 » de l’interface ; son contenu reste géré par React. */}
            <h3>{node.title}</h3>
            {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports. */}
            {node.lines.map((line, i) => (
              // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
              <div key={i}>{line}</div>
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
            ))}
          {/* @ligne Construit ou ferme l’élément « article » de l’interface ; son contenu reste géré par React. */}
          </article>
        // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
        ),
      // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
      },
    // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
    };
  // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
  });
// @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
}
// L'échappement est indispensable : les noms de fichiers, tables et hôtes restent des données.
const xml = (text: string) =>
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  text
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    .replaceAll("&", "&amp;")
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    .replaceAll("<", "&lt;")
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    .replaceAll(">", "&gt;")
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    .replaceAll('"', "&quot;");
// Le SVG est réellement vectoriel et ne dépend pas de foreignObject ou d'une police distante.
function svg(nodes: CanvasNode[], edges: Edge[]) {
  // @ligne Prépare « maxX » pour la prochaine étape du canevas déplaçable et des exports.
  const maxX = Math.max(700, ...nodes.map((n) => n.position.x + 300)),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    maxY = Math.max(
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
      450,
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
      ...nodes.map((n) => n.position.y + 110 + n.data.spec.lines.length * 23),
    // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
    );
  // @ligne Prépare « map » pour la prochaine étape du canevas déplaçable et des exports.
  const map = new Map(nodes.map((n) => [n.id, n]));
  // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du canevas déplaçable et des exports.
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${maxX + 40}" height="${maxY + 40}" viewBox="-20 -20 ${maxX + 40} ${maxY + 40}"><rect x="-20" y="-20" width="100%" height="100%" fill="#f8fafc"/>${edges
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    .map((e) => {
      // @ligne Prépare « a » pour la prochaine étape du canevas déplaçable et des exports.
      const a = map.get(e.source),
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
        b = map.get(e.target);
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du canevas déplaçable et des exports.
      if (!a || !b) return "";
      // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du canevas déplaçable et des exports.
      return `<path d="M${a.position.x + 130},${a.position.y + 70 + a.data.spec.lines.length * 23} L${b.position.x + 130},${b.position.y}" stroke="#64748b" stroke-width="1.5" stroke-dasharray="${e.animated ? "5 4" : "0"}" fill="none"/>`;
    // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
    })
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    .join(
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
      "",
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    )}${nodes.map((n) => `<g transform="translate(${n.position.x} ${n.position.y})"><rect width="260" height="${70 + n.data.spec.lines.length * 23}" rx="10" fill="white" stroke="#94a3b8"/><text x="14" y="27" font-family="sans-serif" font-size="14" fill="#1d4ed8">${xml(n.data.spec.title)}</text>${n.data.spec.lines.map((line, i) => `<text x="14" y="${54 + i * 23}" font-family="monospace" font-size="11" fill="#0f172a">${xml(line.slice(0, 38))}</text>`).join("")}</g>`).join("")}</svg>`;
// @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
}
// Les positions sont propres à un document ; changer de schéma ne déplace pas les autres modèles.
export function GraphCanvas({
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  spec,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  name,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  onSelect,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  onError,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
}: {
  // @ligne Renseigne le champ « spec » de la structure utilisée ici du canevas déplaçable et des exports.
  spec: GraphSpec;
  // @ligne Renseigne le champ « name » de la structure utilisée ici du canevas déplaçable et des exports.
  name: string;
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  onSelect?: (id: string) => void;
  // @ligne Renseigne le champ « onError » de la structure utilisée ici du canevas déplaçable et des exports.
  onError: (s: string) => void;
// @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
}) {
  // @ligne Prépare « key » pour la prochaine étape du canevas déplaçable et des exports.
  const key = JSON.stringify(spec),
    area = useRef<HTMLDivElement>(null); // La clé de contenu évite de réinitialiser le canevas à chaque rendu parent.
  const prepared = useMemo(() => layout(spec), [key]); // Le calcul de graphe est effectué seulement lorsque les données changent.
  const [nodes, setNodes, onNodesChange] = useNodesState<CanvasNode>(prepared); // React Flow gère les mouvements et sélections.
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]); // Les relations du modèle restent distinctes des positions.
  const [ready, setReady] = useState(false); // Le stockage ne doit pas écraser les positions avant leur restauration.
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  useEffect(() => {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    setReady(false);
    // @ligne Prépare « positions » pour la prochaine étape du canevas déplaçable et des exports.
    let positions: Record<string, { x: number; y: number }> = {};
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    try {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
      positions = JSON.parse(
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
        localStorage.getItem("omni-layout:" + name) || "{}",
      // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
      );
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    } catch {
      /* Une préférence locale invalide ne bloque pas le modèle. */
    }
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    setNodes(
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
      prepared.map((node) => ({
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
        ...node,
        // @ligne Renseigne le champ « position » de la structure utilisée ici du canevas déplaçable et des exports.
        position: positions[node.id] || node.position,
      // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
      })),
    // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
    );
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    setEdges(
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
      spec.edges.map((edge, i) => ({
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
        ...edge,
        // @ligne Renseigne le champ « id » de la structure utilisée ici du canevas déplaçable et des exports.
        id: "edge-" + i,
        // @ligne Renseigne le champ « animated » de la structure utilisée ici du canevas déplaçable et des exports.
        animated: edge.logical,
        // @ligne Renseigne le champ « style » de la structure utilisée ici du canevas déplaçable et des exports.
        style: {
          // @ligne Renseigne le champ « stroke » de la structure utilisée ici du canevas déplaçable et des exports.
          stroke: "#64748b",
          // @ligne Renseigne le champ « strokeDasharray » de la structure utilisée ici du canevas déplaçable et des exports.
          strokeDasharray: edge.logical ? "5 4" : undefined,
        // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
        },
      // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
      })),
    // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
    );
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
    setReady(true);
  }, [key, name]); // Une nouvelle observation actualise seulement les données du document choisi.
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
  useEffect(() => {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du canevas déplaçable et des exports.
    if (!ready) return;
    // @ligne Prépare « timer » pour la prochaine étape du canevas déplaçable et des exports.
    const timer = setTimeout(() => {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
      try {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
        localStorage.setItem(
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          "omni-layout:" + name,
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          JSON.stringify(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
            Object.fromEntries(nodes.map((n) => [n.id, n.position])),
          // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
          ),
        // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
        );
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
      } catch {
        /* Le mode privé peut interdire le stockage ; le canevas reste utilisable. */
      }
    // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
    }, 300);
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du canevas déplaçable et des exports.
    return () => clearTimeout(timer);
  }, [nodes, name, ready]); // Les écritures sont regroupées pendant le déplacement.
  // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du canevas déplaçable et des exports.
  return (
    // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
    <div className="graph-section">
      {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
      <div className="actions">
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          onClick={() => {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
            setNodes(prepared);
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
            try {
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
              localStorage.removeItem("omni-layout:" + name);
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
            } catch {}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          }}
        // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
        >
          Réorganiser
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          disabled={!nodes.length}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          onClick={() =>
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
            download(name + ".svg", svg(nodes, edges), "image/svg+xml")
          // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
          }
        // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
        >
          Exporter SVG
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          disabled={!nodes.length}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          onClick={async () => {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
            try {
              // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du canevas déplaçable et des exports.
              if (!area.current) return;
              // @ligne Prépare « url » pour la prochaine étape du canevas déplaçable et des exports.
              const url = await toPng(area.current, {
                // @ligne Renseigne le champ « pixelRatio » de la structure utilisée ici du canevas déplaçable et des exports.
                pixelRatio: 2,
                // @ligne Renseigne le champ « backgroundColor » de la structure utilisée ici du canevas déplaçable et des exports.
                backgroundColor: "#f8fafc",
                // @ligne Renseigne le champ « skipFonts » de la structure utilisée ici du canevas déplaçable et des exports.
                skipFonts: true,
              // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
              });
              // @ligne Prépare « link » pour la prochaine étape du canevas déplaçable et des exports.
              const link = document.createElement("a");
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
              link.href = url;
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
              link.download = name + ".png";
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
              link.click();
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
            } catch (error) {
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
              onError("Export PNG impossible : " + String(error));
            // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
            }
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          }}
        // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
        >
          Exporter PNG
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
      {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
      </div>
      {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
      <div className="interactive-graph" ref={area}>
        {/* @ligne Construit ou ferme l’élément « ReactFlow » de l’interface ; son contenu reste géré par React. */}
        <ReactFlow
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          nodes={nodes}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          edges={edges}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          onNodesChange={onNodesChange}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          onEdgesChange={onEdgesChange}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          onNodeClick={(_, node) => onSelect?.(node.id)}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          fitView
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          fitViewOptions={{ padding: 0.25 }}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          minZoom={0.15}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          maxZoom={2}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          colorMode="light"
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du canevas déplaçable et des exports.
          nodesConnectable={false}
        // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
        >
          {/* @ligne Construit ou ferme l’élément « Background » de l’interface ; son contenu reste géré par React. */}
          <Background color="#cbd5e1" gap={20} />
          {/* @ligne Construit ou ferme l’élément « Controls » de l’interface ; son contenu reste géré par React. */}
          <Controls showInteractive={false} />
          {/* @ligne Construit ou ferme l’élément « MiniMap » de l’interface ; son contenu reste géré par React. */}
          <MiniMap pannable zoomable />
        {/* @ligne Construit ou ferme l’élément « ReactFlow » de l’interface ; son contenu reste géré par React. */}
        </ReactFlow>
      {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
      </div>
      {/* @ligne Construit ou ferme l’élément « small » de l’interface ; son contenu reste géré par React. */}
      <small>
        Déplacez les éléments, zoomez ou sélectionnez un nœud. Les traits
        pointillés représentent des relations logiques.
      {/* @ligne Construit ou ferme l’élément « small » de l’interface ; son contenu reste géré par React. */}
      </small>
    {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
    </div>
  // @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
  );
// @ligne Termine cette construction et revient au contexte parent du canevas déplaçable et des exports.
}
