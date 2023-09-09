const { ApolloServer, gql } = require("apollo-server");
const RocksDB = require("rocksdb");
const path = require("path");
const dbPath = path.join(__dirname, "../db");
const db = RocksDB(dbPath);

function generateKeysInRange(event, start, end) {
    let currentDateTime = new Date(start);
    const endDateTime = new Date(end);
    const keys = [];

    while (currentDateTime <= endDateTime) {
        // Convert to ISO string and modify to match the desired key format
        const isoString = currentDateTime.toISOString();
        const modifiedKey = isoString
            .replace("T", " ")
            .replace(/(\.\d{3})Z$/, "$100000");

        const key = `${event}_${modifiedKey}`;
        keys.push(key);

        // Increment by some interval (e.g., 1 second). Adjust based on your key frequency.
        currentDateTime.setSeconds(currentDateTime.getSeconds() + 1);
    }

    return keys;
}

// schema
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

    input DateTimeRange {
        start: String!
        end: String!
    }

    input SysmonFilter {
        event: String!
        datetime: DateTimeRange!
    }

    type Query {
        sysmon(filter: SysmonFilter!): SysmonResponse
    }
`;

// core
const resolvers = {
    Query: {
        sysmon: (parent, { filter }, context, info) => {
            return new Promise((resolve, reject) => {
                const { start, end } = filter.datetime;
                const keysInRange = generateKeysInRange(
                    filter.event,
                    start,
                    end
                );

                const results = [];
                let fetchedCount = 0;

                keysInRange.forEach((key) => {
                    db.get(Buffer.from(key), (err, value) => {
                        fetchedCount++;
                        // query key print
                        // console.log(`Fetching key: ${key}`);
                        if (err) {
                            // errors print
                            // console.error(`Error fetching key ${key}: ${err.message}`);
                            if (err.message !== "NotFound: ") {
                                return reject(err);
                            }
                        } else if (value) {
                            const parsedValue = JSON.parse(
                                value.toString("utf-8")
                            );
                            results.push(parsedValue);
                        }

                        // Total result print
                        if (fetchedCount === keysInRange.length) {
                            console.log(
                                `Total results found: ${results.length}`
                            );
                            resolve({ SysmonNode: results });
                        }
                    });
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
