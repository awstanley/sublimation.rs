var addon = require('../native');

var exports = {};
var real_exports = {
    // Neon
    get_version: addon.version,
    get_version_string: addon.version_string,

    // Sublimation
    start_app: addon.sublimation_startapp,
    get_app_build: addon.sublimation_app_build,
    get_app_directory: addon.sublimation_app_dir,
};

// Notify the console.
if (addon.sublimation_init()) {
    console.log("Loaded " + addon.version_string());

    // Export from Rust.
    exports = real_exports;
} else {
    console.log("Failed " + addon.version_string())
}

module.exports = exports;