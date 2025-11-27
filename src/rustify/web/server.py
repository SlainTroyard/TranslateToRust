"""
Web Server - Serves the visualization dashboard.

A lightweight Flask-based server for the web UI.
"""

import os
import json
import threading
from typing import Optional, Dict, Any, Callable, List
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
    - Watch mode: monitors states.json for updates
    """
    
    def __init__(self, host: str = "0.0.0.0", port: int = 8765, watch_dir: Optional[str] = None):
        """
        Initialize the web server.
        
        Args:
            host: Host to bind to
            port: Port to listen on
            watch_dir: Directory to watch for states.json updates
        """
        self.host = host
        self.port = port
        self.watch_dir = watch_dir
        self._app = None
        self._thread: Optional[threading.Thread] = None
        self._watch_thread: Optional[threading.Thread] = None
        self._events: list = []
        self._state: Dict[str, Any] = {
            'status': 'Idle',
            'progress': 0.0,
            'files_done': 0,
            'files_total': 0,
            'error_count': 0,
            'current_file': '-',
            'tasks': [],
            'dependency_graph': {'nodes': [], 'edges': []}
        }
        self._callbacks: Dict[str, Callable] = {}
        self._last_state_mtime: float = 0
    
    def _create_app(self):
        """Create the Flask application."""
        try:
            from flask import Flask, jsonify, render_template_string, Response, request
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
        
        @app.route('/api/tasks')
        def tasks():
            return jsonify(self._state.get('tasks', []))
        
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
        
        # Start state file watcher if watch_dir is set
        if self.watch_dir:
            self._start_watcher()
        
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
            print(f"🌐 Dashboard: http://{self.host}:{self.port}")
    
    def _start_watcher(self) -> None:
        """Start watching states.json for updates."""
        import time
        
        def watch_loop():
            while True:
                try:
                    self._check_state_file()
                except Exception as e:
                    pass
                time.sleep(1)  # Check every second
        
        self._watch_thread = threading.Thread(target=watch_loop, daemon=True)
        self._watch_thread.start()
        print(f"👁️  Watching: {self.watch_dir}")
    
    def _check_state_file(self) -> None:
        """Check and load states.json if updated."""
        import os
        
        state_file = os.path.join(self.watch_dir, "states.json")
        if not os.path.exists(state_file):
            return
        
        mtime = os.path.getmtime(state_file)
        if mtime <= self._last_state_mtime:
            return
        
        self._last_state_mtime = mtime
        
        try:
            with open(state_file, 'r') as f:
                data = json.load(f)
            self._update_from_state_file(data)
        except Exception as e:
            print(f"Error reading state: {e}")
    
    def _update_from_state_file(self, data: dict) -> None:
        """Update dashboard state from states.json data."""
        modules = data.get('module_translations', [])
        
        tasks = []
        files_done = 0
        files_total = 0
        error_count = 0
        current_file = '-'
        
        for module in modules:
            module_tasks = module.get('translation_tasks', [])
            files_total += len(module_tasks)
            
            for task in module_tasks:
                task_status = task.get('status', 'pending')
                task_name = task.get('source', {}).get('name', 'unknown')
                
                # Map status
                if task_status == 'done':
                    status = 'done'
                    files_done += 1
                elif task_status == 'error':
                    status = 'error'
                    error_count += 1
                elif task_status in ['translating', 'in_progress']:
                    status = 'translating'
                    current_file = task_name
                else:
                    status = 'pending'
                
                tasks.append({
                    'id': task_name,
                    'name': task_name,
                    'status': status
                })
        
        # Calculate progress
        progress = files_done / files_total if files_total > 0 else 0
        
        # Determine overall status
        if files_done == files_total and files_total > 0:
            overall_status = 'Done'
        elif error_count > 0:
            overall_status = f'Translating ({error_count} errors)'
        elif files_done > 0:
            overall_status = f'Translating: {current_file}'
        else:
            overall_status = 'Starting...'
        
        # Update state
        self._state.update({
            'status': overall_status,
            'progress': progress,
            'files_done': files_done,
            'files_total': files_total,
            'error_count': error_count,
            'current_file': current_file,
            'tasks': tasks
        })
        
        # Build dependency graph with chain edges (each file depends on previous)
        if tasks:
            nodes = [{'id': t['name'], 'name': t['name'], 'status': t['status']} for t in tasks]
            # Create chain: file1 -> file2 -> file3 (translation order)
            edges = []
            for i in range(len(tasks) - 1):
                edges.append({
                    'source': tasks[i]['name'],
                    'target': tasks[i + 1]['name']
                })
            self._state['dependency_graph'] = {'nodes': nodes, 'edges': edges}
    
    def stop(self) -> None:
        """Stop the web server."""
        pass
    
    def emit_event(self, event: TranslationEvent) -> None:
        """Emit a translation event."""
        self._events.append(event)
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
    
    def add_task(self, task_id: str, name: str, status: str = 'pending') -> None:
        """Add a task to the task list."""
        tasks = self._state.get('tasks', [])
        tasks.append({
            'id': task_id,
            'name': name,
            'status': status,
            'progress': 0
        })
        self._state['tasks'] = tasks
    
    def update_task(self, task_id: str, status: str, progress: float = 0) -> None:
        """Update a task's status."""
        tasks = self._state.get('tasks', [])
        for task in tasks:
            if task['id'] == task_id:
                task['status'] = status
                task['progress'] = progress
                break
        self._state['tasks'] = tasks


# Dashboard HTML - Clean Academic Style
DASHBOARD_HTML = '''
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rustify - C to Rust Translator</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link href="https://fonts.googleapis.com/css2?family=Source+Serif+4:opsz,wght@8..60,400;8..60,600&family=Source+Code+Pro:wght@400;500&display=swap" rel="stylesheet">
    <script src="https://cdn.jsdelivr.net/npm/d3@7"></script>
    <style>
        :root {
            --bg-primary: #fafafa;
            --bg-secondary: #ffffff;
            --bg-tertiary: #f5f5f5;
            --text-primary: #1a1a1a;
            --text-secondary: #555555;
            --text-muted: #888888;
            --border-color: #e0e0e0;
            --accent-blue: #2563eb;
            --accent-green: #16a34a;
            --accent-red: #dc2626;
            --accent-yellow: #ca8a04;
        }
        
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'Source Serif 4', Georgia, serif;
            background: var(--bg-primary);
            color: var(--text-primary);
            line-height: 1.6;
            min-height: 100vh;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 40px 24px;
        }
        
        /* Header */
        header {
            margin-bottom: 40px;
            padding-bottom: 20px;
            border-bottom: 2px solid var(--text-primary);
        }
        
        h1 {
            font-size: 2.5rem;
            font-weight: 600;
            letter-spacing: -0.02em;
            margin-bottom: 8px;
        }
        
        .subtitle {
            font-size: 1.1rem;
            color: var(--text-secondary);
            font-style: italic;
        }
        
        /* Status Section */
        .status-section {
            display: flex;
            align-items: center;
            gap: 24px;
            padding: 16px 20px;
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            margin-bottom: 32px;
        }
        
        .status-indicator {
            display: flex;
            align-items: center;
            gap: 10px;
        }
        
        .status-dot {
            width: 10px;
            height: 10px;
            border-radius: 50%;
            background: var(--text-muted);
        }
        
        .status-dot.running {
            background: var(--accent-blue);
            animation: pulse 1.5s infinite;
        }
        
        .status-dot.done { background: var(--accent-green); }
        .status-dot.error { background: var(--accent-red); }
        
        @keyframes pulse {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.5; }
        }
        
        .status-text {
            font-family: 'Source Code Pro', monospace;
            font-size: 0.95rem;
            font-weight: 500;
        }
        
        .current-file {
            font-family: 'Source Code Pro', monospace;
            font-size: 0.9rem;
            color: var(--text-muted);
            margin-left: auto;
        }
        
        /* Stats Grid */
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 20px;
            margin-bottom: 32px;
        }
        
        @media (max-width: 800px) {
            .stats-grid { grid-template-columns: repeat(2, 1fr); }
        }
        
        .stat-card {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            padding: 20px;
            text-align: center;
        }
        
        .stat-value {
            font-family: 'Source Code Pro', monospace;
            font-size: 2rem;
            font-weight: 500;
            color: var(--text-primary);
        }
        
        .stat-value.success { color: var(--accent-green); }
        .stat-value.error { color: var(--accent-red); }
        
        .stat-label {
            font-size: 0.85rem;
            color: var(--text-secondary);
            text-transform: uppercase;
            letter-spacing: 0.05em;
            margin-top: 6px;
        }
        
        /* Progress Bar */
        .progress-section {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            padding: 20px;
            margin-bottom: 32px;
        }
        
        .section-title {
            font-size: 1rem;
            font-weight: 600;
            margin-bottom: 16px;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }
        
        .progress-container {
            height: 24px;
            background: var(--bg-tertiary);
            border: 1px solid var(--border-color);
            position: relative;
        }
        
        .progress-bar {
            height: 100%;
            background: var(--accent-blue);
            transition: width 0.3s ease;
        }
        
        .progress-text {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            font-family: 'Source Code Pro', monospace;
            font-size: 0.85rem;
            font-weight: 500;
        }
        
        /* Two Column Layout */
        .main-grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 24px;
            margin-bottom: 32px;
        }
        
        @media (max-width: 900px) {
            .main-grid { grid-template-columns: 1fr; }
        }
        
        .panel {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
        }
        
        .panel-header {
            padding: 16px 20px;
            border-bottom: 1px solid var(--border-color);
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            font-size: 0.9rem;
        }
        
        .panel-body {
            padding: 0;
        }
        
        /* File List */
        .file-list {
            max-height: 350px;
            overflow-y: auto;
        }
        
        .file-item {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 20px;
            border-bottom: 1px solid var(--bg-tertiary);
            font-family: 'Source Code Pro', monospace;
            font-size: 0.85rem;
        }
        
        .file-item:last-child {
            border-bottom: none;
        }
        
        .file-name {
            display: flex;
            align-items: center;
            gap: 10px;
        }
        
        .file-icon {
            width: 8px;
            height: 8px;
            border-radius: 50%;
            background: var(--text-muted);
        }
        
        .file-icon.pending { background: var(--text-muted); }
        .file-icon.translating { background: var(--accent-blue); }
        .file-icon.done { background: var(--accent-green); }
        .file-icon.error { background: var(--accent-red); }
        
        .file-status {
            font-size: 0.75rem;
            padding: 3px 8px;
            border: 1px solid;
            text-transform: uppercase;
        }
        
        .file-status.pending { 
            border-color: var(--text-muted); 
            color: var(--text-muted); 
        }
        .file-status.translating { 
            border-color: var(--accent-blue); 
            color: var(--accent-blue); 
        }
        .file-status.done { 
            border-color: var(--accent-green); 
            color: var(--accent-green); 
        }
        .file-status.error { 
            border-color: var(--accent-red); 
            color: var(--accent-red); 
        }
        
        /* Dependency Graph */
        #graph-container {
            width: 100%;
            height: 100%;
            min-height: 350px;
            background: var(--bg-tertiary);
            overflow: hidden;
        }
        
        .graph-panel .panel-body {
            height: 350px;
        }
        
        #graph-container svg {
            display: block;
            width: 100%;
            height: 100%;
        }
        
        .node circle {
            stroke: var(--text-primary);
            stroke-width: 1px;
            cursor: pointer;
        }
        
        .node text {
            font-family: 'Source Code Pro', monospace;
            font-size: 9px;
            fill: var(--text-primary);
            pointer-events: none;
        }
        
        .link {
            stroke: #333;
            stroke-width: 2px;
            stroke-opacity: 0.8;
        }
        
        #graph-container {
            cursor: grab;
        }
        
        #graph-container:active {
            cursor: grabbing;
        }
        
        /* Event Log */
        .event-log {
            max-height: 250px;
            overflow-y: auto;
        }
        
        .event {
            padding: 10px 20px;
            border-bottom: 1px solid var(--bg-tertiary);
            font-size: 0.85rem;
        }
        
        .event:last-child {
            border-bottom: none;
        }
        
        .event-header {
            display: flex;
            justify-content: space-between;
            margin-bottom: 4px;
        }
        
        .event-file {
            font-family: 'Source Code Pro', monospace;
            font-weight: 500;
        }
        
        .event-time {
            color: var(--text-muted);
            font-family: 'Source Code Pro', monospace;
            font-size: 0.8rem;
        }
        
        .event-message {
            color: var(--text-secondary);
        }
        
        .event.error .event-file { color: var(--accent-red); }
        .event.success .event-file { color: var(--accent-green); }
        
        /* Empty State */
        .empty-state {
            padding: 40px 20px;
            text-align: center;
            color: var(--text-muted);
        }
        
        /* Footer */
        footer {
            text-align: center;
            padding: 24px;
            color: var(--text-muted);
            font-size: 0.85rem;
            border-top: 1px solid var(--border-color);
            margin-top: 40px;
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>Rustify</h1>
            <p class="subtitle">Automatic C/C++ to Rust Translation</p>
        </header>
        
        <div class="status-section">
            <div class="status-indicator">
                <div class="status-dot" id="status-dot"></div>
                <span class="status-text" id="status-text">Idle</span>
            </div>
            <div class="current-file" id="current-file">—</div>
        </div>
        
        <div class="stats-grid">
            <div class="stat-card">
                <div class="stat-value" id="progress-value">0%</div>
                <div class="stat-label">Progress</div>
            </div>
            <div class="stat-card">
                <div class="stat-value" id="files-value">0/0</div>
                <div class="stat-label">Files</div>
            </div>
            <div class="stat-card">
                <div class="stat-value success" id="success-value">0</div>
                <div class="stat-label">Completed</div>
            </div>
            <div class="stat-card">
                <div class="stat-value error" id="error-value">0</div>
                <div class="stat-label">Errors</div>
            </div>
        </div>
        
        <div class="progress-section">
            <div class="section-title">Translation Progress</div>
            <div class="progress-container">
                <div class="progress-bar" id="progress-bar" style="width: 0%"></div>
                <div class="progress-text" id="progress-text">0%</div>
            </div>
        </div>
        
        <div class="main-grid">
            <div class="panel">
                <div class="panel-header">Translation Queue</div>
                <div class="panel-body">
                    <div class="file-list" id="file-list">
                        <div class="empty-state">No files in queue</div>
                    </div>
                </div>
            </div>
            
            <div class="panel graph-panel">
                <div class="panel-header">Dependency Graph</div>
                <div class="panel-body">
                    <div id="graph-container"></div>
                </div>
            </div>
        </div>
        
        <div class="panel">
            <div class="panel-header">Event Log</div>
            <div class="panel-body">
                <div class="event-log" id="event-log">
                    <div class="empty-state">Waiting for events...</div>
                </div>
            </div>
        </div>
        
        <footer>
            Rustify — C/C++ to Rust Translation Tool
        </footer>
    </div>
    
    <script>
        // Fetch status
        async function fetchStatus() {
            try {
                const res = await fetch('/api/status');
                const data = await res.json();
                updateUI(data);
            } catch (e) {
                console.error('Status fetch failed:', e);
            }
        }
        
        function updateUI(data) {
            const statusDot = document.getElementById('status-dot');
            const statusText = document.getElementById('status-text');
            const currentFile = document.getElementById('current-file');
            
            const status = data.status || 'Idle';
            statusText.textContent = status;
            
            statusDot.className = 'status-dot';
            if (status.toLowerCase().includes('translating') || status.toLowerCase().includes('running')) {
                statusDot.classList.add('running');
            } else if (status.toLowerCase().includes('error')) {
                statusDot.classList.add('error');
            } else if (status.toLowerCase().includes('done') || status.toLowerCase().includes('complete')) {
                statusDot.classList.add('done');
            }
            
            currentFile.textContent = data.current_file || '—';
            
            const progress = Math.round((data.progress || 0) * 100);
            document.getElementById('progress-value').textContent = progress + '%';
            document.getElementById('progress-bar').style.width = progress + '%';
            document.getElementById('progress-text').textContent = progress + '%';
            
            document.getElementById('files-value').textContent = 
                `${data.files_done || 0}/${data.files_total || 0}`;
            document.getElementById('success-value').textContent = data.files_done || 0;
            document.getElementById('error-value').textContent = data.error_count || 0;
            
            if (data.tasks && data.tasks.length > 0) {
                updateFileList(data.tasks);
            }
        }
        
        function updateFileList(taskList) {
            const container = document.getElementById('file-list');
            
            if (!taskList || taskList.length === 0) {
                container.innerHTML = '<div class="empty-state">No files in queue</div>';
                return;
            }
            
            container.innerHTML = taskList.map(task => {
                const status = task.status || 'pending';
                return `
                    <div class="file-item">
                        <div class="file-name">
                            <span class="file-icon ${status}"></span>
                            <span>${task.name}</span>
                        </div>
                        <span class="file-status ${status}">${status}</span>
                    </div>
                `;
            }).join('');
        }
        
        // SSE
        function connectSSE() {
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
            
            eventSource.onerror = function() {
                setTimeout(connectSSE, 3000);
            };
        }
        
        function addEventToLog(event) {
            const log = document.getElementById('event-log');
            const emptyState = log.querySelector('.empty-state');
            if (emptyState) emptyState.remove();
            
            const eventClass = event.type === 'error' ? 'error' : 
                              event.type === 'complete' ? 'success' : '';
            
            const div = document.createElement('div');
            div.className = `event ${eventClass}`;
            div.innerHTML = `
                <div class="event-header">
                    <span class="event-file">${event.task_name}</span>
                    <span class="event-time">${new Date(event.timestamp).toLocaleTimeString()}</span>
                </div>
                <div class="event-message">${event.message}</div>
            `;
            
            log.insertBefore(div, log.firstChild);
            while (log.children.length > 50) {
                log.removeChild(log.lastChild);
            }
        }
        
        // Graph
        async function initGraph() {
            try {
                const res = await fetch('/api/graph');
                const data = await res.json();
                if (data.nodes && data.nodes.length > 0) {
                    drawGraph(data);
                }
            } catch (e) {
                console.error('Graph fetch failed:', e);
            }
        }
        
        function drawGraph(data) {
            const container = document.getElementById('graph-container');
            container.innerHTML = '';
            
            const width = container.offsetWidth || 500;
            const height = container.offsetHeight || 350;
            
            if (!data.nodes.length) return;
            
            // Calculate optimal layout based on node count
            const nodeCount = data.nodes.length;
            const spacing = Math.min(50, Math.max(20, 300 / nodeCount));
            
            const svg = d3.select('#graph-container')
                .append('svg')
                .attr('width', width)
                .attr('height', height)
                .attr('viewBox', [0, 0, width, height]);
            
            // Add zoom behavior
            const g = svg.append('g');
            
            svg.call(d3.zoom()
                .scaleExtent([0.1, 4])
                .on('zoom', (event) => {
                    g.attr('transform', event.transform);
                }));
            
            // Tighter force simulation for compact layout
            const simulation = d3.forceSimulation(data.nodes)
                .force('link', d3.forceLink(data.edges).id(d => d.id).distance(spacing))
                .force('charge', d3.forceManyBody().strength(-30))
                .force('center', d3.forceCenter(width / 2, height / 2))
                .force('collision', d3.forceCollide().radius(15))
                .force('x', d3.forceX(width / 2).strength(0.1))
                .force('y', d3.forceY(height / 2).strength(0.1));
            
            const link = g.append('g')
                .selectAll('line')
                .data(data.edges)
                .join('line')
                .attr('class', 'link');
            
            const node = g.append('g')
                .selectAll('g')
                .data(data.nodes)
                .join('g')
                .attr('class', 'node')
                .call(d3.drag()
                    .on('start', dragstarted)
                    .on('drag', dragged)
                    .on('end', dragended));
            
            const colors = {
                'pending': '#888888',
                'translating': '#2563eb',
                'done': '#16a34a',
                'error': '#dc2626'
            };
            
            node.append('circle')
                .attr('r', 5)
                .attr('fill', d => colors[d.status] || colors.pending);
            
            node.append('text')
                .attr('dx', 8)
                .attr('dy', 3)
                .style('font-size', '9px')
                .text(d => d.name.length > 15 ? d.name.slice(0, 12) + '...' : d.name);
            
            // Auto-fit after simulation settles
            simulation.on('tick', () => {
                link
                    .attr('x1', d => d.source.x)
                    .attr('y1', d => d.source.y)
                    .attr('x2', d => d.target.x)
                    .attr('y2', d => d.target.y);
                
                node.attr('transform', d => `translate(${d.x},${d.y})`);
            });
            
            // Auto zoom to fit all nodes after simulation ends
            simulation.on('end', () => {
                const bounds = g.node().getBBox();
                const fullWidth = bounds.width;
                const fullHeight = bounds.height;
                const midX = bounds.x + fullWidth / 2;
                const midY = bounds.y + fullHeight / 2;
                
                if (fullWidth > 0 && fullHeight > 0) {
                    const scale = 0.85 / Math.max(fullWidth / width, fullHeight / height);
                    const translate = [width / 2 - scale * midX, height / 2 - scale * midY];
                    
                    svg.transition()
                        .duration(500)
                        .call(d3.zoom().transform, d3.zoomIdentity
                            .translate(translate[0], translate[1])
                            .scale(scale));
                }
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
        
        // Init
        fetchStatus();
        initGraph();
        connectSSE();
        setInterval(fetchStatus, 2000);
        // Don't auto-refresh graph to preserve zoom/pan state
        // User can manually refresh by reloading the page
    </script>
</body>
</html>
'''
