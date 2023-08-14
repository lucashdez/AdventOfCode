export class DirGraph {
    

}

export class Node {
    public edges: Array<Edge> = [];
    public identifier = "";

    Node(id: string) {
        this.identifier = id;
    }

}

export class Edge {
    from: Node;
    to: Node;
    cost = 0;

    Edge(f: Node, t: Node, c: number) {
        this.from = f;
        this.to = t;
        this.cost = c;
    }
}
