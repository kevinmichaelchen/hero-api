import React from 'react';

const Hero = React.memo(({ hero }) => (
  <div>
    <pre>{JSON.stringify(hero)}</pre>
  </div>
));

const HeroList = React.memo(({ heroes }) => (
  <div>
    <h1>Superheroes</h1>
    {heroes.map((h, i) => (
      <Hero hero={h} key={i} />
    ))}
  </div>
));

export default HeroList;
