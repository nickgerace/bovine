use crate::{consts, docker, error::Error, rancher, types::cli::Upgrade, util};
use anyhow::Result;
use bollard::{container::Config, models::HostConfig};

pub async fn upgrade(opt: &Upgrade, docker_socket_path: Option<String>) -> Result<()> {
    let docker = docker::docker_client(docker_socket_path).await?;

    // Get both the container container and the host config from the Bovine-managed Rancher
    // container. The labels can be dropped, so we need to re-add them.
    let (mut config, host_config) = docker::get_configs(&docker, &opt.container_id).await?;
    match config.labels {
        Some(ref mut s) => {
            if !s.contains_key(consts::package::LABEL) {
                s.extend(rancher::get_labels());
            }
        }
        None => config.labels = Some(rancher::get_labels()),
    }

    let old_image = match &config.image {
        Some(s) => s,
        None => return Err(Error::DockerContainerImageNotFound(opt.container_id.clone()).into()),
    };

    // For upgrade we need to use either the user provided upgrade image or the old image without
    // its tag.
    let image_name_without_tag = match opt.image.clone() {
        Some(s) => s,
        None => match util::get_tag_from_image(old_image) {
            Ok((s, _)) => s,
            Err(_) => return Err(Error::OCIImageTagInvalid(opt.tag.clone()).into()),
        },
    };
    let new_image = util::get_full_image(&image_name_without_tag.to_string(), &opt.tag);

    docker::stop_container(&docker, &opt.container_id, false).await?;
    let volumes_from = vec![opt.container_id.to_owned()];
    println!(
        "Created temporary container for volume backup: {}",
        util::get_first_n_chars(
            docker
                .create_container::<String, String>(
                    None,
                    Config {
                        image: Some(old_image.clone()),
                        host_config: Some(HostConfig {
                            volumes_from: Some(volumes_from.clone()),
                            ..Default::default()
                        }),
                        labels: Some(rancher::get_labels()),
                        ..Default::default()
                    },
                )
                .await?
                .id,
            12
        )
    );

    docker::pull_image(&docker, &new_image, opt.common.force_pull).await?;

    rancher::launch_rancher(
        docker,
        rancher::build_config(config.clone(), host_config.clone(), Some(volumes_from)),
    )
    .await?;
    println!("Upgrade from [{}] to [{}] complete", old_image, new_image);
    Ok(())
}
