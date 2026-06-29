#!/bin/bash

mod=$1

if [ -z "$mod" ]; then
  echo "Error: Debes pasar el nombre del módulo."
  exit 1
fi

mkdir -p "src/$mod"

cat <<EOF > "src/$mod/mod.rs"
pub mod dtos;
pub mod controller;
pub mod service;
pub mod repository;
EOF

# 2. Creamos los archivos individuales
touch "src/$mod/dtos.rs" \
      "src/$mod/controller.rs" \
      "src/$mod/service.rs" \
      "src/$mod/repository.rs"

echo "Módulo '$mod' creado exitosamente."