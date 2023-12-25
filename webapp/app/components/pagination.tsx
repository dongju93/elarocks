import React, { useState } from "react";
import { PaginationProps } from "../components/types";

const Pagination: React.FC<PaginationProps> = ({
    totalCount,
    itemsPerPage,
    currentPage,
    onPageChange,
}) => {
    const totalPages = Math.ceil(totalCount / itemsPerPage);
    const [pageRange, setPageRange] = useState({ start: 1, end: 5 });

    const updatePageRange = (page: number) => {
        let newStart = Math.max(page - 2, 1);
        let newEnd = newStart + 4;

        if (newEnd > totalPages) {
            newEnd = totalPages;
            newStart = Math.max(totalPages - 4, 1);
        }

        setPageRange({ start: newStart, end: newEnd });
        onPageChange(page);
    };

    const renderPageNumbers = () => {
        let pages = [];

        if (pageRange.start > 1) {
            pages.push(
                <button
                    key={1}
                    onClick={() => updatePageRange(1)}
                    className="px-4 py-2 border border-gray-300 bg-white text-gray-700 hover:bg-gray-100"
                >
                    1
                </button>,
                <span
                    key="ellipsis1"
                    onClick={() => updatePageRange(pageRange.start - 1)}
                    className="px-4 py-2 text-gray-700 cursor-pointer"
                >
                    ...
                </span>
            );
        }

        for (
            let i = pageRange.start;
            i <= Math.min(pageRange.end, totalPages);
            i++
        ) {
            pages.push(
                <button
                    key={i}
                    onClick={() => updatePageRange(i)}
                    className={`px-4 py-2 border border-gray-300 bg-white text-gray-700 hover:bg-gray-100 ${
                        currentPage === i ? "bg-gray-200" : ""
                    }`}
                >
                    {i}
                </button>
            );
        }

        if (pageRange.end < totalPages) {
            pages.push(
                <span
                    key="ellipsis2"
                    onClick={() => updatePageRange(pageRange.end + 1)}
                    className="px-4 py-2 text-gray-700 cursor-pointer"
                >
                    ...
                </span>,
                <button
                    key={totalPages}
                    onClick={() => updatePageRange(totalPages)}
                    className="px-4 py-2 border border-gray-300 bg-white text-gray-700 hover:bg-gray-100"
                >
                    {totalPages}
                </button>
            );
        }

        return pages;
    };

    return (
        <div className="flex items-center justify-center space-x-1">
            {currentPage > 1 && (
                <button
                    onClick={() => updatePageRange(currentPage - 1)}
                    className="px-4 py-2 border border-gray-300 bg-white text-gray-700 hover:bg-gray-100"
                >
                    Previous
                </button>
            )}
            {renderPageNumbers()}
            {currentPage < totalPages && (
                <button
                    onClick={() => updatePageRange(currentPage + 1)}
                    className="px-4 py-2 border border-gray-300 bg-white text-gray-700 hover:bg-gray-100"
                >
                    Next
                </button>
            )}
        </div>
    );
};

export default Pagination;
