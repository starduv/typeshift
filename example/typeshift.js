module.exports = {
    // Path to primary tsconfig.json
    project: "",
    openApi: {
        base: {
            // The only supported version.
            openapi: '3.0.3',
            info: {
                title: "DEFAULT APPLICATION TITLE",
                version: '0.0.0'
            },
            // Paths, optionally defined here, take precedence in merged result.
            paths: {}
        },
        // Glob patterns to modules declaring api paths.
        paths: [],
        // Where the resultanat OpenApi schema is written.
        output: ""
    }
}