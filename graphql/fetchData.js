const { Client } = require("pg");
require("dotenv").config();

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

module.exports = { fetchDataBasedOnTime };
