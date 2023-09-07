const RocksDB = require('rocksdb');
const path = require('path');

const dbPath = path.join(__dirname, '../db');

RocksDB.repair(dbPath, (err) => {
    if (err) {
        console.error('Failed to repair database:', err);
    } else {
        console.log('Database repaired successfully.');
    }
});
