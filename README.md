# Kubewarden policy volumemounts-policy

## Description

Policy that inspects containers, init containers, and ephemeral containers, and
restricts their usage of volumes by  checking the `volume` name being used in
`volumeMounts[*].name`.

The policy can either target `Pods`, or [workload
resources](https://kubernetes.io/docs/concepts/workloads/) (`Deployments`,
`ReplicaSets`, `DaemonSets`, `ReplicationControllers`, `Jobs`, `CronJobs`) by
setting the policy's `spec.rules` accordingly.

Both have trade-offs:
* Policy targets Pods: Different kind of resources (be them native or CRDs) can
  create Pods. By having the policy target Pods, we guarantee that all the Pods
  are going to be compliant, even those created from CRDs.
  However, this could lead to confusion among users, as high level Kubernetes
  resources would be successfully created, but they would stay in a non
  reconciled state. Example: a Deployment creating a non-compliant Pod would be
  created, but it would never have all its replicas running.
* Policy targets workload resources (e.g: Deployment): the policy inspect higher
  order resource (e.g. Deployment): users will get immediate feedback about
  rejections.
  However, non compliant pods created by another high level resource (be it
  native to Kubernetes, or a CRD), may not get rejected.


## Settings
```yaml
reject: anyIn # one of anyIn (default, denylist), anyNotIn (allowlist), allAreUsed, notAllAreUsed
volumeMountNames:  # list of volumeMounts.name to match using the defined reject operator
  - foo
  - bar
  - baz
```

- `anyIn` (default): checks if any of the volumeMountNames are in the Pod/Workload resource
- `anyNotIn`: checks if any of the volumeMountNames are not in the Pod/Workload resource
- `allAreUsed`: checks if all of the volumeMountNames are in the Pod/Workload resource
- `notAllAreUsed`: checks if all of the volumeMountNames are not in the Pod/Workload resource

## Examples

```yaml
# denylist, reject volumeMounts named `my-volume` or `my-volume2`
reject: anyIn
volumeMountsNames:
  - my-volume
  - my-volume2
```

```yaml
# allowlist, only allow volumeMounts named `my-volume3` or `my-volume4`
reject: anyNotIn
volumeMounts:
  - my-volume3
  - my-volume4
```

```yaml
# container cannot use both volumes at once, only one or the other
reject: allAreUsed
volumeMounts:
  - my-volume5
  - my-volume6
```

```yaml
# container can use both volumes at once, but not only one of them
reject: notAllAreSused
volumeMounts:
  - my-volume5
  - my-volume6
```

## License

```
Copyright (C) 2021 Víctor Cuadrado Juan <vcuadradojuan@suse.de>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
