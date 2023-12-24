const { ApolloServer } = require("apollo-server");
const typeDefs = require("./schema/typeDefs"); // Adjust the path as necessary
const { resolvers } = require("./schema/resolvers"); // Adjust the path as necessary

try {
    const server = new ApolloServer({ typeDefs, resolvers });

    server.listen().then(({ url }) => {
        console.log(`🚀 Server ready at ${url}`);
    });
} catch (error) {
    console.error("Error initializing Apollo Server:", error.message);
}
