
resource "github_repository" "trust_rust" {
  name = var.repository_name

  has_issues             = true
  has_discussions        = false
  allow_merge_commit     = false
  delete_branch_on_merge = true
}

resource "github_branch_default" "main_branch" {
  branch     = "main"
  repository = var.repository_name
  rename     = true
}

resource "github_repository_ruleset" "main_protection" {
  enforcement = "active"
  name        = "Main Protection"
  repository  = var.repository_name
  target      = "branch"

  conditions {
    ref_name {
      exclude = []
      include = ["~DEFAULT_BRANCH"]
    }
  }

  rules {
    required_linear_history = true
    required_status_checks {
      required_check {
        context = "Test Suite"
      }
      required_check {
        context = "Rustfmt"
      }
      required_check {
        context = "Clippy"
      }
    }

    pull_request {
    }
  }
}
