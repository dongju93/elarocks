const { ApolloServer, gql } = require("apollo-server");
const RocksDB = require("rocksdb");
const path = require("path");
const dbPath = path.join(__dirname, "../db");
const db = RocksDB(dbPath);

// ket generate with only .xxx00000
function generateKeyForTime(event, time, subMillis = 0) {
    const isoString = time.toISOString();
    const subMillisecondPart = String(subMillis).padStart(5, "0");
    const modifiedKey = isoString
        .replace("T", " ")
        .replace(/(\.\d{3})Z$/, `$1${subMillisecondPart}`);
    return `${event}_${modifiedKey}`;
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

const resolvers = {
    Query: {
        sysmon: async (parent, { filter }, context, info) => {
            const { start, end } = filter.datetime;
            let currentDateTime = new Date(start);
            const endDateTime = new Date(end);
            const allResults = [];

            while (currentDateTime <= endDateTime) {
                const key = generateKeyForTime(filter.event, currentDateTime);
                const result = await fetchKey(key);
                // console.log(`${key}`)

                // if key is not null
                if (result) {
                    allResults.push(result);
                    // .xxx00001 ~ .xxx00009 key generate
                    for (let i = 1; i < 10; i++) {
                        const subsequentKey = generateKeyForTime(
                            filter.event,
                            currentDateTime,
                            i
                        );
                        // console.log(`${subsequentKey}`)
                        const subsequentResult = await fetchKey(subsequentKey);
                        if (subsequentResult) {
                            allResults.push(subsequentResult);
                        }
                    }
                }

                // add ms
                currentDateTime.setMilliseconds(
                    currentDateTime.getMilliseconds() + 1
                );
            }

            return {
                SysmonNode: allResults,
                totalCount: allResults.length,
            };
        },
    },
};

function fetchKey(key) {
    return new Promise((resolve, reject) => {
        db.get(Buffer.from(key), (err, value) => {
            if (err) {
                if (err.message === "NotFound: ") {
                    resolve(null);
                } else {
                    reject(err);
                }
            } else {
                const parsedValue = JSON.parse(value.toString("utf-8"));
                resolve(parsedValue);
            }
        });
    });
}

db.open((err) => {
    if (err) throw err;

    const server = new ApolloServer({ typeDefs, resolvers });

    server.listen().then(({ url }) => {
        console.log(`🚀 Server ready at ${url}`);
    });
});
