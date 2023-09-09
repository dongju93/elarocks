const { ApolloServer, gql } = require("apollo-server");
const RocksDB = require("rocksdb");
const path = require("path");
const dbPath = path.join(__dirname, "../db");
const db = RocksDB(dbPath);
const async = require("async");

function generateKeysInRange(event, start, end) {
    const currentDateTime = new Date(start);
    const endDateTime = new Date(end);
    const keys = [];

    while (currentDateTime <= endDateTime) {
        for (let i = 0; i < 10; i++) {
            const isoString = currentDateTime.toISOString();
            const subMillis = String(i).padStart(5, "0"); // This will generate 00000, 00001, 00002, ...
            const modifiedKey = isoString
                .replace("T", " ")
                .replace(/(\.\d{3})Z$/, `$1${subMillis}`);
            const key = `${event}_${modifiedKey}`;
            keys.push(key);
        }

        // Increment the main milliseconds part by 1
        currentDateTime.setMilliseconds(currentDateTime.getMilliseconds() + 1);
    }

    return keys;
}

// schema
const typeDefs = gql`
    type SysmonResponse {
        SysmonNode: [SysmonNode!]
        totalCount: Int
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
                const initialKeys = generateKeysInRange(
                    filter.event,
                    start,
                    end
                );

                fetchKeys(initialKeys).then((results) => {
                    resolve({
                        SysmonNode: results,
                        totalCount: results.length,
                    });
                });
            });
        },
    },
};

function fetchKeys(keys) {
    return new Promise((resolve, reject) => {
        async.mapLimit(
            keys,
            100,
            (key, callback) => {
                db.get(Buffer.from(key), (err, value) => {
                    console.log("key is: " + `${key}`);
                    if (err) {
                        if (err.message === "NotFound: ") {
                            // console.error("error fetching: "+`${key}`)
                            callback(null, null); // Resolve with null for NotFound errors
                        } else {
                            callback(err);
                        }
                    } else {
                        const parsedValue = JSON.parse(value.toString("utf-8"));
                        callback(null, parsedValue);
                    }
                });
            },
            (err, results) => {
                if (err) {
                    reject(err);
                } else {
                    const filteredResults = results.filter((result) => result);
                    resolve(filteredResults);
                }
            }
        );
    });
}

db.open((err) => {
    if (err) throw err;

    const server = new ApolloServer({ typeDefs, resolvers });

    server.listen().then(({ url }) => {
        console.log(`🚀 Server ready at ${url}`);
    });
});
