const fs = require('fs');
const path = require('path');

function readJson(pathCandidates) {
  for (const candidate of pathCandidates) {
    try {
      if (fs.existsSync(candidate)) {
        const text = fs.readFileSync(candidate, 'utf8');
        return JSON.parse(text);
      }
    } catch (err) {
      console.error('Failed to read', candidate, err.message);
    }
  }
  return [];
}

function normalizeRecord(record) {
  return {
    section: record.Section || record.section || record.id || null,
    title: record.section_title || record.title || '',
    description: record.section_desc || record.description || ''
  };
}

exports.handler = async function (event) {
  const root = path.join(__dirname, '..', '..', 'static');
  const ipc = readJson([path.join(root, 'ipc.json')]);
  const embedded = readJson([path.join(root, 'embedded_ipc.json')]);
  const DATASET = [...ipc, ...embedded].map(normalizeRecord);
  const query = event.queryStringParameters && event.queryStringParameters.query ? event.queryStringParameters.query : '';

  if (!query) {
    return {
      statusCode: 200,
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify([])
    };
  }

  const normalizedQuery = query.toLowerCase();
  const seen = new Set();
  const results = DATASET
    .filter((law) => {
      const title = law.title || '';
      const description = law.description || '';
      const haystack = `${title} ${description}`.toLowerCase();
      return haystack.includes(normalizedQuery);
    })
    .filter((law) => {
      const key = law.section || law.title || '';
      if (seen.has(key)) {
        return false;
      }
      seen.add(key);
      return true;
    })
    .slice(0, 8)
    .map((law) => ({
      section: law.section,
      title: law.title,
      description: law.description,
      similarity: 0.92
    }));

  return {
    statusCode: 200,
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(results)
  };
};
