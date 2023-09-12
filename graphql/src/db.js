const RocksDB = require("rocksdb");
const path = require("path");
const dbPath = path.join(__dirname, "../../db");
const db = RocksDB(dbPath);

function fetchKey(key) {
    return new Promise((resolve, reject) => {
        db.get(Buffer.from(key), (err, value) => {
            if (err) {
                // console.error("Error fetching from RocksDB:", err);
                if (err.message === "NotFound: ") {
                    resolve(null);
                } else {
                    reject(err);
                }
            } else {
                const parsedValue = JSON.parse(value.toString("utf-8"));
                // console.log('Fetched value from RocksDB:', parsedValue);
                resolve(parsedValue);
            }
        });
    });
}

module.exports = { db, fetchKey };
