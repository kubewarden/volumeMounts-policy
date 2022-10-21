#!/usr/bin/env bats

@test "Accept a pod without volumemounts" {
	run kwctl run  --request-path test_data/pod_creation.json --settings-path test_data/settings_reject.yaml annotated-policy.wasm
	[ "$status" -eq 0 ]
	echo "$output"
	[ $(expr "$output" : '.*"allowed":true.*') -ne 0 ]
 }

@test "Reject pod with denylist" {
	run kwctl run  --request-path test_data/pod_creation_volume_mounts.json --settings-path test_data/settings_reject.yaml annotated-policy.wasm
	[ "$status" -eq 0 ]
	echo "$output"
	[ $(expr "$output" : '.*"allowed":false.*') -ne 0 ]
	[ $(expr "$output" : '.*"message":.*test-var.*test-data.*') -ne 0 ]
 }
