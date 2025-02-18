use k8s_openapi::api::core::v1::Namespace;

use super::{
  models::KubeResource,
  utils::{self, UNKNOWN},
};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct KubeNs {
  pub name: String,
  pub status: String,
  k8s_obj: Namespace,
}

impl From<Namespace> for KubeNs {
  fn from(ns: Namespace) -> Self {
    let status = match &ns.status {
      Some(stat) => match &stat.phase {
        Some(phase) => phase.clone(),
        _ => UNKNOWN.into(),
      },
      _ => UNKNOWN.into(),
    };

    KubeNs {
      name: ns.metadata.name.clone().unwrap_or_default(),
      status,
      k8s_obj: utils::sanitize_obj(ns),
    }
  }
}

impl KubeResource<Namespace> for KubeNs {
  fn get_k8s_obj(&self) -> &Namespace {
    &self.k8s_obj
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::app::test_utils::convert_resource_from_file;

  #[test]
  fn test_namespace_from_api() {
    let (nss, ns_list): (Vec<KubeNs>, Vec<_>) = convert_resource_from_file("ns");

    assert_eq!(nss.len(), 4);
    assert_eq!(
      nss[0],
      KubeNs {
        name: "default".into(),
        status: "Active".into(),
        k8s_obj: ns_list[0].clone()
      }
    );
  }
}
