"""
Web Server - Serves the visualization dashboard.

A lightweight Flask-based server for the web UI.
"""

import os
import json
import threading
from typing import Optional, Dict, Any, Callable
from dataclasses import dataclass, asdict
from datetime import datetime


@dataclass
class TranslationEvent:
    """Event for real-time updates."""
    type: str  # 'start', 'progress', 'complete', 'error'
    task_id: str
    task_name: str
    status: str
    progress: float
    message: str
    timestamp: str = ""
    
    def __post_init__(self):
        if not self.timestamp:
            self.timestamp = datetime.now().isoformat()


class WebServer:
    """
    Web server for the Rustify dashboard.
    
    Features:
    - Real-time progress updates via SSE
    - REST API for translation status
    - Interactive dependency graph
    - Error analysis views
    """
    
    def __init__(self, host: str = "127.0.0.1", port: int = 8765):
        """
        Initialize the web server.
        
        Args:
            host: Host to bind to
            port: Port to listen on
        """
        self.host = host
        self.port = port
        self._app = None
        self._thread: Optional[threading.Thread] = None
        self._events: list = []
        self._state: Dict[str, Any] = {}
        self._callbacks: Dict[str, Callable] = {}
    
    def _create_app(self):
        """Create the Flask application."""
        try:
            from flask import Flask, jsonify, render_template_string, Response
            from flask_cors import CORS
        except ImportError:
            print("Warning: Flask not installed. Run: pip install flask flask-cors")
            return None
        
        app = Flask(__name__)
        CORS(app)
        
        @app.route('/')
        def index():
            return render_template_string(DASHBOARD_HTML)
        
        @app.route('/api/status')
        def status():
            return jsonify(self._state)
        
        @app.route('/api/events')
        def events():
            def generate():
                last_idx = 0
                while True:
                    if len(self._events) > last_idx:
                        for event in self._events[last_idx:]:
                            yield f"data: {json.dumps(asdict(event))}\n\n"
                        last_idx = len(self._events)
                    import time
                    time.sleep(0.5)
            
            return Response(generate(), mimetype='text/event-stream')
        
        @app.route('/api/graph')
        def graph():
            return jsonify(self._state.get('dependency_graph', {}))
        
        return app
    
    def start(self, blocking: bool = False) -> None:
        """
        Start the web server.
        
        Args:
            blocking: If True, block the current thread
        """
        self._app = self._create_app()
        if self._app is None:
            return
        
        if blocking:
            self._app.run(host=self.host, port=self.port, debug=False)
        else:
            self._thread = threading.Thread(
                target=lambda: self._app.run(
                    host=self.host, 
                    port=self.port, 
                    debug=False,
                    use_reloader=False
                ),
                daemon=True
            )
            self._thread.start()
            print(f"Dashboard available at http://{self.host}:{self.port}")
    
    def stop(self) -> None:
        """Stop the web server."""
        # Flask doesn't have a clean shutdown, but the daemon thread will die with the process
        pass
    
    def emit_event(self, event: TranslationEvent) -> None:
        """Emit a translation event."""
        self._events.append(event)
        # Keep only last 1000 events
        if len(self._events) > 1000:
            self._events = self._events[-1000:]
    
    def update_state(self, **kwargs) -> None:
        """Update the server state."""
        self._state.update(kwargs)
    
    def set_dependency_graph(self, nodes: list, edges: list) -> None:
        """Set the dependency graph for visualization."""
        self._state['dependency_graph'] = {
            'nodes': nodes,
            'edges': edges
        }


# Dashboard HTML template with embedded CSS and JS
DASHBOARD_HTML = '''
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rustify Dashboard</title>
    <script src="https://cdn.jsdelivr.net/npm/d3@7"></script>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'JetBrains Mono', 'Fira Code', monospace;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 50%, #0f3460 100%);
            color: #e4e4e4;
            min-height: 100vh;
        }
        .container { max-width: 1400px; margin: 0 auto; padding: 20px; }
        header {
            text-align: center;
            padding: 30px 0;
            border-bottom: 1px solid #3a3a5c;
            margin-bottom: 30px;
        }
        h1 {
            font-size: 2.5em;
            background: linear-gradient(90deg, #00d4ff, #7b2cbf);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            margin-bottom: 10px;
        }
        .subtitle { color: #888; font-size: 0.9em; }
        .grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        .card {
            background: rgba(255,255,255,0.05);
            border-radius: 12px;
            padding: 20px;
            border: 1px solid rgba(255,255,255,0.1);
            backdrop-filter: blur(10px);
        }
        .card h2 {
            font-size: 1em;
            color: #00d4ff;
            margin-bottom: 15px;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
        .stat {
            font-size: 2.5em;
            font-weight: bold;
            color: #fff;
        }
        .stat-label { color: #888; font-size: 0.8em; }
        .progress-bar {
            height: 8px;
            background: rgba(255,255,255,0.1);
            border-radius: 4px;
            overflow: hidden;
            margin-top: 10px;
        }
        .progress-fill {
            height: 100%;
            background: linear-gradient(90deg, #00d4ff, #7b2cbf);
            border-radius: 4px;
            transition: width 0.3s ease;
        }
        #graph-container {
            background: rgba(0,0,0,0.3);
            border-radius: 12px;
            height: 400px;
            overflow: hidden;
        }
        .node circle {
            stroke: #fff;
            stroke-width: 2px;
        }
        .node text {
            font-size: 10px;
            fill: #fff;
        }
        .link {
            stroke: #555;
            stroke-opacity: 0.6;
        }
        .event-log {
            max-height: 300px;
            overflow-y: auto;
            font-size: 0.85em;
        }
        .event {
            padding: 8px 12px;
            border-left: 3px solid #00d4ff;
            margin-bottom: 8px;
            background: rgba(0,0,0,0.2);
            border-radius: 0 6px 6px 0;
        }
        .event.error { border-left-color: #ff4757; }
        .event.success { border-left-color: #2ed573; }
        .event-time { color: #666; font-size: 0.8em; }
        .status-badge {
            display: inline-block;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 0.75em;
            font-weight: bold;
        }
        .status-done { background: #2ed573; color: #000; }
        .status-translating { background: #ffa502; color: #000; }
        .status-error { background: #ff4757; color: #fff; }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>⚙️ RUSTIFY</h1>
            <p class="subtitle">C/C++ to Rust Translation Dashboard</p>
        </header>
        
        <div class="grid">
            <div class="card">
                <h2>📊 Progress</h2>
                <div class="stat" id="progress-pct">0%</div>
                <div class="stat-label">Translation Complete</div>
                <div class="progress-bar">
                    <div class="progress-fill" id="progress-bar" style="width: 0%"></div>
                </div>
            </div>
            <div class="card">
                <h2>📁 Files</h2>
                <div class="stat" id="file-count">0 / 0</div>
                <div class="stat-label">Translated / Total</div>
            </div>
            <div class="card">
                <h2>⚡ Status</h2>
                <div class="stat" id="current-status">Idle</div>
                <div class="stat-label" id="current-file">-</div>
            </div>
            <div class="card">
                <h2>❌ Errors</h2>
                <div class="stat" id="error-count">0</div>
                <div class="stat-label">Compilation Errors</div>
            </div>
        </div>
        
        <div class="card" style="margin-bottom: 20px;">
            <h2>🔗 Dependency Graph</h2>
            <div id="graph-container"></div>
        </div>
        
        <div class="card">
            <h2>📜 Event Log</h2>
            <div class="event-log" id="event-log"></div>
        </div>
    </div>
    
    <script>
        // Fetch initial status
        async function fetchStatus() {
            try {
                const res = await fetch('/api/status');
                const data = await res.json();
                updateUI(data);
            } catch (e) {
                console.error('Failed to fetch status:', e);
            }
        }
        
        function updateUI(data) {
            if (data.progress !== undefined) {
                document.getElementById('progress-pct').textContent = Math.round(data.progress * 100) + '%';
                document.getElementById('progress-bar').style.width = (data.progress * 100) + '%';
            }
            if (data.files_done !== undefined && data.files_total !== undefined) {
                document.getElementById('file-count').textContent = data.files_done + ' / ' + data.files_total;
            }
            if (data.status) {
                document.getElementById('current-status').textContent = data.status;
            }
            if (data.current_file) {
                document.getElementById('current-file').textContent = data.current_file;
            }
            if (data.error_count !== undefined) {
                document.getElementById('error-count').textContent = data.error_count;
            }
        }
        
        // SSE for real-time events
        const eventSource = new EventSource('/api/events');
        eventSource.onmessage = function(e) {
            const event = JSON.parse(e.data);
            addEventToLog(event);
            updateUI({
                status: event.status,
                current_file: event.task_name,
                progress: event.progress
            });
        };
        
        function addEventToLog(event) {
            const log = document.getElementById('event-log');
            const div = document.createElement('div');
            div.className = 'event ' + (event.type === 'error' ? 'error' : event.type === 'complete' ? 'success' : '');
            div.innerHTML = `
                <span class="event-time">${new Date(event.timestamp).toLocaleTimeString()}</span>
                <strong>${event.task_name}</strong>: ${event.message}
            `;
            log.insertBefore(div, log.firstChild);
            // Keep only last 50 events in UI
            while (log.children.length > 50) {
                log.removeChild(log.lastChild);
            }
        }
        
        // Initialize D3 graph
        async function initGraph() {
            try {
                const res = await fetch('/api/graph');
                const data = await res.json();
                if (data.nodes && data.edges) {
                    drawGraph(data);
                }
            } catch (e) {
                console.error('Failed to fetch graph:', e);
            }
        }
        
        function drawGraph(data) {
            const container = document.getElementById('graph-container');
            const width = container.clientWidth;
            const height = container.clientHeight;
            
            const svg = d3.select('#graph-container')
                .append('svg')
                .attr('width', width)
                .attr('height', height);
            
            const simulation = d3.forceSimulation(data.nodes)
                .force('link', d3.forceLink(data.edges).id(d => d.id).distance(80))
                .force('charge', d3.forceManyBody().strength(-200))
                .force('center', d3.forceCenter(width / 2, height / 2));
            
            const link = svg.append('g')
                .selectAll('line')
                .data(data.edges)
                .join('line')
                .attr('class', 'link');
            
            const node = svg.append('g')
                .selectAll('g')
                .data(data.nodes)
                .join('g')
                .attr('class', 'node')
                .call(d3.drag()
                    .on('start', dragstarted)
                    .on('drag', dragged)
                    .on('end', dragended));
            
            node.append('circle')
                .attr('r', 8)
                .attr('fill', d => d.status === 'done' ? '#2ed573' : d.status === 'error' ? '#ff4757' : '#00d4ff');
            
            node.append('text')
                .attr('dx', 12)
                .attr('dy', 4)
                .text(d => d.name);
            
            simulation.on('tick', () => {
                link
                    .attr('x1', d => d.source.x)
                    .attr('y1', d => d.source.y)
                    .attr('x2', d => d.target.x)
                    .attr('y2', d => d.target.y);
                
                node.attr('transform', d => `translate(${d.x},${d.y})`);
            });
            
            function dragstarted(event) {
                if (!event.active) simulation.alphaTarget(0.3).restart();
                event.subject.fx = event.subject.x;
                event.subject.fy = event.subject.y;
            }
            
            function dragged(event) {
                event.subject.fx = event.x;
                event.subject.fy = event.y;
            }
            
            function dragended(event) {
                if (!event.active) simulation.alphaTarget(0);
                event.subject.fx = null;
                event.subject.fy = null;
            }
        }
        
        // Initialize
        fetchStatus();
        initGraph();
        setInterval(fetchStatus, 5000);
    </script>
</body>
</html>
'''
