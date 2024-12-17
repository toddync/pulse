module {
  func.func @main() -> i32 {
    // Allocate memory for two input matrices and one result matrix
    %matrixA = memref.alloc() : memref<4x4xi32>
    %matrixB = memref.alloc() : memref<4x4xi32>
    %result = memref.alloc() : memref<4x4xi32>

    // Initialize matrices A and B
    affine.for %i = 0 to 4 {
      affine.for %j = 0 to 4 {
        %valA = arith.addi %i, %j : index
        %valA_i32 = arith.index_cast %valA : index to i32
        %valB = arith.muli %i, %j : index
        %valB_i32 = arith.index_cast %valB : index to i32
        affine.store %valA_i32, %matrixA[%i, %j] : memref<4x4xi32>
        affine.store %valB_i32, %matrixB[%i, %j] : memref<4x4xi32>
      }
    }

    // Perform matrix addition
    affine.for %i = 0 to 4 {
      affine.for %j = 0 to 4 {
        %valA = affine.load %matrixA[%i, %j] : memref<4x4xi32>
        %valB = affine.load %matrixB[%i, %j] : memref<4x4xi32>
        %sum = arith.addi %valA, %valB : i32
        affine.store %sum, %result[%i, %j] : memref<4x4xi32>
      }
    }

    // Print the resulting matrix
    affine.for %i = 0 to 4 {
      affine.for %j = 0 to 4 {
        %value = affine.load %result[%i, %j] : memref<4x4xi32>
        func.call @print_i32(%value) : (i32) -> ()
      }
        func.call @println() : () -> ()
    }

    // Clean up
    memref.dealloc %matrixA : memref<4x4xi32>
    memref.dealloc %matrixB : memref<4x4xi32>
    memref.dealloc %result : memref<4x4xi32>

    // Return 0
    %zero = arith.constant 0 : i32
    func.return %zero : i32
  }

  // Helper function for printing integers
  func.func private @print_i32(%value: i32) attributes { llvm.emit_c_interface }
  func.func private @println() attributes { llvm.emit_c_interface }
}
