import React, { useMemo } from "react";

interface PaginationProps {
    currentPage: number;
    totalPages: number;
    onPageChange: (page: number) => void;
}

const Pagination: React.FC<PaginationProps> = ({
    currentPage,
    totalPages,
    onPageChange,
}) => {
    const PAGE_DISPLAY_LIMIT = 5; // Number of pages to display around the current page
    const START_ELLIPSIS_THRESHOLD = 6; // When to start showing ellipsis after the first page

    const pageNumbers = useMemo(() => {
        let pages: (number | string)[] = [];
        pages.push(1);

        let startPage = Math.max(2, currentPage - PAGE_DISPLAY_LIMIT);
        let endPage = Math.min(
            totalPages - 1,
            currentPage + PAGE_DISPLAY_LIMIT
        );

        if (currentPage < START_ELLIPSIS_THRESHOLD) {
            endPage = Math.min(START_ELLIPSIS_THRESHOLD, totalPages - 1);
        }
        if (totalPages - currentPage < START_ELLIPSIS_THRESHOLD) {
            startPage = Math.max(totalPages - START_ELLIPSIS_THRESHOLD, 2);
        }

        for (let i = startPage; i <= endPage; i++) {
            pages.push(i);
        }

        if (endPage < totalPages - 1) {
            pages.push("...");
        }
        if (!pages.includes(totalPages)) {
            pages.push(totalPages);
        }
        return pages;
    }, [currentPage, totalPages]);

    const handleClick = (page: string | number) => {
        if (page === "...") return;
        onPageChange(Number(page));
    };

    return (
        <div className="flex justify-center my-4">
            {pageNumbers.map((page, index) => (
                <button
                    key={index}
                    onClick={() => handleClick(page)}
                    disabled={page === "..."}
                    className={`mx-1 px-4 py-2 rounded ${
                        currentPage === page
                            ? "bg-blue-500 text-white"
                            : "bg-white dark:bg-gray-700 dark:text-gray-300"
                    }`}
                    aria-label={`Page ${page}`}
                >
                    {page}
                </button>
            ))}
        </div>
    );
};

export default Pagination;
