const { fetchKey } = require("../db");
const { fetchDataBasedOnTime } = require("../fetchData");

const resolvers = {
    Query: {
        RegValueSetEve: async (parent, { filter }, context, info) => {
            if (filter.event !== "Registry value set") {
                throw new Error("Invalid event for RegValueSetEve query");
            }
            return fetchSysmonData(filter, "RegValueSetEve");
        },
        ProcessCreateEve: async (parent, { filter }, context, info) => {
            if (filter.event !== "Process Create") {
                throw new Error("Invalid event for ProcessCreateEve query");
            }
            return fetchSysmonData(filter, "ProcessCreateEve");
        },
        NetworkConnectionEve: async (parent, { filter }, context, info) => {
            if (filter.event !== "Network connection detected") {
                throw new Error("Invalid event for NetworkConnectionEve query");
            }
            return fetchSysmonData(filter, "NetworkConnectionEve");
        },
    },
};

async function fetchSysmonData(filter, nodeType) {
    const { start, end } = filter.datetime;
    const postgresResults = await fetchDataBasedOnTime(
        filter.event,
        start,
        end
    );

    const allResults = [];
    for (const row of postgresResults) {
        const key = `${filter.event}_${row.savedtime}`;
        const result = await fetchKey(key);
        if (result) {
            allResults.push(result);
        }
    }

    // console.log("Final allResults:", allResults);

    switch (nodeType) {
        case "RegValueSetEve":
            return {
                Node: allResults,
                totalCount: allResults.length,
            };
        case "ProcessCreateEve":
            return {
                Node: allResults,
                totalCount: allResults.length,
            };
        case "NetworkConnectionEve":
            return {
                Node: allResults,
                totalCount: allResults.length,
            };
        default:
            throw new Error("Invalid node type");
    }
}

module.exports = resolvers;
