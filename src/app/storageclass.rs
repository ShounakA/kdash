use k8s_openapi::{api::storage::v1::StorageClass, chrono::Utc};

use super::{models::KubeResource, utils};

#[derive(Clone, Debug, PartialEq)]
pub struct KubeStorageClass {
  pub name: String,
  pub provisioner: String,
  pub reclaim_policy: String,
  pub volume_binding_mode: String,
  pub allow_volume_expansion: bool,
  pub age: String,
  k8s_obj: StorageClass,
}

impl From<StorageClass> for KubeStorageClass {
  fn from(storage_class: StorageClass) -> Self {
    KubeStorageClass {
      name: storage_class.metadata.name.clone().unwrap_or_default(),
      provisioner: storage_class.provisioner.clone(),
      reclaim_policy: storage_class.reclaim_policy.clone().unwrap_or_default(),
      volume_binding_mode: storage_class
        .volume_binding_mode
        .clone()
        .unwrap_or_default(),
      allow_volume_expansion: storage_class.allow_volume_expansion.unwrap_or_default(),
      age: utils::to_age(
        storage_class.metadata.creation_timestamp.as_ref(),
        Utc::now(),
      ),
      k8s_obj: utils::sanitize_obj(storage_class),
    }
  }
}

impl KubeResource<StorageClass> for KubeStorageClass {
  fn get_k8s_obj(&self) -> &StorageClass {
    &self.k8s_obj
  }
}

#[cfg(test)]
mod tests {
  use k8s_openapi::chrono::Utc;

  use crate::app::{
    storageclass::KubeStorageClass,
    test_utils::{convert_resource_from_file, get_time},
    utils,
  };

  #[tokio::test]
  async fn test_storageclass_from_api() {
    let (storage_classes, storage_classes_list): (Vec<KubeStorageClass>, Vec<_>) =
      convert_resource_from_file("storageclass");
    assert_eq!(storage_classes_list.len(), 4);
    assert_eq!(
      storage_classes[0],
      KubeStorageClass {
        name: "ebs-performance".into(),
        provisioner: "kubernetes.io/aws-ebs".into(),
        reclaim_policy: "Delete".into(),
        volume_binding_mode: "Immediate".into(),
        allow_volume_expansion: false,
        age: utils::to_age(Some(&get_time("2021-12-14T11:08:59Z")), Utc::now()),
        k8s_obj: storage_classes_list[0].clone(),
      }
    );
  }
}
