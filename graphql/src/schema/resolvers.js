const { fetchKey } = require("../db");
const { fetchDataBasedOnTime } = require("../fetchData");

const resolvers = {
    Query: {
        RegValueSetEve: async (
            parent,
            { filter, pagination },
            context,
            info
        ) => {
            if (filter.event !== "Registry value set") {
                throw new Error("Invalid event for RegValueSetEve query");
            }
            return fetchSysmonData(filter, "RegValueSetEve", pagination);
        },
        ProcessCreateEve: async (
            parent,
            { filter, pagination },
            context,
            info
        ) => {
            if (filter.event !== "Process Create") {
                throw new Error("Invalid event for ProcessCreateEve query");
            }
            return fetchSysmonData(filter, "ProcessCreateEve", pagination);
        },
        NetworkConnectionEve: async (
            parent,
            { filter, pagination },
            context,
            info
        ) => {
            if (filter.event !== "Network connection detected") {
                throw new Error("Invalid event for NetworkConnectionEve query");
            }
            return fetchSysmonData(filter, "NetworkConnectionEve", pagination);
        },
    },
};

async function fetchSysmonData(filter, nodeType, pagination) {
    const { event, datetime, process_id, user, agent_id } = filter;
    const { start, end } = datetime;
    const filters = [];
    const allResults = [];
    const DEFAULT_OFFSET = 0;
    const DEFAULT_LIMIT = 10;
    const offset = pagination?.offset || DEFAULT_OFFSET;
    const limit = pagination?.limit || DEFAULT_LIMIT;
    const postgresResults = await fetchDataBasedOnTime(
        filter.event,
        start,
        end
    );

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
        const key = `${filter.event}_${row.savedtime}`;
        const result = await fetchKey(key);
        // use every method to check filters is true
        if (result) {
            if (result.hashes) {
                result.hashes = result.hashes.split(",");
            }
            if (filters.every((filterFn) => filterFn(result))) {
                allResults.push(result);
            } else {
                allResults.push();
            }
        }
    }

    // console.log("Final allResults:", allResults);

    switch (nodeType) {
        case "RegValueSetEve":
            return {
                node: allResults.slice(offset, offset + limit),
                totalCount: allResults.length,
            };
        case "ProcessCreateEve":
            return {
                node: allResults.slice(offset, offset + limit),
                totalCount: allResults.length,
            };
        case "NetworkConnectionEve":
            return {
                node: allResults.slice(offset, offset + limit),
                totalCount: allResults.length,
            };
        default:
            throw new Error("Invalid node type");
    }
}

module.exports = resolvers;
