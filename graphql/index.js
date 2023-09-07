const { ApolloServer, gql } = require("apollo-server");
const RocksDB = require("rocksdb");
const path = require("path");
const dbPath = path.join(__dirname, "../db");
const db = RocksDB(dbPath);

// schema
const typeDefs = gql`
    type KeyValue {
        key: String!
        value: String!
    }

    type Query {
        sysmon(key: String!): [KeyValue]
    }
`;

// simple resolvers
const resolvers = {
    Query: {
        sysmon: (parent, args, context, info) => {
            return new Promise((resolve, reject) => {
                const partial_timestamp = args.key;
                db.get(Buffer.from(partial_timestamp), (err, value) => {
                    if (err) {
                        return reject(err);
                    }

                    if (value) {
                        const key_str = partial_timestamp;
                        const value_str = value.toString("utf-8");
                        resolve([{ key: key_str, value: value_str }]);
                    } else {
                        resolve([]);
                    }
                });
            });
        },
    },
};

db.open((err) => {
    if (err) throw err;

    const server = new ApolloServer({ typeDefs, resolvers });

    server.listen().then(({ url }) => {
        console.log(`🚀 Server ready at ${url}`);
    });
});
