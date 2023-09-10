const { fetchKey } = require("../db");
const { fetchDataBasedOnTime } = require("../fetchData");

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

module.exports = resolvers;
