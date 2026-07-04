const DATASET = [
  {
    Section: 1,
    section_title: 'Title and extent of operation of the Code',
    section_desc: 'This Act shall be called the Indian Penal Code, and shall extend to the whole of India except the State of Jammu and Kashmir.'
  },
  {
    Section: 2,
    section_title: 'Punishment of offences committed within India',
    section_desc: 'Every person shall be liable to punishment under this Code and not otherwise for every act or omission contrary to the provisions thereof, of which he shall be guilty within India.'
  },
  {
    Section: 97,
    section_title: 'Right of private defence of the body and of property',
    section_desc: 'Every person has a right, subject to the restrictions contained in section 99, to defend his own body, and the body of any other person, against any offence affecting the human body.'
  },
  {
    Section: 104,
    section_title: 'When such right to causing any harm other than death',
    section_desc: 'If the offence, the committing of which, or the attempting to commit which occasions the exercise of the right of private defence, be theft, mischief, or criminal trespass, not of any of the descriptions enumerated in the last preceding section, that right does not extend to the voluntary causing of death.'
  },
  {
    Section: 356,
    section_title: 'Assault or criminal force in attempt to commit theft of property carried by a person',
    section_desc: 'Whoever assaults or uses criminal force to any person, in attempting to commit theft on any property which that person is then wearing or carrying shall be punished with imprisonment of either description for a term which may extend to two years, or with fine, or with both.'
  },
  {
    Section: 378,
    section_title: 'Theft',
    section_desc: "Whoever, intending to take dishonestly any movable property out of the possession of any person without that person's consent, moves that property in order to such taking, is said to commit theft."
  },
  {
    Section: 379,
    section_title: 'Punishment for theft',
    section_desc: 'Whoever commits theft shall be punished with imprisonment of either description for a term which may extend to three years, or with fine, or with both.'
  },
  {
    Section: 380,
    section_title: 'Theft in dwelling house, etc.',
    section_desc: 'Whoever commits theft in any building, tent or vessel, which building, tent or vessel is used as a human dwelling, or used for the custody of property, shall be punished with imprisonment of either description for a term which may extend to seven years, and shall also be liable to fine.'
  }
];

exports.handler = async function (event) {
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
      const title = law.section_title || '';
      const description = law.section_desc || '';
      const haystack = `${title} ${description}`.toLowerCase();
      return haystack.includes(normalizedQuery);
    })
    .filter((law) => {
      const key = law.Section || law.section || law.title || '';
      if (seen.has(key)) {
        return false;
      }
      seen.add(key);
      return true;
    })
    .slice(0, 8)
    .map((law) => ({
      section: law.Section,
      title: law.section_title,
      description: law.section_desc,
      similarity: 0.92
    }));

  return {
    statusCode: 200,
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(results)
  };
};
