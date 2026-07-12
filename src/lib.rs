#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CwtSource {
    pub path: &'static str,
    pub content: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CwtRulesMetadata {
    pub game: &'static str,
    pub name: &'static str,
    pub repository: &'static str,
    pub revision: &'static str,
    pub upstream_repository: &'static str,
    pub upstream_revision: &'static str,
    pub source_directory: &'static str,
    pub virtual_prefix: &'static str,
    pub license: &'static str,
    pub required_cwtools_repository: &'static str,
    pub required_cwtools_revision: &'static str,
    pub content_sha256: &'static str,
    pub source_count: usize,
    pub total_bytes: usize,
}

mod generated {
    include!(concat!(env!("OUT_DIR"), "/hoi4_cwt_sources.rs"));
}

pub const METADATA: CwtRulesMetadata = CwtRulesMetadata {
    game: generated::GAME,
    name: generated::NAME,
    repository: generated::REPOSITORY,
    revision: generated::REVISION,
    upstream_repository: generated::UPSTREAM_REPOSITORY,
    upstream_revision: generated::UPSTREAM_REVISION,
    source_directory: generated::SOURCE_DIRECTORY,
    virtual_prefix: generated::VIRTUAL_PREFIX,
    license: generated::LICENSE,
    required_cwtools_repository: generated::REQUIRED_CWTOOLS_REPOSITORY,
    required_cwtools_revision: generated::REQUIRED_CWTOOLS_REVISION,
    content_sha256: generated::CONTENT_SHA256,
    source_count: generated::SOURCE_COUNT,
    total_bytes: generated::TOTAL_BYTES,
};

pub const SOURCE_COUNT: usize = generated::SOURCE_COUNT;
pub const TOTAL_BYTES: usize = generated::TOTAL_BYTES;
pub const CONTENT_SHA256: &str = generated::CONTENT_SHA256;

pub fn metadata() -> &'static CwtRulesMetadata {
    &METADATA
}

pub fn sources() -> &'static [CwtSource] {
    generated::SOURCES
}

pub fn source_by_path(path: &str) -> Option<&'static CwtSource> {
    sources().iter().find(|source| source.path == path)
}

pub fn required_cwtools_revision() -> &'static str {
    generated::REQUIRED_CWTOOLS_REVISION
}
