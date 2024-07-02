use super::APIError;
use crate::{
    types::{
        query::VersionQuery,
        version::{ResolvedVersionDependency, VersionDependency},
    },
    version::{get_version, get_versions},
    ModrinthProjectVersion,
};
use reqwest::Client;

pub async fn resolve_dependencies<F>(
    client: &Client,
    project: &mut ModrinthProjectVersion,
    version_params: &VersionQuery,
    resolver: F,
) -> Result<(), APIError>
where
    F: Fn(Vec<ModrinthProjectVersion>) -> ModrinthProjectVersion,
{
    if project.dependencies.is_none() || project.dependencies.as_ref().is_some_and(|v| v.is_empty())
    {
        return Err(APIError::NoDependencies);
    }

    for dependency in project.dependencies.as_mut().unwrap().iter_mut() {
        let unresolved_dependency = match dependency {
            VersionDependency::Resolved(ver) => {
                Err(APIError::ResolvedDependency(ver.dependency.name.clone()))
            }
            VersionDependency::Unresolved(ver) => Ok(ver),
        }?;

        if unresolved_dependency.version_id.is_none() && unresolved_dependency.project_id.is_none()
        {
            return Err(APIError::UnresolvableDependency);
        }

        let resolved_version = if unresolved_dependency.version_id.is_some() {
            get_version(client, unresolved_dependency).await?
        } else {
            let version_list = get_versions(client, unresolved_dependency, version_params).await?;

            resolver(version_list)
        };

        *dependency = VersionDependency::Resolved(ResolvedVersionDependency {
            dependency: resolved_version,
            dependency_type: unresolved_dependency.dependency_type.clone(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        get_client, get_project, search_project, IndexBy, Loader, ProjectQueryBuilder,
        VersionQueryBuilder,
    };

    #[tokio::test]
    async fn check_dep_resolution() {
        let client = get_client().await;

        let query = ProjectQueryBuilder::new()
            .query("Almost Unified")
            .limit(1)
            .index(IndexBy::Relevance)
            .build();

        let (res, _) = search_project(&client, &query).await.unwrap();
        let project = get_project(&client, res.first().unwrap()).await.unwrap();

        let v_query = VersionQueryBuilder::new()
            .featured(true)
            .versions(vec!["1.19.2"])
            .loaders(vec![Loader::Forge])
            .build();

        let mut versions = get_versions(&client, &project, &v_query).await.unwrap();
        let version = versions.get_mut(0).unwrap();

        let _err = resolve_dependencies(&client, version, &v_query, |versions| {
            versions.into_iter().next().unwrap()
        })
        .await;

        // if _err.is_err() {
        //     dbg!(_err.unwrap_err());
        // } else {
        //     dbg!(version);
        // }

        assert!(version
            .dependencies
            .as_ref()
            .unwrap()
            .iter()
            .all(|dep| dep.is_resolved()));
    }
}
