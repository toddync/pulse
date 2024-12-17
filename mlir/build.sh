./bin/mlir-opt $1.mlir \
    \
    -lower-affine \
    -convert-scf-to-cf \
    -convert-arith-to-llvm \
    -convert-async-to-llvm \
    -convert-func-to-llvm \
    -convert-cf-to-llvm \
    -convert-to-llvm \
    -reconcile-unrealized-casts \
    > $1_optimized.mlir

./bin/mlir-translate \
    --mlir-to-llvmir $1_optimized.mlir > $1.ll

clang $1.ll sidecar.c -o $1 -lm \
    -Wno-override-module

rm $1.ll $1_optimized.mlir

./$1
