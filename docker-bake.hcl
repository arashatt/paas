variable "IMAGE_NAME" {
  default = "my_app"
}

target "builder" {
  dockerfile = "Dockerfile"
  target = "builder"
  tags = ["${IMAGE_NAME}:builder"]
}

target "runtime" {
  dockerfile = "Dockerfile"
  target = "runtime"
  tags = ["${IMAGE_NAME}:latest"]
}

group "default" {
  targets = ["builder", "runtime"]
}
