const TableItem = ({children, colSpan = 1, rowSpan = 1}) => {
    return <th colSpan={colSpan} rowSpan={rowSpan} className="border border-gray-600 py-1.5">{children}</th>
};

export default TableItem;
