
#!/bin/bash

# FM Synthesizer WebAssembly Build Script

set -e  # Exit on error

echo "🎵 FM Synthesizer WASM Builder"
echo "=============================="

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed!"
    echo "Install it with: cargo install wasm-pack"
    exit 1
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf pkg/
rm -rf target/release/

# Build native binary
echo "🔨 Building native binary..."
cargo build --release

# Build the WebAssembly module
echo "🔨 Building WebAssembly module..."
wasm-pack build --target web --out-dir pkg_temp # Build to a temporary pkg directory

# Create the docs directory
echo "📁 Creating docs directory for GitHub Pages..."
rm -rf docs/
mkdir -p docs/pkg

# Move WASM build output to docs/pkg
echo "📦 Moving WASM build to docs/pkg..."
mv pkg_temp/* docs/pkg/
rm -rf pkg_temp # Clean up temporary pkg directory

# Create index.html in docs directory
echo "📝 Creating docs/index.html for WASM integration..."
cat > docs/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>FM Synthesizer Web CLI</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            background-color: #1a1a1a;
            color: #00ff00;
            font-family: 'Courier New', monospace;
            height: 100vh;
            display: flex;
            flex-direction: column;
        }

        #terminal {
            flex: 1;
            padding: 20px;
            overflow-y: auto;
            background-color: #0a0a0a;
            border: 2px solid #00ff00;
            margin: 10px;
            border-radius: 5px;
            box-shadow: 0 0 20px rgba(0, 255, 0, 0.3);
        }

        .output-line {
            margin: 2px 0;
            white-space: pre-wrap;
        }

        .command-line {
            color: #ffff00;
        }

        .error {
            color: #ff4444;
        }

        .success {
            color: #44ff44;
        }

        .info {
            color: #4444ff;
        }

        #input-container {
            display: flex;
            align-items: center;
            padding: 10px 20px;
            background-color: #0a0a0a;
            border-top: 2px solid #00ff00;
        }

        #prompt {
            color: #00ff00;
            margin-right: 10px;
        }

        #command-input {
            flex: 1;
            background-color: transparent;
            border: none;
            color: #00ff00;
            font-family: 'Courier New', monospace;
            font-size: 16px;
            outline: none;
        }

        .loading {
            color: #ffff00;
            animation: blink 1s infinite;
        }

        @keyframes blink {
            0%, 50% { opacity: 1; }
            51%, 100% { opacity: 0; }
        }

        .header {
            text-align: center;
            margin: 20px 0;
            color: #00ff00;
            text-shadow: 0 0 10px rgba(0, 255, 0, 0.5);
        }

        .ascii-art {
            font-size: 12px;
            line-height: 12px;
            margin: 10px 0;
        }
    </style>
</head>
<body>
    <div id="terminal">
        <div class="header">
            <pre class="ascii-art">
 _____ __  __   ____             _   _     
|  ___|  \/  | / ___| _   _ _ __| |_| |__  
| |_  | |\/| | \___ \| | | | '_ \ __| '_ \ 
|  _| | |  | |  ___) | |_| | | | | |_| | | |
|_|   |_|  |_| |____/ \__, |_| |_|\__|_| |_|
                      |___/                  
            </pre>
            <h2>WebAssembly FM Synthesizer</h2>
        </div>
        <div id="output">
            <div class="output-line loading">Loading WebAssembly module...</div>
        </div>
    </div>
    <div id="input-container">
        <span id="prompt">&gt;</span>
        <input type="text" id="command-input" placeholder="Enter command..." disabled>
    </div>

    <script type="module">
        import init, { WebFMSynth } from './pkg/fm_synth.js';

        let synth = null;
        const output = document.getElementById('output');
        const input = document.getElementById('command-input');
        
        function addOutput(text, className = '') {
            const line = document.createElement('div');
            line.className = 'output-line' + (className ? ' ' + className : '');
            line.textContent = text;
            output.appendChild(line);
            output.parentElement.scrollTop = output.parentElement.scrollHeight;
        }

        function clearOutput() {
            output.innerHTML = '';
        }

        function printHelp() {
            addOutput("Commands:", 'info');
            addOutput("  list presets  - Show all available presets");
            addOutput("  list melodies - Show all available melodies");
            addOutput("  play <preset> <melody> - Play a melody with a preset");
            addOutput("  demo - Play a quick demo");
            addOutput("  clear - Clear the terminal");
            addOutput("  help - Show this menu");
            addOutput("");
        }

        async function processCommand(command) {
            const parts = command.trim().split(/\s+/);
            if (parts.length === 0 || parts[0] === '') return;

            addOutput('> ' + command, 'command-line');

            try {
                switch (parts[0].toLowerCase()) {
                    case 'help':
                        printHelp();
                        break;

                    case 'list':
                        if (parts[1] === 'presets') {
                            addOutput("Available Presets:", 'info');
                            const presets = synth.list_presets();
                            presets.split('\n').forEach(line => addOutput('  ' + line));
                        } else if (parts[1] === 'melodies') {
                            addOutput("Available Melodies:", 'info');
                            const melodies = synth.list_melodies();
                            melodies.split('\n').forEach(line => addOutput('  ' + line));
                        } else {
                            addOutput("Usage: list <presets|melodies>", 'error');
                        }
                        break;

                    case 'play':
                        if (parts.length >= 3) {
                            const presetId = parseInt(parts[1]) - 1;
                            const melodyId = parseInt(parts[2]) - 1;
                            
                            if (isNaN(presetId) || isNaN(melodyId)) {
                                addOutput("Please use numbers for preset and melody selection", 'error');
                                addOutput("Example: play 1 3", 'info');
                            } else {
                                addOutput("Playing melody...", 'success');
                                await synth.play_melody(presetId, melodyId);
                                addOutput("Done!", 'success');
                            }
                        } else {
                            addOutput("Usage: play <preset_number> <melody_number>", 'error');
                            addOutput("Example: play 1 3", 'info');
                        }
                        break;

                    case 'demo':
                        addOutput("Playing demo...", 'success');
                        for (let i = 0; i < 3; i++) {
                            addOutput(`  Playing preset ${i + 1}...`, 'info');
                            await synth.play_melody(i, 4); // Chromatic scale
                        }
                        addOutput("Demo complete!", 'success');
                        break;

                    case 'clear':
                        clearOutput();
                        break;

                    default:
                        addOutput(`Unknown command: ${parts[0]}`, 'error');
                        addOutput("Type 'help' for available commands.", 'info');
                }
            } catch (error) {
                addOutput(`Error: ${error}`, 'error');
            }
            
            addOutput("");
        }

        // Initialize
        async function initialize() {
            try {
                await init();
                synth = new WebFMSynth();
                
                clearOutput();
                addOutput("Welcome to the FM Synthesizer Web CLI!", 'success');
                addOutput("Type 'help' for available commands.");
                addOutput("");
                
                input.disabled = false;
                input.focus();
                
                input.addEventListener('keypress', async (e) => {
                    if (e.key === 'Enter') {
                        const command = input.value;
                        input.value = '';
                        await processCommand(command);
                    }
                });
            } catch (error) {
                addOutput(`Failed to initialize: ${error}`, 'error');
            }
        }

        initialize();
    </script>
</body>
</html>
EOF

echo "✅ Build complete for GitHub Pages!"
echo ""
echo "📁 Output files in docs/ directory:"
echo "  - docs/pkg/         (WASM module and bindings)"
echo "  - docs/index.html   (Web interface)"
echo ""
echo "🚀 To run locally before pushing to GitHub Pages:"
echo "  1. Run this build script: ./build.sh"
echo "  2. Start a local server in the project root:"
echo "     python3 -m http.server 8000"
echo "  3. Open http://localhost:8000/docs/index.html"
echo ""
echo "🎹 Happy synthesizing!"
