const DOCKERFILE: &str = include_str!("../res/Dockerfile");

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dockerfile() {
        assert_eq!(
            DOCKERFILE,
            r#"FROM eclipse-temurin:17-jre-alpine

WORKDIR /srv/minecraft
COPY dockerfs .

EXPOSE 25565 25575
ENTRYPOINT [ "./run-server.sh" ]"#
        )
    }
}
