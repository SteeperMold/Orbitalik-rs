module.exports = {
    plugins: [
        {
            plugin: require("craco-cesium")()
        },
    ],
    babel: {
        plugins: ["babel-plugin-root-import"],
    },
};