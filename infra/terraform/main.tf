
resource "runpod_network_volume" "storage" {
  name           = "quicksilicon-storage"
  size           = 20
  data_center_id = "US-CA-2"
}

resource "runpod_pod" "gpu_instance" {
  name       = "quicksilicon-gpu"
  image_name = "runpod/pytorch:2.1.0-py3.10-cuda11.8.0-devel"

  gpu_type_ids   = ["NVIDIA RTX A6000"]
  data_center_ids = ["US-CA-2"]

  gpu_count         = 1
  cloud_type        = "COMMUNITY"
  support_public_ip = true

  network_volume_id = runpod_network_volume.storage.id

  volume_in_gb         = 20
  container_disk_in_gb = 20

  ports = ["8888/http", "22/tcp"]
}
