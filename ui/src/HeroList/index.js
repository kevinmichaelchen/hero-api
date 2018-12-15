import React from 'react';
import { Link } from 'react-router-dom';

const Hero = ({ hero }) => (
  <div>
    <Link to={`/hero/${hero.id}`}>
      <pre>{JSON.stringify(hero)}</pre>
    </Link>
  </div>
);

export default ({ heroes }) => (
  <div>
    <h1>Superheroes</h1>
    {heroes.map((h, i) => (
      <Hero hero={h} key={i} />
    ))}
  </div>
);
