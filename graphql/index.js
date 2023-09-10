const { ApolloServer, gql } = require("apollo-server");
const RocksDB = require("rocksdb");
const path = require("path");
const dbPath = path.join(__dirname, "../db");
const db = RocksDB(dbPath);
const { Client } = require("pg");
require("dotenv").config();

// postgres connect
const client = new Client({
    host: process.env.DB_HOST,
    user: process.env.DB_USER,
    password: process.env.DB_PASSWORD,
    database: process.env.DB_NAME,
});
client.connect();

async function fetchDataBasedOnTime(start, end) {
    // modify utc format input for query
    start = start.replace("T", " ").replace("Z", "00000");
    end = end.replace("T", " ").replace("Z", "99999");
    // console.log(start+" and "+end)

    const query = process.env.SQL_QUERY_REG;

    try {
        const result = await client.query(query, [start, end]);
        return result.rows;
    } catch (error) {
        console.error("Error executing query", error.stack);
        return [];
    }
}

// graphql schema
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
            const postgresResults = await fetchDataBasedOnTime(start, end);

            const allResults = [];
            for (const row of postgresResults) {
                const key = `${filter.event}_${row.savedtime}`;
                const result = await fetchKey(key);
                if (result) {
                    allResults.push(result);
                }
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
