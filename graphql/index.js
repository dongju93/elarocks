const { ApolloServer, gql } = require("apollo-server");
const RocksDB = require("rocksdb");
const path = require("path");
const dbPath = path.join(__dirname, "../db");
const db = RocksDB(dbPath);

// Updated schema
const typeDefs = gql`
    type SysmonResponse {
        SysmonNode: [SysmonNode!]
    }

    type SysmonNode {
        agent_name: String!
        agent_id: String!
        event_action: String!
        event_type: String!
        utc_time: String!
        process_guid: String!
        process_id: Int!
        image: String!
        target_object: String!
        details: String!
        user: String!
    }

    input SysmonFilter {
        event: String!
        datetime: String!
    }

    type Query {
        sysmon(filter: SysmonFilter!): SysmonResponse
    }
`;

// Updated resolvers
const resolvers = {
    Query: {
        sysmon: (parent, { filter }, context, info) => {
            return new Promise((resolve, reject) => {
                const key = `${filter.event}_${filter.datetime}`;

                db.get(Buffer.from(key), (err, value) => {
                    if (err) {
                        return reject(err);
                    }

                    if (value) {
                        const parsedValue = JSON.parse(value.toString("utf-8"));
                        resolve({ SysmonNode: [parsedValue] });
                    } else {
                        resolve({ SysmonNode: [] });
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
