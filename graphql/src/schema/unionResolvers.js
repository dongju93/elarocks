const { fetchDataBasedOnTime } = require("../fetchData");

async function eventList(parent, { filter, pagination = {} }, context, info) {
    const { datetime } = filter;
    const { start: startDate, end: endDate } = datetime;

    const processCreateEvents = await fetchDataBasedOnTime(
        "Process Create",
        startDate,
        endDate,
        filter
    );
    const regValueSetEvents = await fetchDataBasedOnTime(
        "Registry value set",
        startDate,
        endDate,
        filter
    );
    const networkConnectionEvents = await fetchDataBasedOnTime(
        "Network connection detected",
        startDate,
        endDate,
        filter
    );

    // combine the results
    const combinedResults = [
        ...processCreateEvents,
        ...regValueSetEvents,
        ...networkConnectionEvents,
    ].sort((a, b) => a.savedtimeEpoch - b.savedtimeEpoch);

    const sliceStart = pagination.after
        ? combinedResults.findIndex(
              (item) => item.savedtimeEpoch === pagination.after
          ) + 1
        : 0;

    const sliceEnd = pagination.before
        ? combinedResults.findIndex(
              (item) => item.savedtimeEpoch === pagination.before
          )
        : undefined;

    combinedResults.sort(
        (a, b) => Number(a.savedtimeEpoch) - Number(b.savedtimeEpoch)
    );

    const slicedResults = combinedResults.slice(sliceStart, sliceEnd);

    const edges = slicedResults.map((item) => ({
        cursor: item.savedtimeEpoch,
        node: item,
    }));

    const hasNextPage =
        pagination &&
        pagination.after &&
        combinedResults.length > (pagination.after || 0);
    const hasPreviousPage =
        pagination &&
        pagination.before &&
        combinedResults.length < (pagination.before || 0);

    return {
        edges,
        pageInfo: {
            endCursor: edges[edges.length - 1]?.cursor || null,
            hasNextPage,
            hasPreviousPage,
        },
        totalCount: combinedResults.length,
    };
}

function resolveType(obj) {
    if (obj.event_type) {
        return "RegValueSetEve";
    }
    if (obj.protocol) {
        return "NetworkConnectionEve";
    }
    return "ProcessCreateEve";
}

module.exports = {
    eventList,
    resolveType,
};
