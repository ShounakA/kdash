use k8s_openapi::{
  api::networking::v1::{Ingress, IngressBackend, IngressRule},
  chrono::Utc,
};

use super::{
  models::KubeResource,
  utils::{self},
};

#[derive(Clone, Debug, PartialEq)]
pub struct KubeIng {
  pub namespace: String,
  pub name: String,
  pub ing_class_name: String,
  pub default_backend: String,
  pub rules: Vec<String>,
  pub age: String,
  k8s_obj: Ingress,
}

impl KubeResource<Ingress> for KubeIng {
  fn get_k8s_obj(&self) -> &Ingress {
    &self.k8s_obj
  }
}

impl From<Ingress> for KubeIng {
  fn from(ingress: Ingress) -> Self {
    let (class_name, default_backend, rules) = match &ingress.spec {
      Some(spec) => {
        let d_backend = get_backend(spec.default_backend.as_ref().unwrap());
        let class_name = spec.ingress_class_name.clone().unwrap_or_default();
        let spec_rules = spec.rules.as_ref().unwrap();
        let rules = get_rules(spec_rules);
        (class_name, d_backend, rules)
      }
      None => (String::default(), String::default(), Vec::<String>::new()),
    };

    KubeIng {
      name: ingress.metadata.name.clone().unwrap_or_default(),
      namespace: ingress.metadata.namespace.clone().unwrap_or_default(),
      ing_class_name: class_name,
      default_backend,
      rules,
      age: utils::to_age(ingress.metadata.creation_timestamp.as_ref(), Utc::now()),
      k8s_obj: utils::sanitize_obj(ingress),
    }
  }
}

fn get_backend(ingress_backend: &IngressBackend) -> String {
  match &ingress_backend.resource {
    Some(resource) => {
      let rsrc = format!("[{} -> {}]", &resource.kind, &resource.name);
      rsrc
    }
    None => match &ingress_backend.service {
      Some(svc) => {
        let port = &svc.port.clone().unwrap_or_default();
        let port_str = match &port.name {
          Some(name) => name.clone(),
          None => match &port.number {
            Some(num) => num.to_string(),
            None => String::default(),
          },
        };
        let svcs = format!("[{}:{}]", &svc.name, port_str);
        svcs
      }
      None => String::default(),
    },
  }
}

fn get_rules(rules: &[IngressRule]) -> Vec<String> {
  let rules_str = rules.iter().map(get_ing_rule).collect();
  rules_str
}

fn get_ing_rule(rule: &IngressRule) -> String {
  let host = rule.host.clone().unwrap_or_default();

  let rule_list = rule
    .http
    .clone()
    .unwrap_or_default()
    .paths
    .iter()
    .map(|path| {
      format!(
        "\t\t{} => {}:{}",
        get_backend(&path.backend),
        &path.path_type,
        &path.path.as_ref().unwrap()
      )
    })
    .collect::<Vec<String>>()
    .join("\n");

  let ing_rule = format!("{}\n {}", host, rule_list);
  ing_rule
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::app::test_utils::*;

//   #[test]
//   fn test_ingresses_from_api() {
//     let (_ing, _ing_list): (Vec<KubeIng>, Vec<_>) = convert_resource_from_file("ingresses");
//   }
// }
