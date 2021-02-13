//! Querying-related structures.

/// Describes options for a single query.
pub struct Query {
    /// String passed to the api as `query`
    pub string: Option<String>,
    /// Number of the page to get, numbering starts at 1
    pub page: Option<usize>,
    /// Number of results per page
    pub per_page: Option<usize>,
    /// Match crates that contain a certain string keyword
    pub keyword: Option<String>,
    /// Specify one of the available categories
    pub category: Option<Category>,
    /// Sort the results on the API query level
    pub sort: Option<Sorting>,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            string: None,
            page: None,
            per_page: Some(100),
            keyword: None,
            category: None,
            sort: None,
        }
    }
}

impl Query {
    /// Parses raw string into a query struct.
    ///
    /// # Examples
    ///
    /// Search for `api` string in the `web-programming` category, showing the
    /// most recently updated crates first:
    ///
    /// ```text
    /// api cat=web sort=update
    /// ```
    ///
    /// Search for `net` string in the `game-development` category, sorting by
    /// amount of recent downloads.
    ///
    /// ```text
    /// net cat=gamedev sort=rdl
    /// ```
    pub fn from_str(input: &str) -> Self {
        let mut query = Query::default();
        // split on whitespaces
        let split = input.split(" ").collect::<Vec<&str>>();
        for s in split {
            if s.contains("cat=") || s.contains("category=") {
                if !s.ends_with("=") {
                    query.category = Category::from_str(s.split("=").collect::<Vec<&str>>()[1]);
                }
            } else if s.contains("keyword=") || s.contains("key") || s.contains("kw") {
                if !s.ends_with("=") {
                    query.keyword = Some(s.split("=").collect::<Vec<&str>>()[1].to_string());
                }
            } else if s.contains("sort=") && !s.ends_with("=") {
                query.sort = Sorting::from_str(s.split("=").collect::<Vec<&str>>()[1]);
            } else if s.contains("page=") && !s.ends_with("=") {
                let page_s = s.split("=").collect::<Vec<&str>>()[1];
                if let Ok(page) = page_s.parse() {
                    query.page = Some(page);
                }
            } else if s.contains("per-page") || s.contains("num") {
                if !s.ends_with("=") {
                    let per_page_s = s.split("=").collect::<Vec<&str>>()[1];
                    if let Ok(per_page) = per_page_s.parse() {
                        query.per_page = Some(per_page);
                    }
                }
            } else if !s.contains("=") {
                query.string = Some(s.to_string());
            }
        }
        query
    }
}

/// Available sorting schemes.
pub enum Sorting {
    Alphabetical,
    AllTimeDownloads,
    RecentDownloads,
    RecentUpdates,
    NewlyAdded,
}

impl Sorting {
    pub fn to_str(&self) -> &str {
        match self {
            Sorting::Alphabetical => "alpha",
            Sorting::AllTimeDownloads => "downloads",
            Sorting::RecentDownloads => "recent-downloads",
            Sorting::RecentUpdates => "recent-updates",
            Sorting::NewlyAdded => "new",
        }
    }

    pub fn from_str(input: &str) -> Option<Self> {
        let sort = match input {
            "alpha" | "alphabet" | "alphabetic" | "alphabetical" => Self::Alphabetical,
            "downloads" | "download" | "dl" | "all-time" => Self::AllTimeDownloads,
            "recent-downloads" | "rdl" | "new-downloads" => Self::RecentDownloads,
            "recent-updates" | "new-updates" | "updates" | "update" | "rup" => Self::RecentUpdates,
            "newly-added" | "new" | "newest" | "latest" => Self::NewlyAdded,
            &_ => return None,
        };
        Some(sort)
    }
}

/// Categories available on `crates.io`.
pub enum Category {
    Accessibility,
    Algorithms,
    ApiBindings,
    Asynchronous,
    Authentication,
    Caching,
    CommandLineInterface,
    CommandLineUtilities,
    Compilers,
    Compression,
    ComputerVision,
    Concurrency,
    Config,
    Cryptography,
    Database,
    DatabaseImplementations,
    DataStructures,
    DateAndTime,
    DevelopmentTools,
    Email,
    Embedded,
    Emulators,
    Encoding,
    ExternalFfiBindings,
    Filesystem,
    GameDevelopment,
    GameEngines,
    Games,
    Graphics,
    Gui,
    HardwareSupport,
    Internationalization,
    Localization,
    Mathematics,
    MemoryManagement,
    Multimedia,
    NetworkProgramming,
    NoStd,
    Os,
    ParserImplementations,
    Parsing,
    Rendering,
    RustPatterns,
    Science,
    Simulation,
    TemplateEngine,
    TextEditors,
    TextProcessing,
    ValueFormatting,
    Visualization,
    Wasm,
    WebProgramming,
}

impl Category {
    pub fn to_str(&self) -> &str {
        match self {
            Category::Accessibility => "accessibility",
            Category::Algorithms => "algorithms",
            Category::ApiBindings => "api-bindings",
            Category::Asynchronous => "asynchronous",
            Category::Authentication => "authentication",
            Category::Caching => "caching",
            Category::CommandLineInterface => "command-line-interface",
            Category::CommandLineUtilities => "command-line-utilities",
            Category::Compilers => "compilers",
            Category::Compression => "compression",
            Category::ComputerVision => "computer-vision",
            Category::Concurrency => "concurrency",
            Category::Config => "config",
            Category::Cryptography => "cryptography",
            Category::Database => "database",
            Category::DatabaseImplementations => "database-implementations",
            Category::DataStructures => "data-structures",
            Category::DateAndTime => "date-and-time",
            Category::DevelopmentTools => "development-tools",
            Category::Email => "email",
            Category::Embedded => "embedded",
            Category::Emulators => "emulators",
            Category::Encoding => "encoding",
            Category::ExternalFfiBindings => "external-ffi-bindings",
            Category::Filesystem => "filesystems",
            Category::GameDevelopment => "game-development",
            Category::GameEngines => "game-engines",
            Category::Games => "games",
            Category::Graphics => "graphics",
            Category::Gui => "gui",
            Category::HardwareSupport => "hardware-support",
            Category::Internationalization => "internationalization",
            Category::Localization => "localization",
            Category::Mathematics => "mathematics",
            Category::MemoryManagement => "memory-management",
            Category::Multimedia => "multimedia",
            Category::NetworkProgramming => "network-programming",
            Category::NoStd => "no-std",
            Category::Os => "os",
            Category::ParserImplementations => "parser-implementations",
            Category::Parsing => "parsing",
            Category::Rendering => "rendering",
            Category::RustPatterns => "rust-patterns",
            Category::Science => "science",
            Category::Simulation => "simulation",
            Category::TemplateEngine => "template-engine",
            Category::TextEditors => "text-editors",
            Category::TextProcessing => "text-processing",
            Category::ValueFormatting => "value-formatting",
            Category::Visualization => "visualization",
            Category::Wasm => "wasm",
            Category::WebProgramming => "web-programming",
        }
    }

    pub fn from_str(input: &str) -> Option<Self> {
        let cat = match input {
            "accessibility" | "access" | "accessible" => Self::Accessibility,
            "algorithms" | "algo" | "algorithm" | "algorithmic" => Self::Algorithms,
            "api-bindings" | "bindings" | "api" => Self::ApiBindings,
            "asynchronous" | "async" => Self::Asynchronous,
            "authentication" | "auth" | "authenticate" => Self::Authentication,
            "caching" | "cache" => Self::Caching,
            "command-line-interface" | "cli" => Self::CommandLineInterface,
            "command-line-utilities" | "util" | "utility" | "utilities" => {
                Self::CommandLineUtilities
            }
            "compilers" | "compiler" => Self::Compilers,
            "compression" | "compress" => Self::Compression,
            "computer-vision" | "vision" => Self::ComputerVision,
            "concurrency" | "concurrent" => Self::Concurrency,
            "config" | "cfg" | "conf" => Self::Config,
            "cryptography" | "crypto" => Self::Cryptography,
            "database" | "db" => Self::Database,
            "database-implementations" | "db-impl" => Self::DatabaseImplementations,
            "data-structures" | "struct" | "structs" | "structures" => Self::DataStructures,
            "date-and-time" | "date" | "time" | "datetime" => Self::DateAndTime,
            "development-tools" | "dev-tools" | "tools" => Self::DevelopmentTools,
            "email" | "mail" => Self::Email,
            "embedded" | "embed" => Self::Embedded,
            "emulators" | "emulation" | "emulate" => Self::Emulators,
            "encoding" | "encode" | "encoders" => Self::Encoding,
            "external-ffi-bindings" | "ffi" => Self::ExternalFfiBindings,
            "filesystem" | "fs" | "filesystems" => Self::Filesystem,
            "game-development" | "gamedev" | "game-dev" => Self::GameDevelopment,
            "game-engines" | "game-engine" | "engines" => Self::GameEngines,
            "games" | "game" => Self::Games,
            "graphics" => Self::Graphics,
            "gui" | "ui" => Self::Gui,
            "hardware-support" | "hardware" => Self::HardwareSupport,
            "internationalization" | "i18n" => Self::Internationalization,
            "localization" | "localizations" => Self::Localization,
            "mathematics" | "maths" | "math" => Self::Mathematics,
            "memory-management" | "memory" | "mem" => Self::MemoryManagement,
            "multimedia" | "media" => Self::Multimedia,
            "network-programming" | "net" | "network" | "networking" => Self::NetworkProgramming,
            "no-std" | "nostd" => Self::NoStd,
            "os" | "operating-system" => Self::Os,
            "parser-implementations" | "parsers" => Self::ParserImplementations,
            "parsing" | "parse" => Self::Parsing,
            "rendering" | "render" => Self::Rendering,
            "rust-patterns" | "patterns" => Self::RustPatterns,
            "science" | "scientific" | "sci" => Self::Science,
            "simulation" | "sim" | "simulators" => Self::Simulation,
            "template-engine" | "template-engines" | "template" => Self::TemplateEngine,
            "text-editors" | "editors" => Self::TextEditors,
            "text-processing" | "text" | "processing" => Self::TextProcessing,
            "value-formatting" | "formatting" => Self::ValueFormatting,
            "visualization" | "visual" | "vis" | "visualize" => Self::Visualization,
            "wasm" => Self::Wasm,
            "web-programming" | "web" => Self::WebProgramming,
            &_ => return None,
        };
        Some(cat)
    }
}
