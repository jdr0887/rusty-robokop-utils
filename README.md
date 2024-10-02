## Welcome to Rusty Robokop Utils

### How to build
```shell
cargo build --release
```

### Generate truncated KGX files:
```shell
./target/release/ru_generate_truncated_kgx_files \
  -n <path_to_kgx_nodes_input>/nodes.tsv \
  -m <path_to_kgx_nodes_output>/nodes_truncated.tsv \
  -e <path_to_kgx_edges_input>/edges.tsv \
  -f <path_to_kgx_edges_output>/edges_truncated.tsv \
  -s 0 -o 1 -c 1000
```

### Transform bool columns to label columns:
```shell
./target/release/ru_transform_bool_columns_to_label \
  --input <path_to_kgx_nodes_input>/nodes.tsv \
  --output <path_to_kgx_nodes_output>/nodes_fixed.tsv
```
