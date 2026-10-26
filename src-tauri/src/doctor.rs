//! Sondes bornées et remédiations fixes : aucune commande shell fournie par le frontend.
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use anyhow::{Result, bail};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use serde::{Deserialize, Serialize};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use std::{
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    net::{IpAddr, TcpListener, UdpSocket},
    // @ligne [module] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    time::Duration,
};
// @ligne [module] Importe explicitement les types et fonctions nécessaires à ce module.
use tokio::process::Command;

// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Serialize, Deserialize, Debug)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct Check {
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub id: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub title: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub ok: bool,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub detail: String,
}
// @ligne [module] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Serialize, Deserialize, Debug)]
// @ligne [module] Définit les données échangées par ce module, sans comportement implicite.
pub struct Doctor {
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub os: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub arch: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub distribution: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub ready: bool,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub checks: Vec<Check>,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub installation: String,
    // @ligne [module] Expose ce champ du contrat sérialisé avec son type explicite.
    pub addresses: Vec<String>,
}

// @ligne [output] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn output(program: &str, args: &[&str]) -> Result<String> {
    // @ligne [output] Attend l’opération asynchrone avant d’utiliser son résultat.
    output_timeout(program, args, 12).await
}

// @ligne [output_timeout] Délimite cette opération pour isoler sa validation et son résultat.
async fn output_timeout(program: &str, args: &[&str], seconds: u64) -> Result<String> {
    // @ligne [output_timeout] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut cmd = Command::new(program);
    // @ligne [output_timeout] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    cmd.args(args).kill_on_drop(true);
    // @ligne [output_timeout] Attend l’opération asynchrone avant d’utiliser son résultat.
    let result = tokio::time::timeout(Duration::from_secs(seconds), cmd.output()).await??;
    // @ligne [output_timeout] Vérifie cette condition avant d’appliquer l’action protégée.
    if !result.status.success() {
        // @ligne [output_timeout] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!(
            // @ligne [output_timeout] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            "{} : {}",
            // @ligne [output_timeout] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            program,
            // @ligne [output_timeout] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            crate::suite::redact(&String::from_utf8_lossy(&result.stderr))
        );
    }
    // @ligne [output_timeout] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(String::from_utf8_lossy(&result.stdout).trim().to_owned())
}

// @ligne [docker] Délimite cette opération pour isoler sa validation et son résultat.
pub fn docker() -> Result<bollard::Docker> {
    // Le transport local par défaut ignore volontairement DOCKER_HOST vers un serveur distant.
    // @ligne [docker] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(bollard::Docker::connect_with_local_defaults()?)
}

// @ligne [addresses] Délimite cette opération pour isoler sa validation et son résultat.
pub fn addresses() -> Vec<String> {
    // @ligne [addresses] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut values = vec!["127.0.0.1".into()];
    // @ligne [addresses] Vérifie cette condition avant d’appliquer l’action protégée.
    if let Ok(list) = local_ip_address::list_afinet_netifas() {
        // @ligne [addresses] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
        for (_, ip) in list {
            // @ligne [addresses] Vérifie cette condition avant d’appliquer l’action protégée.
            if ip.is_ipv4() && !ip.is_loopback() {
                // @ligne [addresses] Ajoute cette donnée à la collection en cours de préparation.
                values.push(ip.to_string());
            }
        }
    }
    // @ligne [addresses] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    values.sort();
    // @ligne [addresses] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    values.dedup();
    // @ligne [addresses] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    values
}

// @ligne [inspect] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn inspect() -> Doctor {
    // @ligne [inspect] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let os = std::env::consts::OS.to_string();
    // @ligne [inspect] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let distribution = std::fs::read_to_string("/etc/os-release").unwrap_or_else(|_| os.clone());
    // @ligne [inspect] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let installation=if os=="windows" { "winget install --exact --id Docker.DockerDesktop (WSL2 requis)" }
        // @ligne [inspect] Vérifie cette condition avant d’appliquer l’action protégée.
        else if distribution.contains("ID=arch")||distribution.contains("ID_LIKE=arch") { "sudo pacman -S docker docker-buildx docker-compose" }
        // @ligne [inspect] Vérifie cette condition avant d’appliquer l’action protégée.
        else if os=="macos" { "Installez Docker Desktop depuis docs.docker.com/desktop/setup/install/mac-install/" }
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        else { "Suivez docs.docker.com/engine/install/ pour votre distribution ; installez docker-ce, docker-buildx-plugin et docker-compose-plugin depuis le dépôt officiel." }.to_string();
    // @ligne [inspect] Prépare une valeur locale qui sera complétée pendant cette opération.
    let mut checks = Vec::new();
    // @ligne [inspect] Parcourt les éléments ou attend la condition de sortie avec les bornes prévues autour de cette boucle.
    for (id, title, args) in [
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ("docker", "Client Docker", vec!["--version"]),
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ("compose", "Docker Compose", vec!["compose", "version"]),
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ("buildx", "Buildx / BuildKit", vec!["buildx", "version"]),
    // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    ] {
        // @ligne [inspect] Attend l’opération asynchrone avant d’utiliser son résultat.
        let result = output("docker", &args).await;
        // @ligne [inspect] Ajoute cette donnée à la collection en cours de préparation.
        checks.push(Check {
            // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            id: id.into(),
            // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            title: title.into(),
            // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            ok: result.is_ok(),
            // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            detail: result.unwrap_or_else(|e| e.to_string()),
        });
    }
    // @ligne [inspect] Vérifie cette condition avant d’appliquer l’action protégée.
    if cfg!(windows) {
        // @ligne [inspect] Attend l’opération asynchrone avant d’utiliser son résultat.
        let result = output("wsl", &["--status"]).await;
        // @ligne [inspect] Ajoute cette donnée à la collection en cours de préparation.
        checks.push(Check {
            // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            id: "wsl".into(),
            // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            title: "WSL2".into(),
            // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            ok: result.is_ok(),
            // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            detail: result.unwrap_or_else(|e| e.to_string()),
        });
    }
    // @ligne [inspect] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let ping = async {
        // @ligne [inspect] Attend l’opération asynchrone avant d’utiliser son résultat.
        docker()?.ping().await?;
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        Ok::<_, anyhow::Error>(())
    };
    // @ligne [inspect] Attend l’opération asynchrone avant d’utiliser son résultat.
    let result = tokio::time::timeout(Duration::from_secs(4), ping).await;
    // @ligne [inspect] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let ok = matches!(result, Ok(Ok(())));
    // @ligne [inspect] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let detail = match result {
        // @ligne [inspect] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(Ok(())) => "Socket local accessible et démon répondant".into(),
        // @ligne [inspect] Renvoie une réussite explicite avec les données attendues par l’appelant.
        Ok(Err(e)) => e.to_string(),
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        Err(_) => "Délai de réponse du démon dépassé".into(),
    };
    // @ligne [inspect] Ajoute cette donnée à la collection en cours de préparation.
    checks.push(Check {
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        id: "daemon".into(),
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        title: "Démon et accès utilisateur".into(),
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ok,
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        detail,
    });
    // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    Doctor {
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        os,
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        arch: std::env::consts::ARCH.into(),
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        distribution,
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ready: checks.iter().all(|c| c.ok),
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        checks,
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        installation,
        // @ligne [inspect] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        addresses: addresses(),
    }
}

/// Seuls ces verbes fermés peuvent ouvrir une demande Polkit/UAC, après clic explicite.
// @ligne [remedy] Délimite cette opération pour isoler sa validation et son résultat.
pub async fn remedy(action: &str) -> Result<String> {
    // @ligne [remedy] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    match action {
        // @ligne [remedy] Attend l’opération asynchrone avant d’utiliser son résultat.
        "start" if cfg!(target_os="linux")=>output("pkexec",&["systemctl","start","docker.service"]).await,
        // @ligne [remedy] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        "group" if cfg!(target_os="linux")=>{
            // @ligne [remedy] Attend l’opération asynchrone avant d’utiliser son résultat.
            let user=output("id",&["-un"]).await?;
            // @ligne [remedy] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
            if user.starts_with('-')||!user.chars().all(|c|c.is_ascii_alphanumeric()||"_-".contains(c)) { bail!("Nom utilisateur invalide"); }
            // @ligne [remedy] Attend l’opération asynchrone avant d’utiliser son résultat.
            output("pkexec",&["usermod","-aG","docker",&user]).await?;
            // @ligne [remedy] Renvoie une réussite explicite avec les données attendues par l’appelant.
            Ok("Groupe ajouté. Fermez puis rouvrez votre session système ; rafraîchir seul ne recharge pas les groupes du processus.".into())
        }
        // @ligne [remedy] Attend l’opération asynchrone avant d’utiliser son résultat.
        "install" if cfg!(windows)=>output("powershell",&["-NoProfile","-Command","Start-Process winget -Verb RunAs -ArgumentList 'install --exact --id Docker.DockerDesktop'"]).await,
        // @ligne [remedy] Attend l’opération asynchrone avant d’utiliser son résultat.
        "launch" if cfg!(windows)=>output("powershell",&["-NoProfile","-Command","Start-Process 'C:\\Program Files\\Docker\\Docker\\Docker Desktop.exe'"]).await,
        // @ligne [remedy] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        _=>bail!("Action indisponible sur ce système ; utilisez les instructions officielles affichées"),
    }
}

// @ligne [remedy] Applique cet attribut au type ou à la fonction qui suit.
#[derive(Serialize)]
// @ligne [remedy] Définit les données échangées par ce module, sans comportement implicite.
pub struct PortCheck {
    // @ligne [remedy] Expose ce champ du contrat sérialisé avec son type explicite.
    pub available: bool,
    // @ligne [remedy] Expose ce champ du contrat sérialisé avec son type explicite.
    pub suggested: u16,
}

// @ligne [port_check] Délimite cette opération pour isoler sa validation et son résultat.
pub fn port_check(ip: &str, port: u16, protocol: &str) -> Result<PortCheck> {
    // @ligne [port_check] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let ip: IpAddr = ip.parse()?;
    // @ligne [port_check] Vérifie cette condition avant d’appliquer l’action protégée.
    if port == 0 {
        // @ligne [port_check] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Le port doit être compris entre 1 et 65535");
    }
    // @ligne [port_check] Vérifie cette condition avant d’appliquer l’action protégée.
    if !addresses().contains(&ip.to_string()) && !ip.is_unspecified() {
        // @ligne [port_check] Arrête le parcours avec une erreur explicite plutôt que poursuivre avec un état invalide.
        bail!("Cette adresse n’appartient pas à la machine");
    }
    // @ligne [port_check] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let available = |p| {
        // @ligne [port_check] Vérifie cette condition avant d’appliquer l’action protégée.
        if protocol == "udp" {
            // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            UdpSocket::bind((ip, p)).is_ok()
        // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        } else {
            // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            TcpListener::bind((ip, p)).is_ok()
        }
    };
    // @ligne [port_check] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let free = available(port);
    // @ligne [port_check] Calcule cette valeur avant de l’utiliser dans la suite de l’opération.
    let suggested = if free {
        // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        port
    // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
    } else {
        // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        ((port as u32 + 1)..=(port as u32 + 100).min(65535))
            // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .map(|p| p as u16)
            // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .find(|p| available(*p))
            // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
            .unwrap_or(0)
    };
    // @ligne [port_check] Renvoie une réussite explicite avec les données attendues par l’appelant.
    Ok(PortCheck {
        // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        available: free,
        // @ligne [port_check] Renseigne cette partie du traitement ; les validations englobantes définissent ses conditions d’exécution.
        suggested,
    })
}
