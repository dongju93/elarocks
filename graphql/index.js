const RocksDB = require("rocksdb");
const path = require("path");
const express = require("express");
const { ApolloServer, gql } = require("apollo-server-express");

const dbPath = path.join(__dirname, "../db");
const db = RocksDB(dbPath);

// GraphQL Schema
const typeDefs = gql`
    type Query {
        getValue(key: String!): String
        getKeys(prefix: String): [String]
    }

    type Mutation {
        putValue(key: String!, value: String!): Boolean
        deleteValue(key: String!): Boolean
    }
`;

// Resolvers
// Resolvers
const resolvers = {
    Query: {
        getValue: async (_, { key }) => {
            return new Promise((resolve, reject) => {
                db.get(key, (err, value) => {
                    if (err) {
                        if (err.notFound) {
                            resolve(null);
                            return;
                        }
                        console.error(err); // Log the error for debugging
                        resolve(null); // Resolve with null in case of an error
                        return;
                    }
                    resolve(value ? value.toString() : null);
                });
            });
        },
        getKeys: async (_, { prefix = "" }) => {
            const keys = [];
            return new Promise((resolve, reject) => {
                db.createKeyStream({
                    gte: prefix,
                    lte: `${prefix}\xff`,
                })
                    .on("data", (key) => {
                        keys.push(key.toString());
                    })
                    .on("error", (err) => {
                        reject(err);
                    })
                    .on("end", () => {
                        resolve(keys);
                    });
            });
        },
    },
    Mutation: {
        putValue: async (_, { key, value }) => {
            return new Promise((resolve, reject) => {
                db.put(key, value, (err) => {
                    if (err) {
                        reject(false);
                        return;
                    }
                    resolve(true);
                });
            });
        },
        deleteValue: async (_, { key }) => {
            return new Promise((resolve, reject) => {
                db.del(key, (err) => {
                    if (err) {
                        reject(false);
                        return;
                    }
                    resolve(true);
                });
            });
        },
    },
};

// Initialize Apollo Server
const server = new ApolloServer({ typeDefs, resolvers });

const app = express();

RocksDB.repair(dbPath, (err) => {
    if (err) {
        console.log("Repair failed:", err);
        return;
    }

    console.log("Repair successful.");
    // Open the database and start the server
    db.open(async (err) => {
        if (err) {
            console.error("Failed to open the RocksDB:", err);
            return;
        }

        // Make sure to start the Apollo Server before applying middleware
        await server.start();

        server.applyMiddleware({ app });

        app.listen({ port: 4000 }, () =>
            console.log(
                `🚀 Server ready at http://localhost:4000${server.graphqlPath}`
            )
        );
    });
});
