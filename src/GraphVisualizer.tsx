import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { SparseMatrix } from "./State";


interface Node extends d3.SimulationNodeDatum {
    id: string;
}

interface Link extends d3.SimulationLinkDatum<Node> {
    value: number;
}

interface GraphVisualizerProps {
    sparseMatrix: SparseMatrix;
    path: number[];
}

const GraphVisualizer: React.FC<GraphVisualizerProps> = ({ sparseMatrix, path }) => {
    const d3Container = useRef<SVGSVGElement | null>(null);

    useEffect(() => {
        if (sparseMatrix && d3Container.current) {
            const svg = d3.select(d3Container.current);
            svg.selectAll("*").remove(); // Clear svg content

            const width = 800;  // Adjust as needed
            const height = 600; // Adjust as needed

            // Convert sparse matrix to nodes and links
            const nodes: Node[] = Object.keys(sparseMatrix).map(id => ({ id }));
            const links: Link[] = Object.entries(sparseMatrix).flatMap(([source, targets]) =>
                Object.entries(targets).map(([target, value]) => ({ source, target, value: value as number }))
            );
            const pathSet: Set<number> = new Set(path);
            console.log('pathSet', pathSet);
            console.log('nodes', nodes);

            // Setup the force simulation
            const simulation = d3.forceSimulation()
                .force("link", d3.forceLink().id((d: any) => d.id))
                .force("charge", d3.forceManyBody())
                .force("center", d3.forceCenter(width / 2, height / 2));

            // Create links (lines)
            const link = svg.append("g")
                .attr("class", "links")
                .selectAll("line")
                .data(links)
                .enter().append("line")
                .style("stroke-width", 5)
                .style("stroke", "black");

            // Create nodes (circles)
            const node = svg.append("g")
                .attr("class", "nodes")
                .selectAll("circle")
                .data(nodes)
                .enter().append("circle")
                .attr("r", 5)
                .style("fill", d => pathSet.has(Number(d.id)) ? "red" : "blue");

            // Add labels to nodes
            const label = svg.append("g")
                .attr("class", "labels")
                .selectAll("text")
                .data(nodes)
                .enter().append("text")
                .text(d => d.id);

            const ticked = () => {
                link
                    .attr("x1", (d: any) => d.source.x)
                    .attr("y1", (d: any) => d.source.y)
                    .attr("x2", (d: any) => d.target.x)
                    .attr("y2", (d: any) => d.target.y);

                node
                    .attr("cx", (d: any) => d.x)
                    .attr("cy", (d: any) => d.y);

                label
                    .attr("x", (d: any) => d.x + 6)
                    .attr("y", (d: any) => d.y - 6);
            }

            // Update force simulation nodes and links
            simulation
                .nodes(nodes as any)
                .on("tick", ticked);

            if (simulation.force("link")) {
                simulation!.force<d3.ForceLink<Node, Link>>("link")!.links(links);
            }
        }
    }, [sparseMatrix, path]);

    return (
        <svg
            className="d3-component w-100 h-100"
            style={{minHeight: '1000px'}}
            ref={d3Container}
        />
    );
}

export default GraphVisualizer;
