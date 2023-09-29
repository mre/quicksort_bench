.PHONY: flamegraph
flamegraph:
	sudo cargo flamegraph --release --output flamegraphs/flamegraph.svg