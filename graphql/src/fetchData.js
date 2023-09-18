const { Client } = require("pg");
require("dotenv").config();

const client = new Client({
    host: process.env.DB_HOST,
    user: process.env.DB_USER,
    password: process.env.DB_PASSWORD,
    database: process.env.DB_NAME,
});
client.connect();

async function fetchDataBasedOnTime(event, start, end) {
    start = start.replace("T", " ").replace("Z", "00000");
    end = end.replace("T", " ").replace("Z", "99999");
    // console.log(start+" and "+end)

    let query;
    switch (event) {
        case "Registry value set":
            query = process.env.SQL_QUERY_REG;
            break;
        case "Process Create":
            query = process.env.SQL_QUERY_PRO;
            break;
        case "Network connection detected":
            query = process.env.SQL_QUERY_NET;
            break;
        default:
            throw new Error(`Unsupported event type: ${event}`);
    }

    try {
        const result = await client.query(query, [start, end]);
        // console.log('Postgres Results:', result.rows);
        return result.rows;
    } catch (error) {
        console.error("Error executing query", error.stack);
        return [];
    }
}

module.exports = { fetchDataBasedOnTime };
