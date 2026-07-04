const searchForm = document.getElementById('searchForm');
const searchInput = document.getElementById('searchInput');
const loadingDiv = document.getElementById('loading');
const errorDiv = document.getElementById('error');
const resultsDiv = document.getElementById('results');
const resultsListDiv = document.getElementById('resultsList');
const noResultsDiv = document.getElementById('noResults');

searchForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    const query = searchInput.value.trim();

    if (!query) return;

    await performSearch(query);
});

async function performSearch(query) {
    // Clear previous results
    hideAllResults();
    showLoading();

    try {
        const response = await fetch(`/.netlify/functions/search?query=${encodeURIComponent(query)}`);

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const results = await response.json();

        hideLoading();

        if (results.length === 0) {
            showNoResults();
        } else {
            displayResults(results);
        }
    } catch (error) {
        hideLoading();
        showError(`Error searching: ${error.message}`);
        console.error('Search error:', error);
    }
}

function displayResults(results) {
    resultsListDiv.innerHTML = '';

    results.forEach((result, index) => {
        const card = createResultCard(result);
        resultsListDiv.appendChild(card);
    });

    resultsDiv.classList.remove('hidden');
}

function createResultCard(result) {
    const card = document.createElement('div');
    card.className = 'result-card';
    
    const similarityPercent = (result.similarity * 100).toFixed(1);
    
    card.innerHTML = `
        <span class="similarity-badge">Match: ${similarityPercent}%</span>
        <div class="section">Section ${result.section}</div>
        <h3 class="result-title">${escapeHtml(result.title)}</h3>
        <p class="result-description">${escapeHtml(result.description)}</p>
    `;

    return card;
}

function escapeHtml(text) {
    const map = {
        '&': '&amp;',
        '<': '&lt;',
        '>': '&gt;',
        '"': '&quot;',
        "'": '&#039;'
    };
    return text.replace(/[&<>"']/g, (m) => map[m]);
}

function showLoading() {
    loadingDiv.classList.remove('hidden');
}

function hideLoading() {
    loadingDiv.classList.add('hidden');
}

function showError(message) {
    errorDiv.textContent = message;
    errorDiv.classList.remove('hidden');
}

function showNoResults() {
    noResultsDiv.classList.remove('hidden');
}

function hideAllResults() {
    resultsDiv.classList.add('hidden');
    noResultsDiv.classList.add('hidden');
    errorDiv.classList.add('hidden');
}

// Focus on search input on load
document.addEventListener('DOMContentLoaded', () => {
    searchInput.focus();
});
