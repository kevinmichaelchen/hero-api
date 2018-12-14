import React from 'react';

const Hero = React.memo(({ hero }) => (
  <div>
    <pre>{JSON.stringify(hero)}</pre>
  </div>
));

export default ({ heroes }) => (
  <div>
    <h1>Superheroes</h1>
    {heroes.map((h, i) => (
      <Hero hero={h} key={i} />
    ))}
  </div>
);
