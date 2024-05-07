resource "aws_ecrpublic_repository" "ecr_projects" {
  provider = aws.us_east_1

  repository_name = "609e6f97dc3bd7a240c3fc0587448b72_projects"
}