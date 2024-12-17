module {
  func.func private @print_chr(%value: i32) attributes { llvm.emit_c_interface }
  func.func private @println() attributes { llvm.emit_c_interface }

  func.func @main() -> i32 {
    %size = arith.constant 13 : index
    %string = memref.alloc(%size) : memref<?xi32>

    %0 = arith.constant 72 : i32
    %_0 = arith.constant 0 : index
    memref.store %0, %string[%_0] : memref<? x i32>

    %1 = arith.constant 101 : i32
    %_1 = arith.constant 1 : index
    memref.store %1, %string[%_1] : memref<? x i32>

    %2 = arith.constant 108 : i32
    %_2 = arith.constant 2 : index
    memref.store %2, %string[%_2] : memref<? x i32>

    %3 = arith.constant 108 : i32
    %_3 = arith.constant 3 : index
    memref.store %3, %string[%_3] : memref<? x i32>

    %4 = arith.constant 111 : i32
    %_4 = arith.constant 4 : index
    memref.store %4, %string[%_4] : memref<? x i32>

    %5 = arith.constant 44 : i32
    %_5 = arith.constant 5 : index
    memref.store %5, %string[%_5] : memref<? x i32>

    %6 = arith.constant 32 : i32
    %_6 = arith.constant 6 : index
    memref.store %6, %string[%_6] : memref<? x i32>

    %7 = arith.constant 77 : i32
    %_7 = arith.constant 7 : index
    memref.store %7, %string[%_7] : memref<? x i32>

    %8 = arith.constant 76 : i32
    %_8 = arith.constant 8 : index
    memref.store %8, %string[%_8] : memref<? x i32>

    %9 = arith.constant 73 : i32
    %_9 = arith.constant 9 : index
    memref.store %9, %string[%_9] : memref<? x i32>

    %10 = arith.constant 82 : i32
    %_10 = arith.constant 10 : index
    memref.store %10, %string[%_10] : memref<? x i32>

    %11 = arith.constant 33 : i32
    %_11 = arith.constant 11 : index
    memref.store %11, %string[%_11] : memref<? x i32>

    %12 = arith.constant 10 : i32
    %_12 = arith.constant 12 : index
    memref.store %11, %string[%_12] : memref<? x i32>

    affine.for %i = 0 to 14 {
      %char_val = memref.load %string[%i] : memref<? x i32>
      func.call @print_chr(%char_val) : (i32) -> ()
    }

    func.call @println() : () -> ()
    %zero = arith.constant 0 : i32
    func.return %zero : i32
  }
}
