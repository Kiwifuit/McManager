pub mod docker;

use std::process::Command;

const DOCKERFILE: &str = include_str!("../res/Dockerfile");
const REPLACEMENT_CHAR: char = '�';

pub fn test_docker() -> bool {
    Command::new("docker").spawn().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dockerfile() {
        assert_eq!(
            DOCKERFILE,
            r#"FROM eclipse-temurin:�-jre-alpine

WORKDIR /srv/minecraft
COPY dockerfs .

EXPOSE 25565 25575
ENTRYPOINT [ "./run-server.sh" ]"#
        )
    }
}
