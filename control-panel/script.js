const QEOS_CORE = 'http://localhost:3030';

async function checkConnection() {
  const statusEl = document.getElementById('connection-status');
  
  try {
    const response = await fetch(`${QEOS_CORE}/health`);
    const data = await response.json();
    
    statusEl.className = 'connected';
    statusEl.textContent = '✅ Connected to qeOS core';
    return true;
  } catch (error) {
    statusEl.className = 'disconnected';
    statusEl.textContent = '❌ qeOS core not running';
    return false;
  }
}

async function loadSource() {
  try {
    const response = await fetch(`${QEOS_CORE}/source`);
    const data = await response.json();
    document.getElementById('source-id').textContent = data.id;
  } catch (error) {
    document.getElementById('source-id').textContent = 'Error loading source';
  }
}

async function loadRules() {
  // This would come from the core API
  const rulesList = document.getElementById('rules-list');
  rulesList.innerHTML = '<p>Sample rules (API coming soon)</p>';
  
  // Mock rules for display
  const mockRules = [
    { name: 'News Coherence', type: 'Direct', strength: 0.65 },
    { name: 'Crypto-Gold Hedge', type: 'Inverse', strength: 0.7 },
    { name: 'Focus Cascade', type: 'Cascade', strength: 0.5 }
  ];
  
  rulesList.innerHTML = mockRules.map(rule => `
    <div class="rule-item">
      <div>
        <div class="rule-name">${rule.name}</div>
        <div class="rule-type">${rule.type} · ${rule.strength} strength</div>
      </div>
      <div class="rule-controls">
        <button onclick="toggleRule('${rule.name}')">Toggle</button>
        <button onclick="editRule('${rule.name}')">Edit</button>
      </div>
    </div>
  `).join('');
}

// Mock functions
window.toggleRule = (name) => {
  console.log('Toggle rule:', name);
};

window.editRule = (name) => {
  console.log('Edit rule:', name);
};

document.getElementById('reset-source').addEventListener('click', async () => {
  if (confirm('Reset source? All correlations will be lost.')) {
    // Call reset API
    alert('Source reset (mock)');
  }
});

document.getElementById('add-rule').addEventListener('click', () => {
  alert('Rule editor coming soon');
});

document.getElementById('browse-rules').addEventListener('click', (e) => {
  e.preventDefault();
  alert('Rule directory coming soon at github.com/qeos/rules');
});

// Initialize
async function init() {
  const connected = await checkConnection();
  if (connected) {
    await loadSource();
    await loadRules();
    
    // Mock field stats
    document.getElementById('split-count').textContent = '42';
    document.getElementById('correlation-count').textContent = '12';
    document.getElementById('last-updated').textContent = new Date().toLocaleTimeString();
  }
}

init();
