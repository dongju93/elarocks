const { ApolloServer } = require("apollo-server");
const typeDefs = require("./schema/typeDefs");
const resolvers = require("./schema/resolvers");
const { db, fetchKey } = require("./db");
const { fetchDataBasedOnTime } = require("./fetchData");

// rocksdb open
db.open((err) => {
    if (err) throw err;

    const server = new ApolloServer({
        typeDefs,
        resolvers,
    });

    server.listen().then(({ url }) => {
        console.log(`🚀 Server ready at ${url}`);
    });
});
