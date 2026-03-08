use serde::Serialize;
use std::env;
use std::io::Result;
use tinytemplate::TinyTemplate;

#[derive(Serialize, Debug)]
struct TemplateContext {
    service: String,
    envs: Vec<TemplateEntry>,
    volumes: Vec<TemplateEntry>,
}

#[derive(Serialize, Debug)]
struct TemplateEntry {
    source: String,
    dest: String,
}

static TEMPLATE: &str = include_str!("../../templates/docker-compose.yml");

/// Write a temporary docker-compose override file that forwards the SSH agent
/// socket into the named `service` container and injects any additional
/// environment variables, returning the path to the written file.
///
/// # Errors
/// Returns an error if the template cannot be rendered or the file cannot be written.
pub(crate) fn create_compose_override(
    service: &str,
    env_vars: &[(String, String)],
) -> Result<String> {
    let dir = env::temp_dir();
    let file = dir.join("docker-compose.yml");
    let mut volumes = vec![];
    let mut envs: Vec<TemplateEntry> = env_vars
        .iter()
        .map(|(k, v)| TemplateEntry {
            source: k.clone(),
            dest: v.clone(),
        })
        .collect();

    if let Ok(ssh_auth_sock) = env::var("SSH_AUTH_SOCK") {
        volumes.push(TemplateEntry {
            source: ssh_auth_sock,
            dest: "/ssh-agent".to_string(),
        });
        envs.push(TemplateEntry {
            source: "SSH_AUTH_SOCK".to_string(),
            dest: "/ssh-agent".to_string(),
        });
    }

    let context = TemplateContext {
        service: service.to_string(),
        envs,
        volumes,
    };

    let mut tt = TinyTemplate::new();
    tt.add_template("docker-compose.yml", TEMPLATE)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let rendered = tt
        .render("docker-compose.yml", &context)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    std::fs::write(&file, rendered)?;

    Ok(file.to_string_lossy().to_string())
}
