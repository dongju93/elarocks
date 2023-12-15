const { fetchKey } = require("../db");
const { fetchDataBasedOnTime } = require("../fetchData");
// for login function not yet implements
const bcrypt = require("bcrypt");
const jwt = require("jsonwebtoken");

const resolvers = {
    Query: {
        RegValueSetEve: async (
            parent,
            { filter, pagination },
            context,
            info
        ) => {
            return fetchSysmonData(filter, "Registry value set", pagination);
        },
        ProcessCreateEve: async (
            parent,
            { filter, pagination },
            context,
            info
        ) => {
            return fetchSysmonData(filter, "Process Create", pagination);
        },
        NetworkConnectionEve: async (
            parent,
            { filter, pagination },
            context,
            info
        ) => {
            return fetchSysmonData(
                filter,
                "Network connection detected",
                pagination
            );
        },
    },
};

function datetimeToEpoch(datetime) {
    const [datePart, timePart] = datetime.split(" ");
    const [year, month, day] = datePart.split("-").map(Number);
    const [hour, minute, fullSecond] = timePart.split(":");
    const second = parseInt(fullSecond, 10);
    const fractionalSecond = parseFloat(fullSecond) - second;
    const nanoseconds = Math.round(fractionalSecond * 1e9);

    const dateObj = new Date(
        Date.UTC(year, month - 1, day, hour, minute, second)
    );
    const epochSeconds = Math.floor(dateObj.getTime() / 1000);

    return `${epochSeconds}${String(nanoseconds).padStart(9, "0")}`;
}

async function fetchSysmonData(filter, nodeType, pagination) {
    const { datetime, process_id, user, agent_id } = filter;
    const { start, end } = datetime;
    const DEFAULT_LIMIT = 50;
    const filters = [];
    const allResults = [];
    const { first, last, before, after } = pagination;
    const postgresResults = await fetchDataBasedOnTime(nodeType, start, end);

    if (process_id) {
        filters.push((result) => result.process_id == process_id);
    }

    if (user) {
        filters.push((result) => result.user == user);
    }

    if (agent_id) {
        filters.push(
            (result) => result.agent_id && result.agent_id.includes(agent_id)
        );
    }

    for (const row of postgresResults) {
        const key = `${nodeType}_${row.savedtime}`;
        const result = await fetchKey(key);
        // use every method to check filters is true
        if (result) {
            if (result.hashes) {
                result.hashes = result.hashes.split(",");
            }
            // for cursor epoch
            const epochSavedTime = datetimeToEpoch(row.savedtime);
            if (filters.every((filterFn) => filterFn(result))) {
                allResults.push({ ...result, savedtimeEpoch: epochSavedTime });
            } else {
                allResults.push();
            }
        }
    }

    // Sort results by utc_time
    allResults.sort((a, b) => new Date(a.utc_time) - new Date(b.utc_time));

    // before and after arguments
    let startIndex = 0;
    if (after) {
        startIndex =
            allResults.findIndex((item) => item.savedtimeEpoch === after) + 1;
    }

    let endIndex = allResults.length;
    if (before) {
        endIndex = allResults.findIndex(
            (item) => item.savedtimeEpoch === before
        );
    }

    // first and last arguments
    if (typeof first === "number") {
        results = allResults.slice(startIndex, startIndex + first);
    } else if (typeof last === "number") {
        startIndex = endIndex - last > 0 ? endIndex - last : 0;
        results = allResults.slice(startIndex, endIndex);
    } else {
        results = allResults.slice(startIndex, startIndex + DEFAULT_LIMIT);
    }

    const edges = results.map((item) => ({
        cursor: item.savedtimeEpoch,
        node: item,
    }));

    const hasNextPage = before
        ? true
        : (after ? startIndex + first : first) < allResults.length;
    const hasPreviousPage = before ? endIndex - last > 0 : startIndex > 0;

    return {
        edges,
        pageInfo: {
            startCursor: edges[0]?.cursor || null,
            endCursor: edges[edges.length - 1]?.cursor || null,
            hasNextPage,
            hasPreviousPage,
        },
        totalCount: allResults.length,
    };
}

module.exports = resolvers;
