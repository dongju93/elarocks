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
    const { event, datetime, process_id, user, agent_id } = filter;
    const { start, end } = datetime;
    const postgresResults = await fetchDataBasedOnTime(
        filter.event,
        start,
        end
    );

    const filters = [];

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

    const allResults = [];
    for (const row of postgresResults) {
        const key = `${filter.event}_${row.savedtime}`;
        const result = await fetchKey(key);
        // use every method to check filters is true
        if (result) {
            if (filters.every((filterFn) => filterFn(result))) {
                allResults.push(result);
            } else {
                allResults.push();
            }
        }

        // if (result) {
        //     let matches = true;
        //     if (process_id) {
        //         matches = matches && result.process_id == process_id;
        //     }
        //     if (user) {
        //         matches = matches && result.user == user;
        //     }
        //     if (matches) {
        //         allResults.push(result);
        //     } else {
        //         allResults.push();
        //     }
        // }
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
