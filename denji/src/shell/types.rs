use serde::Deserialize;

#[derive(Debug)]
pub enum CommandOutput {
    Message(String),
    DockerImageBuilder(DockerBuilderCommandOutput),
}

#[derive(Debug, Deserialize)]
pub struct DockerBuilderCommandOutput {
    #[serde(default)]
    pub vertexes: Vec<VertexData>,
    #[serde(default)]
    pub statuses: Vec<StatusData>,
}

#[derive(Debug, Deserialize)]
pub struct VertexData {
    pub digest: String,
    pub name: String,
    pub started: Option<String>,
    pub competed: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StatusData {
    pub id: String,
    pub vertex: String,
    pub current: u32,
    pub timestamp: String,
    pub started: String,
    pub competed: Option<String>,
}
