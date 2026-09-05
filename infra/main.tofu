locals {
  repository_name = "trust-rust"
}


module "repository" {
  source = "./modules/repository"

  repository_name = local.repository_name
}