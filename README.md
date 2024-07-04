# ArgoCD Sync App

This tool allows you to sync an ArgoCD application from GitHub Actions using the ArgoCD API. It supports updating the image tag and synchronising the application.

## Features

- Sync ArgoCD application
- Update image tag for the application
- Works with GitHub Actions
- Can be ran locally

### Build & run locally

#### Prerequisites

- Rust / Cargo

```sh
cargo run -- sync --address <ARGOCD_SERVER_ADDRESS> --token <ARGOCD_TOKEN> --application <APPLICATION_NAME> --image-tag <IMAGE_TAG> --debug <true|false>
```

### Running locally with Docker

```sh
docker build -t argocd-sync-app .
docker run --rm argocd-sync sync --address <ARGOCD_SERVER_ADDRESS> --token <ARGOCD_TOKEN> --application <APPLICATION_NAME> --image-tag <IMAGE_TAG> --debug <true|false>
```

### Running with GitHub Actions

```yaml
deploy-dev:
  permissions:
    id-token: write
    contents: write
  runs-on: dind
  steps:
    - name: Configure AWS credentials
      uses: aws-actions/configure-aws-credentials@v4
      with:
        aws-region: eu-west-1
        role-to-assume: <ROLE_ARN>
    - name: Login to ECR
      id: login-ecr
      uses: aws-actions/amazon-ecr-login@v2
    - name: Sync
      uses: <REPO>
      with:
        address: <ARGO SERVER ADDRESS>
        token: <ARGO TOKEN>
        application: <ARGO APPLICATION NAME>
        image-tag: <DESIRED IMAGE TAG>
```

