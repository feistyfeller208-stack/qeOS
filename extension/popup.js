const QEOS_CORE = 'http://localhost:3030';

document.addEventListener('DOMContentLoaded', async () => {
  const statusDiv = document.getElementById('status');
  const fieldContainer = document.getElementById('fieldContainer');
  const probabilityDiv = document.getElementById('probability');
  const correlatedDiv = document.getElementById('correlated');
  const refreshBtn = document.getElementById('refresh');
  
  async function checkConnection() {
    try {
      const response = await fetch(`${QEOS_CORE}/health`);
      const data = await response.json();
      
      statusDiv.className = 'status connected';
      statusDiv.textContent = '✅ Connected to qeOS core';
      
      // Get current tab context
      const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
      const url = new URL(tab.url);
      const context = url.hostname;
      
      // Query field
      const queryResponse = await fetch(`${QEOS_CORE}/query`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ context })
      });
      
      const query = await queryResponse.json();
      
      fieldContainer.style.display = 'block';
      probabilityDiv.textContent = `${Math.round(query.probability * 100)}%`;
      
      if (query.correlated_contexts.length > 0) {
        correlatedDiv.innerHTML = `<strong>Correlated with:</strong> ${query.correlated_contexts.join(', ')}`;
      }
      
    } catch (error) {
      statusDiv.className = 'status disconnected';
      statusDiv.textContent = '❌ qeOS core not running';
      fieldContainer.style.display = 'none';
    }
  }
  
  refreshBtn.addEventListener('click', checkConnection);
  await checkConnection();
});
