# Architecture Diagrams

This directory contains Mermaid diagrams visualizing Ferrisbot's architecture.

## Diagrams

- [System Architecture](./system-architecture.md) - Overall system design
- [Message Flow](./message-flow.md) - Request/response flow
- [Component Diagram](./components.md) - Module relationships
- [Deployment](./deployment.md) - Runtime deployment view
- [State Machine](./state-machine.md) - Message processing states

## Viewing Diagrams

### GitHub
Mermaid diagrams render automatically in GitHub markdown.

### VS Code
Install the "Markdown Preview Mermaid Support" extension.

### Command Line
```bash
# Install mermaid-cli
npm install -g @mermaid-js/mermaid-cli

# Generate PNG
mmdc -i system-architecture.md -o system-architecture.png
```

### Online
Copy the mermaid code to [https://mermaid.live/](https://mermaid.live/)

## Updating Diagrams

When architecture changes:
1. Update the corresponding diagram
2. Add version/date to the diagram
3. Update this index if adding new diagrams
4. Consider adding to ADR if it's a significant decision
