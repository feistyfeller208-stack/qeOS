// qeOS Background Service Worker

const QEOS_CORE = 'http://localhost:3030';

// Check if core is running
async function checkCore() {
  try {
    const response = await fetch(`${QEOS_CORE}/health`);
    const data = await response.json();
    console.log('✅ qeOS core connected:', data);
    return true;
  } catch (error) {
    console.log('❌ qeOS core not running');
    return false;
  }
}

// Initialize
checkCore().then(isRunning => {
  if (isRunning) {
    chrome.storage.local.set({ qeosConnected: true });
  }
});

// Listen for tab updates
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === 'complete') {
    console.log('Tab updated:', tab.url);
  }
});
