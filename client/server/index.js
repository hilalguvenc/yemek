// Require the framework and instantiate it
const fastify = require('fastify')({ logger: true });
const path = require('path');
const recipeMockData = require('./mock-data/recipes');

fastify.register(require('fastify-static'), {
  root: path.join(__dirname, '..'),
  prefix: '/static/',
});

fastify.get('/', async (request, reply) => {
  return reply.sendFile('index.html', path.join(__dirname, '../pages'));
});

fastify.get('/recipe/:id', async (request, reply) => {
  return reply.sendFile('recipe.html', path.join(__dirname, '../pages'));
});

fastify.get('/api/recipes/home', async (request, reply) => {
  return recipeMockData.map((recipe) => (
    {
      name: recipe.name,
      calories: recipe.calories,
      people: recipe.people,
      prep_minutes: recipe.prep_minutes,
      cook_minutes: recipe.cook_minutes,
      images: recipe.images
    }
  ))
})

// Run the server!
const start = async () => {
  try {
    await fastify.listen(3003)
  } catch (err) {
    fastify.log.error(err)
    process.exit(1)
  }
}
start()