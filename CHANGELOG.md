# Changelog
All changes to this project will be documented in this file

## [0.1.0]
### Added
- Support for text embeddings, only supports non-batched requests
- Some minimal in-code documentation
- Code examples

### Fixed 
- The Cargo.toml to prevent panic upon build due to the macron dependency

## [0.1.1]
### Added
- Implemented handling of batched requests for text embeddings

## [0.1.2]
### Added
- A method to the Context struct of chat that allows you to insert context 
from RAG into the system prompt
- A method to Chat that allows changing the URL of your LM Studio server