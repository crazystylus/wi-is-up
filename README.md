# workload-identity-is-up Rust
![Docker Image Size (tag)](https://img.shields.io/docker/image-size/kartiksharma522/wi-is-up/latest)
![Docker Pulls](https://img.shields.io/docker/pulls/kartiksharma522/wi-is-up)
![GitHub](https://img.shields.io/github/license/crazystylus/wi-is-up)

## What
Safe and tidy docker image to avoid race conditions with GKE Metadata Server based on Rust

## Why Rust Hyper and not Alpinecurl
https://aws.amazon.com/blogs/opensource/how-using-hyper-in-curl-can-help-make-the-internet-safer/

## Why
[Workload Identity](https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity) is a GKE feature that grants Kubernetes workloads (pods) ability to assume unique Google Service Account identity without relying on JSON Keys or node's identity. However, it has a [well documented](https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity) flaw:

> The GKE metadata server needs a few seconds before it can start accepting requests on a newly created Pod. Therefore, attempts to authenticate using Workload Identity within the first few seconds of a Pod's life might fail. Retrying the call will resolve the problem. See the [Troubleshooting section](https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity#troubleshoot-timeout) for more details.

When this happens, your applications are likely to fail with `Could not load the default credentials.` message.

Troubleshooting section recommends adding an `initContainer` to prevent main application container(s) from starting before metadata server is up and running. Recommended snippet looks like this:
```yaml
  initContainers:
  - image: gcr.io/google.com/cloudsdktool/cloud-sdk:326.0.0-alpine
    name: workload-identity-initcontainer
    command:
    - '/bin/bash'
    - '-c'
    - |
      curl -s -H 'Metadata-Flavor: Google' 'http://169.254.169.254/computeMetadata/v1/instance/service-accounts/default/token' --retry 30 --retry-connrefused --retry-max-time 30 > /dev/null || exit 1
```
There are two problems with this approach:
1. Suggested `gcr.io/google.com/cloudsdktool/cloud-sdk:326.0.0-alpine` image is 6 months old (as of 18th of August)
2. It is also **630MB** in size

## Solution
Use `rust hyper` based `kartiksharma522/wi-is-up` instead which is **1.74MB** in size (362 times smaller!) and 5 lines of YAML shorter. This is safer than curl.
```yaml
initContainers:
  - image: kartiksharma522/wi-is-up
    name: wi-is-up
```

## Warning
This image is hosted on DockerHub for demonstration purposes only. It is not recommend to usw Docker Hub for production workloads due to the download rate limits that can impact availability of production applications.

### Reference
1. https://github.com/loveholidays/workload-identity-is-up
