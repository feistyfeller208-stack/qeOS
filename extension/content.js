// qeOS Content Script
console.log('🔮 qeOS field active on this page');

let currentSplit = null;
const QEOS_CORE = 'http://localhost:3030';

// Measure page engagement
function measurePage() {
  return {
    url: window.location.hostname,
    path: window.location.pathname,
    title: document.title,
    time_on_page: performance.now() / 1000,
    scroll_depth: (window.scrollY + window.innerHeight) / document.documentElement.scrollHeight,
    word_count: document.body.innerText.split(/\s+/).length,
    timestamp: Date.now()
  };
}

// Get or create split
async function getSplit() {
  const context = window.location.hostname;
  
  try {
    const response = await fetch(`${QEOS_CORE}/split`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ context })
    });
    
    currentSplit = await response.json();
    
    // Query probability for this context
    const queryResponse = await fetch(`${QEOS_CORE}/query`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ context })
    });
    
    const query = await queryResponse.json();
    
    // Add subtle visual indicator
    addFieldIndicator(query.probability);
    
    return { split: currentSplit, field: query };
  } catch (error) {
    console.log('qeOS core not running');
    return null;
  }
}

// Add subtle UI element
function addFieldIndicator(probability) {
  const indicator = document.createElement('div');
  indicator.id = 'qeos-indicator';
  indicator.style.cssText = `
    position: fixed;
    bottom: 10px;
    right: 10px;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: ${getColorForProbability(probability)};
    opacity: 0.5;
    z-index: 10000;
    pointer-events: none;
    transition: opacity 0.3s;
  `;
  
  if (!document.getElementById('qeos-indicator')) {
    document.body.appendChild(indicator);
  }
}

function getColorForProbability(p) {
  if (p > 0.66) return '#00ff00';
  if (p > 0.33) return '#ffff00';
  return '#ff0000';
}

// Initialize
getSplit();

// Refresh on navigation
let lastUrl = location.href;
new MutationObserver(() => {
  const url = location.href;
  if (url !== lastUrl) {
    lastUrl = url;
    getSplit();
  }
}).observe(document, { subtree: true, childList: true });
